#!/bin/bash
set -xe
cargo build
target/debug/letcom 2>log.txt
cat log.txt
