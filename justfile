default:
    just -l

check:
    cargo clippy --no-deps -- -D warnings
