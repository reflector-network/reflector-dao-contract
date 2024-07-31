#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::{Address as _, Ledger, LedgerInfo}, token::StellarAssetClient, vec, Env, String};

fn init_contract_with_admin<'a>() -> (Env, DAOContractClient<'a>, ContractConfig) {
    let env = Env::default();

    let admin = Address::generate(&env);

    let contract_id = env.register_contract(None, DAOContract);
    let client: DAOContractClient<'a> =
    DAOContractClient::new(&env, &contract_id);

    let token = env.register_stellar_asset_contract(admin.clone());

    env.mock_all_auths();

    StellarAssetClient::new(&env, &token).mint(&admin, &10000000000);

    let init_data = ContractConfig {
        admin: admin.clone(),
        token,
        amount: 10000000000,
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
    token_client.mint(&owner, &200000);

    env.as_contract(&client.address, || {
            let balance = env.get_dao_balance();
            assert_eq!(balance, 10000000000);
        });

    let ballot_id = client.create_ballot(&BallotInitParams {
        category: BallotCategory::AddAsset,
        title: String::from_str(&env, "Testing...."),
        description: String::from_str(&env, "https://test.com"),
        initiator: owner.clone(),
    });

    env.as_contract(&client.address, || {
        let balance = env.get_dao_balance();
        assert_eq!(balance, 10000005000);
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

    let ledger_info = env.ledger().get();
    env.ledger().set(LedgerInfo {
        timestamp: (UNLOCK_PERIOD * 2) as u64,
        ..ledger_info
    });

    client.retract_ballot(&ballot_id);

    env.as_contract(&client.address, || {
        let balance = env.get_dao_balance();
        assert_eq!(balance, 10000005000 - 6250);//6250 is the deposit + 25% for draft status
    });

    let ballot_id = client.create_ballot(&BallotInitParams {
        category: BallotCategory::AddAsset,
        title: String::from_str(&env, "Test2....."),
        description: String::from_str(&env, "https://test.com"),
        initiator: owner.clone(),
    });

    client.vote(&ballot_id, &false);

    client.retract_ballot(&ballot_id);

    env.as_contract(&client.address, || {
        let balance = env.get_dao_balance();
        assert_eq!(balance, 10000002500 - 3750);//3750 is 75% the deposit
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
}
