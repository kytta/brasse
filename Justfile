all: build test
all-release: build-release test-release



# compile the binary
@build:
    cargo build

# compile the binary (in release mode)
@build-release:
    cargo build --release --verbose

# check that the binary can compile
@check:
    cargo check



# run unit tests
@test:
    cargo test --workspace -- --quiet

# run unit tests (in release mode)
@test-release:
    cargo test --workspace --release --verbose



# lint the code
@clippy:
    touch src/main.rs
    cargo clippy



# print versions of the necessary build tools
@versions:
    rustc --version
    cargo --version
