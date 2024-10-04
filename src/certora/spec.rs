use soroban_sdk::{Env, Map, Address, Vec};
use crate::extensions::env_extensions::EnvExtensions;
use crate::{DAOContract, BallotStatus, get_ballot};
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
    // create_ballot should have failed because the initiator did not authorize
    assert!(false)
}

#[no_mangle]
pub fn certora_ballot_id_increasing(env: Env, params: BallotInitParams) {    
    let before = env.get_last_ballot_id();
    require!(before < u64::MAX, "ballot_id can't overflow");
    let id = DAOContract::create_ballot(env.clone(), params.clone());
    let after = env.get_last_ballot_id();
    assert!(after == id);
    // Check that the ballot_id is increasing, and that it's increasing *slowly*, so it can't overflow the 64-bit int.
    assert!(after == before + 1);
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
pub fn certora_retract_ballot_can_only_be_called_once(e: Env, ballot_id: u64) {
    let before = get_ballot(&e, ballot_id).status;
    DAOContract::retract_ballot(e.clone(), ballot_id);
    let after = get_ballot(&e, ballot_id).status;
    assert!(before != BallotStatus::Retracted);
    assert!(after == BallotStatus::Retracted);
}

#[no_mangle]
pub fn certora_vote_sanity(e: Env, ballot_id: u64, accepted: bool) {
    require!(is_auth(e.get_admin().unwrap()), "authorized");
    require!(get_ballot(&e, ballot_id).status != BallotStatus::Retracted, "not retracted");
    DAOContract::vote(e, ballot_id, accepted);
    satisfy!(true);
}

#[no_mangle]
pub fn certora_vote_must_be_admin(env: Env, ballot_id: u64, accepted: bool) {
    require!(!is_auth(env.get_admin().unwrap()), "not authorized");
    DAOContract::vote(env, ballot_id, accepted);
    assert!(false);
}

#[no_mangle]
pub fn certora_cannot_vote_on_retracted_ballot(e: Env, ballot_id: u64, accepted: bool) {
    require!(get_ballot(&e, ballot_id).status == BallotStatus::Retracted, "retracted");
    DAOContract::vote(e, ballot_id, accepted);
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


#[no_mangle]
pub fn certora_retracted_ballot_cannot_be_retracted(e: Env, ballot_id: u64) {
    require!(DAOContract::get_ballot(e.clone(), ballot_id).status == BallotStatus::Retracted, "ballot retracted");
    DAOContract::retract_ballot(e.clone(), ballot_id);
    assert!(false);
}

#[no_mangle]
pub fn certora_accepted_ballot_cannot_be_retracted(e: Env, ballot_id: u64) {
    require!(DAOContract::get_ballot(e.clone(), ballot_id).status == BallotStatus::Accepted, "ballot accepted");
    DAOContract::retract_ballot(e.clone(), ballot_id);
    assert!(false);
}

#[no_mangle]
pub fn certora_retracted_ballot_cannot_be_voted(e: Env, ballot_id: u64, accepted: bool) {
    require!(DAOContract::get_ballot(e.clone(), ballot_id).status == BallotStatus::Retracted, "ballot retracted");
    DAOContract::vote(e.clone(), ballot_id, accepted);
    assert!(false);
}

#[no_mangle]
pub fn certora_accepted_ballot_cannot_be_voted(e: Env, ballot_id: u64, accepted: bool) {
    require!(DAOContract::get_ballot(e.clone(), ballot_id).status == BallotStatus::Accepted, "ballot accepted");
    DAOContract::vote(e.clone(), ballot_id, accepted);
    assert!(false);
}

#[no_mangle]
pub fn certora_voted_ballot_was_draft(e: Env, ballot_id: u64, accepted: bool) {
    let before = e.get_ballot(ballot_id).unwrap();
    DAOContract::vote(e.clone(), ballot_id, accepted);
    assert!(before.status == BallotStatus::Draft);
}
