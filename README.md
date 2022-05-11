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

### CI
This project uses github actions for CI
- Jobs run automatically on push and PRs
- Must give `GITHUB_TOKEN` read and write permissions

# Tools
### Cargo Expand
Install
- `cargo install cargo-expand`
- Needs to use the nightly build `rustup toolchain install nightly --allow-downgrade`

Run
- `cargo +nightly expand`

# Database
Set env variable
- `export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter`
Create DB in Docker
- `./scripts/init_db.sh`
Add Table
- `sqlx migrate add table_name`
Run migrations
- `sqlx migrate run`

