#![feature(impl_trait_in_assoc_type)]

use std::net::SocketAddr;
//use volo_example::FilterLayer;
use std::fs::File;
use volo_example::{FilterLayer, S};
use log::{error, warn, info, debug, trace};
#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    trace!("跟踪服务端");
    let mut db = S::new();
    let mut db_ = S::new();
    db_.init();
    let addr: SocketAddr = "[::]:33333".parse().unwrap();
    let addr = volo::net::Address::from(addr);
    db.init();
    volo_gen::volo::example::ItemServiceServer::new(db)
        .layer_front(FilterLayer)
        .run(addr)
        .await
        .unwrap();
}
