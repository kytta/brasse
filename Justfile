release_exe := justfile_directory() + "/target/" + arch() + "-apple-darwin/release/brasse"

# build a debug executable
build:
    cargo build

# build an optimized executable
[private]
build-release-one cpu_arch=(arch()):
    cargo build --release --target={{ cpu_arch }}-apple-darwin --verbose

# build optimized executables for both platforms
build-release: (build-release-one "aarch64") (build-release-one "x86_64")

# do a Cargo check
check:
    cargo check

# run benchmarks for one subcommand
[private]
bench-one subcommand='list':
    hyperfine --warmup 20 --shell=none -n "brew {{ subcommand }}" 'brew {{ subcommand }}' -n "brasse {{ subcommand }}" "{{ release_exe }} list"

# run all benchmarks
bench: build-release-one && (bench-one 'list')

# run pre-commit checks
pre-commit:
    pre-commit run --all-files

# clean project
clean:
    cargo clean

install dest='~/.local/bin': build-release-one
    install -m 755 {{ release_exe }} {{ dest }}/brasse
