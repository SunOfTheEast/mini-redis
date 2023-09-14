#![feature(impl_trait_in_assoc_type)]

use std::process::exit;
use std::{net::SocketAddr, env};
use volo_example::{FilterLayer, S};
use log::trace;

#[volo::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        tracing::error!("No Argument for Address!");
    }

    tracing_subscriber::fmt::init();
    trace!("跟踪服务端");
    let mut db = S::new();
    let _ = db.init();
    let mut args: Vec<String> = std::env::args().collect();
    let port = args.remove(1).clone();
    println!("cur_port: {}", port);
    let addr = format!("[::]:{}", port);
    let addr: SocketAddr = addr.parse().unwrap();
    let addr = volo::net::Address::from(addr);
    db.set_port(port.as_str());

    // if args.len() > 1 { // if has master
    //     let master_port = args.remove(1).clone().to_string();
    //     db.set_master(master_port);
    // }
    let ms = args.remove(1).clone();
    if ms == "-m".to_string() {                                 // if is master
        while args.len() > 1 {
            db.set_slave(args.remove(1).clone());
        }
    } else if ms == "-s".to_string() {                          // if is slave
        db.set_master(args.remove(1).clone());
    } else {
        eprintln!("Error: Master or Slave?");
        exit(1);
    }

    volo_gen::volo::example::ItemServiceServer::new(db)
        .layer_front(FilterLayer)
        .run(addr)
        .await
        .unwrap();
}

// args: [0:server] [1:port] [2:-m/-s] [3..: master/slaves]