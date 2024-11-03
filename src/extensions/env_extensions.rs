#![allow(non_upper_case_globals)]
use soroban_sdk::storage::{Instance, Persistent};
use soroban_sdk::{panic_with_error, Address, Env};

use crate::types;

use types::{error::Error, ballot::Ballot, ballot_category::BallotCategory};
const ADMIN_KEY: &str = "admin";
const LAST_BALLOT_ID: &str = "last_ballot_id";
const LAST_UNLOCK: &str = "last_unlock";
const TOKEN_KEY: &str = "token";
const DAO_BALANCE: &str = "dao_balance";

pub trait EnvExtensions {
    fn get_admin(&self) -> Option<Address>;

    fn set_admin(&self, admin: &Address);

    fn get_token(&self) -> Address;

    fn set_token(&self, token: &Address);

    fn get_last_ballot_id(&self) -> u64;

    fn set_last_ballot_id(&self, last_ballot_id: u64);

    fn set_last_unlock(&self, last_unlock: u64);

    fn get_last_unlock(&self) -> u64;

    fn get_ballot(&self, ballot_id: u64) -> Option<Ballot>;

    fn set_ballot(&self, ballot_id: u64, ballot: &Ballot);

    fn set_deposit(&self, ballot_category: BallotCategory, amount: i128);

    fn get_deposit(&self, ballot_category: BallotCategory) -> i128;

    fn get_dao_balance(&self) -> i128;

    fn set_dao_balance(&self, balance: i128);

    fn get_available_balance(&self, address: &Address) -> i128;

    fn set_available_balance(&self, address: &Address, balance: i128);

    fn extend_ballot_ttl(&self, ballot_id: u64, extend_to: u32);

    fn panic_if_not_admin(&self);

    fn is_initialized(&self) -> bool;
}

impl EnvExtensions for Env {
    fn is_initialized(&self) -> bool {
        get_instance_storage(&self).has(&ADMIN_KEY)
    }

    fn get_admin(&self) -> Option<Address> {
        get_instance_storage(&self).get(&ADMIN_KEY)
    }

    fn set_admin(&self, admin: &Address) {
        get_instance_storage(&self).set(&ADMIN_KEY, admin);
    }

    fn get_token(&self) -> Address {
        get_instance_storage(&self).get(&TOKEN_KEY).unwrap()
    }

    fn set_token(&self, token: &Address) {
        get_instance_storage(&self).set(&TOKEN_KEY, token);
    }

    fn get_dao_balance(&self) -> i128 {
        get_instance_storage(&self).get(&DAO_BALANCE).unwrap_or(0)
    }

    fn set_dao_balance(&self, balance: i128) {
        get_instance_storage(&self).set(&DAO_BALANCE, &balance);
    }

    fn get_available_balance(&self, address: &Address) -> i128 {
        get_instance_storage(&self).get(&address.to_string()).unwrap_or(0)
    }

    fn set_available_balance(&self, address: &Address, balance: i128) {
        get_instance_storage(&self).set(&address.to_string(), &balance);
    }

    fn get_last_ballot_id(&self) -> u64 {
        get_instance_storage(&self)
            .get(&LAST_BALLOT_ID)
            .unwrap_or(0)
    }

    fn set_last_ballot_id(&self, last_ballot_id: u64) {
        get_instance_storage(&self).set(&LAST_BALLOT_ID, &last_ballot_id);
    }

    fn get_ballot(&self, ballot_id: u64) -> Option<Ballot> {
        get_persistent_storage(&self).get(&ballot_id)
    }

    fn set_ballot(&self, ballot_id: u64, ballot: &Ballot) {
        get_persistent_storage(&self).set(&ballot_id, ballot);
    }

    fn set_deposit(&self, ballot_category: BallotCategory, amount: i128) {
        get_instance_storage(&self).set(&ballot_category, &amount);
    }

    fn get_deposit(&self, ballot_category: BallotCategory) -> i128 {
        get_instance_storage(&self).get(&ballot_category).unwrap()
    }

    fn get_last_unlock(&self) -> u64 {
        get_instance_storage(&self).get(&LAST_UNLOCK).unwrap_or(0)
    }

    fn set_last_unlock(&self, last_uplock: u64) {
        get_instance_storage(&self).set(&LAST_UNLOCK, &last_uplock);
    }

    fn extend_ballot_ttl(&self, ballot_id: u64, extend_to: u32) {
        get_persistent_storage(&self).extend_ttl(&ballot_id, extend_to, extend_to)
    }

    fn panic_if_not_admin(&self) {
        let admin = self.get_admin();
        if admin.is_none() {
            panic_with_error!(self, Error::Unauthorized);
        }
        admin.unwrap().require_auth()
    }
}

fn get_instance_storage(e: &Env) -> Instance {
    e.storage().instance()
}

fn get_persistent_storage(e: &Env) -> Persistent {
    e.storage().persistent()
}
