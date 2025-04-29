//! Package net provides a portable interface for network I/O, including TCP/IP, UDP, domain name resolution, and Unix domain sockets.
#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
// pub mod http;
pub use gostd_http as http;
pub use gostd_url as url;

use crate::builtin::*;
use crate::io;
use crate::time;
use std::io::Error;

/* pub trait Addr { */
/* fn Newwork(&self) -> String; */
/* fn String(&self) -> String; */
/* } */

/* pub trait Conn: io::Reader + io::Writer { */
/* // Read reads data from the connection. */
/* // Read can be made to time out and return an error after a fixed */
/* // time limit; see SetDeadline and SetReadDeadline. */
/* fn Read(&mut self, b: Vec<byte>) -> Result<int, Error>; */

/* // Write writes data to the connection. */
/* // Write can be made to time out and return an error after a fixed */
/* // time limit; see SetDeadline and SetWriteDeadline. */
/* fn Write(&mut self, b: Vec<byte>) -> Result<int, Error>; */

/* // Close closes the connection. */
/* // Any blocked Read or Write operations will be unblocked and return errors. */
/* // fn Close(&self) -> Result<(), Error>; */

/* // LocalAddr returns the local network address. */
/* // fn LocalAddr(&self) -> Box<dyn Addr>; */

/* // RemoteAddr returns the remote network address. */
/* // fn RemoteAddr(&self) -> Box<dyn Addr>; */

/* // SetDeadline sets the read and write deadlines associated */
/* // with the connection. It is equivalent to calling both */
/* // SetReadDeadline and SetWriteDeadline. */
/* // */
/* // A deadline is an absolute time after which I/O operations */
/* // fail instead of blocking. The deadline applies to all future */
/* // and pending I/O, not just the immediately following call to */
/* // Read or Write. After a deadline has been exceeded, the */
/* // connection can be refreshed by setting a deadline in the future. */
/* // */
/* // If the deadline is exceeded a call to Read or Write or to other */
/* // I/O methods will return an error that wraps os.ErrDeadlineExceeded. */
/* // This can be tested using errors.Is(err, os.ErrDeadlineExceeded). */
/* // The error's Timeout method will return true, but note that there */
/* // are other possible errors for which the Timeout method will */
/* // return true even if the deadline has not been exceeded. */
/* // */
/* // An idle timeout can be implemented by repeatedly extending */
/* // the deadline after successful Read or Write calls. */
/* // */
/* // A zero value for t means I/O operations will not time out. */
/* // fn SetDeadline(&mut self, t: time::Time) -> Result<(), Error>; */

/* // SetReadDeadline sets the deadline for future Read calls */
/* // and any currently-blocked Read call. */
/* // A zero value for t means Read will not time out. */
/* // fn SetReadDeadline(&mut self, t: time::Time) -> Result<(), Error>; */

/* // SetWriteDeadline sets the deadline for future Write calls */
/* // and any currently-blocked Write call. */
/* // Even if write times out, it may return n > 0, indicating that */
/* // some of the data was successfully written. */
/* // A zero value for t means Write will not time out. */
/* // fn SetWriteDeadline(&mut self, t: time::Time) -> Result<(), Error>; */
/* } */

use std::io::prelude::*;
use std::net::TcpStream;
type TcpConn = TcpStream;

/* impl io::Reader for TcpConn {
    fn Read(&mut self, b: Vec<byte>) -> Result<int, Error> {
        let i = self.re(b.clone())?;
        Ok(int!(i))
    }
}

impl io::Writer for TcpConn {
    fn Write(&mut self, b: Vec<byte>) -> Result<int, Error> {
        let i = self.write(b.as_slice())?;
        Ok(int!(i))
    }
} */
