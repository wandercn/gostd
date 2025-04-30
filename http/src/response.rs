#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use bytes::BytesMut;
use gostd_builtin::*;

use crate::{header::Header, request::Request};

#[derive(Default, Debug, Clone)]
pub struct Response {
    pub Status: String,
    pub StatusCode: int,
    pub Proto: String,
    pub ProtoMajor: int,
    pub ProtoMinor: int,
    pub Header: Header,
    pub ContentLength: int64,
    pub TransferEncoding: Vec<String>,
    pub Body: Option<BytesMut>,
    pub Close: bool,
    pub Uncompressed: bool,
    pub Trailer: Header,
    pub Request: Request,
}
