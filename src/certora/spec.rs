use soroban_sdk::{Env, Map, Address, Vec};
use crate::extensions::env_extensions::EnvExtensions;
use crate::DAOContract;
use crate::types::{
    ballot_category::BallotCategory, ballot_init_params::BallotInitParams, contract_config::ContractConfig
};
use cvt::{require, assert, satisfy};
use cvt_soroban::{is_auth};


#[no_mangle]
pub fn certora_config_sanity(env: Env, config: ContractConfig) {
    DAOContract::config(env, config);
    satisfy!(true);
}

#[no_mangle]
pub fn certora_config_can_only_be_called_once(env: Env, config: ContractConfig) {
    DAOContract::config(env.clone(), config.clone());
    // Second call should fail
    DAOContract::config(env.clone(), config.clone());
    // Check that the second call failed (i.e., we should not reach this point).
    assert!(false);
}

#[no_mangle]
pub fn certora_create_ballot_sanity(env: Env, params: BallotInitParams) {
    require!(is_auth(params.initiator.clone()), "authorized");
    DAOContract::create_ballot(env, params);
    satisfy!(true);
}

#[no_mangle]
pub fn certora_create_ballot_must_be_initiator(env: Env, params: BallotInitParams) {    
    require!(!is_auth(params.initiator.clone()), "not authorized");
    DAOContract::create_ballot(env, params);
    // create_ballot should have failed because the initiator is not authorized
    assert!(false)
}

#[no_mangle]
pub fn certora_ballot_id_increasing(env: Env, params: BallotInitParams) {
    let ballot_id = DAOContract::create_ballot(env.clone(), params.clone());
    let ballot_id2 = DAOContract::create_ballot(env.clone(), params.clone());
    assert!(ballot_id2 > ballot_id);
}

#[no_mangle]
pub fn certora_retract_ballot_sanity(env: Env, ballot_id: u64) {
    let ballot = DAOContract::get_ballot(env.clone(), ballot_id);
    require!(is_auth(ballot.initiator), "authorized");
    DAOContract::retract_ballot(env, ballot_id);
    satisfy!(true);
}

#[no_mangle]
pub fn certora_retract_ballot_must_be_initiator(env: Env, ballot_id: u64) {
    let ballot = DAOContract::get_ballot(env.clone(), ballot_id);
    require!(!is_auth(ballot.initiator), "not authorized");
    DAOContract::retract_ballot(env, ballot_id);
    assert!(false);
}

#[no_mangle]
pub fn certora_retract_ballot_can_only_be_called_once(env: Env, ballot_id: u64) {
    DAOContract::retract_ballot(env.clone(), ballot_id);
    DAOContract::retract_ballot(env.clone(), ballot_id);
    assert!(false);
}

#[no_mangle]
pub fn certora_vote_sanity(env: Env, ballot_id: u64, accepted: bool) {
    require!(is_auth(env.get_admin().unwrap()), "authorized");
    DAOContract::vote(env, ballot_id, accepted);
    satisfy!(true);
}

#[no_mangle]
pub fn certora_vote_must_be_admin(env: Env, ballot_id: u64, accepted: bool) {
    require!(!is_auth(env.get_admin().unwrap()), "not authorized");
    DAOContract::vote(env, ballot_id, accepted);
    assert!(false);
}

#[no_mangle]
pub fn certora_cannot_vote_on_retracted_ballot(env: Env, ballot_id: u64, accepted: bool) {
    DAOContract::retract_ballot(env.clone(), ballot_id);
    DAOContract::vote(env, ballot_id, accepted);
    assert!(false);
}

#[no_mangle]
pub fn certora_set_deposit_sanity(env: Env, deposit_params: Map<BallotCategory, i128>) {
    require!(is_auth(env.get_admin().unwrap()), "authorized");
    DAOContract::set_deposit(env, deposit_params);
    satisfy!(true);
}

#[no_mangle]
pub fn certora_set_deposit_must_be_admin(env: Env, deposit_params: Map<BallotCategory, i128>) {
    require!(!is_auth(env.get_admin().unwrap()), "not authorized");
    DAOContract::set_deposit(env, deposit_params);
    assert!(false);
}

#[no_mangle]
pub fn certora_unlock_sanity(env: Env, developer: Address, operators: Vec<Address>) {
    require!(is_auth(env.get_admin().unwrap()), "authorized");
    DAOContract::unlock(env, developer, operators);
    satisfy!(true);
}

#[no_mangle]
pub fn certora_unlock_must_be_admin(env: Env, developer: Address, operators: Vec<Address>) {
    require!(!is_auth(env.get_admin().unwrap()), "not authorized");
    DAOContract::unlock(env, developer, operators);
    assert!(false);
}
