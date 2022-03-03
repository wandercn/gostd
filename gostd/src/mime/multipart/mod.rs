#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::borrow::{Borrow, BorrowMut};
// #[cfg(test)]
// mod tests;
use crate::builtin::*;
use crate::bytes;
use crate::io;
use crate::io::StringWriter;
use crate::strings;
use rand::prelude::*;
use std::collections::HashMap;

use std::io::Error;

#[derive(Debug)]
struct Writer<W>
where
    W: io::Writer,
{
    w: W,
    boundary: String,
    lastpart: bool,
    // lastpart: Option<Box<part<'a>>>,
}

impl<W> Writer<W>
where
    W: io::Writer,
{
    fn new(writer: W) -> Writer<W> {
        Writer {
            w: writer,
            boundary: randomBoundary(),
            lastpart: false,
            // lastpart: None,
        }
    }

    fn Boundary(&self) -> &str {
        &self.boundary
    }

    fn FormDataContentType(&self) -> String {
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

    fn CreatePart(
        &mut self,
        header: HashMap<String, Vec<String>>,
    ) -> Result<Box<dyn io::Writer>, Error> {
        // if self.lastpart.is_some() {
        //     self.lastpart.as_mut().unwrap().close();
        // }
        let mut b = bytes::Buffer::new();
        if self.lastpart {
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

        Ok(Box::new(b))
    }

    fn CreateFormFile(
        &mut self,
        fieldname: &str,
        filename: &str,
    ) -> Result<Box<dyn io::Writer>, Error> {
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

    fn CreateFormField(&mut self, fieldname: &str) -> Result<Box<dyn io::Writer>, Error> {
        let mut h: HashMap<String, Vec<String>> = HashMap::new();
        h.insert(
            "Content-Disposition".to_string(),
            vec![format!(r#"form-data; name="{}""#, escapeQuotes(fieldname))],
        );
        self.CreatePart(h)
    }

    fn WriteField(&mut self, fieldname: &str, value: &str) -> Result<(), Error> {
        let mut p = self.CreateFormField(fieldname)?;
        match p.Write(value.as_bytes().to_vec()) {
            Err(err) => return Err(err),
            Ok(_) => return Ok(()),
        }
    }

    fn Close(&mut self) -> Result<(), Error> {
        if self.lastpart {}
        self.lastpart = true;
        let bound = format!("\r\n--{}--\r\n", self.boundary);
        match self.w.Write(bound.as_bytes().to_vec()) {
            Err(err) => return Err(err),
            Ok(n) => return Ok(()),
        }
    }
}

fn escapeQuotes(s: &str) -> String {
    let p = vec![("\\", "\\\\"), (r#"""#, r#"\\\""#)];
    let r = strings::Replacer::new(p);
    r.Replace(s)
}

fn randomBoundary() -> String {
    "1617b70c8a3c4bc49a9a3ae659fb224f".to_string()
}

struct part<W>
where
    W: io::Writer,
{
    mw: Writer<W>,
    closed: bool,
    we: Error,
}

impl<W> part<W>
where
    W: io::Writer,
{
    fn new(w: Writer<W>) -> part<W> {
        part {
            mw: w,
            closed: false,
            we: Error::new(std::io::ErrorKind::Other, "error"),
        }
    }
    fn close(&mut self) -> Error {
        self.closed = true;
        Error::new(std::io::ErrorKind::Other, self.we.to_string())
    }
}

impl<W> io::Writer for part<W>
where
    W: io::Writer,
{
    fn Write(&mut self, d: Vec<byte>) -> Result<int, Error> {
        if self.closed {
            return Err(Error::new(std::io::ErrorKind::Other, "multipart: can"));
        }
        if let Ok(n) = self.mw.w.Write(d) {
            return Ok(n);
        }
        return Err(Error::new(std::io::ErrorKind::Other, "error"));
    }
}
