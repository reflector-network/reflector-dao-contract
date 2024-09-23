
build:
    RUSTFLAGS="-C strip=none --emit=llvm-ir" cargo build --target=wasm32-unknown-unknown --release

wasm2wat := "wasm2wat"
wat2wasm := "wat2wasm"

wat: build
    {{wasm2wat}} target/wasm32-unknown-unknown/release/reflector_subscriptions.wasm --generate-names -o foo.wat
    {{wat2wasm}} foo.wat --debug-names -o bar.wasm
    {{wasm2wat}} bar.wasm -o reflector_subscriptions.wat
    rm foo.wat bar.wasm

build-llvm:
    env RUSTFLAGS="--emit=llvm-ir" cargo build --target=wasm32-unknown-unknown --release
    @echo "See target/wasm32-unknown-unknown/release/deps/reflector_subscriptions.wasm.ll"

clean:
    cargo clean

update:
    cargo update -p nondet