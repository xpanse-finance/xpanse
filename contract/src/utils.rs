use near_sdk::{env, ext_contract};

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
fn get_all_staked_seeds() -> Vec<String> {
    let mut staked_seeds: Vec<String> = Vec::new();
    // Do a cross-contract Call
    env::log(format!("SUCCESS! Fetched all Staked Seeds").as_bytes());
    return staked_seeds;
}

fn claim_using_seeds(seed_id: String) -> bool {
    let mut harvesting_status = true;

    if !harvesting_status {
        env::log(format!("ERROR! Not all were successfull").as_bytes());
    }

    return harvesting_status;
}

pub fn claim_rewards() -> bool {
    let mut harvesting_status = true;

    ext_ref_farming_contract::claim_reward_by_seed(
        "exchange.ref-dev.testnet@5".to_string(),
        &REF_FARMING_CONTRACT_ID,
        0,
        100_000_000_000_000,
    );

    let staked_seeds: Vec<String> = get_all_staked_seeds();
    if staked_seeds.len() == 0 {
        env::log(format!("ERROR! Length of Staked Seeds is Zero").as_bytes());
        return false;
    }

    for seed_id in staked_seeds {
        harvesting_status = harvesting_status && claim_using_seeds(seed_id);
    }

    return harvesting_status;
}
