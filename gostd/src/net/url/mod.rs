//! Package url parses URLs and implements query escaping.
// #![allow(unused_assignments)]
#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::builtin::*;

#[derive(Default, PartialEq, PartialOrd, Debug, Clone)]
pub struct URL<'a> {
    pub Scheme: String,
    pub Opaque: String,             // encoded opaque data
    pub User: Option<Userinfo<'a>>, // username and password information
    pub Host: String,               // host or host:port
    pub Path: String,               // path (relative paths may omit leading slash)
    pub RawPath: String,            // encoded path hint (see EscapedPath method)
    pub ForceQuery: bool,           // append a query ('?') even if RawQuery is empty
    pub RawQuery: String,           // encoded query values, without '?'
    pub Fragment: String,           // fragment for references, without '#'
    pub RawFragment: String,        // encoded fragment hint (see EscapedFragment method)
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

    fn setFragment(&mut self, f: &str) -> Result<(), Error> {
        todo!()
    }

    fn setPath(&mut self, p: &str) -> Result<(), Error> {
        let path = unescape(p, Encoding::encodePath)?;
        self.Path = path.clone();
        let escp = escape(path.as_str(), Encoding::encodePath);
        if p == escp.as_str() {
            self.RawPath = "".to_string();
        } else {
            self.RawPath = p.to_string();
        }
        Ok(())
    }
}
use std::collections::HashMap;
use std::io::Error;
use std::io::ErrorKind;
pub struct Values(HashMap<String, Vec<String>>);

impl Values {
    pub fn Encode(&self) -> String {
        todo!()
    }
}
use crate::strings;

pub fn Parse(rawurl: &str) -> Result<URL, Error> {
    // Cut off #frag
    let (u, frag, _) = strings::Cut(rawurl, "#");
    let mut url = parse(u, false)?;
    if frag == "" {
        return Ok(url);
    }
    url.setFragment(frag)?;
    Ok(url)
}

fn parse<'a>(rawurl: &'a str, viaRequest: bool) -> Result<URL, Error> {
    if rawurl == "" && viaRequest {
        return Err(Error::new(ErrorKind::Other, "empty url"));
    }
    let mut url = URL::default();
    if rawurl == "*" {
        url.Path = "*".to_string();
        return Ok(url);
    }
    let (scheme, mut rest) = getscheme(rawurl)?;
    url.Scheme = strings::ToLower(scheme);

    if strings::HasSuffix(rest, "?") && strings::Count(rest, "?") == 1 {
        url.ForceQuery = true;
        rest = &rest[..len!(rest) - 1];
    } else {
        let res = strings::Cut(rest, "?");
        rest = res.0;
        url.RawQuery = res.1.to_string();
    }
    if !strings::HasPrefix(rest, "/") {
        if url.Scheme != "".to_string() {
            url.Opaque = rest.to_string();
            return Ok(url);
        }
        if viaRequest {
            return Err(Error::new(ErrorKind::Other, "invalid URI for request"));
        }
        let colon = strings::Index(rest, ":");
        let slash = strings::Index(rest, "/");
        if colon >= 0 && (slash < 0 || colon < slash) {
            return Err(Error::new(
                ErrorKind::Other,
                "first path segment in URL cannot contain colon",
            ));
        }
    }

    // 待增加基础用户密码验证功能，不太常用咱不实现
    /* if (url.Scheme != "" || !viaRequest && !strings.HasPrefix(rest, "///")) && strings.HasPrefix(rest, "//") {
        var authority string
        authority, rest = split(rest[2:], '/', false)
        url.User, url.Host, err = parseAuthority(authority)
        if err != nil {
            return nil, err
        }
    } */
    url.setPath(rest)?;
    Ok(url)
}

fn getscheme(rawurl: &str) -> Result<(&str, &str), Error> {
    for i in 0..len!(rawurl) {
        let mut c = rawurl.chars().nth(i).unwrap();
        match c {
            'a'..='z' | 'A'..='Z' => (),
            // do nothing
            '0'..='9' | '+' | '-' | '.' => {
                if i == 0 {
                    return Ok(("", rawurl));
                }
            }
            ':' => {
                if i == 0 {
                    return Err(Error::new(ErrorKind::Other, "missing protocol scheme"));
                }
                return Ok((rawurl.get(..i).unwrap(), rawurl.get((i + 1)..).unwrap()));
            }
            _ => return Ok(("", rawurl)),
        }
    }
    Ok(("", rawurl))
}

fn ishex(c: char) -> bool {
    match c {
        '0'..='9' => return true,
        'a'..='f' => return true,
        'A'..='F' => return true,
        _ => (),
    }
    return false;
}

fn unhex(c: byte) -> byte {
    match c {
        b'0'..=b'9' => return c - b'0',
        b'a'..=b'f' => return c - b'a' + 10,
        b'A'..=b'F' => return c - b'A' + 10,
        _ => (),
    }
    return 0;
}
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
enum Encoding {
    encodePath,
    encodePathSegment,
    encodeHost,
    encodeZone,
    encodeUserPassword,
    encodeQueryComponent,
    encodeFragment,
}

use crate::io::*;

fn unescape(mut s: &str, mode: Encoding) -> Result<String, Error> {
    let mut n = 0;
    let mut hasPlus = false;

    for mut i in 0..len!(s) {
        match s.as_bytes()[i] {
            b'%' => {
                n += 1;
                if i + 2 >= len!(s)
                    || !ishex(s.as_bytes()[i + 1] as char)
                    || !ishex(s.as_bytes()[i + 2] as char)
                {
                    s = &s[i..];
                    if len!(s) > 3 {
                        s = &s[..3];
                    }
                    return Err(Error::new(ErrorKind::Other, "invalid URL escape"));
                }

                if mode == Encoding::encodeHost
                    && unhex(s.as_bytes()[i + 1]) < 8
                    && s.get(i..i + 3) != Some("%25")
                {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "invalid URL escape encodeHost",
                    ));
                }
                if mode == Encoding::encodeZone {
                    let v = unhex(s.as_bytes()[i + 1]) << 4 | unhex(s.as_bytes()[i + 2]);
                    if s.get(i..i + 3) != Some("%25")
                        && v != b' '
                        && shouldEscape(v, Encoding::encodeHost)
                    {
                        return Err(Error::new(
                            ErrorKind::Other,
                            "invalid URL escape encodeZone",
                        ));
                    }
                }
                i += 3;
            }
            b'+' => {
                hasPlus = (mode == Encoding::encodeQueryComponent);
                i += 1;
            }
            _ => {
                if (mode == Encoding::encodeHost || mode == Encoding::encodeZone)
                    && s.as_bytes()[i] < 0x80
                    && shouldEscape(s.as_bytes()[i], mode)
                {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "invalid URL escape encodeHost",
                    ));
                }
                i += 1;
            }
        }
    }

    if n == 0 && !hasPlus {
        return Ok(s.to_string());
    }

    let mut t = strings::Builder::new();
    t.Grow(int!(len!(s) - 2 * n));
    for mut i in 0..len!(s) {
        match s.as_bytes()[i] {
            b'%' => {
                t.WriteByte(unhex(s.as_bytes()[i + 1] << 4 | unhex(s.as_bytes()[i + 2])));
                i += 2;
            }
            b'+' => {
                if mode == Encoding::encodeQueryComponent {
                    t.WriteByte(b' ');
                } else {
                    t.WriteByte(b'+');
                }
            }
            _ => {
                t.WriteByte(s.as_bytes()[i]);
            }
        }
    }
    Ok(t.String())
}

fn shouldEscape(c: byte, mode: Encoding) -> bool {
    todo!()
}
fn escape(s: &str, mode: Encoding) -> String {
    todo!()
}
