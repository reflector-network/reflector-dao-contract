use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum BallotStatus {
    /// Ballot has been recently created and has no votes yet
    Draft = 0,
    /// Accepted by DAO members
    Accepted = 1,
    /// Rejected by DAO members
    Rejected = 2,
    /// Retracted by the initiator
    Retracted = 3
}