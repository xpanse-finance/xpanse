use near_sdk::{env, ext_contract};

// Seeds
pub const STAKED_SEEDS: &str = "exchange.ref-dev.testnet@5";

// Contracts
pub const TOKEN1_CONTRACT_ID: &str = "paras.fakes.testnet";
pub const TOKEN2_CONTRACT_ID: &str = "ref.fakes.testnet";
pub const REWARDS_CONTRACT_IDS: (&str, &str) = ("ref.fakes.testnet", "pulse.fakes.testnet");
pub const REF_FARMING_CONTRACT_ID: &str = "farm110.ref-dev.testnet";
pub const REF_EXCHANGE_CONTRACT_ID: &str = "exchange.ref-dev.testnet";

// Pools ( -1 means no pool exist)
pub const LIQUIDITY_POOL_ID: u32 = 5;
pub const REWARDS_TOKEN1_SWAP_POOLS_ID: (&i32, &i32) = (&5, &6);
pub const REWARDS_TOKEN2_SWAP_POOLS_ID: (&i32, &i32) = (&-1, &290);

#[ext_contract(ext_ref_farming_contract)]
trait RefFarmingContract {
    fn claim_reward_by_seed(&mut self, seed_id: String);
}

// Claim Rewards
pub fn claim_rewards() {
    ext_ref_farming_contract::claim_reward_by_seed(
        STAKED_SEEDS.to_string(),
        &REF_FARMING_CONTRACT_ID,
        0,
        100_000_000_000_000,
    );
}
