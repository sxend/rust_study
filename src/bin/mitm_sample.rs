
extern crate futures;
extern crate futures_cpupool;
extern crate tokio_core;
extern crate tokio_io;

use futures::{Future, Stream};
use tokio_io::{io, AsyncRead};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use futures_cpupool::Builder;
use std::io::BufReader;
use std::io::Write;

fn main() {
//    let pool = Builder::new().pool_size(2).name_prefix("mitm").create();
    let addr = "0.0.0.0:8888".parse().unwrap();
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let tcp = TcpListener::bind(&addr, &handle).unwrap();
    let server = tcp.incoming()
        .for_each(move |(tcp, _)| {
            let (reader, mut writer) = tcp.split();
            let reader = BufReader::new(reader);
            let mut result = io::read_until(reader, b'\n', vec![0u8]).and_then(|(reader, buf)| {
                Ok((reader, String::from_utf8(buf.to_vec()).unwrap()))
            }).boxed();
            let mut count: i32 = 0;
            loop {
                if count == 10 {
                    break;
                }
                result = result.and_then(|(reader, prev)| {
                    io::read_until(reader, b'\n', vec![0u8]).and_then(|(reader, buf)| {
                        Ok((reader, prev + String::from_utf8(buf.to_vec()).unwrap().as_str()))
                    })
                }).boxed();
                count = count + 1;
            }
            let result = result.map(move |(_, line)| {
                writer.write(line.as_bytes());
                println!("{}", line)
            }).map_err(|err| {
                println!("IO error {:?}", err)
            }).boxed();
            handle.spawn(result);
            Ok(())
    });

    core.run(server).unwrap();
}
