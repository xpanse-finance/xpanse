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
    pub fn swap_rewards_for_pool_tokens_callback(&self) -> String {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");

        let token_1_volume: u128 = TOKEN_100;
        let token_2_volume: u128 = match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic(b"Unable to make comparison"),
            PromiseResult::Successful(result) => near_sdk::serde_json::from_slice::<U128>(&result)
                .unwrap()
                .into(),
        };
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
        return "Success".to_string();
    }
}
