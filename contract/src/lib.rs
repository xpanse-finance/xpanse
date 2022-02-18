use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, setup_alloc, AccountId};

use crate::utils::{
    add_liquidity_util, claim_rewards, deposit_rewards_into_ref_wallet,
    swap_rewards_for_pool_tokens, withdraw_farm_rewards,
};

mod callbacks;
mod utils;
mod token_receiver;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Strategy {
    records: LookupMap<String, u128>, // Map of address -> shares in contract
    total_supply: u128 // total supply of shares issued from the contract 
}

impl Default for Strategy {
    fn default() -> Self {
        Self {
            records: LookupMap::new(b"a".to_vec()),
            total_supply: 0
        }
    }
}

#[near_bindgen]
impl Strategy {
    /// Deposit mft tokens to strategy
    /// use callback
    pub fn deposit(&mut self, sender: AccountId, amount: u128) {
        let account_id = env::signer_account_id();
        
    
        env::log(format!("Deposited amount '{}' from '{}'", amount, account_id,).as_bytes());
    }

    pub fn harvesting_step_1(&mut self) {
        if env::signer_account_id() != env::current_account_id() {
            env::log(format!("ERROR! Signer Id is not same as Contract Owner").as_bytes());
            return;
        }

        // Claim Rewards
        env::log(format!("SUCCESS! Starting Process Claim Rewards").as_bytes());
        claim_rewards();

        // Withdraw Rewards
        env::log(format!("SUCCESS! Starting Process Withdraw Rewards").as_bytes());
        withdraw_farm_rewards();

        env::log(format!("SUCCESS! Harvesting Step 1 Complete").as_bytes());
    }

    pub fn harvesting_step_2(&mut self) {
        if env::signer_account_id() != env::current_account_id() {
            env::log(format!("ERROR! Signer Id is not same as Contract Owner").as_bytes());
            return;
        }

        // Deposit Rewards into REF Wallet
        env::log(format!("SUCCESS! Starting Process Deposit Rewards into REF Wallet").as_bytes());
        deposit_rewards_into_ref_wallet();

        env::log(format!("SUCCESS! Harvesting Step 2 Complete").as_bytes());
    }

    pub fn harvesting_step_3(&mut self) {
        if env::signer_account_id() != env::current_account_id() {
            env::log(format!("ERROR! Signer Id is not same as Contract Owner").as_bytes());
            return;
        }

        // Swap rewards for Pool Tokens
        env::log(format!("SUCCESS! Starting Process Swap rewards for Pool Tokens").as_bytes());
        swap_rewards_for_pool_tokens();

        // Add Liquidity
        env::log(format!("SUCCESS! Starting Process Add Liquidity").as_bytes());

        env::log(format!("SUCCESS! Harvesting Step 3 Complete").as_bytes());
    }

    pub fn harvesting_step_4(&mut self) {
        if env::signer_account_id() != env::current_account_id() {
            env::log(format!("ERROR! Signer Id is not same as Contract Owner").as_bytes());
            return;
        }

        // Add Liquidity
        env::log(format!("SUCCESS! Starting Process Add Liquidity").as_bytes());
        add_liquidity_util();

        env::log(format!("SUCCESS! Harvesting Step 4 Complete").as_bytes());
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // #[test]
    // fn set_then_get_greeting() {
    //     let context = get_context(vec![], false);
    //     testing_env!(context);
    //     let mut contract = Welcome::default();
    //     // contract.set_greeting("howdy".to_string());
    //     assert_eq!(
    //         "howdy".to_string(),
    //         contract.get_greeting("bob_near".to_string())
    //     );
    // }

    // #[test]
    // fn get_default_greeting() {
    //     let context = get_context(vec![], true);
    //     testing_env!(context);
    //     let contract = Welcome::default();
    //     // this test did not call set_greeting so should return the default "Hello" greeting
    //     assert_eq!(
    //         "Hello".to_string(),
    //         contract.get_greeting("francis.near".to_string())
    //     );
    // }
}
