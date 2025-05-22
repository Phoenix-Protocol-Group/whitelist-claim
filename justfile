# Build debug version
build:
    cargo build --target wasm32-unknown-unknown

# Build release version
release:
    cargo build --release --target wasm32-unknown-unknown

# Format all code
fmt:
    cargo fmt

# Run Clippy with warnings treated as errors
lints:
    cargo clippy --all-targets -- -D warnings
