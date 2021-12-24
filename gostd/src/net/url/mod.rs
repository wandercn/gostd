//! Package url parses URLs and implements query escaping.
// #![allow(unused_assignments)]
#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[cfg(test)]
mod tests;
use crate::builtin::*;

#[derive(Default, PartialEq, PartialOrd, Debug, Clone)]
pub struct URL {
    pub Scheme: String,
    pub Opaque: String,         // encoded opaque data
    pub User: Option<Userinfo>, // username and password information
    pub Host: String,           // host or host:port
    pub Path: String,           // path (relative paths may omit leading slash)
    pub RawPath: String,        // encoded path hint (see EscapedPath method)
    pub ForceQuery: bool,       // append a query ('?') even if RawQuery is empty
    pub RawQuery: String,       // encoded query values, without '?'
    pub Fragment: String,       // fragment for references, without '#'
    pub RawFragment: String,    // encoded fragment hint (see EscapedFragment method)
}

// The Userinfo type is an immutable encapsulation of username and
// password details for a URL. An existing Userinfo value is guaranteed
// to have a username set (potentially empty, as allowed by RFC 2396),
// and optionally a password.
#[derive(Default, PartialEq, PartialOrd, Debug, Clone)]
pub struct Userinfo {
    username: String,
    password: String,
    passwordSet: bool,
}

impl URL {
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

    pub fn Hostname(&self) -> String {
        let (host, _, _) = strings::Cut(self.Host.as_str(), ":");
        host.to_string()
    }

    pub fn Port(&self) -> String {
        let (_, port, _) = strings::Cut(self.Host.as_str(), ":");
        port.to_string()
    }

    pub fn RequestURI(&self) -> String {
        let mut result = self.Opaque.clone();
        if result == "" {
            result = escape(self.Path.as_str(), Encoding::encodePath);
            if result == "" {
                result = "/".to_owned()
            }
        } else {
            if strings::HasPrefix(result.as_str(), "//") {
                result = strings::Join(vec![self.Scheme.as_str(), result.as_str()], ":").clone();
            }
        }
        if self.ForceQuery || self.RawQuery != "" {
            result = strings::Join(vec![result.as_str(), self.RawQuery.as_str()], "?");
        }
        result
    }
}
use std::collections::HashMap;
use std::io::Error;
use std::io::ErrorKind;
#[derive(Default, PartialEq, Debug, Clone)]
pub struct Values(HashMap<String, Vec<String>>);

impl Values {
    pub fn new(m: HashMap<String, Vec<String>>) -> Values {
        Values(m)
    }
    pub fn Encode(&self) -> String {
        let v = &self.0.clone();
        if v.len() == 0 {
            return "".to_string();
        }
        let mut buf = strings::Builder::new();
        let mut keys: Vec<String> = Vec::with_capacity(len!(self.0));
        for k in v.keys() {
            keys.push(k.to_string());
        }
        keys.sort();
        for k in keys.iter() {
            let vs = v.get(k).unwrap();
            let keyEscaped = QueryEscape(k.as_str());
            for v in vs.into_iter() {
                if buf.Len() > 0 {
                    buf.WriteByte(b'&');
                }
                buf.WriteString(keyEscaped.as_str());
                buf.WriteByte(b'=');
                buf.WriteString(QueryEscape(v).as_str());
            }
        }
        println!("{}", buf.String());
        buf.String()
    }
}
use crate::strings;
/// Parse parses rawurl into a URL structure.
///
/// The rawurl may be relative (a path, without a host) or absolute
/// (starting with a scheme). Trying to parse a hostname and path
/// without a scheme is invalid but may not necessarily return an
/// error, due to parsing ambiguities.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Parse函数解析rawurl为一个URL结构体，rawurl可以是绝对地址，也可以是相对地址。
/// </details>
///
/// # Example
///
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
    if (url.Scheme != "" || !viaRequest && !strings::HasPrefix(rest, "///"))
        && strings::HasPrefix(rest, "//")
    {
        let res2 = strings::Cut(strings::TrimPrefix(rest, "//"), "/");
        url.Host = res2.0.to_string();
        rest = res2.1;
    }
    let mut l = String::from("/");
    l.push_str(rest);
    url.setPath(l.as_str())?;
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
/// QueryUnescape does the inverse transformation of QueryEscape,
/// converting each 3-byte encoded substring of the form "%AB" into the
/// hex-decoded byte 0xAB.
/// It returns an error if any % is not followed by two hexadecimal
/// digits.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// QueryUnescape函数用于将QueryEscape转码的字符串还原。它会把%AB改为字节0xAB，将'+'改为' '。如果有某个%后面未跟两个十六进制数字，本函数会返回错误。
/// </details>
///
/// # Example
///
pub fn QueryUnescape(s: &str) -> Result<string, Error> {
    unescape(s, Encoding::encodeQueryComponent)
}

/// PathUnescape does the inverse transformation of PathEscape,
/// converting each 3-byte encoded substring of the form "%AB" into the
/// hex-decoded byte 0xAB. It returns an error if any % is not followed
/// by two hexadecimal digits.
///
/// PathUnescape is identical to QueryUnescape except that it does not
/// unescape '+' to ' ' (space).
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// PathUnescape进行PathEscape的逆变换，将“%AB”形式的每个3字节编码子字符串转换为十六进制解码字节0xAB。如果未遵循任何%，则返回错误由两个十六进制数字组成。PathunScape与QuerynScape相同，只是它不相同将“+”卸载到“”（空格）。
/// </details>
///
/// # Example
///
pub fn PathUnescape(s: &str) -> Result<string, Error> {
    unescape(s, Encoding::encodePathSegment)
}

/// QueryEscape escapes the string so it can be safely placed
/// inside a URL query.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// QueryEscape函数对s进行转码使之可以安全的用在URL查询里。
/// </details>
///
/// # Example
///
pub fn QueryEscape(s: &str) -> String {
    return escape(s, Encoding::encodeQueryComponent);
}

/// PathEscape escapes the string so it can be safely placed inside a URL path segment,
/// replacing special characters (including /) with %XX sequences as needed.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// PathEscape将转义字符串，以便将其安全地放置在URL路径段中，
/// 根据需要将特殊字符（包括/）替换为%XX序列。
/// </details>
///
/// # Example
///
pub fn PathEscape(s: &str) -> String {
    return escape(s, Encoding::encodePathSegment);
}

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
    let c = c as char;
    if 'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z' || '0' <= c && c <= '9' {
        return false;
    }

    if mode == Encoding::encodeHost || mode == Encoding::encodeZone {
        match c as char {
            '!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '=' | ':' | '[' | ']'
            | '<' | '>' | '"' => return false,
            _ => (),
        }
    }

    match c as char {
        '-' | '_' | '.' | '~' => return false,
        '$' | '&' | '+' | ',' | '/' | ':' | ';' | '=' | '?' | '@' => match mode {
            Encoding::encodePath => return c == '?',
            Encoding::encodePathSegment => return c == '/' || c == ';' || c == ',' || c == '?',
            Encoding::encodeUserPassword => return c == '@' || c == '/' || c == '?' || c == ':',
            Encoding::encodeQueryComponent => return true,
            Encoding::encodeFragment => return false,
            _ => (),
        },
        _ => (),
    }
    if mode == Encoding::encodeFragment {
        match c {
            '!' | '(' | ')' | '*' => return false,
            _ => (),
        }
    }
    true
}
const upperhex: &str = "0123456789ABCDEF";

fn escape(s: &str, mode: Encoding) -> String {
    let mut spaceCount = 0;
    let mut hexCount = 0;
    for i in 0..len!(s) {
        let c = s.as_bytes()[i];
        if shouldEscape(c, mode) {
            if c == b' ' && mode == Encoding::encodeQueryComponent {
                spaceCount += 1;
            } else {
                hexCount += 1;
            }
        }
    }
    if spaceCount == 0 && hexCount == 0 {
        return s.to_string();
    }

    let mut buf: [byte; 64] = [0; 64];
    let mut t = Vec::<byte>::new();
    let mut required = len!(s) + 2 * hexCount;
    if required <= len!(buf) {
        t = buf[..required].to_vec();
    } else {
        t = Vec::<byte>::with_capacity(required);
        for i in 0..t.capacity() {
            t.push(0);
        }
    }

    if hexCount == 0 {
        t.copy_from_slice(s.as_bytes());
        for i in 0..len!(s) {
            if s.as_bytes()[i] == b' ' {
                t[i] = b'+';
            }
        }
        return string(t.as_slice());
    }
    let mut j = 0;
    for i in 0..len!(s) {
        let c = s.as_bytes()[i];

        if c == b' ' && mode == Encoding::encodeQueryComponent {
            t[j] = b'+';
            j += 1;
        } else if shouldEscape(c, mode) {
            t[j] = b'%';
            t[j + 1] = upperhex.as_bytes()[uint!(c >> 4)];
            t[j + 2] = upperhex.as_bytes()[uint!(c & 15)];
            j += 3;
        } else {
            t[j] = s.as_bytes()[i];
            j += 1;
        }
    }
    string(t.as_slice())
}
