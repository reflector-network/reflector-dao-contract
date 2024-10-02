
wasm := "target/wasm32-unknown-unknown/release/reflector_dao_contract.wasm"
loop_iter := "4"

build:
    RUSTFLAGS="-C strip=none --emit=llvm-ir" cargo build --target=wasm32-unknown-unknown --release

wat: build
    wasm2wat {{wasm}} -o contract.wat


clean:
    cargo clean

prove target: build
    certoraRun.py {{wasm}} --loop_iter {{loop_iter}} --prover_args "-target {{target}}"

check_config_sanity: (prove "certora_config_sanity")
check_config_can_only_be_called_once: (prove "certora_config_can_only_be_called_once")

check_create_ballot_sanity: (prove "certora_create_ballot_sanity")
check_create_ballot_must_be_initiator: (prove "certora_create_ballot_must_be_initiator")
check_ballot_id_increasing: (prove "certora_ballot_id_increasing")

check_retract_ballot_sanity: (prove "certora_retract_ballot_sanity")
check_retract_ballot_must_be_initiator: (prove "certora_retract_ballot_must_be_initiator")
check_retract_ballot_can_only_be_called_once: (prove "certora_retract_ballot_can_only_be_called_once")

check_vote_sanity: (prove "certora_vote_sanity")
check_vote_must_be_admin: (prove "certora_vote_must_be_admin")
check_cannot_vote_on_retracted_ballot: (prove "certora_cannot_vote_on_retracted_ballot")

check_set_deposit_sanity: (prove "certora_set_deposit_sanity")
check_set_deposit_must_be_admin: (prove "certora_set_deposit_must_be_admin")

check_unlock_sanity: (prove "certora_unlock_sanity")
check_unlock_must_be_admin: (prove "certora_unlock_must_be_admin")

