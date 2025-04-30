#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::collections::HashSet;

use crate::{header::Header, response::Response};
use gostd_builtin::*;
use gostd_strings as strings;
use gostd_time as time;
use gostd_url as url;

fn validHeaderFieldByte(b: byte) -> bool {
    let isTokenTable: HashSet<char> = [
        '!', '#', '$', '%', '&', '\'', '*', '+', '.', '0', '1', '2', '3', '4', '5', '6', '7', '8',
        '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
        'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f',
        'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
        'y', 'z', '|', '~',
    ]
    .iter()
    .cloned()
    .collect();

    isTokenTable.contains(&(b as char))
}
fn isCookieNameValid(raw: &str) -> bool {
    if raw == "" {
        return false;
    }
    strings::IndexFunc(raw, isNotToken) < 0
}

fn isNotToken(r: rune) -> bool {
    !validHeaderFieldByte(r as u8)
}

fn validCookieValueByte(b: byte) -> bool {
    return 0x20 <= b && b < 0x7f && b != b'"' && b != b';' && b != b'\\';
}

fn parseCookieValue(mut raw: &str, allowDoubleQuote: bool) -> (string, bool) {
    // Strip the quotes, if present.
    if allowDoubleQuote
        && len!(raw) > 1
        && raw.bytes().nth(0) == Some(b'"')
        && raw.bytes().nth(len!(raw) - 1) == Some(b'"')
    {
        raw = &raw[1..len!(raw) - 1]
    }
    for i in 0..len!(raw) {
        if !validCookieValueByte(raw.as_bytes()[i as usize]) {
            return ("".to_string(), false);
        }
    }
    return (raw.to_string(), true);
}

impl Response {
    pub fn Cookies(&self) -> Vec<Cookie> {
        readSetCookies(&self.Header)
    }
}

fn readSetCookies(h: &Header) -> Vec<Cookie> {
    let cookieCount = len!(h.0.get(&"Set-Cookie".to_string()).unwrap());
    if cookieCount == 0 {
        return vec![];
    }
    let mut cookies = Vec::with_capacity(cookieCount);
    for line in h.0.get("Set-Cookie").unwrap() {
        let mut parts = strings::Split(strings::TrimSpace(line.as_str()), ";");
        if len!(parts) == 1 && parts[0] == "" {
            continue;
        }
        parts[0] = strings::TrimSpace(parts[0]);

        let j = strings::Index(parts[0], "=");
        if j < 0 {
            continue;
        }
        let mut name = &parts[0][..j as usize];
        let mut value = &parts[0][j as usize + 1..];
        if !isCookieNameValid(name) {
            continue;
        }
        let cookie = parseCookieValue(value, true);
        value = &cookie.0;
        let ok = &cookie.1;
        if !ok {
            continue;
        }
        let mut c = Cookie::default();
        c.Name = name.to_string();
        c.Value = value.to_string();
        c.Raw = line.to_string();

        for i in 1..len!(parts) {
            parts[i] = strings::TrimSpace(parts[i]);
            if len!(parts[i]) == 0 {
                continue;
            }
            let mut attr = parts[i];
            let mut val = "";
            let j = strings::Index(attr, "=");
            if j >= 0 {
                attr = &attr[..j as usize];
                val = &attr[j as usize + 1..];
            }
            if !attr.is_ascii() {
                continue;
            }

            let cok = parseCookieValue(val, false);
            val = &cok.0;
            let ok = &cok.1;
            if !ok {
                c.Unparsed.push(parts[i].to_string());
                continue;
            }
            let lowerAttr = strings::ToLower(attr);
            match lowerAttr.as_str() {
                "sameste" => {
                    if !val.is_ascii() {
                        c.SameSite = SameSite::SameSiteDefaultMode;
                        continue;
                    }
                    let lowerVal = strings::ToLower(val);
                    match lowerVal.as_str() {
                        "lax" => c.SameSite = SameSite::SameSiteLaxMode,
                        "strict" => c.SameSite = SameSite::SameSiteStrictMode,
                        "none" => c.SameSite = SameSite::SameSiteNoneModepub,
                        _ => c.SameSite = SameSite::SameSiteDefaultMode,
                    }
                    continue;
                }
                "secure" => {
                    c.Secure = true;
                    continue;
                }
                "httponly" => {
                    c.HttpOnly = true;
                    continue;
                }
                "domain" => {
                    c.Domain = val.to_string();
                    continue;
                }
                "max-age" => {
                    let mut secs: int = 0;
                    let res = val.parse::<int>();
                    if res.is_err() || (secs != 0 && val.bytes().nth(0) == Some(b'0')) {
                        continue;
                    }
                    secs = res.unwrap();
                    if secs <= 0 {
                        secs = -1;
                    }
                    c.MaxAge = secs;
                    continue;
                }
                "expires" => {
                    c.RawExpires = val.to_string();
                    if let Ok(mut exptime) = time::Parse(time::RFC1123, val) {
                        c.Expires = exptime.UTC();
                    } else {
                        if let Ok(mut exptime) = time::Parse("Mon, 02-Jan-2006 15:04:05 MST", val) {
                            c.Expires = exptime.UTC();
                        } else {
                            c.Expires = time::Time::default();
                            continue;
                        }
                    }
                    continue;
                }
                "path" => {
                    c.Path = val.to_string();
                    continue;
                }
                _ => (),
            }
            c.Unparsed.push(parts[i].to_string());
        }
        cookies.push(c);
    }
    cookies
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum SameSite {
    SameSiteDefaultMode,
    SameSiteLaxMode,
    SameSiteStrictMode,
    SameSiteNoneModepub,
}

impl Default for SameSite {
    fn default() -> Self {
        SameSite::SameSiteDefaultMode
    }
}
pub trait CookieJar {
    fn SetCookies(&mut self, u: &url::URL, cookies: Vec<Cookie>);

    fn Cookies(&self, u: &url::URL) -> Vec<Cookie>;
}

#[derive(Default, PartialEq, PartialOrd, Debug, Clone)]
pub struct Cookie {
    Name: String,
    Value: String,
    Path: String,        // optional
    Domain: String,      // optional
    Expires: time::Time, // optional
    RawExpires: String,  // for reading cookies only

    // MaxAge=0 means no 'Max-Age' attribute specified.
    // MaxAge<0 means delete cookie now, equivalently 'Max-Age: 0'
    // MaxAge>0 means Max-Age attribute present and given in seconds
    MaxAge: int,
    Secure: bool,
    HttpOnly: bool,
    SameSite: SameSite,
    Raw: String,
    Unparsed: Vec<String>, // Raw text of unparsed attribute-value pairs
}

impl CookieJar for Cookie {
    fn SetCookies(&mut self, u: &url::URL, cookies: Vec<Cookie>) {
        todo!()
    }

    fn Cookies(&self, u: &url::URL) -> Vec<Cookie> {
        todo!()
    }
}
// SameSite allows a server to define a cookie attribute making it impossible for
// the browser to send this cookie along with cross-site requests. The main
// goal is to mitigate the risk of cross-origin information leakage, and provide
// some protection against cross-site request forgery attacks.
//
// See https://tools.ietf.org/html/draft-ietf-httpbis-cookie-same-site-00 for details.
// type SameSite = int;
