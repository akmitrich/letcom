#!/bin/bash
cargo build --release
target/release/letcom 2>log.txt
cat log.txt
