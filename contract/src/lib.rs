use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env, near_bindgen, setup_alloc, AccountId};
use std::convert::TryFrom;

use crate::utils::{
    add_liquidity_util, claim_rewards, deposit_rewards_into_ref_wallet, ext_ref_exchange_contract,
    ext_ref_farming_contract, ext_self, necessary_swaps_required_util,
    swap_rewards_for_pool_tokens, withdraw_farm_rewards, GAS_10, GAS_100, GAS_160,
    REF_EXCHANGE_CONTRACT_ID, REF_FARMING_CONTRACT_ID, TOKEN_ID, YOCTO_NEAR_0, YOCTO_NEAR_1,
};

mod callbacks;
mod token_receiver;
mod utils;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Strategy {
    records: LookupMap<String, u128>, // Map of address -> shares in contract
    total_supply: u128,               // total supply of shares issued from the contract
    claim: LookupMap<String, u128>,
    to_deposit: u128,
    to_claim: u128,
}

impl Default for Strategy {
    fn default() -> Self {
        Self {
            records: LookupMap::new(b"a".to_vec()),
            claim: LookupMap::new(b"aa".to_vec()),
            total_supply: 0,
            to_deposit: 0,
            to_claim: 0,
        }
    }
}

#[near_bindgen]
impl Strategy {
    /// Deposit mft tokens to strategy
    /// use callback
    pub(crate) fn deposit(&mut self, sender: AccountId, amount: u128) {
        // exchange rate = balance of mft / total_supply
        env::log(format!("Deposited amount entered!").as_bytes());
        ext_ref_farming_contract::list_user_seeds(
            ValidAccountId::try_from(env::current_account_id()).unwrap(),
            &REF_FARMING_CONTRACT_ID,
            YOCTO_NEAR_0,
            GAS_10,
        )
        .then(ext_self::internal_deposit(
            sender.clone(),
            amount,
            &env::current_account_id(),
            YOCTO_NEAR_0,
            GAS_160,
        ));
        env::log(format!("Deposited amount '{}' from '{}'", amount, sender).as_bytes());
    }

    pub fn deposit_to_farm(&mut self) {
        ext_ref_exchange_contract::mft_balance_of(
            TOKEN_ID.to_string(),
            ValidAccountId::try_from(env::current_account_id()).unwrap(),
            &REF_EXCHANGE_CONTRACT_ID,
            YOCTO_NEAR_0,
            20_000_000_000_000,
        )
        .then(ext_self::internal_deposit_to_farm(
            &env::current_account_id(),
            YOCTO_NEAR_0,
            220_000_000_000_000,
        ));
    }

    pub fn withdraw(&mut self, amount: U128) {
        let sender = env::signer_account_id();
        if self.records.get(&sender).unwrap() < amount.into() {
            env::panic(format!("Not enough balance!").as_bytes());
        }
        // exchange rate = balance of mft / total_supply
        ext_ref_farming_contract::list_user_seeds(
            ValidAccountId::try_from(env::current_account_id()).unwrap(),
            &REF_FARMING_CONTRACT_ID,
            YOCTO_NEAR_0,
            GAS_10,
        )
        .then(ext_self::internal_withdraw(
            sender.clone(),
            amount.into(),
            &env::current_account_id(),
            YOCTO_NEAR_0,
            260_000_000_000_000,
        ));
        env::log(format!("Withdraw amount '{:?}' from '{:?}'", amount, sender).as_bytes());
    }

    pub fn claim(&mut self) {
        let sender = env::signer_account_id();
        let claim_val = self.claim.get(&sender);
        let mut res = 0;
        if claim_val != None {
            res = claim_val.unwrap();
        }

        if res > 0 {
            ext_ref_exchange_contract::mft_transfer(
                TOKEN_ID.to_string(),
                ValidAccountId::try_from(sender.clone()).unwrap(),
                res.into(),
                None,
                &REF_EXCHANGE_CONTRACT_ID,
                YOCTO_NEAR_1,
                GAS_100,
            )
            .then(ext_self::post_claim(
                sender.clone(),
                res,
                &env::current_account_id(),
                YOCTO_NEAR_0,
                GAS_160,
            ));
        }
        env::log(format!("Claimed amount '{:?}' to '{:?}'", res, sender).as_bytes());
    }

    pub fn harvesting_step_1(&mut self) {
        // Claim Rewards
        env::log(format!("SUCCESS! Starting Process Claim Rewards").as_bytes());
        claim_rewards();

        // Withdraw Rewards
        env::log(format!("SUCCESS! Starting Process Withdraw Rewards").as_bytes());
        withdraw_farm_rewards();

        env::log(format!("SUCCESS! Harvesting Step 1 Complete").as_bytes());
    }

    pub fn harvesting_step_2(&mut self) {
        // Deposit Rewards into REF Wallet
        env::log(format!("SUCCESS! Starting Process Deposit Rewards into REF Wallet").as_bytes());
        deposit_rewards_into_ref_wallet();

        env::log(format!("SUCCESS! Harvesting Step 2 Complete").as_bytes());
    }

    pub fn harvesting_step_3(&mut self) {
        // Necessary Swaps Required in case Rewards does have direct pool with Required Tokens
        env::log(format!("SUCCESS! Starting Process Necessary Swaps").as_bytes());
        necessary_swaps_required_util();

        env::log(format!("SUCCESS! Harvesting Step 3 Complete").as_bytes());
    }

    pub fn harvesting_step_4(&mut self) {
        // Swap rewards for Pool Tokens
        env::log(format!("SUCCESS! Starting Process Swap rewards for Pool Tokens").as_bytes());
        swap_rewards_for_pool_tokens();

        // Add Liquidity
        env::log(format!("SUCCESS! Starting Process Add Liquidity").as_bytes());

        env::log(format!("SUCCESS! Harvesting Step 4 Complete").as_bytes());
    }

    pub fn harvesting_step_5(&mut self) {
        // Add Liquidity
        env::log(format!("SUCCESS! Starting Process Add Liquidity").as_bytes());
        add_liquidity_util();

        env::log(format!("SUCCESS! Harvesting Step 5 Complete").as_bytes());
    }

    pub fn harvesting_step_6(&mut self) {
        // Deposit to Farm
        env::log(format!("SUCCESS! Deposit to Farm").as_bytes());
        ext_self::deposit_to_farm(
            &env::current_account_id(),
            YOCTO_NEAR_0,
            275_000_000_000_000,
        );

        env::log(format!("SUCCESS! Harvesting Step 6 Complete").as_bytes());
    }
}
