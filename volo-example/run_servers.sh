#!/bin/bash
cd /mnt/c/Users/23215/Desktop/mini-redis-master/volo-example
cargo run --bin server 4333 -p 33333 22222 &
cargo run --bin server 33333 -m 6666 5566 &
cargo run --bin server 22222 -m 7788 8899 &
cargo run --bin server 6666 -s 33333 &
wait
