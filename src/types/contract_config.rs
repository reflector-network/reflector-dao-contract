use soroban_sdk::{contracttype, Address};

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
}
