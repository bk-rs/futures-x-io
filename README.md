# futures-x-io

* [Cargo package](https://crates.io/crates/futures-x-io)

## Dev

```
cargo clippy --all --all-features -- -D clippy::all
cargo +nightly clippy --all --all-features -- -D clippy::all

cargo fmt --all -- --check
```

```
cargo build-all-features

╰─➤ cargo test-all-features -- --nocapture 2>/dev/null | grep 'test only_' | grep ' ... ok' | wc -l
51
```
