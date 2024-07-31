use soroban_sdk::{contracttype, Address, String};

use super::{ballot_status::BallotStatus, ballot_category::BallotCategory};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]

/// Ballot registered in DAO contract
pub struct Ballot {
    /// Address of the initiator account
    pub initiator: Address,
    /// Ballot type
    pub category: BallotCategory,
    /// Short title
    pub title: String,
    /// Description text or URL
    pub description: String,
    /// Current status
    pub status: BallotStatus,
    /// Deposited DAO tokens amount
    pub deposit: i128,
    /// Creation timestamp
    pub created: u64,
}
