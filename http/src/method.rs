#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

/// Common HTTP methods.
#[derive(Copy, Clone)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Patch, // RFC 5789
    Delete,
    Connect,
    Options,
    Trace,
}

impl Method {
    pub fn String(&self) -> &str {
        match self {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Patch => "PATCH",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Options => "OPTHONS",
            Method::Trace => "TRACE",
        }
    }
}
