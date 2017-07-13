extern crate hyper;
extern crate futures;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate time;

use std::io;
use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use futures::*;
use futures::future::*;

#[derive(Serialize, Deserialize)]
struct Metadata {
    timestamp: i64
}

#[derive(Serialize, Deserialize)]
struct ResponseData<A> {
    metadata: Metadata,
    payload: A,
}

#[derive(Serialize, Deserialize)]
struct Message {
    message: String
}

struct Server;

impl Service for Server {
    type Request = hyper::Request;
    type Response = hyper::Response;
    type Error = hyper::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;
    fn call(&self, _req: Request) -> Self::Future {
        let data = gen_data();
        serialize_message(data)
            .map_err(|t| hyper::Error::Io(io::Error::from(t)))
            .map(|x| wrap_response(x)).boxed()
    }

}

fn wrap_response(str: String) -> Response {
    Response::new()
        .with_header(ContentLength(str.len() as u64))
        .with_body(str)
}
fn serialize_message(data: ResponseData<Message>) -> FutureResult<String, serde_json::Error> {
    futures::future::result(serde_json::to_string(&data))
}
fn gen_data() -> ResponseData<Message> {
    let timestamp = time::now_utc().to_timespec();
    ResponseData {
        metadata: Metadata {
            timestamp: (timestamp.sec * 1000) + (timestamp.nsec / 1000000) as i64
        },
        payload: Message {
            message: "hello".to_string()
        }
    }
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Server)).unwrap();
    server.run().unwrap();
}
