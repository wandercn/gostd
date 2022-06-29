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
        let mut buf = strings::Builder::new();
        if self.Scheme != "" {
            buf.WriteString(self.Scheme.as_str());
            buf.WriteByte(b':');
        }
        if self.Opaque != "" {
            buf.WriteString(self.Opaque.as_str());
        } else {
            if self.Scheme != "" || self.Host != "" {
                if self.Host != "" || self.Path != "" {
                    buf.WriteString("//");
                }
                /* let ui = u.User;
                if ui != nil {
                    buf.WriteString(ui.String());
                    buf.WriteByte('@');
                } */
                let h = self.Host.to_string();
                if h != "" {
                    buf.WriteString(escape(h.as_str(), Encoding::encodeHost).as_str());
                }
            }
            let mut path = self.EscapedPath();
            if path != "" && path.as_bytes()[0] != b'/' && self.Host != "" {
                buf.WriteByte(b'/');
            }
            if buf.Len() == 0 {
                // RFC 3986 §4.2
                // A path segment that contains a colon character (e.g., "this:that")
                // cannot be used as the first segment of a relative-path reference, as
                // it would be mistaken for a scheme name. Such a segment must be
                // preceded by a dot-segment (e.g., "./this:that") to make a relative-
                // path reference.
                let i = strings::IndexByte(path.as_str(), b':');
                if i > -1 && strings::IndexByte(&path[..i as usize], b'/') == -1 {
                    buf.WriteString("./");
                }
            }
            buf.WriteString(path.as_str());
        }
        if self.ForceQuery || self.RawQuery != "" {
            buf.WriteByte(b'?');
            buf.WriteString(self.RawQuery.as_str());
        }
        if self.Fragment != "" {
            buf.WriteByte(b'#');
            buf.WriteString(self.EscapedFragment().as_str());
        }
        return buf.String();
    }

    pub fn EscapedFragment(&self) -> String {
        if self.RawFragment != ""
            && validEncoded(self.RawFragment.as_str(), Encoding::encodeFragment)
        {
            if let Ok(f) = unescape(self.RawFragment.as_str(), Encoding::encodeFragment) {
                if f == self.Fragment {
                    return self.RawFragment.to_string();
                }
            }
        }
        escape(self.Fragment.as_str(), Encoding::encodeFragment)
    }

    pub fn EscapedPath(&self) -> String {
        if self.RawPath != "" && validEncoded(self.RawPath.as_str(), Encoding::encodePath) {
            if let Ok(p) = unescape(self.RawPath.as_str(), Encoding::encodePath) {
                if p == self.Path {
                    return self.RawPath.to_string();
                }
            }
        }
        if self.Path == "*" {
            return "*".to_string(); // don't escape (Issue 11202)
        }
        escape(self.Path.as_str(), Encoding::encodePath)
    }

    fn setFragment(&mut self, f: &str) -> Result<(), Error> {
        let frag = unescape(f, Encoding::encodeFragment)?;
        self.Fragment = frag.to_string();
        let escf = escape(frag.as_str(), Encoding::encodeFragment);
        if f == escf {
            // Default encoding is fine.
            self.RawFragment = "".to_string();
        } else {
            self.RawFragment = f.to_string();
        }
        Ok(())
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

    pub fn Parse(&self, url: &str) -> Result<URL, Error> {
        let refurl = Parse(url)?;
        Ok(self.ResolveReference(refurl))
    }

    pub fn ResolveReference(&self, refurl: URL) -> URL {
        let mut url = refurl.clone();
        if refurl.Scheme == "" {
            url.Scheme = self.Scheme.to_owned();
        }
        if refurl.Scheme != "" || refurl.Host != "" {
            // || refurl.User != nil
            // The "absoluteURI" or "net_path" cases.
            // We can ignore the error from setPath since we know we provided a
            // validly-escaped path.
            url.setPath(resolvePath(refurl.EscapedPath().as_str(), "").as_str());
            return url;
        }
        if refurl.Opaque != "" {
            // url.User = nil;
            url.Host = "".to_string();
            url.Path = "".to_string();
            return url;
        }
        if refurl.Path == "" && refurl.RawQuery == "" {
            url.RawQuery = self.RawQuery.to_string();
            if refurl.Fragment == "" {
                url.Fragment = self.Fragment.to_string();
                url.RawFragment = self.RawFragment.to_string();
            }
        }
        // The "abs_path" or "rel_path" cases.
        url.Host = self.Host.to_string();
        // url.User = self.User;
        url.setPath(
            resolvePath(self.EscapedPath().as_str(), refurl.EscapedPath().as_str()).as_str(),
        );
        url
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

fn resolvePath(base: &str, refurl: &str) -> String {
    let mut full = String::new();
    if refurl == "" {
        full = base.to_string();
    } else if refurl.as_bytes()[0] != b'/' {
        let i = strings::LastIndex(base, "/");
        full = strings::Join(vec![&base[..i as usize + 1], refurl], "").to_owned();
    } else {
        full = refurl.to_string();
    }
    if full == "" {
        return "".to_string();
    }

    let mut last = String::new();
    let mut elem = String::new();
    let mut i: int = 0;
    let mut dst = strings::Builder::new();
    let mut first = true;
    let mut remaining = full;
    while i >= 0 {
        i = strings::IndexByte(remaining.as_str(), b'/');
        if i < 0 {
            last = remaining.to_string();
            elem = remaining.to_string();
            remaining = "".to_string();
        } else {
            elem = remaining.as_str()[..i as usize].to_string();
            remaining = remaining.as_str()[i as usize + 1..].to_string();
        }
        if elem == "." {
            first = false;
            // drop
            continue;
        }

        if elem == ".." {
            let strs = dst.String();
            let index = strings::LastIndexByte(strs.as_str(), b'/');

            dst.Reset();
            if index == -1 {
                first = true;
            } else {
                dst.WriteString(&strs.as_str()[..index as usize]);
            }
        } else {
            if !first {
                dst.WriteByte(b'/');
            }
            dst.WriteString(elem.as_str());
            first = false;
        }
    }

    if last == "." || last == ".." {
        dst.WriteByte(b'/');
    }
    let mut r = dst.String();
    if len!(r) > 1 && r.as_bytes()[1] == b'/' {
        r = r.as_str()[1..].to_string();
    }
    r
}
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::io::Error;
use std::io::ErrorKind;
#[derive(Default, PartialEq, Debug, Clone)]
pub struct Values(HashMap<String, Vec<String>>);

impl Values {
    pub fn new(m: HashMap<String, Vec<String>>) -> Values {
        Values(m)
    }

    /// Get gets the first value associated with the given key.
    /// If there are no values associated with the key, Get returns
    /// the empty string. To access multiple values, use the HashMap
    /// directly.
    pub fn Get(&self, key: &str) -> String {
        if self.0.is_empty() {
            return "".to_string();
        }
        if let Some(v) = self.0.get(key).and_then(|x| x.get(0)).and_then(|x| Some(x)) {
            return v.to_owned();
        }
        "".to_owned()
    }
    /// Set sets the key to value. It replaces any existing
    /// values.
    pub fn Set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), vec![value.to_string()]);
    }
    /// Add adds the value to key. It appends to any existing
    /// values associated with key.
    pub fn Add(&mut self, key: &str, value: &str) {
        if let Some(v) = self.0.get_mut(key) {
            v.push(value.to_string())
        } else {
            self.Set(key, value)
        }
    }
    /// Del deletes the values associated with key.
    pub fn Del(&mut self, key: &str) {
        self.0.remove(key);
    }
    /// Has checks whether a given key is set.
    pub fn Has(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }
    /// Encode encodes the values into ``URL encoded'' form
    /// ("bar=baz&foo=quux") sorted by key.
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
        buf.String()
    }
}
/// ParseQuery parses the URL-encoded query string and returns
/// a map listing the values specified for each key.
/// ParseQuery always returns a non-nil map containing all the
/// valid query parameters found; err describes the first decoding error
/// encountered, if any.
///
/// Query is expected to be a list of key=value settings separated by ampersands.
/// A setting without an equals sign is interpreted as a key set to an empty
/// value.
/// Settings containing a non-URL-encoded semicolon are considered invalid.
pub fn ParseQuery(mut query: &str) -> Result<Values, Error> {
    let mut m = Values::new(HashMap::new());
    parseQuery(&mut m, query)?;
    Ok(m)
}

fn parseQuery(m: &mut Values, mut query: &str) -> Result<(), Error> {
    while !query.is_empty() {
        let mut key = query;
        let i = strings::IndexAny(key, "&");
        if i >= 0 {
            query = key.get((i + 1) as usize..).unwrap();
            key = key.get(..i as usize).unwrap();
        } else {
            query = "";
        }
        if strings::Contains(key, ";") {
            continue;
        }
        if key.is_empty() {
            continue;
        }
        let mut value = "".to_string();
        let i = strings::Index(key, "=");
        if i >= 0 {
            value = key.get((i + 1) as usize..).unwrap().to_string();
            key = key.get(..i as usize).unwrap();
        }

        let value1 = QueryUnescape(&value)?;
        let key1 = QueryUnescape(key)?;
        m.Add(&key1, &value1);
    }
    Ok(())
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

fn ishex(c: byte) -> bool {
    match c {
        b'0'..=b'9' => return true,
        b'a'..=b'f' => return true,
        b'A'..=b'F' => return true,
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
                if i + 2 >= len!(s) || !ishex(s.as_bytes()[i + 1]) || !ishex(s.as_bytes()[i + 2]) {
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
    let mut i: usize = 0;
    while i < len!(s) {
        match s.as_bytes()[i] {
            b'%' => {
                t.WriteByte(unhex(s.as_bytes()[i + 1]) << 4 | unhex(s.as_bytes()[i + 2]));
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
        i += 1;
    }
    Ok(t.String())
}

fn validEncoded(s: &str, mode: Encoding) -> bool {
    for i in 0..len!(s) {
        // RFC 3986, Appendix A.
        // pchar = unreserved / pct-encoded / sub-delims / ":" / "@".
        // shouldEscape is not quite compliant with the RFC,
        // so we check the sub-delims ourselves and let
        // shouldEscape handle the others.
        match s.as_bytes()[i] as char {
            '!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '=' | ':' | '@' => (),
            // ok
            '[' | ']' =>
            // ok - not specified in RFC 3986 but left alone by modern browsers
            {
                ()
            }
            '%' => (),
            // ok - percent encoded, will decode
            _ => {
                if shouldEscape(s.as_bytes()[i], mode) {
                    return false;
                }
            }
        }
    }
    return true;
}

fn shouldEscape(c: byte, mode: Encoding) -> bool {
    let c = c;
    if b'a' <= c && c <= b'z' || b'A' <= c && c <= b'Z' || b'0' <= c && c <= b'9' {
        return false;
    }

    if mode == Encoding::encodeHost || mode == Encoding::encodeZone {
        match c {
            b'!' | b'$' | b'&' | b'\'' | b'(' | b')' | b'*' | b'+' | b',' | b';' | b'=' | b':'
            | b'[' | b']' | b'<' | b'>' | b'"' => return false,
            _ => (),
        }
    }

    match c {
        b'-' | b'_' | b'.' | b'~' => return false,
        b'$' | b'&' | b'+' | b',' | b'/' | b':' | b';' | b'=' | b'?' | b'@' => match mode {
            Encoding::encodePath => return c == b'?',
            Encoding::encodePathSegment => return c == b'/' || c == b';' || c == b',' || c == b'?',
            Encoding::encodeUserPassword => {
                return c == b'@' || c == b'/' || c == b'?' || c == b':'
            }
            Encoding::encodeQueryComponent => return true,
            Encoding::encodeFragment => return false,
            _ => (),
        },
        _ => (),
    }
    if mode == Encoding::encodeFragment {
        match c {
            b'!' | b'(' | b')' | b'*' => return false,
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
