extern crate docopt;
extern crate iron;
extern crate router;
#[macro_use]
extern crate serde_derive;

use iron::*;
use iron::headers::*;
use router::Router;
use docopt::Docopt;

#[derive(Debug, Deserialize, Clone)]
struct Args {
    flag_port: i32,
    flag_webui_addr: String,
    flag_webui_port: i32,
    flag_webui_threads: usize,
    flag_config_dir: String,
}

fn main() {
    let home_dir = std::env::var("HOME").unwrap_or("/tmp".to_string());
    let usage = format!(
        "
mitm

Usage:
  mitm [--port=<p>] [--webui-addr=<wa>] [--webui-port=<wp>] [--webui-threads=<wt>]
  mitm (-h | --help)
  mitm --version

Options:
  --port=<p>            bind proxy port [default: 8889].
  --webui-addr=<wa>     bind webui port [default: 0.0.0.0].
  --webui-port=<wp>     bind webui port [default: 8899].
  --webui-threads=<wt>  bind webui port [default: 2].
  --config-dir=<cd>     config save dir [default: {HOME}/.mitm].
  -h --help             Show this screen.
  --version             Show version.
",
        HOME = home_dir
    );
    let args: Args = Docopt::new(usage)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    Proxy::run(args.clone());
    println!("{:?}", args);
}

struct Proxy {}

impl Proxy {
    fn run(args: Args) {
        let mut router = Router::new();
        router.get("/", handler, "GET /");
        let chain = Chain::new(router);
        let mut iron = Iron::new(chain);
        iron.threads = args.flag_webui_threads;
        iron.http(format!("{}:{}", args.flag_webui_addr, args.flag_webui_port))
            .unwrap();
    }
}

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, ContentType::html().0, "hello")))
}
