# futures-x-io

* [Cargo package](https://crates.io/crates/futures-x-io)

## Dev

```
cargo clippy --all-targets --all-features -- -D clippy::all && \
cargo fmt --all -- --check
```

```
╰─➤ cargo test-all-features --all 2>/dev/null | grep 'test only_' | grep ' ... ok' | wc -l
51
```
