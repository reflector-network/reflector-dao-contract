use soroban_sdk::contracttype;


#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum BallotCategory {
    AddNode = 0,
    AddPriceFeed = 1,
    AddAsset = 2,
    General = 3
}