use near_sdk::json_types::U128;
use near_sdk::{env, ext_contract};

// Yocto Near
pub const YOCTO_NEAR_0: u128 = 0;
pub const YOCTO_NEAR_1: u128 = 1;

// Gas
pub const GAS_5: u64 = 5_000_000_000_000;
pub const GAS_40: u64 = 40_000_000_000_000;
pub const GAS_100: u64 = 100_000_000_000_000;
pub const GAS_200: u64 = 200_000_000_000_000;
pub const GAS_300: u64 = 300_000_000_000_000;

// Seeds
pub const STAKED_SEEDS: &str = "exchange.ref-dev.testnet@5";

// Contracts
pub const TOKEN1_CONTRACT_ID: &str = "paras.fakes.testnet";
pub const TOKEN2_CONTRACT_ID: &str = "ref.fakes.testnet";
pub const REWARDS_CONTRACT_IDS: [&str; 2] = ["ref.fakes.testnet", "pulse.fakes.testnet"];
pub const REF_FARMING_CONTRACT_ID: &str = "farm110.ref-dev.testnet";
pub const REF_EXCHANGE_CONTRACT_ID: &str = "exchange.ref-dev.testnet";

// Pools ( -1 means no pool exist)
pub const LIQUIDITY_POOL_ID: u32 = 5;
pub const REWARDS_TOKEN1_SWAP_POOLS_ID: [&i32; 2] = [&5, &6];
pub const REWARDS_TOKEN2_SWAP_POOLS_ID: [&i32; 2] = [&-1, &290];

// Traits
#[ext_contract(ext_ref_farming_contract)]
trait RefFarmingContract {
    fn claim_reward_by_seed(&mut self, seed_id: String);
    fn withdraw_reward(&mut self, token_id: String, amount: String);
}

#[ext_contract(ext_ft)]
pub trait FungibleToken {
    fn ft_balance_of(&mut self, account_id: AccountId) -> U128;
    fn ft_transfer_call(&mut self, receiver_id: AccountId, amount: U128, msg: String);
}

#[ext_contract(ext_self)]
pub trait MyContract {
    // fn generic_callback(&self, util_name: String) -> String;
    fn deposit_rewards_into_ref_wallet_callback(&self, reward_id: String) -> String;
}

// Claim Rewards
pub fn claim_rewards() {
    ext_ref_farming_contract::claim_reward_by_seed(
        STAKED_SEEDS.to_string(),
        &REF_FARMING_CONTRACT_ID,
        YOCTO_NEAR_0,
        GAS_100,
    );
}

// Withdraw Rewards
pub fn withdraw_farm_rewards() {
    for reward_id in REWARDS_CONTRACT_IDS {
        env::log(format!("SUCCESS! Withdrawing rewards for {}", reward_id).as_bytes());
        ext_ref_farming_contract::withdraw_reward(
            reward_id.to_string(),
            "0".to_string(),
            &REF_FARMING_CONTRACT_ID,
            YOCTO_NEAR_1,
            GAS_40,
        );
    }
}

pub fn deposit_rewards_into_ref_wallet() {
    for reward_id in REWARDS_CONTRACT_IDS {
        env::log(
            format!(
                "SUCCESS! Depositing rewards into Ref Wallet - {}",
                reward_id
            )
            .as_bytes(),
        );

        ext_ft::ft_balance_of(env::current_account_id(), &reward_id, YOCTO_NEAR_0, GAS_5).then(
            ext_self::deposit_rewards_into_ref_wallet_callback(
                reward_id.to_string(),
                &env::current_account_id(),
                YOCTO_NEAR_1,
                GAS_100,
            ),
        );
    }
}
