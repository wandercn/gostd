//! Package url parses URLs and implements query escaping.
// #![allow(unused_assignments)]
#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::builtin::*;

#[derive(Default, PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct URL<'a> {
    pub Scheme: &'a str,
    pub Opaque: &'a str,            // encoded opaque data
    pub User: Option<Userinfo<'a>>, // username and password information
    pub Host: &'a str,              // host or host:port
    pub Path: &'a str,              // path (relative paths may omit leading slash)
    pub RawPath: &'a str,           // encoded path hint (see EscapedPath method)
    pub ForceQuery: bool,           // append a query ('?') even if RawQuery is empty
    pub RawQuery: &'a str,          // encoded query values, without '?'
    pub Fragment: &'a str,          // fragment for references, without '#'
    pub RawFragment: &'a str,       // encoded fragment hint (see EscapedFragment method)
}

// The Userinfo type is an immutable encapsulation of username and
// password details for a URL. An existing Userinfo value is guaranteed
// to have a username set (potentially empty, as allowed by RFC 2396),
// and optionally a password.
#[derive(Default, PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Userinfo<'a> {
    username: &'a str,
    password: &'a str,
    passwordSet: bool,
}

impl<'a> URL<'a> {
    pub fn String(&self) -> String {
        todo!()
    }
}
use std::collections::HashMap;
pub struct Values(HashMap<String, Vec<String>>);

impl Values {
    pub fn Encode(&self) -> String {
        todo!()
    }
}
