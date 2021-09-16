#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused)]

pub type uint8 = u8;
pub type uint16 = u16;
pub type uint32 = u32;
pub type uint64 = u64;

pub type int8 = i8;
pub type int16 = i16;
pub type int32 = i32;
pub type int64 = i64;

// type complex64
// type complex128

pub type string = String;

pub type int = isize;
pub type uint = usize;

pub type uintptr = usize;
pub type byte = uint8;

pub type rune = int32;

pub const iota: int32 = 0;

/* fn new(t: <T>)-><&T>{
   t::defaunt().as_ref()
} */

// type error = std::error::Error;
