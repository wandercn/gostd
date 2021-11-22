//! This module is waiting to be developed.
#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#[macro_use]
use crate::builtin::*;
use std::io::Error;

pub trait Reader {
    fn Read(&mut self, p: Vec<byte>) -> Result<int, Error>
    where
        Self: Sized;
}

pub trait Writer {
    fn Write(&mut self, p: Vec<byte>) -> Result<int, Error>
    where
        Self: Sized;
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
    fn Seek(&mut self, offset: int64, whence: int) -> Result<int64, Error>
    where
        Self: Sized;
}

pub trait ByteScanner {
    fn UnreadByte(&self) -> Result<int, &str>
    where
        Self: Sized;
}

pub trait WriterTo {
    fn WriteTo(&self, w: Box<dyn Writer>) -> Result<int64, &str>
    where
        Self: Sized;
}
