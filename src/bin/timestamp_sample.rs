extern crate time;

use std::ops::Add;

fn main() {
    println!("{:?}", time::now().to_timespec());
    println!(
        "{}",
        time::now()
            .add(time::Duration::seconds(3600))
            .to_utc()
            .rfc822()
    );
}
