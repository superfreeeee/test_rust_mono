# test_rust_mono

Try run some rust code for following reason:
1. Testing basic and internal features of Rust lang
2. Testing basic usage of open source libraries
3. Try encapsulate some useful tools with rust
   - Consider move to isolate repo when encapsulate code is stable and ready for publish

## Contribute

- Create new repo

```bash
cargo new crates/test_xxx
```

- Run specific repo

```bash
# run bin (main)
cargo run -p crates/test_xxx
# run test
cargo test -p crates/test_xxx
```

- lint

```bash
cargo clippy
```
