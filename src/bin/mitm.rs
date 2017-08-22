
#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;

#[derive(Debug, Deserialize)]
struct Args {
    flag_port: i32,
    flag_webui_port: i32,
    flag_config_dir: String,
}

fn main() {
    let home_dir = std::env::var("HOME").unwrap_or("/tmp".to_string());
    let usage = format!("
mitm

Usage:
  options_sample [--port=<p>]
  options_sample (-h | --help)
  options_sample --version

Options:
  --port=<p>         bind proxy port [default: 8889].
  --webui-port=<mp>  bind webui port [default: 8899].
  --config-dir=<dr>  config save dir [default: {HOME}/.mitm].
  -h --help          Show this screen.
  --version          Show version.
", HOME = home_dir);
    let args: Args = Docopt::new(usage)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}