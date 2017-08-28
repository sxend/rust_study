
extern crate futures;
extern crate futures_cpupool;
extern crate tokio_core;
extern crate tokio_io;

use futures::{Future, Stream};
use tokio_io::{io, AsyncRead};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use futures_cpupool::Builder;
use std::io::Read;
use std::io::BufReader;
use std::error::Error;

fn main() {
    let pool = Builder::new().pool_size(2).name_prefix("mitm").create();
    let addr = "0.0.0.0:8888".parse().unwrap();
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let tcp = TcpListener::bind(&addr, &handle).unwrap();
    let server = tcp.incoming()
        .for_each(move |(mut tcp, _)| {
            let (reader, writer) = tcp.split();
            let reader = BufReader::new(reader);
            let result = io::read_until(reader, b'\n', vec![0u8]).and_then(|(conn, buf)| {
                println!("{}", std::str::from_utf8(&buf).unwrap());
                Ok((conn, buf))
            }).boxed();
            let result = result.and_then(|(conn, buf)| {
                io::read_until(conn, b'\n', vec![0u8]).and_then(|(conn, buf)| {
                    println!("{}", std::str::from_utf8(&buf).unwrap());
                    Ok((conn, buf))
                })
            }).boxed();
            let result = result.and_then(|(conn, buf)| {
                io::read_until(conn, b'\n', vec![0u8]).and_then(|(conn, buf)| {
                    println!("{}", std::str::from_utf8(&buf).unwrap());
                    Ok((conn, buf))
                })
            }).map(|(conn, buf)| {
                println!("{}", std::str::from_utf8(&buf).unwrap())
            }).boxed();
            let result = result.map_err(|err| {
                println!("IO error {:?}", err)
            }).boxed();
            handle.spawn(result);
            Ok(())
    });

    core.run(server).unwrap();
}
