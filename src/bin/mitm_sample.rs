extern crate futures;
extern crate futures_cpupool;
extern crate tokio_core;
extern crate tokio_io;

use futures::{BoxFuture, Future, Stream};
use tokio_io::{io, AsyncRead};
use tokio_io::io::ReadHalf;
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::Core;
use std::io::{BufReader, Write, ErrorKind as IoErrorKind, Error as IoError};

type TcpReadBuffer = BufReader<ReadHalf<TcpStream>>;

fn main() {
    let addr = "0.0.0.0:8888".parse().unwrap();
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let tcp = TcpListener::bind(&addr, &handle).unwrap();
    let server = tcp.incoming()
        .for_each(move |(tcp, _)| {
            let (reader, mut writer) = tcp.split();
            let reader: TcpReadBuffer = BufReader::new(reader);
            let mut result = read_until_rn(reader);
            let mut count: i32 = 0;
            loop {
                if count == 10 {
                    break;
                }
                result = result.and_then(|(reader, line)| {
                    read_until_rn(reader).and_then(move |(reader, line1)| {
                        Ok((reader, line + line1.as_str()))
                    })
                }).boxed();
                count = count + 1;
            }
            let result = result.and_then(move |(_, line)| {
                writer.write(line.as_bytes()).map(move |_| println!("{}", line))
            }).map_err(|err| println!("IO error {:?}", err)).boxed();
            handle.spawn(result);
            Ok(())
        });
    core.run(server).unwrap();
}

fn read_until_rn(reader: TcpReadBuffer) -> BoxFuture<(TcpReadBuffer, String), IoError> {
    io::read_until(reader, b'\r', vec![0u8]).and_then(|(reader, buf)| {
        vec_to_string(buf.to_vec()).map(move |line0| (reader, line0))
    }).and_then(|(reader, line0)| {
        io::read_until(reader, b'\n', vec![0u8]).and_then(move |(reader, buf)| {
            vec_to_string(buf.to_vec()).map(move |line1| (reader, line0 + line1.as_str()))
        })
    }).boxed()
}

fn vec_to_string(v: Vec<u8>) -> Result<String, IoError> {
    String::from_utf8(v).map_err(|err| IoError::new(IoErrorKind::Other, err))
}