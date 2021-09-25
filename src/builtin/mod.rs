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

// float32 is the set of all IEEE-754 32-bit floating-point numbers.
pub type float32 = f32;

// float64 is the set of all IEEE-754 64-bit floating-point numbers.
pub type float64 = f64;
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

// type error = std!!!::error::Error;
#[macro_export]
macro_rules! uint8 {
    ($x:ident) => {
        ($x as uint8)
    };
    ($x:expr) => {
        ($x as uint8)
    };
}

#[macro_export]
macro_rules! uint16 {
    ($x:ident) => {
        ($x as uint16)
    };
    ($x:expr) => {
        ($x as uint16)
    };
}

#[macro_export]
macro_rules! uint32 {
    ($x:ident) => {
        ($x as uint32)
    };
    ($x:expr) => {
        ($x as uint32)
    };
}

#[macro_export]
macro_rules! uint64 {
    ($x:ident) => {
        $x as uint64;
    };
    ($x:expr) => {
        ($x as uint64)
    };
}

#[macro_export]
macro_rules! int8 {
    ($x:ident) => {
        ($x as int8)
    };
    ($x:expr) => {
        ($x as int8)
    };
}

#[macro_export]
macro_rules! int16 {
    ($x:ident) => {
        ($x as int16)
    };
    ($x:expr) => {
        ($x as int16)
    };
}

#[macro_export]
macro_rules! int32 {
    ($x:ident) => {
        ($x as int32)
    };
    ($x:expr) => {
        ($x as int32)
    };
}

// 强制类型转换宏 int64!() 等价于 as int64
#[macro_export]
macro_rules! int64 {
    ($x:ident) => {
        ($x as int64)
    };
    ($x:expr) => {
        ($x as int64)
    };
}

#[macro_export]
macro_rules! float32 {
    ($x:ident) => {
        ($x as float32)
    };
    ($x:expr) => {
        ($x as float32)
    };
}

#[macro_export]
macro_rules! float64 {
    ($x:ident) => {
        ($x as float64)
    };
    ($x:expr) => {
        ($x as float64)
    };
}

#[macro_export]
macro_rules! int {
    ($x:ident) => {
        ($x as int)
    };
    ($x:expr) => {
        ($x as int)
    };
}

#[macro_export]
macro_rules! uint {
    ($x:ident) => {
        ($x as uint)
    };
    ($x:expr) => {
        ($x as uint)
    };
}

#[macro_export]
macro_rules! uintptr {
    ($x:ident) => {
        ($x as uintptr)
    };
    ($x:expr) => {
        ($x as uintptr)
    };
}

#[macro_export]
macro_rules! byte {
    ($x:ident) => {
        ($x as byte)
    };
    ($x:expr) => {
        ($x as byte)
    };
}

#[macro_export]
macro_rules! rune {
    ($x:ident) => {
        ($x as rune)
    };
    ($x:expr) => {
        ($x as rune)
    };
}

pub fn string(b: &[byte]) -> string {
    // String::from_utf8_lossy(b).to_string()
    string::from_utf8(b[..].to_vec()).unwrap()
}
