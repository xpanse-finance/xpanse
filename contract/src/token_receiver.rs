use crate::utils::{MFT_TAG, STAKED_SEEDS};
use crate::*;
use near_sdk::json_types::U128;
use near_sdk::{AccountId, PromiseOrValue};

pub trait MFTTokenReceiver {
    fn mft_on_transfer(
        &mut self,
        token_id: String,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

enum TokenOrPool {
    Token(AccountId),
    Pool(u64),
}

/// a sub token would use a format ":<u64>"
fn try_identify_sub_token_id(token_id: &String) -> Result<u64, &'static str> {
    if token_id.starts_with(":") {
        if let Ok(pool_id) = str::parse::<u64>(&token_id[1..token_id.len()]) {
            Ok(pool_id)
        } else {
            Err("Illegal pool id")
        }
    } else {
        Err("Illegal pool id")
    }
}

fn parse_token_id(token_id: String) -> TokenOrPool {
    if let Ok(pool_id) = try_identify_sub_token_id(&token_id) {
        TokenOrPool::Pool(pool_id)
    } else {
        TokenOrPool::Token(token_id)
    }
}

/// lp token deposit
#[near_bindgen]
impl MFTTokenReceiver for Strategy {
    /// Callback on receiving tokens by this contract.
    fn mft_on_transfer(
        &mut self,
        token_id: String,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        // write deposit logic here
        let seed_id: String;
        match parse_token_id(token_id.clone()) {
            TokenOrPool::Pool(pool_id) => {
                seed_id = format!("{}{}{}", env::predecessor_account_id(), MFT_TAG, pool_id);
            }
            TokenOrPool::Token(_) => {
                // for seed deposit, using mft to transfer 'root' token is not supported.
                env::panic(format!("ILLEGAL TOKEN ID").as_bytes());
            }
        }

        if seed_id != STAKED_SEEDS {
            env::panic(format!("SEED IS WRONG!!").as_bytes())
        }

        assert!(msg.is_empty(), "ERR_MSG_INCORRECT");

        // if seed not exist, it will panic
        let amount: u128 = amount.into();
        self.deposit(sender_id.clone(), amount);
        // let seed_farm = self.get_seed(&seed_id);
        // if amount < seed_farm.get_ref().min_deposit {
        //     env::panic(
        //         format!(
        //             "{} {}",
        //             ERR34_BELOW_MIN_SEED_DEPOSITED,
        //             seed_farm.get_ref().min_deposit
        //         )
        //         .as_bytes()
        //     )
        // }

        // self.internal_seed_deposit(&seed_id, &sender_id, amount, SeedType::MFT);

        // self.assert_storage_usage(&sender_id);

        env::log(
            format!(
                "{} deposit MFT seed {} with amount {}.",
                sender_id, seed_id, amount,
            )
            .as_bytes(),
        );

        PromiseOrValue::Value(U128(0))
    }
}
