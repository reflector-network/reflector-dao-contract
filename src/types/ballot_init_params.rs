use soroban_sdk::{contracttype, Address, String};

use super::ballot_category::BallotCategory;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]

/// New ballot initialization parameters
pub struct BallotInitParams {
    /// Initiator account address
    pub initiator: Address,
    /// Ballot type
    pub category: BallotCategory,
    /// Short title
    pub title: String,
    /// Description text or URL
    pub description: String,
}
