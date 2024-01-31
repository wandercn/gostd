use crate::unicode::*;
use lazy_static::lazy_static;
use std::sync::Arc;

pub const MaxRune: i32 = '\u{10FFFF}' as i32; // Maximum valid Unicode code point.
pub const ReplacementChar: i32 = '\u{FFFD}' as i32; // Represents invalid code points.
pub const MaxASCII: i32 = '\u{007F}' as i32; // maximum ASCII value.
pub const MaxLatin1: i32 = '\u{00FF}' as i32; // maximum Latin-1 value.

pub(crate) const PC: isize = 1 << 0; // a control character.
pub(crate) const pP: isize = 1 << 1; // a punctuation character.
pub(crate) const pN: isize = 1 << 2; // a numeral.
pub(crate) const pS: isize = 1 << 3; // a symbolic character.
pub(crate) const pZ: isize = 1 << 4; // a spacing character.
pub(crate) const pLu: isize = 1 << 5; // an upper-case letter.
pub(crate) const pLl: isize = 1 << 6; // a lower-case letter.
pub(crate) const pp: isize = 1 << 7; // a printable character according to Go's definition.
pub(crate) const pg: isize = pp | pZ; // a graphical character according to the Unicode definition.
pub(crate) const pLo: isize = pLl | pLu; // a letter that is neither upper nor lower case.
pub(crate) const pLmask: isize = pLo;

// IsPrint reports whether the rune is defined as printable by Go. Such
// characters include letters, marks, numbers, punctuation, symbols, and the
// ASCII space character, from categories L, M, N, P, S and the ASCII space
// character. This categorization is the same as IsGraphic except that the
// only spacing character is ASCII space, U+0020.
pub fn IsPrint(r: i32) -> bool {
    if r as u32 <= MaxLatin1 as u32 {
        return properties[r as usize] as isize & pp != 0;
    }
    In(r, &PrintRanges)
}

// In reports whether the rune is a member of one of the ranges.
pub fn In(r: i32, ranges: &[Arc<RangeTable>]) -> bool {
    for inside in ranges {
        if Is(inside.clone(), r) {
            return true;
        }
    }
    false
}

lazy_static! {
    static ref PrintRanges: Arc<Vec<Arc<RangeTable>>> =
        Arc::new(vec![L.clone(), M.clone(), N.clone(), P.clone(), S.clone(),]);
}

lazy_static::lazy_static! {
    static ref L: Arc<RangeTable> =_L.clone();
}

lazy_static::lazy_static! {
    static ref M: Arc<RangeTable> = _M.clone();
}
lazy_static::lazy_static! {
    static ref N: Arc<RangeTable> = _N.clone();
}

lazy_static::lazy_static! {
    static ref P: Arc<RangeTable> = _P.clone();
}
lazy_static::lazy_static! {
    static ref S: Arc<RangeTable> = _S.clone();
}
