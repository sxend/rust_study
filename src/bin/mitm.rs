
#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str = "
mitm

Usage:
  options_sample [--port=<p>]
  options_sample (-h | --help)
  options_sample --version


Options:
  --port=<p>         bind proxy port [default: 8889].
  --webui-port=<mp>  bind webui port [default: 8899].
  -h --help          Show this screen.
  --version          Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_port: i32,
    flag_webui_port: i32,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}