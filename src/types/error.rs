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
    /// Invalid ballot create parameters
    InvalidBallotParams = 4,
    /// Overflow occurred during the operation
    Overflow = 5,
    /// Operators param is invalid
    InvalidOperators = 6,
    /// Last unlock process has been executed less than a week ago
    UnlockUnavailable = 10,
    /// Proposal has been created less than two weeks ago and refund is not available yet, or the ballot has been closed
    RefundUnavailable = 11,
    /// Ballot with such ID has not been registered or expired
    BallotNotFound = 20,
    /// Ballot voting has ended and it cannot be modified
    BallotClosed = 21,
}
