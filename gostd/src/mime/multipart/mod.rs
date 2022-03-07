#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// #[cfg(test)]
// mod tests;
use crate::io::*;
use crate::{builtin::*, bytes, io, strings};
use rand::RngCore;
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::io::Error;

#[derive(Debug)]
pub struct Writer<'a, W>
where
    W: io::Writer,
{
    w: &'a mut W,
    boundary: String,
    lastpart: bool,
}

impl<'a, W> Writer<'a, W>
where
    W: io::Writer,
{
    pub fn new(writer: &mut W) -> Writer<W> {
        Writer {
            w: writer,
            boundary: randomBoundary(),
            lastpart: false,
        }
    }

    pub fn Boundary(&self) -> &str {
        &self.boundary
    }

    pub fn FormDataContentType(&self) -> String {
        let mut b = "".to_string();
        if strings::ContainsAny(self.boundary.clone().as_str(), "()<>@,;:\"/[]?=") {
            b.push('"');
            b.push_str(self.boundary.clone().as_str());
            b.push('"')
        } else {
            b.push_str(self.boundary.clone().as_str());
        }
        format!("multipart/form-data; boundary={}", b)
    }

    pub fn CreatePart(&mut self, header: HashMap<String, Vec<String>>) -> Result<&mut W, Error> {
        if self.lastpart {
            return Err(Error::new(std::io::ErrorKind::Other, "is closed"));
        }
        let mut b = bytes::Buffer::new();
        if !self.lastpart {
            b.WriteString(format!("\r\n--{}\r\n", self.boundary.clone()).as_str());
        } else {
            b.WriteString(format!("--{}\r\n", self.boundary.clone()).as_str());
        }
        let mut keys: Vec<String> = Vec::with_capacity(len!(header));
        for k in header.keys() {
            keys.push(k.to_owned());
        }
        keys.sort();
        for k in keys {
            for v in header.get(&k).unwrap() {
                b.WriteString(format!("{}: {}\r\n", k, v).as_str());
            }
        }
        b.WriteString("\r\n");
        self.w.Write(b.Bytes());

        Ok(self.w.borrow_mut())
    }

    pub fn CreateFormFile(&mut self, fieldname: &str, filename: &str) -> Result<&mut W, Error> {
        let mut h: HashMap<String, Vec<String>> = HashMap::new();
        h.insert(
            "Content-Disposition".to_string(),
            vec![format!(
                r#"form-data; name="{}"; filename="{}""#,
                escapeQuotes(fieldname),
                escapeQuotes(filename)
            )],
        );
        h.insert(
            "Content-Type".to_string(),
            vec!["application/octet-stream".to_string()],
        );
        self.CreatePart(h)
    }

    pub fn CreateFormField(&mut self, fieldname: &str) -> Result<&mut W, Error> {
        let mut h: HashMap<String, Vec<String>> = HashMap::new();
        h.insert(
            "Content-Disposition".to_string(),
            vec![format!(r#"form-data; name="{}""#, escapeQuotes(fieldname))],
        );
        self.CreatePart(h)
    }

    pub fn WriteField(&mut self, fieldname: &str, value: &str) -> Result<(), Error> {
        let mut p = self.CreateFormField(fieldname)?;
        match p.Write(value.as_bytes().to_vec()) {
            Err(err) => Err(err),
            Ok(_) => Ok(()),
        }
    }

    pub fn Close(&mut self) -> Result<(), Error> {
        if self.lastpart {
            return Err(Error::new(std::io::ErrorKind::Other, "is closed"));
        }
        self.lastpart = true;
        let bound = format!("\r\n--{}--\r\n", self.boundary);
        match self.w.Write(bound.as_bytes().to_vec()) {
            Err(err) => return Err(err),
            Ok(n) => return Ok(()),
            _ => Ok(()),
        }
    }
}

fn escapeQuotes(s: &str) -> String {
    let p = vec![("\\", "\\\\"), (r#"""#, r#"\\\""#)];
    let r = strings::Replacer::new(p);
    r.Replace(s)
}

fn randomBoundary() -> String {
    let mut bytes = [0; 30];
    rand::thread_rng().fill_bytes(&mut bytes);

    fn as_u32(slice: &[u8]) -> u32 {
        let mut copy = [0; 4];
        copy.copy_from_slice(slice);
        u32::from_ne_bytes(copy)
    }

    let a = as_u32(&bytes[0..4]);
    let b = as_u32(&bytes[4..8]);
    let c = as_u32(&bytes[8..12]);
    let d = as_u32(&bytes[12..16]);
    let e = as_u32(&bytes[16..20]);
    let f = as_u32(&bytes[20..24]);
    let g = as_u32(&bytes[24..28]);
    format!("{:x}{:x}{:x}{:x}{:x}{:x}{:x}xx", a, b, c, d, e, f, g)
}
