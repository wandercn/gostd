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
pub const UTFMax: uint = 4; // maximum number of bytes of a UTF-8 encoded Unicode character.

const surrogateMin: int = 0xD800;
const surrogateMax: int = 0xDFFF;

const maskx: int = 0b00111111;
const mask2: int = 0b00011111;
const mask3: int = 0b00001111;
const mask4: int = 0b00000111;

const rune1Max: int = (1 << 7) - 1;
const rune2Max: int = 1 << 11 - 1;
const rune3Max: int = 1 << 16 - 1;

const t1: int = 0b00000000;
const tx: int = 0b10000000;
const t2: int = 0b11000000;
const t3: int = 0b11100000;
const t4: int = 0b11110000;
const t5: int = 0b11111000;

// EncodeRune writes into p (which must be large enough) the UTF-8 encoding of the rune.
// If the rune is out of range, it writes the encoding of RuneError.
// It returns the number of bytes written.
pub fn EncodeRune(mut p: Vec<byte>, mut r: rune) -> int {
    // Negative values are erroneous. Making it unsigned addresses the problem.
    let i = uint32!(r);
    if i <= uint32!(rune1Max) {
        p[0] = byte!(r);
        return 1;
    }

    if i <= uint32!(rune2Max) {
        // _ = p[1]; // eliminate bounds checks
        p[0] = byte!(t2) | byte!(r >> 6);
        p[1] = byte!(tx) | byte!(r) & byte!(maskx);
        return 2;
    }
    if i > MaxRune || uint32!(surrogateMin.abs()) <= i && i <= uint32!(surrogateMax) {
        r = RuneError
    }
    if i <= uint32!(rune3Max) {
        // _ = p[2] // eliminate bounds checks
        p[0] = byte!(t3) | byte!(r >> 12);
        p[1] = byte!(tx) | byte!(r >> 6) & byte!(maskx);
        p[2] = byte!(tx) | byte!(r) & byte!(maskx);
        return 3;
    } else {
        // _ = p[3] // eliminate bounds checks
        p[0] = byte!(t4) | byte!(r >> 18);
        p[1] = byte!(tx) | byte!(r >> 12) & byte!(maskx);
        p[2] = byte!(tx) | byte!(r >> 6) & byte!(maskx);
        p[3] = byte!(tx) | byte!(r) & byte!(maskx);
        return 4;
    }
}
