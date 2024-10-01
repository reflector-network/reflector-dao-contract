
build:
    RUSTFLAGS="-C strip=none --emit=llvm-ir" cargo build --target=wasm32-unknown-unknown --release

wasm2wat := "wasm2wat"
wat2wasm := "wat2wasm"

wat: build
    {{wasm2wat}} target/wasm32-unknown-unknown/release/reflector_dao_contract.wasm --generate-names -o foo.wat
    {{wat2wasm}} foo.wat --debug-names -o bar.wasm
    {{wasm2wat}} bar.wasm -o reflector_dao_contract.wat
    rm foo.wat bar.wasm

build-llvm:
    env RUSTFLAGS="--emit=llvm-ir" cargo build --target=wasm32-unknown-unknown --release
    @echo "See target/wasm32-unknown-unknown/release/deps/reflector_dao_contract.wasm.ll"

clean:
    cargo clean

update:
    cargo update -p nondet

trapAsAssert := "false"

config_loop_iter := "4"
check_config_sanity: wat
    certoraRun.py reflector_dao_contract.wat --loop_iter {{config_loop_iter}} --prover_args "-trapAsAssert {{trapAsAssert}} -target certora_config_sanity"
check_config_can_only_be_called_once: wat
    certoraRun.py reflector_dao_contract.wat --loop_iter {{config_loop_iter}} --prover_args "-trapAsAssert {{trapAsAssert}} -target certora_config_can_only_be_called_once"

create_ballot_loop_iter := "3"
check_create_ballot_sanity: wat
    certoraRun.py reflector_dao_contract.wat --loop_iter {{create_ballot_loop_iter}} --prover_args "-trapAsAssert {{trapAsAssert}} -target certora_create_ballot_sanity"
check_create_ballot_must_be_initiator: wat
    certoraRun.py reflector_dao_contract.wat --loop_iter {{create_ballot_loop_iter}} --prover_args "-trapAsAssert {{trapAsAssert}} -target certora_create_ballot_must_be_initiator"
check_ballot_id_increasing: wat
    certoraRun.py reflector_dao_contract.wat --loop_iter {{create_ballot_loop_iter}} --prover_args "-trapAsAssert {{trapAsAssert}} -target certora_ballot_id_increasing"

retract_ballot_loop_iter := "4"
check_retract_ballot_sanity: wat
    certoraRun.py reflector_dao_contract.wat --loop_iter {{retract_ballot_loop_iter}} --prover_args "-trapAsAssert {{trapAsAssert}} -target certora_retract_ballot_sanity"
check_retract_ballot_must_be_initiator: wat
    certoraRun.py reflector_dao_contract.wat --loop_iter {{retract_ballot_loop_iter}} --prover_args "-trapAsAssert {{trapAsAssert}} -target certora_retract_ballot_must_be_initiator"
check_retract_ballot_can_only_be_called_once: wat
    certoraRun.py reflector_dao_contract.wat --loop_iter {{retract_ballot_loop_iter}} --prover_args "-trapAsAssert {{trapAsAssert}} -target certora_retract_ballot_can_only_be_called_once"

vote_loop_iter := "4"
check_vote_sanity: wat
    certoraRun.py reflector_dao_contract.wat --loop_iter {{vote_loop_iter}} --prover_args "-trapAsAssert {{trapAsAssert}} -target certora_vote_sanity"
check_cannot_vote_on_retracted_ballot: wat
    certoraRun.py reflector_dao_contract.wat --loop_iter {{vote_loop_iter}} --prover_args "-trapAsAssert {{trapAsAssert}} -target certora_cannot_vote_on_retracted_ballot"

set_deposit_loop_iter := "4"
check_set_deposit_sanity: wat
    certoraRun.py reflector_dao_contract.wat --loop_iter {{set_deposit_loop_iter}} --prover_args "-trapAsAssert {{trapAsAssert}} -target certora_set_deposit_sanity"
check_set_deposit_must_be_admin: wat
    certoraRun.py reflector_dao_contract.wat --loop_iter {{set_deposit_loop_iter}} --prover_args "-trapAsAssert {{trapAsAssert}} -target certora_set_deposit_must_be_admin"