#!/bin/bash

cargo run --bin server 10001 -m &
sleep 1
cargo run --bin client set zju 1897 10001
sleep 1
cargo run --bin client set zjuzjg 1897zjg 10001
sleep 1
cargo run --bin client set zjuyq 1897yq 10001
sleep 1
ps -ef | grep 10001 | grep -v grep | cut -c 9-15 | xargs kill -9
sleep 2
cargo run --bin server 10001 -m &
sleep 1
cargo run --bin client get zju 10001
sleep 1
cargo run --bin client get zjuzjg 10001
sleep 1
cargo run --bin client get zjuyq 10001
sleep 2
cargo run --bin client del zju 10001
cargo run --bin client del zjuzjg 10001
cargo run --bin client del zjuyq 10001
ps -ef | grep 10001 | grep -v grep | cut -c 9-15 | xargs kill -9
