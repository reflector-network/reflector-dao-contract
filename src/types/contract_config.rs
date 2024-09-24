use nondet::Nondet;
use soroban_sdk::{contracttype, Address, Map, Val, Env, TryFromVal};

use super::ballot_category::BallotCategory;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]

/// DAO contract configuration parameters
pub struct ContractConfig {
    /// Admin account address
    pub admin: Address,
    /// DAO token address
    pub token: Address,
    /// Initial funding amount
    pub amount: i128,
    /// Initial deposit amounts for each ballot category
    pub deposit_params: Map<BallotCategory, i128>
}

fn i128_nondet() -> i128 {
    let u1 = u64::nondet();
    let u2 = u64::nondet();
    ((u1 as u128) << 64 | u2 as u128) as i128
}

extern "C" {
    fn CVT_nondet_map_c() -> u64;    
}

fn nondet_map<K, V>() -> Map<K, V> 
    where 
        soroban_sdk::Val: TryFromVal<Env, K>, 
        soroban_sdk::Val: TryFromVal<Env, V>,
        K: soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>,
        V: soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> {
    let map = unsafe { CVT_nondet_map_c() };
    Map::try_from_val(&Env::default(), &Val::from_payload(map)).unwrap()
}

impl Nondet for ContractConfig {
    fn nondet() -> Self {
        ContractConfig {
            admin: Address::nondet(),
            token: Address::nondet(),
            amount: i128_nondet(),
            deposit_params: nondet_map()
        }
    }
}