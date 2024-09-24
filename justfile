
build:
    RUSTFLAGS="-C strip=none --emit=llvm-ir" cargo build --target=wasm32-unknown-unknown --release

wasm2wat := "wasm2wat"
wat2wasm := "wat2wasm"

trapAsAssert := "false"

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

check_config_can_only_be_called_once: wat
    certoraRun.py reflector_dao_contract.wat --loop_iter 4 --prover_args "-trapAsAssert {{trapAsAssert}} -target certora_config_can_only_be_called_once"