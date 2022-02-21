use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, ext_contract};
use uint::construct_uint;

pub const MFT_TAG: &str = "@";

// Yocto Near
pub const YOCTO_NEAR_0: u128 = 0;
pub const YOCTO_NEAR_1: u128 = 1;

// Gas
pub const GAS_5: u64 = 5_000_000_000_000;
pub const GAS_10: u64 = 10_000_000_000_000;
pub const GAS_40: u64 = 40_000_000_000_000;
pub const GAS_52: u64 = 52_000_000_000_000;
pub const GAS_100: u64 = 100_000_000_000_000;
pub const GAS_120: u64 = 120_000_000_000_000;
pub const GAS_160: u64 = 160_000_000_000_000;
pub const GAS_200: u64 = 200_000_000_000_000;

// Token Amount
pub const TOKEN_100: u128 = 100;

// Seeds
pub const STAKED_SEEDS: &str = "exchange.ref-dev.testnet@107";

pub const TOKEN_ID: &str = ":107";

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct u256(4);
}

// Contracts
pub const TOKEN1_CONTRACT_ID: &str = "eth.fakes.testnet";
pub const TOKEN2_CONTRACT_ID: &str = "aurora.fakes.testnet";
pub const REWARDS_CONTRACT_IDS: [&str; 2] = ["ref.fakes.testnet", "paras.fakes.testnet"];
pub const REF_FARMING_CONTRACT_ID: &str = "farm110.ref-dev.testnet";
pub const REF_EXCHANGE_CONTRACT_ID: &str = "exchange.ref-dev.testnet";

// Pools ( False means no pool exist)
pub const LIQUIDITY_POOL_ID: u64 = 107;

pub const REWARDS_SWAPPED_CONTRACT_IDS: [&str; 1] = ["ref.fakes.testnet"];
pub const REWARDS_TOKEN1_SWAP_POOLS_ID: [bool; 1] = [true];
pub const REWARDS_TOKEN1_SWAP_POOLS_ID_U64: [u64; 1] = [321];
pub const REWARDS_TOKEN2_SWAP_POOLS_ID: [bool; 1] = [true];
pub const REWARDS_TOKEN2_SWAP_POOLS_ID_U64: [u64; 1] = [20];

pub const NECESSARY_SWAPS_REQUIRED: [[&str; 3]; 1] =
    // Token In, Token Out, Pool ID
    [["paras.fakes.testnet", "ref.fakes.testnet", "5"]];

pub(crate) type SeedId = String;
// Traits
#[ext_contract(ext_ref_farming_contract)]
trait RefFarmingContract {
    fn claim_reward_by_seed(&mut self, seed_id: String);
    fn withdraw_reward(&mut self, token_id: String, amount: String);

    fn withdraw_seed(&mut self, seed_id: SeedId, amount: U128);
    fn list_user_seeds(&self, account_id: ValidAccountId) -> HashMap<SeedId, U128>;
}

#[ext_contract(ext_ref_exchange_contract)]
trait RefExchangeContract {
    fn get_return(
        &self,
        pool_id: u64,
        token_in: String,
        amount_in: String,
        token_out: String,
    ) -> U128;
    fn get_deposit(&self, account_id: String, token_id: String) -> U128;
    fn swap(&mut self, actions: Vec<SwapAction>) -> U128;
    fn add_liquidity(&mut self, pool_id: u64, amounts: Vec<String>);

    // MFT
    fn mft_balance_of(&self, token_id: String, account_id: ValidAccountId) -> U128;
    fn mft_transfer_call(
        &mut self,
        token_id: String,
        receiver_id: ValidAccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128>;
    fn mft_transfer(
        &mut self,
        token_id: String,
        receiver_id: ValidAccountId,
        amount: U128,
        memo: Option<String>,
    );
}

#[ext_contract(ext_ft)]
pub trait FungibleToken {
    fn ft_balance_of(&mut self, account_id: AccountId) -> U128;
    fn ft_transfer_call(&mut self, receiver_id: AccountId, amount: U128, msg: String);
}

#[ext_contract(ext_self)]
pub trait Strategy {
    fn deposit_rewards_into_ref_wallet_callback(&self, reward_id: String) -> String;
    fn swap_rewards_for_pool_tokens_callback(&self, reward_id: String) -> String;
    fn necessary_swaps_required_util_callback(
        &self,
        token_in: String,
        token_out: String,
        given_pool_id: String,
    ) -> String;
    fn add_liquidity_util_callback(&self) -> String;
    fn internal_deposit(&mut self, sender: AccountId, amount: u128);
    fn post_mft_transfer(&mut self) -> String;
    fn internal_withdraw(&mut self, sender: AccountId, amount: u128);
    fn post_withdraw_seed(&mut self, sender: AccountId, amount: u128, issue: u128);
    fn internal_deposit_to_farm(&mut self) -> String;
    fn deposit_to_farm(&mut self) -> String;
    fn post_claim(&mut self, sender: AccountId, res: u128) -> String;
}

// Copied from RefFinance Code
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SwapAction {
    /// Pool which should be used for swapping.
    pub pool_id: u64,
    /// Token to swap from.
    pub token_in: String,
    /// Amount to exchange.
    /// If amount_in is None, it will take amount_out from previous step.
    /// Will fail if amount_in is None on the first step.
    pub amount_in: String,
    /// Token to swap into.
    pub token_out: String,
    /// Required minimum amount of token_out.
    pub min_amount_out: String,
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

// Deposit Rewards into REF Wallet
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
                YOCTO_NEAR_0,
                GAS_120,
            ),
        );
    }
}

// Swap Rewards for Pool Tokens
pub fn swap_rewards_for_pool_tokens() {
    for reward_id in REWARDS_SWAPPED_CONTRACT_IDS {
        ext_ref_exchange_contract::get_return(
            LIQUIDITY_POOL_ID,
            TOKEN1_CONTRACT_ID.to_string(),
            TOKEN_100.to_string(),
            TOKEN2_CONTRACT_ID.to_string(),
            &REF_EXCHANGE_CONTRACT_ID,
            YOCTO_NEAR_0,
            GAS_5,
        )
        .and(ext_ref_exchange_contract::get_deposit(
            env::current_account_id(),
            reward_id.to_string(),
            &REF_EXCHANGE_CONTRACT_ID,
            YOCTO_NEAR_0,
            GAS_5,
        ))
        .then(ext_self::swap_rewards_for_pool_tokens_callback(
            reward_id.to_string(),
            &env::current_account_id(),
            YOCTO_NEAR_0,
            GAS_120,
        ));
    }
}

// Add Liquidity
pub fn add_liquidity_util() {
    ext_ref_exchange_contract::get_deposit(
        env::current_account_id(),
        TOKEN1_CONTRACT_ID.to_string(),
        &REF_EXCHANGE_CONTRACT_ID,
        YOCTO_NEAR_0,
        GAS_5,
    )
    .and(ext_ref_exchange_contract::get_deposit(
        env::current_account_id(),
        TOKEN2_CONTRACT_ID.to_string(),
        &REF_EXCHANGE_CONTRACT_ID,
        YOCTO_NEAR_0,
        GAS_5,
    ))
    .then(ext_self::add_liquidity_util_callback(
        &env::current_account_id(),
        YOCTO_NEAR_0,
        GAS_200,
    ));
}

pub fn necessary_swaps_required_util() {
    for nec_swaps in NECESSARY_SWAPS_REQUIRED {
        ext_ref_exchange_contract::get_deposit(
            env::current_account_id(),
            nec_swaps[0].to_string(), // Token In
            &REF_EXCHANGE_CONTRACT_ID,
            YOCTO_NEAR_0,
            GAS_5,
        )
        .then(ext_self::necessary_swaps_required_util_callback(
            nec_swaps[0].to_string(), // Token In
            nec_swaps[1].to_string(), // Token Out
            nec_swaps[2].to_string(), // Pool Id
            &env::current_account_id(),
            YOCTO_NEAR_0,
            GAS_160,
        ));
    }
}
