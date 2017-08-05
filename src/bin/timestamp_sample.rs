extern crate time;

fn main() {
    println!("{:?}", time::now().to_timespec());
}