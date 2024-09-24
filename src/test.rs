#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{storage::Persistent, Address as _, Ledger, LedgerInfo},
    token::StellarAssetClient,
    vec, Env, String,
};

fn init_contract_with_admin<'a>() -> (Env, DAOContractClient<'a>, ContractConfig) {
    let env = Env::default();

    let admin = Address::generate(&env);

    let contract_id = env.register_contract(None, DAOContract);
    let client: DAOContractClient<'a> = DAOContractClient::new(&env, &contract_id);

    let token = env.register_stellar_asset_contract(admin.clone());

    env.mock_all_auths();

    // extend ttl for the contract and token
    env.as_contract(&contract_id, || {
        env.storage()
            .instance()
            .extend_ttl(1_000_000, 1_000_000);
    });

    env.as_contract(&token, || {
        env.storage()
            .instance()
            .extend_ttl(1_000_000, 1_000_000);
    });

    let amount = 10_000_000_000_0000000;

    StellarAssetClient::new(&env, &token).mint(&admin, &amount);

    let init_data = ContractConfig {
        admin: admin.clone(),
        token,
        amount: amount,
        deposit_params: Map::from_array(&env, [
            (BallotCategory::AddNode, 50_000_0000000),
            (BallotCategory::AddPriceFeed, 100_000_0000000),
            (BallotCategory::AddAsset, 5_000_0000000),
            (BallotCategory::General, 10_000_0000000),
        ]),
        start_date: 0,
    };

    //set admin
    client.config(&init_data);

    (env, client, init_data)
}

#[test]
fn test() {
    let (env, client, config) = init_contract_with_admin();

    let owner = Address::generate(&env);

    let token_client = StellarAssetClient::new(&env, &config.token);
    token_client.mint(&owner, &200_000_0000000);

    env.as_contract(&client.address, || {
        let balance = env.get_dao_balance();
        assert_eq!(balance, 10_000_000_000_0000000);
    });

    let ballot_id = client.create_ballot(&BallotInitParams {
        category: BallotCategory::AddAsset,
        title: String::from_str(&env, "Testing...."),
        description: String::from_str(&env, "https://test.com"),
        initiator: owner.clone(),
    });

    env.as_contract(&client.address, || {
        let balance = env.get_dao_balance();
        assert_eq!(balance, 10_000_005_000_0000000);
    });

    client.vote(&ballot_id, &true);

    let ballot = client.get_ballot(&ballot_id);
    assert_eq!(ballot.status, BallotStatus::Accepted);

    let ballot_id = client.create_ballot(&BallotInitParams {
        category: BallotCategory::AddAsset,
        title: String::from_str(&env, "Test2....."),
        description: String::from_str(&env, "https://test.com"),
        initiator: owner.clone(),
    });

    env.as_contract(&client.address, || {
        let entry_ttl = env.storage().persistent().get_ttl(&ballot_id);
        assert_eq!(entry_ttl, BALLOT_RENTAL_PERIOD);
    });

    let ledger_info = env.ledger().get();
    let ledger_sequence: u32 = 17280 * 14;
    env.ledger().set(LedgerInfo {
        timestamp: (UNLOCK_PERIOD * 2) as u64,
        sequence_number: ledger_sequence,
        ..ledger_info
    });

    client.retract_ballot(&ballot_id);

    env.as_contract(&client.address, || {
        let balance = env.get_dao_balance();
        assert_eq!(balance, 10_000_005_000_0000000 - 6_250_0000000); //6250 is the deposit + 25% for draft status
    });

    let ballot_id = client.create_ballot(&BallotInitParams {
        category: BallotCategory::AddAsset,
        title: String::from_str(&env, "Test2....."),
        description: String::from_str(&env, "https://test.com"),
        initiator: owner.clone(),
    });

    env.as_contract(&client.address, || {
        let entry_ttl = env.storage().persistent().get_ttl(&ballot_id);
        assert_eq!(entry_ttl, ledger_sequence + BALLOT_RENTAL_PERIOD);
    });

    client.vote(&ballot_id, &false);

    client.retract_ballot(&ballot_id);

    env.as_contract(&client.address, || {
        let balance = env.get_dao_balance();
        assert_eq!(balance, 10_000_002_500_0000000 - 3_750_0000000); //3750 is 75% the deposit
    });

    let developer = Address::generate(&env);
    let operators = vec![&env, Address::generate(&env)];
    client.unlock(&developer, &operators);

    env.as_contract(&client.address, || {
        let balance = env.get_available_balance(&developer);
        assert!(balance > 0);
    });

    env.as_contract(&client.address, || {
        let balance = env.get_available_balance(&operators.first().unwrap());
        assert!(balance > 0);
    });

    env.as_contract(&client.address, || {
        let last_unlock = env.get_last_unlock();
        assert_eq!(last_unlock, UNLOCK_PERIOD as u64);
    });    

    //unlock again
    client.unlock(&developer, &operators);

    env.as_contract(&client.address, || {
        let last_unlock = env.get_last_unlock();
        assert_eq!(last_unlock, (UNLOCK_PERIOD * 2) as u64);
    }); 

    let available = client.available(&developer);
    assert!(available > 0);

    client.claim(&developer, &developer, &available);

    env.as_contract(&client.address, || {
        let balance = env.get_available_balance(&developer);
        assert_eq!(balance, 0);
    });
}
