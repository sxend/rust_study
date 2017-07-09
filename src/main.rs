extern crate hyper;
extern crate futures;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate time;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};

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
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, _req: Request) -> Self::Future {
        let timestamp = time::now_utc().to_timespec();
        let data = ResponseData {
          metadata: Metadata {
            timestamp: (timestamp.sec * 1000) + (timestamp.nsec / 1000000) as i64
          },
            payload: Message {
                message: "hello".to_string()
            }
        };
        let body = serde_json::to_string(&data).unwrap();
        futures::future::ok(
            Response::new()
                .with_header(ContentLength(body.len() as u64))
                .with_body(body)
        )
    }
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Server)).unwrap();
    server.run().unwrap();
}
