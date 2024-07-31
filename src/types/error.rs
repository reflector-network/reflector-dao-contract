use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// Contract-specific error codes
pub enum Error {
    /// The contract is not initialized.
    NotInitialized = 0,
    /// Contract has been already initialized earlier
    AlreadyInitialized = 1,
    /// Caller is not authorized to perform the operation
    Unauthorized = 2,
    /// Operation amount is invalid
    InvalidAmount = 3,
    /// Last unlock process has been executed less than a week ago
    UnlockUnavailable = 10,
    /// Proposal has been created less than two weeks ago and refund is not available yet, or the ballot has been closed
    RefundUnavailable = 11,
    /// Ballot with such ID has not been registered or expired
    BallotNotFound = 20,
    /// Ballot voting has ended and it cannot be modified
    BallotClosed = 21,
}
