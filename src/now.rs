#![cfg_attr(not(feature = "export-abi"), no_main)]

// Set up a global memory allocator using MiniAlloc for efficient memory management in the smart contract.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;
extern crate alloc;

use stylus_sdk::{alloy_primitives::U256, prelude::*};
use alloy_primitives::Address;
use stylus_sdk::storage::{StorageString, StorageVec};
use stylus_sdk::{console, block};

// Structure to hold campaign data similar to Solidity struct
sol_storage! {
    #[entrypoint]
    pub struct CrowdFunding {
        mapping(U256 => CampaignStorage) campaigns;   // Maps campaign ID to the campaign details
        U256 number_of_campaigns;                     // Tracks total number of campaigns
    }

    // Storage struct for Campaign
    struct CampaignStorage {
        Address owner;                                // Campaign owner's address
        StorageString title;                          // Title of the campaign
        StorageString description;                    // Description of the campaign
        U256 target;                                  // Target amount for the campaign
        U256 deadline;                                // Campaign deadline (timestamp)
        U256 amount_collected;                        // Total amount collected
        StorageString image;                          // Campaign image URL
        StorageVec<Address> donators;                 // List of donators' addresses
        StorageVec<U256> donations;                   // List of donation amounts
    }
}

#[external]
impl CrowdFunding {
    // Create a new campaign similar to the Solidity function
    pub fn create_campaign(
        &mut self,
        owner: Address,
        title: String,
        description: String,
        target: U256,
        deadline: U256,
        image: String,
    ) -> U256 {
        // Get the current number of campaigns
        let number_of_campaigns = self.number_of_campaigns.get();

        // Validate that the deadline is in the future
        let current_time = U256::from(block::timestamp());
        if deadline <= current_time {
            console!("Error: The deadline must be in the future.");
            return U256::from(0); // Invalid campaign, return zero
        }

        // Initialize the new campaign data
        let mut campaign_accessor = self.campaigns.setter(number_of_campaigns);
        campaign_accessor.owner.set(owner);
        campaign_accessor.title.set_str(&title);
        campaign_accessor.description.set_str(&description);
        campaign_accessor.target.set(target);
        campaign_accessor.deadline.set(deadline);
        campaign_accessor.amount_collected.set(U256::from(0));  // Initialize with zero amount collected
        campaign_accessor.image.set_str(&image);

        // Increment the number of campaigns
        self.number_of_campaigns.set(number_of_campaigns + U256::from(1));

        console!("Campaign created with ID: {:?}", number_of_campaigns);

        // Return the ID of the newly created campaign
        number_of_campaigns
    }

    // Donate to a campaign, similar to Solidity's `donateToCampaign` function
    pub fn donate_to_campaign(&mut self, campaign_id: U256, amount: U256, donator: Address) {
        let mut campaign_accessor = self.campaigns.setter(campaign_id);
        let current_time = U256::from(block::timestamp());
        let campaign_deadline = campaign_accessor.deadline.get();

        // Ensure the campaign is still active
        if current_time > campaign_deadline {
            console!("Error: Campaign has expired.");
            return;
        }

        // Add the donator's address and donation amount
        let mut donators_accessor = campaign_accessor.donators.grow();
        donators_accessor.set(donator);
        let mut donations_accessor = campaign_accessor.donations.grow();
        donations_accessor.set(amount);

        // Transfer the donation amount to the campaign owner (just log here for demo purposes)
        let owner = campaign_accessor.owner.get();
        console!("Transferred {:?} tokens to owner {:?}", amount, owner);

        // Update the total amount collected
        let current_amount = campaign_accessor.amount_collected.get();
        campaign_accessor.amount_collected.set(current_amount + amount);

        console!("Donation received: {:?} donated {:?} to campaign {:?}", donator, amount, campaign_id);
    }

    // Get a campaign's donators and their donations
    #[view]
    pub fn get_donators(&self, campaign_id: U256) -> (Vec<Address>, Vec<U256>) {
        let campaign_accessor = self.campaigns.get(campaign_id);
        let donators = campaign_accessor.donators.iter().map(|d| d.unwrap()).collect();
        let donations = campaign_accessor.donations.iter().map(|d| d.unwrap()).collect();
        (donators, donations)
    }

    // Get all campaigns, similar to Solidity's `getCampaigns` function
    #[view]
    pub fn get_campaigns(&self) -> Vec<CampaignStorage> {
        let number_of_campaigns = self.number_of_campaigns.get();
        let mut all_campaigns = Vec::new();

        for i in 0..number_of_campaigns.as_usize() {
            let campaign_accessor = self.campaigns.get(U256::from(i));
            all_campaigns.push(campaign_accessor);
        }

        all_campaigns
    }
}
