
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

prove_all: \
    (prove "certora_config_sanity") \
    (prove "certora_config_can_only_be_called_once") \
    (prove "certora_create_ballot_sanity") \
    (prove "certora_create_ballot_must_be_initiator") \
    (prove "certora_ballot_id_increasing") \
    (prove "certora_retract_ballot_sanity") \
    (prove "certora_retract_ballot_must_be_initiator") \
    (prove "certora_retract_ballot_can_only_be_called_once") \
    (prove "certora_vote_sanity") \
    (prove "certora_vote_must_be_admin") \
    (prove "certora_cannot_vote_on_retracted_ballot") \
    (prove "certora_set_deposit_sanity") \
    (prove "certora_set_deposit_must_be_admin") \
    (prove "certora_unlock_sanity") \

