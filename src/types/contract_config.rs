use soroban_sdk::{contracttype, Address, Map};

use super::ballot_category::BallotCategory;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]

/// DAO contract configuration parameters
pub struct ContractConfig {
    /// Admin account address
    pub admin: Address,
    /// DAO token address
    pub token: Address,
    /// Initial funding amount
    pub amount: i128,
    /// Initial deposit amounts for each ballot category
    pub deposit_params: Map<BallotCategory, i128>,
    /// DAO start date
    pub start_date: u64
}
