# reflector-dao-contract

DAO contract for decentralized Reflector oracle network

## Interface

### Initialization and funding of the DAO contract

Executed during the deployment

```rust
pub fn config(e: Env, config: ContractConfig)
```

### Unlock DAO tokens

Unlocks tokens distributed to the developer organization and operators on a weekly basis. Requires admin permissions.

```rust
pub fn unlock(e: Env, developer: Address, operators: Vec<Address>)
```

### Get available balance for an account

Fetches the amount of DAO tokens available for claiming by a given account.

```rust
pub fn available(e: Env, claimant: Address) -> i128
```

### Claim tokens

Claims tokens unlocked for a given account address.

```rust
pub fn claim(e: Env, claimant: Address, to: Address, amount: i128)
```

### Create a new ballot

Creates a new ballot and deposits the tokens to the DAO.

```rust
pub fn create_ballot(e: Env, params: BallotInitParams) -> u64
```

### Load ballot

Fetches a ballot by its unique ID.

```rust
pub fn get_ballot(e: Env, ballot_id: u64) -> Ballot
```

### Retract ballot

Retracts the proposal and initiates the deposit refund on behalf of the ballot initiator.

```rust
pub fn retract_ballot(e: Env, ballot_id: u64)
```

### Vote

Confirms ballot decision based on the operators voting (decision requires the majority of signatures)

```rust
pub fn vote(e: Env, ballot_id: u64, accepted: bool)
```