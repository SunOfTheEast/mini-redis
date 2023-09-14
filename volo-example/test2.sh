#!/bin/bash

cargo run --bin server 8080 -s 10002 &
cargo run --bin server 9090 -s 10002 &
cargo run --bin server 11111 -s 10002 &
sleep 1
cargo run --bin server 10002 -m 8080 9090 11111 &

sleep 2
cargo run --bin client set zju 1897 8080
sleep 1
cargo run --bin client set zjuzjg 1897zjg 9090
sleep 1
cargo run --bin client set zjuyq 1897yq 11111
sleep 1
cargo run --bin client set zju 1897 10002
sleep 1
cargo run --bin client set zjuzjg 1897zjg 10002
sleep 1
cargo run --bin client set zjuyq 1897yq 10002
sleep 1
cargo run --bin client get zju 8080
sleep 1
cargo run --bin client get zjuzjg 9090
sleep 1
cargo run --bin client get zjuyq 11111
sleep 1
cargo run --bin client del zju 8080
sleep 1
cargo run --bin client del zjuzjg 9090
sleep 1
cargo run --bin client del zjuyq 11111
sleep 1
cargo run --bin client del zju 10002
sleep 1
cargo run --bin client del zjuzjg 10002
sleep 1
cargo run --bin client del zjuyq 10002
sleep 1
ps -ef | grep 8080 | grep -v grep | cut -c 9-15 | xargs kill -9
ps -ef | grep 9090 | grep -v grep | cut -c 9-15 | xargs kill -9
ps -ef | grep 10002 | grep -v grep | cut -c 9-15 | xargs kill -9
ps -ef | grep 11111 | grep -v grep | cut -c 9-15 | xargs kill -9