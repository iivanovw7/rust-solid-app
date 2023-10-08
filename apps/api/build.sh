#!/bin/bash
cargo build --release
cargo install diesel_cli --features postgres
diesel database setup
