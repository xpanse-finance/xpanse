use crate::utils::{
    ext_ft, ext_ref_exchange_contract, SwapAction, GAS_100, GAS_52, LIQUIDITY_POOL_ID,
    REF_EXCHANGE_CONTRACT_ID, REWARDS_CONTRACT_IDS, REWARDS_TOKEN1_SWAP_POOLS_ID,
    REWARDS_TOKEN1_SWAP_POOLS_ID_U64, REWARDS_TOKEN2_SWAP_POOLS_ID,
    REWARDS_TOKEN2_SWAP_POOLS_ID_U64, TOKEN1_CONTRACT_ID, TOKEN2_CONTRACT_ID, TOKEN_100,
    YOCTO_NEAR_0, YOCTO_NEAR_1,
};
use crate::*;
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, PromiseResult};

#[near_bindgen]
impl Strategy {
    #[private]
    pub fn internal_deposit(&mut self, sender: AccountId, amount: u128) {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => "oops!".to_string(),
            PromiseResult::Successful(result) => {
                let balance = near_sdk::serde_json::from_slice::<U128>(&result).unwrap();
                env::log(format!("SUCCESS! Balance of {} = {:?}", env::current_account_id(), balance).as_bytes());
                if self.total_supply == 0 || balance == U128(0) {
                    let exchange_rate = 1;
                    let issue = exchange_rate * amount;
                    self.total_supply += issue;
                    self.records.insert(&sender, &(self.records.get(&sender).unwrap() + issue));
                } else {
                    let exchange_rate = balance / self.total_supply;
                    let issue = exchange_rate * amount;
                    self.total_supply += issue;
                    self.records.insert(&sender, &(self.records.get(&sender).unwrap() + issue));
                }
                
                return "Success".to_string();
            }
        }
    }

    #[private]
    pb fn mft_transfer_callback(&mut self, sender: AccountId, amount: u128, balance: u128) {
        
    }

    #[private]
    pub fn deposit_rewards_into_ref_wallet_callback(&self, reward_id: String) -> String {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => "oops!".to_string(),
            PromiseResult::Successful(result) => {
                let balance = near_sdk::serde_json::from_slice::<U128>(&result).unwrap();
                env::log(format!("SUCCESS! Balance of {} = {:?}", reward_id, balance).as_bytes());
                if balance.0 != 0 {
                    ext_ft::ft_transfer_call(
                        REF_EXCHANGE_CONTRACT_ID.to_string(),
                        balance,
                        "".to_string(),
                        &reward_id,
                        YOCTO_NEAR_1,
                        GAS_100,
                    );
                }
                return "Success".to_string();
            }
        }
    }

    #[private]
    pub fn swap_rewards_for_pool_tokens_callback(&self, reward_id: String) -> String {
        assert_eq!(
            env::promise_results_count(),
            2,
            "Did not receive Equal Callbacks"
        );

        let token_1_volume: u128 = TOKEN_100;
        let token_2_volume: u128 = match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic(b"Unable to make comparison"),
            PromiseResult::Successful(result) => near_sdk::serde_json::from_slice::<U128>(&result)
                .unwrap()
                .into(),
        };
        let total_volume: u128 = token_1_volume + token_2_volume;
        env::log(
            format!(
                "SUCCESS! If Token 1 = {} then Token 2 = {}",
                token_1_volume, token_2_volume
            )
            .as_bytes(),
        );

        if token_2_volume < 2 {
            env::log(format!("ERROR! Bad Ratio in Pool",).as_bytes());
            return "Error. Bad Ratio in Pool.".to_string();
        }

        let token_1_percentage: u128 = (token_1_volume * 100) / total_volume;
        let token_2_percentage: u128 = 100 - token_1_percentage;

        // Added Liquidity should be opposite
        let required_token_1_percentage: u128 = 100 - token_1_percentage;
        let required_token_2_percentage: u128 = 100 - token_2_percentage;
        env::log(
            format!(
                "SUCCESS! Required Token 1 {} and Token 2 {} Percentage",
                required_token_1_percentage, required_token_2_percentage
            )
            .as_bytes(),
        );

        let reward_amount_in_wallet: u128 = match env::promise_result(1) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic(b"Unable to make comparison"),
            PromiseResult::Successful(result) => near_sdk::serde_json::from_slice::<U128>(&result)
                .unwrap()
                .into(),
        };
        env::log(
            format!(
                "SUCCESS! Reward Amount in Wallet {}",
                reward_amount_in_wallet
            )
            .as_bytes(),
        );

        let required_token_1_volume: u128 =
            (required_token_1_percentage * reward_amount_in_wallet) / 100;
        let required_token_2_volume: u128 = reward_amount_in_wallet - required_token_1_volume;
        env::log(
            format!(
                "SUCCESS! Required Token 1 {} and Token 2 {} Volumes",
                required_token_1_volume, required_token_2_volume
            )
            .as_bytes(),
        );

        let mut index = 0;
        for temp_reward in REWARDS_CONTRACT_IDS {
            if temp_reward == reward_id {
                break;
            }
            index = index + 1;
        }

        if REWARDS_TOKEN1_SWAP_POOLS_ID[index] && required_token_1_volume != 0 {
            let swap_details_1 = SwapAction {
                pool_id: REWARDS_TOKEN1_SWAP_POOLS_ID_U64[index],
                token_in: reward_id.clone(),
                amount_in: required_token_1_volume.to_string(),
                token_out: TOKEN1_CONTRACT_ID.to_string(),
                min_amount_out: "0".to_string(),
            };
            ext_ref_exchange_contract::swap(
                vec![swap_details_1],
                &REF_EXCHANGE_CONTRACT_ID,
                YOCTO_NEAR_0,
                GAS_52,
            );
        }

        if REWARDS_TOKEN2_SWAP_POOLS_ID[index] && required_token_2_volume != 0 {
            let swap_details_2 = SwapAction {
                pool_id: REWARDS_TOKEN2_SWAP_POOLS_ID_U64[index],
                token_in: reward_id.clone(),
                amount_in: required_token_2_volume.to_string(),
                token_out: TOKEN2_CONTRACT_ID.to_string(),
                min_amount_out: "0".to_string(),
            };
            ext_ref_exchange_contract::swap(
                vec![swap_details_2],
                &REF_EXCHANGE_CONTRACT_ID,
                YOCTO_NEAR_0,
                GAS_52,
            );
        }
        return "Success".to_string();
    }

    #[private]
    pub fn add_liquidity_util_callback(&self) -> String {
        assert_eq!(
            env::promise_results_count(),
            2,
            "Did not receive Equal Callbacks for add_liquidity_util_callback"
        );

        let token_1_amount_in_wallet: u128 = match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic(b"Unable to make comparison"),
            PromiseResult::Successful(result) => near_sdk::serde_json::from_slice::<U128>(&result)
                .unwrap()
                .into(),
        };
        let token_2_amount_in_wallet: u128 = match env::promise_result(1) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic(b"Unable to make comparison"),
            PromiseResult::Successful(result) => near_sdk::serde_json::from_slice::<U128>(&result)
                .unwrap()
                .into(),
        };
        env::log(
            format!(
                "SUCCESS! Token 1 in wallet = {}, Token 2 in wallet = {}",
                token_1_amount_in_wallet, token_2_amount_in_wallet
            )
            .as_bytes(),
        );

        if token_1_amount_in_wallet > 1000000000000 && token_2_amount_in_wallet > 1000000000000 {
            ext_ref_exchange_contract::add_liquidity(
                LIQUIDITY_POOL_ID,
                vec![
                    token_1_amount_in_wallet.to_string(),
                    token_2_amount_in_wallet.to_string(),
                ],
                &REF_EXCHANGE_CONTRACT_ID,
                YOCTO_NEAR_1,
                GAS_100,
            );
        }
        return "Success".to_string();
    }
}
