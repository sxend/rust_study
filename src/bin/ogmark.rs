extern crate markdown;

fn main() {
  println!("{}", markdown::to_html("*foo*"));
}