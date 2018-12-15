use futures::{Future, Stream};
use std::io::{BufReader, Error as IoError, ErrorKind as IoErrorKind, Write};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::Core;
use tokio_io::io::{ReadHalf, WriteHalf};
use tokio_io::{io, AsyncRead};

type TcpReadBuffer = BufReader<ReadHalf<TcpStream>>;

fn main() {
    let addr = "0.0.0.0:8888".parse().unwrap();
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let tcp = TcpListener::bind(&addr, &handle).unwrap();
    let server = tcp.incoming().for_each(move |(tcp, _)| {
        let (reader, writer) = tcp.split();
        let result = read_lines(BufReader::new(reader), Vec::new())
            .and_then(move |_| write_response(writer))
            .map_err(|err| println!("IO error {:?}", err));
        let result = Box::new(result);
        handle.spawn(result);
        Ok(())
    });
    core.run(server).unwrap();
}

fn write_response(mut writer: WriteHalf<TcpStream>) -> Result<(), IoError> {
    writer
        .write(b"HTTP/1.0 200 OK\r\nConnection: Close\r\n")
        .map(|_| ())
}

fn read_lines(
    reader: TcpReadBuffer,
    lines: Vec<String>,
) -> Box<Future<Item = (TcpReadBuffer, Vec<String>), Error = IoError>> {
    let result = read_line_until_rn(reader, lines).and_then(|(reader, lines)| {
        if !lines.is_empty() && lines[lines.len() - 1] == "\u{0}\r\u{0}\n" {
            Box::new(futures::future::result(Ok((reader, lines))))
        } else {
            read_lines(reader, lines)
        }
    });
    Box::new(result)
}

fn read_line_until_rn(
    reader: TcpReadBuffer,
    mut lines: Vec<String>,
) -> Box<Future<Item = (TcpReadBuffer, Vec<String>), Error = IoError>> {
    let result = read_until_with_string(reader, b'\r').and_then(|(reader, line0)| {
        read_until_with_string(reader, b'\n').map(move |(reader, line1)| {
            lines.push(line0 + &line1);
            (reader, lines)
        })
    });
    Box::new(result)
}

fn read_until_with_string(
    reader: TcpReadBuffer,
    byte: u8,
) -> Box<Future<Item = (TcpReadBuffer, String), Error = IoError>> {
    let result = io::read_until(reader, byte, vec![0u8])
        .and_then(|(reader, buf)| vec_to_string(buf.to_vec()).map(move |line| (reader, line)));
    Box::new(result)
}

fn vec_to_string(v: Vec<u8>) -> Result<String, IoError> {
    String::from_utf8(v).map_err(|err| IoError::new(IoErrorKind::Other, err))
}
