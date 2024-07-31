use soroban_sdk::contracttype;


#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum BallotCategory {
    AddNode = 0,
    AddPriceFeed = 1,
    AddAsset = 2,
    General = 3
}

impl BallotCategory {
    pub fn iterator() -> impl Iterator<Item = BallotCategory> {
        [
            BallotCategory::AddNode,
            BallotCategory::AddPriceFeed,
            BallotCategory::AddAsset,
            BallotCategory::General,
        ]
        .iter()
        .copied()
    }
}