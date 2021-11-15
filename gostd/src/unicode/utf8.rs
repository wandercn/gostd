#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#[macro_use]
use crate::builtin::*;
pub const RuneError: rune = 0xFFFD; // the "error" Rune or "Unicode replacement character"
pub const RuneSelf: rune = 0x80; // characters below RuneSelf are represented as themselves in a single byte.
pub const MaxRune: rune = 0x0010FFFF; // Maximum valid Unicode code point.
pub const UTFMax: int = 4; // maximum number of bytes of a UTF-8 encoded Unicode character.
