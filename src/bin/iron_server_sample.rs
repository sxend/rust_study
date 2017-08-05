extern crate iron;
extern crate router;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate time;

use iron::*;
use iron::headers::ContentType;
use router::Router;

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "top");
    let chain = Chain::new(router);
    Iron::new(chain).http("localhost:3000").unwrap();
}

fn handler(_: &mut Request) -> IronResult<Response> {
    serde_json::to_string(&gen_data())
        .map_err(|err| IronError::new(err, "server error"))
        .map(|response| {
            Response::with((ContentType::json().0, status::Ok, response))
        })
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