#![no_std]
use extensions::env_extensions::EnvExtensions;
use soroban_sdk::{contract, contractimpl, token::TokenClient, Address, Env, Map, Vec};
use types::{
    ballot::Ballot, ballot_category::BallotCategory, ballot_init_params::BallotInitParams,
    ballot_status::BallotStatus, contract_config::ContractConfig, error::Error,
};

mod extensions;
mod types;

// 0.24% weekly distribution
const OPERATORS_SHARE: i128 = 24;

// 0.06% weekly distribution
const DEVELOPERS_SHARE: i128 = 6;

// 1 week
const UNLOCK_PERIOD: u32 = 604800;

// 2 weeks
const BALLOT_DURATION: u32 = 604800 * 2;

// 2 months
const BALLOT_RENTAL_PERIOD: u32 = 17280 * 30 * 2;

#[contract]
pub struct DAOContract;

#[contractimpl]
impl DAOContract {
    /// Initializes and funds the DAO contract
    /// Requires admin permissions
    ///
    /// # Arguments
    ///
    /// * `config` - Initial contract configuration
    ///
    /// # Panics
    ///
    /// Panics if the contract has been already initialized
    /// Panics if the deposit amounts is invalid
    /// Panics if the deposit amount is not set for all categories
    pub fn config(e: Env, config: ContractConfig) {
        // check admin permissions
        config.admin.require_auth();
        // can be executed only once
        if e.is_initialized() {
            e.panic_with_error(Error::AlreadyInitialized);
        }
        // validate the funding amount
        if config.amount <= 0 {
            e.panic_with_error(Error::InvalidAmount);
        }
        // save the configuration
        e.set_admin(&config.admin);
        e.set_token(&config.token);
        e.set_last_unlock(e.ledger().timestamp());
        //set deposit params
        set_deposit(&e, config.deposit_params);
        // transfer tokens to the DAO contract
        token(&e).transfer(&config.admin, &e.current_contract_address(), &config.amount);
        // set initial DAO balance
        update_dao_balance(&e, &config.amount.into());
    }

    /// Sets the deposit amount for each ballot category
    /// Requires admin permissions
    /// 
    /// # Arguments
    /// 
    /// * `deposit_params` - Map of deposit amounts for each ballot category
    /// 
    /// # Panics
    /// 
    /// Panics if the caller doesn't match admin address
    /// Panics if the deposit amount is invalid
    /// Panics if the deposit amount is not set for all categories
    pub fn set_deposit(e: Env, deposit_params: Map<BallotCategory, i128>) {
        e.panic_if_not_admin();
        set_deposit(&e, deposit_params);
    }

    /// Unlocks tokens distributed to the developer organization and operators on a weekly basis
    /// Requires admin permissions
    ///
    /// # Arguments
    ///
    /// * `developer` - Developer organization account address
    /// * `operators` - Operators' account addresses
    ///
    /// # Panics
    ///
    /// Panics if the caller doesn't match admin address
    /// Panics if the unlock process has been initiated too early
    pub fn unlock(e: Env, developer: Address, operators: Vec<Address>) {
        // check admin permissions
        e.panic_if_not_admin();
        // retrieve last recorded unlock period timestamp
        let last_unlock = e.get_last_unlock();
        let now = e.ledger().timestamp();
        // panic if the unlock routine has been initiated too early
        if now - last_unlock < UNLOCK_PERIOD as u64 {
            e.panic_with_error(Error::UnlockUnavailable);
        }
        // fetch the remaining DAO balance
        let dao_balance = e.get_dao_balance();
        // actual unlocked amount can be different from the calculated percentage due to rounding errors
        let mut total_unlocked = 0i128;
        // calculate unlocked amount that goes to operators
        let operators_unlocked = calc_percentage(dao_balance, OPERATORS_SHARE);
        // the amount a single operator would get
        let unlock_per_operator = &(operators_unlocked / operators.len() as i128);
        // update available balances for every operator
        for operator in operators.iter() {
            // increase outstanding available balance
            update_available_balance(&e, &operator, unlock_per_operator);
            total_unlocked += unlock_per_operator;
        }
        // get developer unlocked amount
        let developer_unlocked = &calc_percentage(dao_balance, DEVELOPERS_SHARE);
        // increase outstanding developer available balance
        update_available_balance(&e, &developer, developer_unlocked);
        total_unlocked += developer_unlocked;
        // add week to last unlock date
        e.set_last_unlock(last_unlock + UNLOCK_PERIOD as u64);
        // update dao balance
        e.set_dao_balance(dao_balance - total_unlocked);
    }

    /// Fetches the DAO tokens amount available for claiming
    ///
    /// # Arguments
    ///
    /// * `claimant` - Claimant's account address
    ///
    /// # Returns
    ///
    /// * `available` - Amount of DAO tokens available for claiming
    ///
    /// # Panics
    ///
    /// Panics if the caller doesn't match the claimant address
    pub fn available(e: Env, claimant: Address) -> i128 {
        // check if the claimant authorized the operation
        claimant.require_auth();
        // return available balance
        e.get_available_balance(&claimant)
    }

    /// Claims tokens unlocked for a given account address
    ///
    /// # Arguments
    ///
    /// * `claimant` - Claimant's account address
    /// * `to` - Destination address that will receive claimed tokens
    /// * `amount` - Amount of tokens to claim
    ///
    /// # Panics
    ///
    /// Panics if the caller doesn't match the claimant address
    /// Panics if the claimed amount is larger than the available unlocked amount
    pub fn claim(e: Env, claimant: Address, to: Address, amount: i128) {
        // check if the claimant authorized the operation
        claimant.require_auth();
        // fetch the unlocked amount for the given claimant
        let balance = e.get_available_balance(&claimant);
        // check if the unlocked amount is sufficient
        if amount <= 0 || balance < amount {
            e.panic_with_error(Error::InvalidAmount);
        }
        // transfer tokens to the destination address
        token(&e).transfer(&e.current_contract_address(), &to, &amount);

        // update available balance
        update_available_balance(&e, &claimant, &(-amount));
    }

    /// Create a new ballot
    ///
    /// # Arguments
    ///
    /// * `params` - Ballot initialization parameters
    ///
    /// # Returns
    ///
    /// * `ballot_id` - Unique ID of a newly created ballot
    ///
    /// # Panics
    ///
    /// Panics if the caller doesn't match the initiator address
    pub fn create_ballot(e: Env, params: BallotInitParams) -> u64 {
        params.initiator.require_auth();
        // generate new ballot id
        let ballot_id = e.get_last_ballot_id() + 1;
        // calculate deposit requirements for the ballot
        let deposit = e.get_deposit(params.category);
        if params.title.len() < 10
            || params.title.len() > 40
            || params.description.len() < 10
            || params.description.len() > 160
        {
            e.panic_with_error(Error::InvalidBallotParams);
        }
        // create a ballot object
        let ballot = Ballot {
            initiator: params.initiator,
            status: BallotStatus::Draft,
            category: params.category,
            title: params.title,
            description: params.description,
            deposit,
            created: e.ledger().timestamp(),
        };
        // transfer deposit to DAO fund
        token(&e).transfer(&ballot.initiator, &e.current_contract_address(), &deposit);
        // update internal DAO balance
        update_dao_balance(&e, &deposit);
        // save ballot
        e.set_ballot(ballot_id, &ballot);
        // extend ballot TTL
        e.extend_ballot_ttl(ballot_id, e.ledger().sequence() + BALLOT_RENTAL_PERIOD);
        // update ID counter
        e.set_last_ballot_id(ballot_id);
        // return created ballot ID
        ballot_id
    }

    /// Fetch the ballot by its unique ID
    ///
    /// # Arguments
    ///
    /// * `ballot_id` - Unique ballot ID
    ///
    /// # Returns
    ///
    /// * `ballot` - Ballot object
    ///
    /// # Panics
    ///
    /// Panics if the ballot is not found
    pub fn get_ballot(e: Env, ballot_id: u64) -> Ballot {
        get_ballot(&e, ballot_id)
    }

    /// Retract the proposal and initiate the deposit refund
    ///
    /// # Arguments
    ///
    /// * `ballot_id` - Unique ballot ID
    ///
    /// # Panics
    ///
    /// Panics if the caller doesn't match the initiator address
    /// Panics if the ballot status is in invalid state (not Draft or Rejected)
    /// Panics if the voting period is not over
    /// Panics if the ballot is not found
    pub fn retract_ballot(e: Env, ballot_id: u64) {
        // load the ballot
        let mut ballot = get_ballot(&e, ballot_id);
        // only initiator can retract the ballot
        ballot.initiator.require_auth();
        // calculate the refund amount based on the ballot status
        let refunded = match ballot.status {
            // if the proposal has been rejected by the DAO, the initiator receives 75% refund
            BallotStatus::Rejected => (ballot.deposit * 75) / 100,
            // if the DAO members haven't voted in a timely manner, the initiator receives extra 25% of the deposit
            BallotStatus::Draft => {
                // draft ballots can be retracted only after the voting period is over
                if e.ledger().timestamp() - ballot.created < BALLOT_DURATION as u64 {
                    e.panic_with_error(Error::RefundUnavailable);
                }
                (ballot.deposit * 125) / 100
            }
            _ => e.panic_with_error(Error::RefundUnavailable),
        };
        // refund tokens to the initiator address
        token(&e).transfer(&e.current_contract_address(), &ballot.initiator, &refunded);
        // update remaining DAO balance
        update_dao_balance(&e, &(-refunded));
        // update ballot status
        ballot.status = BallotStatus::Retracted;
        e.set_ballot(ballot_id, &ballot);
    }

    /// Set ballot decision based on the operators voting (decision requires the majority of signatures)
    /// Requires admin permissions
    ///
    /// # Arguments
    ///
    /// * `ballot_id` - Unique ballot ID
    /// * `accepted` - Whether the proposal has been accepted or rejected by the majority of operators
    ///
    /// # Panics
    ///
    /// Panics if the caller doesn't match admin address
    /// Panics if the ballot status is not Draft
    /// Panics if the ballot is not found
    pub fn vote(e: Env, ballot_id: u64, accepted: bool) {
        // check admin permissions
        e.panic_if_not_admin();
        // fetch ballot
        let mut ballot = get_ballot(&e, ballot_id);
        // it shouldn't be closed
        if ballot.status != BallotStatus::Draft {
            e.panic_with_error(Error::BallotClosed);
        }
        // resolve new status
        let new_status = if accepted {
            BallotStatus::Accepted
        } else {
            BallotStatus::Rejected
        };
        // calculate the amount of DAO tokens to burn
        let burn_amount = match new_status {
            BallotStatus::Rejected => (ballot.deposit * 25) / 100,
            BallotStatus::Accepted => ballot.deposit,
            _ => e.panic_with_error(Error::BallotClosed),
        };
        // burn tokens from the deposit according to the decision
        token(&e).burn(&e.current_contract_address(), &burn_amount);
        // update current DAO balance
        update_dao_balance(&e, &(-burn_amount));
        // update ballot status
        ballot.status = new_status;
        e.set_ballot(ballot_id, &ballot);
    }
}

fn set_deposit(e: &Env, deposit_params: Map<BallotCategory, i128>) {
    for category in BallotCategory::iterator() {
        let amount = deposit_params.get(category).unwrap_or(0);
        if amount <= 0 {
            e.panic_with_error(Error::InvalidAmount);
        }
        e.set_deposit(category, amount);
    }
}

// fetch ballot from the persistent storage
fn get_ballot(e: &Env, ballot_id: u64) -> Ballot {
    // fetch ballot by ID
    let ballot = e.get_ballot(ballot_id);
    // panic if not found
    if ballot.is_none() {
        e.panic_with_error(Error::BallotNotFound);
    }
    ballot.unwrap()
}

// create an instance of the SAC token client
fn token(e: &Env) -> TokenClient {
    TokenClient::new(e, &e.get_token())
}

// calculate percentage from a given amount
fn calc_percentage(value: i128, percentage: i128) -> i128 {
    (value * percentage) / 10000
}

// update the balance available for claiming for a particular account
fn update_available_balance(e: &Env, address: &Address, amount: &i128) {
    let balance = e.get_available_balance(address);
    e.set_available_balance(address, balance + amount);
}

// update the remaining DAO balance
fn update_dao_balance(e: &Env, amount: &i128) {
    let dao_balance = e.get_dao_balance();
    e.set_dao_balance(dao_balance + amount);
}

#[no_mangle]
pub fn certora_config_sanity(env: Env, config: ContractConfig) {
    DAOContract::config(env, config);
    cvt::assert!(false);
}

#[no_mangle]
pub fn certora_config_can_only_be_called_once(env: Env, config: ContractConfig) {
    DAOContract::config(env.clone(), config.clone());
    // Second call should fail
    DAOContract::config(env.clone(), config.clone());
    // Check that the second call failed (i.e., we should not reach this point).
    cvt::assert!(false);
}

extern "C" {
    fn CVT_SOROBAN_is_auth(address: u64) -> u64;
}

fn is_auth(address: Address) -> bool {
    unsafe { CVT_SOROBAN_is_auth(address.to_val().get_payload()) != 0 }
}

#[no_mangle]
pub fn certora_create_ballot_sanity(env: Env, params: BallotInitParams) {
    cvt::require!(is_auth(params.initiator.clone()), "Initiator is authorized");
    DAOContract::create_ballot(env, params);
    cvt::assert!(false);
}

#[no_mangle]
pub fn certora_create_ballot_must_be_initiator(env: Env, params: BallotInitParams) {    
    cvt::require!(!is_auth(params.initiator.clone()), "Initiator is not authorized");
    DAOContract::create_ballot(env, params);
    // create_ballot should have failed because the initiator is not authorized
    cvt::assert!(false);
}

mod test;
