set dotenv-load := true

run:
    cargo run

build:
    cargo build

test:
    cargo test

start-db:
    docker compose up -d postgres

stop-db:
    docker compose down
