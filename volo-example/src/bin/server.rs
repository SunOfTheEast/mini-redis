#![feature(impl_trait_in_assoc_type)]

use std::{net::SocketAddr, env};
//use volo_example::FilterLayer;
use std::fs::File;
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
<<<<<<< Updated upstream
    let mut db = S::new();
    let mut db_ = S::new();
    db_.init();
    let addr: SocketAddr = "[::]:33333".parse().unwrap();
    let addr = volo::net::Address::from(addr);
    db.init();
    volo_gen::volo::example::ItemServiceServer::new(db)
        .layer_front(FilterLayer)
        .run(addr)
=======
    let addr = format!("[::]:{}", args[1]);
    let addr: SocketAddr = addr.parse().unwrap();
    let addr = volo::net::Address::from(addr);

    let server = volo_gen::volo::example::ItemServiceServer::new(S::new()).layer_front(FilterLayer);

    server.run(addr)
>>>>>>> Stashed changes
        .await
        .unwrap();
}
