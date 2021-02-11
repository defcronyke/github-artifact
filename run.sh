#!/bin/bash

source credentials.env || true

cargo watch -w src/ -w Cargo.toml -s "cargo run"
