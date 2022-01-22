#!/usr/bin/env bash

PATH=$PATH:$HOME/.cargo/bin

set -ex

svd2rust -i EMCU.svd
form -i lib.rs -o src
rm lib.rs
cargo fmt
