extern crate iron;
extern crate router;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate time;
extern crate uuid;

use std::error::Error;
use iron::*;
use iron::typemap::*;
use iron::headers::ContentType;
use router::Router;
use uuid::Uuid;

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");
    let mut chain = Chain::new(router);
    chain.link_before(gen_request_id);
    Iron::new(chain).http("localhost:3000").unwrap();
}

fn handler(req: &mut Request) -> IronResult<Response> {
    serde_json::to_string_pretty(&gen_response_data(req))
        .map_err(|err| {
            let description = err.description().to_string();
            IronError::new(err, description)
        })
        .map(|response| {
            Response::with((ContentType::json().0, status::Ok, response))
        })
}

struct RequestId {}

impl Key for RequestId {
    type Value = String;
}

fn gen_request_id(req: &mut Request) -> IronResult<()> {
    req.extensions.insert::<RequestId>(Uuid::new_v4().hyphenated().to_string());
    Ok(())
}

fn gen_response_data(req: &mut Request) -> ResponseData<Message> {
    ResponseData {
        metadata: Metadata {
            request_id: req.extensions.get::<RequestId>().unwrap().to_owned(),
            timestamp: current_time_millis()
        },
        payload: Message {
            message: "hello".to_string()
        }
    }
}

fn current_time_millis() -> i64 {
    let timestamp = time::now_utc().to_timespec();
    (timestamp.sec * 1000) + (timestamp.nsec as i64 / 1000000)
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    request_id: String,
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