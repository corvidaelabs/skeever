set dotenv-load := true

default:
    @just --list

# Build the project
build:
    cargo build

# Run the project
run:
    cargo run

dev: watch-run
# Watch for changes and run the project
watch-run:
    watchexec -r -w ./api cargo run

# Check formatting
fmt:
    cargo fmt --all -- --check

# Format code
fmt-fix:
    cargo fmt --all

# Run clippy
lint:
    cargo clippy -- -D warnings

# Clean build artifacts
clean:
    cargo clean

# Create a new stream
create-stream:
    cargo run --bin create-oblivion-es -- --stream-name="ODDLAWS_EVENTS" --subjects="oddlaws.events.>" --description="Oddlaws Event Stream"
