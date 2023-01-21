coverage-all:
    @cargo tarpaulin -v --workspace --exclude-files src/lib.rs

coverage PACKAGE:
    @cargo tarpaulin -v -p {{PACKAGE}} --exclude-files src/lib.rs

test-all:
    @cargo nextest run --workspace

test PACKAGE:
    @cargo nextest run -p {{PACKAGE}}

clean:
    @rm config/genesis.toml
    @rm crates/genesis_lib/config/genesis.toml

udeps:
    @cargo +nightly udeps
