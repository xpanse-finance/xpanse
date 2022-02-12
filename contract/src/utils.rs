use near_sdk::env;

// Pools
pub const DAI_ETH_POOL_ID: u32 = 193;
pub const REF_ETH_POOL_ID: u32 = 321;
pub const DAI_REF_POOL_ID: u32 = 326;

// Contracts
pub const REF_FARMING_CONTRACT: &str = "farm110.ref-dev.testnet";
pub const REF_EXCHANGE_CONTRACT: &str = "exchange.ref-dev.testnet";
pub const REF_TOKEN_CONTRACT: &str = "ref.fakes.testnet";
pub const DAI_TOKEN_CONTRACT: &str = "dai.fakes.testnet";
pub const ETH_TOKEN_CONTRACT: &str = "eth.fakes.testnet";

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
