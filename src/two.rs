// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]

// Set up a global memory allocator using MiniAlloc for efficient memory management in the smart contract.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
// Import necessary types and functions from the Stylus SDK and Alloy Primitives crates.
// These include uint256 for large integers, Address for user addresses, and various
// storage types for managing data on the blockchain.


use stylus_sdk::{alloy_primitives::U256, prelude::*};
use stylus_sdk::call::transfer_eth;
use alloy_primitives::{Address, Uint};
use stylus_sdk::{block, console, msg};
use stylus_sdk::storage::{StorageAddress, StorageString, StorageU256, StorageVec};


// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct CrowdFunding {
        uint256 no_of_campaigns;                     // Tracks total number of campaigns
        mapping(uint256 => CampaignStorage) campaigns;   // Maps campaign ID to the campaign details
    }

    // Storage struct for Campaign
    struct CampaignStorage {
        StorageAddress owner;                         // Campaign owner's address
        StorageString title;                          // Title of the campaign
        StorageString description;                    // Description of the campaign
        uint256 target;                                  // Target amount for the campaign
        uint256 deadline;                                // Campaign deadline (timestamp)
        uint256 amount_collected;                        // Total amount collected
        StorageString image;                          // Campaign image URL
        StorageVec<StorageAddress> donators;          // List of donators' addresses
        StorageVec<StorageU256> donations;                   // List of donation amounts
    }

}   
#[external]
impl CrowdFunding {
    pub fn create_campaign(
        &mut self,
        owner: Address,
        title: String,
        description: String,
        target: Uint<256, 4>,
        deadline: Uint<256, 4>,
        image: String,
    ) -> Uint<256, 4> {
        // Get the current number of campaigns
        let number_of_campaigns = self.no_of_campaigns.get();

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
        campaign_accessor.amount_collected.set(U256::from(0));
        campaign_accessor.image.set_str(&image);
        // campaign_accessor.donators.initialize();
        // campaign_accessor.donations.initialize();

        // Increment the number of campaigns
        self.no_of_campaigns.set(number_of_campaigns + U256::from(1));

        // Return the new campaign ID
        number_of_campaigns
    }

    #[payable]
    pub fn donate_to_campaign(&mut self, campaign_id: U256)  {
        // Get the campaign details
        let campaign_accessor = self.campaigns.get(campaign_id);

        // Validate that the campaign exists
        if campaign_accessor.owner.get() == Address::default() {
            console!("Error: Campaign does not exist.");
            return;
            // Err(());
        }

        // Validate that the deadline has not passed
        let current_time = U256::from(block::timestamp());
        if campaign_accessor.deadline.get() <= current_time {
            console!("Error: The deadline for this campaign has passed.");
            // Err(());
            return;
        }

        // Validate that the donation amount is greater than zero
        let donation_amount = msg::value();
        if donation_amount == U256::from(0) {
            console!("Error: Donation amount must be greater than zero.");
            // Err(());
            return;
        }

        // Update the campaign details
        // let mut campaign_accessor_mut = self.campaigns.setter(campaign_id);
        // let new_amount_collected = campaign_accessor.amount_collected.get() + donation_amount;
        // campaign_accessor_mut.amount_collected.set(new_amount_collected);
        // campaign_accessor_mut.donators.grow().set(msg::sender());
        // // campaign_accessor_mut.donations.push(donation_amount);
        // campaign_accessor_mut.donations.grow().set(donation_amount);

        console!("Donation of {:?} received for campaign ID: {:?}", donation_amount, campaign_id);

        // Transfer the donation amount to the contract
        // let transfer_result = msg::payable(Address::from(campaign_accessor.owner.get()), donation_amount);
        transfer_eth(Address::from(campaign_accessor.owner.get()), msg::value());
        // Ok(());
        return;
        

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

     // Get all campaigns, similar to Solidity's `getCampaigns` function
    //  #[view]
    //  pub fn get_campaigns(&self) -> (
    //         Vec<Address>,
    //         Vec<String>,
    //         Vec<String>,
    //         Vec<U256>,
    //         Vec<U256>,
    //         Vec<String>,
    //         Vec<Vec<Address>>,
    //         Vec<Vec<U256>>,
    //     ) {
    //         let number_of_campaigns = self.no_of_campaigns.get();
    //         let mut owners = Vec::new();
    //         let mut titles = Vec::new();
    //         let mut descriptions = Vec::new();
    //         let mut targets = Vec::new();
    //         let mut deadlines = Vec::new();
    //         let mut images = Vec::new();
    //         let mut donators = Vec::new();
    //         let mut donations = Vec::new();
    
    //         for i in 0..number_of_campaigns {
    //             let campaign_accessor = self.campaigns.get(U256::from(i));
    //             owners.push(campaign_accessor.owner.get());
    //             titles.push(campaign_accessor.title.get());
    //             descriptions.push(campaign_accessor.description.get());
    //             targets.push(campaign_accessor.target.get());
    //             deadlines.push(campaign_accessor.deadline.get());
    //             images.push(campaign_accessor.image.get());
    
    //             let mut campaign_donators = Vec::new();
    //             let mut campaign_donations = Vec::new();
    //             for j in 0..campaign_accessor.donators.len() {
    //                 if let Some(donator) = campaign_accessor.donators.get(j) {
    //                     campaign_donators.push(donator);
    //                 }
    //                 if let Some(donation) = campaign_accessor.donations.get(j) {
    //                     campaign_donations.push(donation);
    //                 }
    //             }
    //             donators.push(campaign_donators);
    //             donations.push(campaign_donations);
    //         }
    
    //         (owners, titles, descriptions, targets, deadlines, images, donators, donations)
    //     }
    //  ) {
    //      let number_of_campaigns = self.no_of_campaigns.get();
    //      let mut all_campaigns = Vec::new();
 
    //      for i in 0..number_of_campaigns.as_usize() {
    //          let campaign_accessor = self.campaigns.get(U256::from(i));
    //          all_campaigns.push(campaign_accessor);
    //      }
 
    //      all_campaigns
    //  }
}