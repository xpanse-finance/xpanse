use crate::utils::{ext_ft, GAS_100, REF_EXCHANGE_CONTRACT_ID, YOCTO_NEAR_1};
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
}
