
wasm := "target/wasm32-unknown-unknown/release/reflector_dao_contract.wasm"
java_args := ""
loop_iter := "4" 
precise_bitwise_ops := "true" 
optimistic_loop := "true"

build:
    RUSTFLAGS="-C strip=none --emit=llvm-ir" cargo build --target=wasm32-unknown-unknown --release

wat: build
    wasm2wat {{wasm}} -o contract.wat

clean:
    cargo clean

prove target="" : build wat
    certoraRun.py \
        {{wasm}} \
        --loop_iter {{loop_iter}} \
        --prover_args "{{ if target == "" { "" } else { "-target " + target } }}" \
        --java_args "{{java_args}}" \
        {{if precise_bitwise_ops == "true" { "--precise_bitwise_ops" } else { "" }  }} \
        {{if optimistic_loop == "true" { "--optimistic_loop" } else { "" }  }} \
