#[macro_use]
extern crate serde_derive;

use docopt::Docopt;

const USAGE: &str = "
options sample command

Usage:
  options_sample [--speed=<kn>]
  options_sample (-h | --help)
  options_sample --version


Options:
  --speed=<kn>  Speed in knots [default: 10].
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_speed: i32,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}
