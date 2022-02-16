use crate::utils::{ext_ft, GAS_100, REF_EXCHANGE_CONTRACT_ID, TOKEN_100, YOCTO_NEAR_1};
use crate::*;
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, PromiseResult};

#[near_bindgen]
impl Welcome {
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

        if token_2_volume < 5 {
            env::log(format!("ERROR! Bad Ratio in Pool",).as_bytes());
            return "Error. Bad Ratio in Pool.".to_string();
        }

        let token_1_percentage: u128 = (token_1_volume * 100) / total_volume;
        let token_2_percentage: u128 = 100 - token_1_percentage;

        // Added Liquidity should be opposite
        let required_token_1_percentage = 100 - token_1_percentage;
        let required_token_2_percentage = 100 - token_2_percentage;
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

        let required_token_1_volume = (required_token_1_percentage * reward_amount_in_wallet) / 100;
        let required_token_2_volume = reward_amount_in_wallet - required_token_1_volume;
        env::log(
            format!(
                "SUCCESS! Required Token 1 {} and Token 2 {} Volumes",
                required_token_1_volume, required_token_2_volume
            )
            .as_bytes(),
        );

        // swap_deposit_with_tokens(required_token_1_percentage, required_token_2_percentage);
        return "Success".to_string();
    }
}
