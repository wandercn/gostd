//! This module is waiting to be developed.
#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#[macro_use]
use crate::builtin::*;
use std::io::Error;

pub enum Whence {
    SeekStat,
    SeekCurrent,
    SeekEnd,
}
pub trait Reader {
    fn Read(&mut self, b: Vec<byte>) -> Result<int, Error>
    where
        Self: Sized;
}

pub trait Writer {
    fn Write(&mut self, b: Vec<byte>) -> Result<int, Error>;
}

pub trait ReaderAt {
    fn ReadAt(&mut self, b: Vec<byte>, off: int64) -> Result<int, Error>
    where
        Self: Sized;
}

pub trait ByteReader {
    fn ReadByte(&mut self) -> Result<byte, Error>
    where
        Self: Sized;
}

pub trait RuneReader {
    fn ReadRune(&mut self) -> Result<(rune, int), Error>
    where
        Self: Sized;
}

pub trait Seeker {
    fn Seek(&mut self, offset: int64, whence: Whence) -> int64
    where
        Self: Sized;
}

pub trait ByteScanner {
    fn UnreadByte(&mut self) -> Result<(), Error>
    where
        Self: Sized;
}

pub trait ByteWriter {
    fn WriteByte(&mut self, c: byte) -> Result<(), Error>;
}

pub trait WriterTo {
    fn WriteTo(&mut self, w: Box<dyn Writer>) -> Result<int64, Error>
    where
        Self: Sized;
}

pub trait StringWriter {
    fn WriteString(&mut self, s: &str) -> Result<int, Error>
    where
        Self: Sized;
}

pub fn WriteString(mut w: Box<dyn Writer>, s: &str) -> Result<int, Error> {
    w.Write(s.as_bytes().to_owned())
}
