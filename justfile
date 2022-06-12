coverage:
    @cargo tarpaulin -v --follow-exec --skip-clean

test:
    @cargo nextest run
