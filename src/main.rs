#[macro_use]
extern crate futures;
extern crate bytes;
extern crate http;
extern crate httparse;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_io;

extern crate hyper;
extern crate tokio_fs;

mod chat;
// mod rpc;
mod rpc2;

use tokio::prelude::*;
use tokio::runtime::Runtime;

pub fn main() {
    println!("hello wold");
    let mut rt = Runtime::new().unwrap();
    chat::start_server(&mut rt);
    println!("chat server running on localhost:6142");
    rpc2::start_server(&mut rt);
    println!("rpc server running on 127.0.0.1:8080");
    rt.shutdown_on_idle().wait().unwrap();
}
