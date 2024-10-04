
wasm := "target/wasm32-unknown-unknown/release/reflector_dao_contract.wasm"
java_args := ""

build:
    RUSTFLAGS="-C strip=none --emit=llvm-ir" cargo build --target=wasm32-unknown-unknown --release

wat: build
    wasm2wat {{wasm}} -o contract.wat


clean:
    cargo clean

prove target loop_iter="4": build
    certoraRun.py \
        {{wasm}} \
        --loop_iter {{loop_iter}} \
        --prover_args "-target {{target}}" \
        --java_args "{{java_args}}"

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
    (prove "certora_retracted_ballot_cannot_be_retracted") \
    (prove "certora_accepted_ballot_cannot_be_retracted") \
    (prove "certora_retracted_ballot_cannot_be_voted") \
    (prove "certora_accepted_ballot_cannot_be_voted") \
    (prove "certora_voted_ballot_was_draft") \
