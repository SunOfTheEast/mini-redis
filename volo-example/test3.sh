#!/bin/bash

cargo run --bin server 8080 -m &
cargo run --bin server 9090 -m &
cargo run --bin server 11111 -m &
sleep 1
cargo run --bin server 7777 -p 8080 9090 11111 &

sleep 5
cargo run --bin client set zju 1897 7777
sleep 1
cargo run --bin client set zjuzjg 1897zjg 7777
sleep 1
cargo run --bin client set zjuyq 1897yq 7777
sleep 1
cargo run --bin client get zju 7777
sleep 1
cargo run --bin client get zjuzjg 7777
sleep 1
cargo run --bin client get zjuyq 7777
sleep 1
cargo run --bin client del zju 7777
sleep 1
cargo run --bin client del zjuzjg 7777
sleep 1
cargo run --bin client del zjuyq 7777
sleep 1
cargo run --bin client get zju 7777
sleep 1
cargo run --bin client get zjuzjg 7777
sleep 1
cargo run --bin client get zjuyq 7777
sleep 1
ps -ef | grep 8080 | grep -v grep | cut -c 9-15 | xargs kill -9
ps -ef | grep 9090 | grep -v grep | cut -c 9-15 | xargs kill -9
ps -ef | grep 7777 | grep -v grep | cut -c 9-15 | xargs kill -9
ps -ef | grep 11111 | grep -v grep | cut -c 9-15 | xargs kill -9

# zju to port 8080
# zjuzjg to port 9090
# zjuyq to port 11111
