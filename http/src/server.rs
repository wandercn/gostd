#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::{
    io::{self, BufReader, BufWriter},
    net::TcpStream,
    sync::{mpsc::Receiver, Arc},
};

use crate::{error::HttpResult, header::Header, request::Request};
trait Handler {
    fn ServeHttp(&self, response_writer: &mut impl ResponseWriter, request: &Request);
}

trait ResponseWriter {
    // Returns the header map that will be sent by WriteHeader.
    fn Header(&self) -> &Header;

    // Writes the data to the connection as part of an HTTP reply.
    fn Write(&mut self, buf: &[u8]) -> HttpResult<usize>;

    // Sends an HTTP response header with the provided status code.
    fn WriteHeader(&mut self, status_code: u16);
}

trait Flusher {
    // Flush sends any buffered data to the client.
    fn Flush(&mut self) -> io::Result<()>;
}

trait Hijacker {
    // Hijack lets the caller take over the connection.
    fn Hijack(self) -> io::Result<(TcpStream, BufReader<TcpStream>, BufWriter<TcpStream>)>;
}

trait CloseNotifier {
    // CloseNotify returns a channel that receives at most a single value (true) when the client connection has gone away.
    fn CloseNotify(&self) -> Receiver<bool>;
}
