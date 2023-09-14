#![feature(impl_trait_in_assoc_type)]

use std::process::exit;
use std::{net::SocketAddr, env};
use volo_example::{FilterLayer, S};
use log::trace;

// const DEFAULT_PROXY_ADDR: &str = "7777";

#[volo::main]
async fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        tracing::error!("No Argument for Address!");
    }

    tracing_subscriber::fmt::init();
    trace!("跟踪服务端");
    let mut fname = String::new();
    if let Some(ms) = args.get(2) {
        if *ms == "-m".to_string() {
            fname.push_str(&args[1] as &str);
        } else if *ms == "-s".to_string() {
            fname.push_str(&args[3] as &str);
        }
    }
    fname.push_str(".txt");

    let mut db = S::new(fname);

    let port = args.remove(1).clone();
    println!("cur_port: {}", port);
    let addr = format!("[::]:{}", port);
    let addr: SocketAddr = addr.parse().unwrap();
    let addr = volo::net::Address::from(addr);
    db.set_port(port.as_str());

    let ms = args.remove(1).clone();
    if ms == "-m".to_string() { 
        let _ = db.init();                                // if is master

        while args.len() > 1 {
            db.set_slave(args.remove(1).clone());
        }
    } else if ms == "-s".to_string() { 
        let _ = db.init();                         // if is slave

        db.set_master(args.remove(1).clone());
    } else if ms == "-p".to_string() {              // if is proxy
        // let port = DEFAULT_PROXY_ADDR;
        // let addr = format!("[::]:{}", DEFAULT_PROXY_ADDR);
        // let addr: SocketAddr = addr.parse().unwrap();
        // let addr = volo::net::Address::from(addr);
        // db.set_port(port);
        while args.len() > 1 {
            db.add_cluster(args.remove(1).clone());
        }
        // volo_gen::volo::example::ItemServiceServer::new(db)
        //     .layer_front(FilterLayer)
        //     .run(addr)
        //     .await
        //     .unwrap();
        // return;

    } else {
        eprintln!("Error: Server Type not Specified!");
        exit(1);
    }

    volo_gen::volo::example::ItemServiceServer::new(db)
        .layer_front(FilterLayer)
        .run(addr)
        .await
        .unwrap();
}

// args: [0:server] [1:port] [2:-m/-s/-p] [3..: master/slaves/servers_in_cluster]