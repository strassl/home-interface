#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

HOST=pi@10.8.0.24
DEPLOY_FILE="/usr/bin/home-interface"

RUSTFLAGS='-C relocation-model=static' docker run --volume $PWD:/home/cross/project --volume $PWD/deb-deps:/home/cross/deb-deps ragnaroek/rust-raspberry:1.14.0 build --release
scp $PWD/target/arm-unknown-linux-gnueabihf/release/home-interface $HOST:/tmp/home-interface
ssh $HOST "sudo mv /tmp/home-interface $DEPLOY_FILE; sudo systemctl restart home-interface"
