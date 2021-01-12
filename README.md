# futures-x-io

* [Cargo package](https://crates.io/crates/futures-x-io)

## Dev

```
cargo +nightly clippy --all-targets --all-features -- -D clippy::all && \
cargo fmt --all -- --check
```

```
cargo build-all-features

╰─➤ cargo test-all-features 2>/dev/null | grep 'test only_' | grep ' ... ok' | wc -l
51
```
