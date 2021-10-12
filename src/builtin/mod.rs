#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused)]

//! Package builtin binds the basic type in go through the type alias, and implements the basic type cast macro function.
//! <details class="rustdoc-toggle top-doc">
//! <summary class="docblock">zh-cn</summary>
//! builtin 包通过类型别名绑定Go中的基础类型，并实现了基础类型强制转换宏函数。
//! </details>
//!
// prelude
// 导出宏函数 byte!(),int8!()等.避免用gostd::byte!()，可以直接byte!()调用。
pub use super::{
    byte, float32, float64, int, int16, int32, int64, int8, len, rune, uint, uint16, uint32,
    uint64, uint8, uintptr,
};

/// Go: uint8 type eq Rust: u8 .
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Go中的uint8类型 等价于 Rust的u8 。
/// </details>
pub type uint8 = u8;

/// Go: uint16 type eq Rust: u16
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Go中的uint16类型 等价于 Rust的u16
/// </details>
pub type uint16 = u16;

/// Go: uint32 type eq Rust: u32
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Go中的uint32类型 等价于 Rust的u32
/// </details>
pub type uint32 = u32;

/// Go: uint64 type eq Rust: u64
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Go中的uint64类型 等价于 Rust的u64
/// </details>
pub type uint64 = u64;

/// Go: int8 type eq Rust: i8
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Go中的int8类型 等价于 Rust的i8
/// </details>
pub type int8 = i8;

/// Go: int16 type eq Rust: i16
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Go中的int16类型 等价于 Rust的i16
/// </details>
pub type int16 = i16;

/// Go: int32 type eq Rust: i32
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Go中的int32类型 等价于 Rust的i32
/// </details>
pub type int32 = i32;

/// Go: int64 type eq Rust: i64
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Go中的int64类型 等价于 Rust的i64
/// </details>
pub type int64 = i64;

/// Go: float32 type eq Rust: float32
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Go中的float32类型 等价于 Rust的 float32
/// </details>
pub type float32 = f32;

/// Go: float64 type eq Rust: float64
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Go中的float64类型 等价于 Rust的 float64
/// </details>
pub type float64 = f64;

// type complex64 is no implementation
// type complex128 is no implementation

/// Go: string type eq Rust: String
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Go中的string类型 等价于 Rust的 String
/// </details>
pub type string = String;

/// Go: int type eq Rust: isize
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Go中的int类型 等价于 Rust的 isize,但是rust中数组和切片中的索引都是usize，相当于Go中的uint。
/// </details>
pub type int = isize;

/// Go: uint type eq Rust: usize
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Go中的uint类型 等价于 Rust的 usize,rust中数组和切片中的索引都是uint。
/// </details>
pub type uint = usize;

/// Go: uintptr eq Rust: usize
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Go中的uintptr类型 等价于 Rust的 usize
/// </details>
pub type uintptr = usize;

/// Go: byte eq Rust: u8
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Go中的byte类型 等价于 Rust的 u8
/// </details>
pub type byte = u8;

/// Go: rune eq Rust: u32
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Go中的rune类型 等价于 Rust的 u32
/// </details>
pub type rune = u32;

pub const iota: int32 = 0;

// type error = std::error::Error;
//
/// uint8!() eq Go: uint8()
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// uint8!() 等价于 Go中的uint8()
/// </details>
///
/// # Example
///
/// ```rust
/// use gostd::builtin::*;
/// let c: int = 128;
/// let a = uint8!(c);
/// let b: uint8 = 128;
/// assert_eq!(a, b)
/// ```
#[macro_export]
macro_rules! uint8 {
    ($x:ident) => {
        ($x as uint8)
    };
    ($x:expr) => {
        ($x as uint8)
    };
}

/// uint16!() eq Go: uint16()
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// uint16!() 等价于 Go中的uint16()
/// </details>
///
/// # Example
///
/// ```rust
/// use gostd::builtin::*;
/// let c: int = 128;
/// let a = uint16!(c);
/// let b: uint16 = 128;
/// assert_eq!(a, b)
/// ```
#[macro_export]
macro_rules! uint16 {
    ($x:ident) => {
        ($x as uint16)
    };
    ($x:expr) => {
        ($x as uint16)
    };
}

/// uint32!() eq Go: uint32()
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// uint32!() 等价于 Go中的uint32()
/// </details>
///
/// # Example
///
/// ```rust
/// use gostd::builtin::*;
/// let c: int = 128;
/// let a = uint32!(c);
/// let b: uint32 = 128;
/// assert_eq!(a, b)
/// ```
#[macro_export]
macro_rules! uint32 {
    ($x:ident) => {
        ($x as uint32)
    };
    ($x:expr) => {
        ($x as uint32)
    };
}

/// uint64!() eq Go: uint64()
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// uint64!() 等价于 Go中的uint64()
/// </details>
///
/// # Example
///
/// ```rust
/// use gostd::builtin::*;
/// let c: int = 128;
/// let a = uint64!(c);
/// let b: uint64 = 128;
/// assert_eq!(a, b)
/// ```
#[macro_export]
macro_rules! uint64 {
    ($x:ident) => {
        $x as uint64;
    };
    ($x:expr) => {
        ($x as uint64)
    };
}

/// int8!() eq Go: int8()
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// int8!() 等价于 Go中的int8()
/// </details>
///
/// # Example
///
/// ```rust
/// use gostd::builtin::*;
/// let c: int = 127;
/// let a = int8!(c);
/// let b: int8 = 127;
/// assert_eq!(a, b)
/// ```
#[macro_export]
macro_rules! int8 {
    ($x:ident) => {
        ($x as int8)
    };
    ($x:expr) => {
        ($x as int8)
    };
}

/// int16!() eq Go: int16()
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// int16!() 等价于 Go中的int16()
/// </details>
///
/// # Example
///
/// ```rust
/// use gostd::builtin::*;
/// let c: int = 30;
/// let a = int16!(c);
/// let b: int16 = 30;
/// assert_eq!(a, b)
/// ```
#[macro_export]
macro_rules! int16 {
    ($x:ident) => {
        ($x as int16)
    };
    ($x:expr) => {
        ($x as int16)
    };
}

/// int32!() eq Go: int32()
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// int32!() 等价于 Go中的int32()
/// </details>
///
/// # Example
///
/// ```rust
/// use gostd::builtin::*;
/// let c: int = 80;
/// let a = int32!(c);
/// let b: int32 = 80;
/// assert_eq!(a, b)
/// ```
#[macro_export]
macro_rules! int32 {
    ($x:ident) => {
        ($x as int32)
    };
    ($x:expr) => {
        ($x as int32)
    };
}

/// int64!() eq Go: int64()
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// int64!() 等价于 Go中的int64()
/// </details>
///
/// # Example
///
/// ```rust
/// use gostd::builtin::*;
/// let c: int = 1000;
/// let a = int64!(c);
/// let b: int64 = 1000;
/// assert_eq!(a, b)
/// ```
#[macro_export]
macro_rules! int64 {
    ($x:ident) => {
        ($x as int64)
    };
    ($x:expr) => {
        ($x as int64)
    };
}

/// float32!() eq Go: float32()
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// float32!() 等价于 Go中的float32()
/// </details>
///
/// # Example
///
/// ```rust
/// use gostd::builtin::*;
/// let c: int32 = 2000;
/// let a = float32!(c);
/// let b: float32 = 2000.0;
/// assert_eq!(a, b)
/// ```
#[macro_export]
macro_rules! float32 {
    ($x:ident) => {
        ($x as float32)
    };
    ($x:expr) => {
        ($x as float32)
    };
}

/// float64!() eq Go: float64()
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// float64!() 等价于 Go中的float64()
/// </details>
///
/// # Example
///
/// ```rust
/// use gostd::builtin::*;
/// let c: int32 = 2000;
/// let a = float64!(c);
/// let b: float64 = 2000.0;
/// assert_eq!(a, b)
/// ```
#[macro_export]
macro_rules! float64 {
    ($x:ident) => {
        ($x as float64)
    };
    ($x:expr) => {
        ($x as float64)
    };
}

/// int!() eq Go: int()
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// int!() 等价于 Go中的int()
/// </details>
///
/// # Example
///
/// ```rust
/// use gostd::builtin::*;
/// let c: float32 = 20.0;
/// let a = int!(c);
/// let b: int = 20;
/// assert_eq!(a, b)
/// ```
#[macro_export]
macro_rules! int {
    ($x:ident) => {
        ($x as int)
    };
    ($x:expr) => {
        ($x as int)
    };
}

/// uint!() eq Go: uint()
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// uint!() 等价于 Go中的uint()
/// </details>
///
/// # Example
///
/// ```rust
/// use gostd::builtin::*;
/// let c: int = 11;
/// let a = uint!(c);
/// let b: uint = 11;
/// assert_eq!(a, b)
/// ```
#[macro_export]
macro_rules! uint {
    ($x:ident) => {
        ($x as uint)
    };
    ($x:expr) => {
        ($x as uint)
    };
}

/// uintptr!() eq Go: uintptr()
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// uintptr!() 等价于 Go中的uintptr()
/// </details>
///
/// # Example
///
/// ```rust
/// use gostd::builtin::*;
/// let c: int = 100;
/// let a = uintptr!(c);
/// let b: uintptr = 100;
/// assert_eq!(a, b)
/// ```
#[macro_export]
macro_rules! uintptr {
    ($x:ident) => {
        ($x as uintptr)
    };
    ($x:expr) => {
        ($x as uintptr)
    };
}

/// byte!() eq Go: byte()
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// byte!() 等价于 Go中的byte()
/// </details>
///
/// # Example
///
/// ```rust
/// use gostd::builtin::*;
/// let c: int = 1;
/// let a = byte!(c);
/// let b: byte = 1;
/// assert_eq!(a, b)
/// ```
#[macro_export]
macro_rules! byte {
    ($x:ident) => {
        ($x as byte)
    };
    ($x:expr) => {
        ($x as byte)
    };
}

/// rune!() eq Go: rune()
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// rune!() 等价于 Go中的rune()
/// </details>
///
/// # Example
///
/// ```rust
/// use gostd::builtin::*;
/// let c: int8 = 1;
/// let a = rune!(c);
/// let b: rune = 1;
/// assert_eq!(a, b)
/// ```
#[macro_export]
macro_rules! rune {
    ($x:ident) => {
        ($x as rune)
    };
    ($x:expr) => {
        ($x as rune)
    };
}

/// string() eq Go: string(), Convert []byte to string .
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// string() 等价于 Go中的string(),把 []byte数组转换成字符串
/// </details>
///
/// # Example
///
/// ```rust
/// use gostd::builtin::*;
/// let sparkle_array:[byte;4] = [240, 159, 146, 150];
/// let sparkle_heart = string(&sparkle_array);
/// assert_eq!("💖", sparkle_heart);
/// ```
pub fn string(b: &[byte]) -> String {
    // String::from_utf8_lossy(b).to_string()
    use std::str;
    let s = unsafe { str::from_utf8_unchecked(b) };
    s.to_string()
}

/// len!() eq Go: len()
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// len!() 等价于 Go中的len()
/// </details>
///
/// # Example
///
/// ```rust
/// use gostd::builtin::*;
/// let l = vec![1,2,34,5];
/// let length = len!(l);
/// assert_eq!(length, 4)
/// ```
#[macro_export]
macro_rules! len {
    ($x:ident) => {
        $x.len()
    }; // ($x:expr) => {
       // ($x.len())
       // };
}
