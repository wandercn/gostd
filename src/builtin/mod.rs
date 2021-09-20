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

// type error = std::error::Error;

// 强制类型转换宏 int64() 等价于 as int64
#[macro_export]
macro_rules! int64 {
    ($x:ident) => {
        $x as int64
    };
}
#[macro_export]
macro_rules! int32 {
    ($x:ident) => {
        $x as int32
    };
}
#[macro_export]
macro_rules! uint64 {
    ($x:ident) => {
        $x as uint64
    };
}

#[macro_export]
macro_rules! int {
    ($x:ident) => {
        $x as int
    };
}
