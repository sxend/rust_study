extern crate hyper;
extern crate futures;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate time;
extern crate tokio_core;
extern crate tokio_io;

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
    fn call(&self, _: Request) -> Self::Future {
        serialize_message(gen_data())
            .map_err(|err| hyper::Error::Io(io::Error::from(err)))
            .and_then(|data| {futures::future::result(Ok(data))})
            .map(|data| wrap_response(data)).boxed()
    }

}

fn wrap_response(data: String) -> Response {
    Response::new()
        .with_header(ContentLength(data.len() as u64))
        .with_body(data)
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
