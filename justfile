set shell := ["cmd.exe", "/c"]

default:
    @just --list

# Auto-format the source tree
fmt:
    treefmt

# Run the project locally
watch $RUST_BACKTRACE="1":
    cargo +nightly leptos watch --hot-reload

# Run cargo in release mode (prints red panic)
watch-release:
    cargo +nightly leptos watch --release

# Run tests (backend & frontend)
test:
    cargo +nightly watch -- cargo leptos test