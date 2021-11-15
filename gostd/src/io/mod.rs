//! This module is waiting to be developed.
#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#[macro_use]
use crate::builtin::*;

pub trait Reader {
    fn Read(&self, p: Vec<byte>) -> Result<int, &str>
    where
        Self: Sized;
}

pub trait Writer {
    fn Write(&self, p: Vec<byte>) -> Result<int, &str>
    where
        Self: Sized;
}

pub trait ReaderAt {
    fn ReadAt(&self, b: Vec<byte>, off: int64) -> Result<int, &str>
    where
        Self: Sized;
}

pub trait ByteReader {
    fn ReadByte(&self) -> Result<byte, &str>
    where
        Self: Sized;
}

pub trait RuneReader {
    fn ReadRune(&self) -> Result<(rune, int), &str>
    where
        Self: Sized;
}

pub trait Seeker {
    fn Seek(&self, offset: int64, whence: int) -> Result<int64, &str>
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
