# Setup
### Testing
##### Project to measure code coverage 
```
cargo install cargo-tarpaulin
cargo tarpaulin --ignore-tests
```

### Linter
```
cargo clippy
cargo clippy -- -D warnings
```

### Formatting
```
cargo fmt
cargo fmt -- --check
```

### Security vulnerabilities
```
cargo install cargo-audit
cargo audit
```