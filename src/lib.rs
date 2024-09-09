// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]

// Set up a global memory allocator using MiniAlloc for efficient memory management in the smart contract.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

extern crate alloc;

use stylus_sdk::{alloy_primitives::U256, prelude::*, evm, alloy_sol_types::sol};
use stylus_sdk::call::transfer_eth;
use alloy_primitives::{Address, Uint};
use stylus_sdk::{block, console, msg};
use stylus_sdk::storage::{StorageAddress, StorageString, StorageU256, StorageVec};

sol_storage! {
    #[entrypoint]
    pub struct CrowdFunding {
        uint256 no_of_campaigns;
        mapping(uint256 => CampaignStorage) campaigns;
    }

    struct CampaignStorage {
        StorageAddress owner;
        StorageString title;
        StorageString description;
        uint256 target;
        uint256 deadline;
        uint256 amount_collected;
        StorageString image;
        StorageVec<StorageAddress> donators;
        StorageVec<StorageU256> donations;
    }
}

sol! {
    event CampaignCreated(uint256 indexed campaignId, address owner, string title, uint256 target, uint256 deadline);
    event DonationMade(uint256 indexed campaignId, address donor, uint256 amount);
}

#[external]
impl CrowdFunding {
    pub fn create_campaign(
        &mut self,
        owner: Address,
        title: String,
        description: String,
        target: U256,
        deadline: U256,
        image: String,
    ) -> U256 {
        let number_of_campaigns = self.no_of_campaigns.get();

        let current_time = U256::from(block::timestamp());
        if deadline <= current_time {
            console!("Error: The deadline must be in the future.");
            return U256::from(0);
        }

        let mut campaign_accessor = self.campaigns.setter(number_of_campaigns);

        campaign_accessor.owner.set(owner);
        campaign_accessor.title.set_str(&title);
        campaign_accessor.description.set_str(&description);
        campaign_accessor.target.set(target);
        campaign_accessor.deadline.set(deadline);
        campaign_accessor.amount_collected.set(U256::from(0));
        campaign_accessor.image.set_str(&image);
        // StorageVec is automatically initialized, no need to call initialize()

        self.no_of_campaigns.set(number_of_campaigns + U256::from(1));

          // Emit CampaignCreated event
          evm::log(CampaignCreated {
            campaignId: number_of_campaigns,
            owner,
            title,
            target,
            deadline,
        });

        number_of_campaigns
    }

    #[payable]
    pub fn donate_to_campaign(&mut self, campaign_id: U256) {
        let mut campaign_accessor = self.campaigns.setter(campaign_id);

        if campaign_accessor.owner.get() == Address::default() {
            console!("Error: Campaign does not exist.");
            return;
        }

        let current_time = U256::from(block::timestamp());
        if campaign_accessor.deadline.get() <= current_time {
            console!("Error: The deadline for this campaign has passed.");
            return;
        }

        let donation_amount = msg::value();
        if donation_amount == U256::from(0) {
            console!("Error: Donation amount must be greater than zero.");
            return;
        }

        let new_amount_collected = campaign_accessor.amount_collected.get() + donation_amount;
        campaign_accessor.amount_collected.set(new_amount_collected);
        campaign_accessor.donators.push(msg::sender());
        campaign_accessor.donations.push(donation_amount);

        console!("Donation of {:?} received for campaign ID: {:?}", donation_amount, campaign_id);

        if let Err(_) = transfer_eth(campaign_accessor.owner.get(), msg::value()) {
            console!("Error: Failed to transfer ETH to campaign owner.");
        }

        // Emit DonationMade event
        evm::log(DonationMade {
            campaignId: campaign_id,
            donor: msg::sender(),
            amount: donation_amount,
        });
    }

    #[view]
    pub fn get_donators(&self, campaign_id: U256) -> (Vec<Address>, Vec<U256>) {
        let campaign_accessor = self.campaigns.get(campaign_id);
        let mut donators = Vec::new();
        let mut donations = Vec::new();

        for i in 0..campaign_accessor.donators.len() {
            if let Some(donator) = campaign_accessor.donators.get(i) {
                donators.push(donator);
            }
            if let Some(donation) = campaign_accessor.donations.get(i) {
                donations.push(donation);
            }
        }
        (donators, donations)
    }

    #[view]
    pub fn get_campaigns(&self) -> (
        Vec<Address>,
        Vec<String>,
        Vec<String>,
        Vec<U256>,
        Vec<U256>,
        Vec<String>,
        Vec<Vec<Address>>,
        Vec<Vec<U256>>,
    ) {
        let number_of_campaigns = self.no_of_campaigns.get();
        let mut owners = Vec::new();
        let mut titles = Vec::new();
        let mut descriptions = Vec::new();
        let mut targets = Vec::new();
        let mut deadlines = Vec::new();
        let mut images = Vec::new();
        let mut donators = Vec::new();
        let mut donations = Vec::new();

        for i in 0..number_of_campaigns.as_limbs()[0] {
            let campaign_accessor = self.campaigns.get(U256::from(i));
            owners.push(campaign_accessor.owner.get());
            titles.push(campaign_accessor.title.get_string());
            descriptions.push(campaign_accessor.description.get_string());
            targets.push(campaign_accessor.target.get());
            deadlines.push(campaign_accessor.deadline.get());
            images.push(campaign_accessor.image.get_string());

            let mut campaign_donators = Vec::new();
            let mut campaign_donations = Vec::new();
            for j in 0..campaign_accessor.donators.len() {
                if let Some(donator) = campaign_accessor.donators.get(j) {
                    campaign_donators.push(donator);
                }
                if let Some(donation) = campaign_accessor.donations.get(j) {
                    campaign_donations.push(donation);
                }
            }
            donators.push(campaign_donators);
            donations.push(campaign_donations);
        }

        (owners, titles, descriptions, targets, deadlines, images, donators, donations)
    }
}