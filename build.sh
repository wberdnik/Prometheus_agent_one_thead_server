#!/bin/bash

CWDR=$(pwd)

if ! $HOME/.cargo/bin/cargo --version ; then


sudo apt update
sudo apt install build-essential

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$HOME/.cargo/bin/rustup update
$HOME/.cargo/bin/rustc --version
fi
rm -rf ./Cargo.lock
rm -rf ./target

$HOME/.cargo/bin/cargo build --release

sudo mkdir -p /opt/wasm-log-parser
mv ./target/release/wasm-log-parser /opt/wasm-log-parser

cp ./wasm-log-parser.service /lib/systemd/system

ln -s /lib/systemd/system/wasm-log-parser.service /etc/systemd/system/multi-user.target.wants/wasm-log-parser.service

systemctl restart wasm-log-parser && systemctl enable wasm-log-parser
systemctl daemon-reload

systemctl status wasm-log-parser.service