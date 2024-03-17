test:
    cargo test

build:
    cargo build --release
    cp ./target/release/slice .

benchmark: build
    hyperfine -r 10000 \
        './slice Cargo.lock:50:100' \
        'cat -n Cargo.lock | head -n 100 | tail -n +50'

install:
    cargo install --path .
