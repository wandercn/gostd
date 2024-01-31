use crate::unicode::*;
use lazy_static::lazy_static;
use std::sync::Arc;

pub const MaxRune: i32 = '\u{10FFFF}' as i32; // Maximum valid Unicode code point.
pub const ReplacementChar: i32 = '\u{FFFD}' as i32; // Represents invalid code points.
pub const MaxASCII: i32 = '\u{007F}' as i32; // maximum ASCII value.
pub const MaxLatin1: i32 = '\u{00FF}' as i32; // maximum Latin-1 value.

const PC: isize = 1 << 0; // a control character.
const pP: isize = 1 << 1; // a punctuation character.
const pN: isize = 1 << 2; // a numeral.
const pS: isize = 1 << 3; // a symbolic character.
const pZ: isize = 1 << 4; // a spacing character.
const pLu: isize = 1 << 5; // an upper-case letter.
const pLl: isize = 1 << 6; // a lower-case letter.
const pp: isize = 1 << 7; // a printable character according to Go's definition.
const pg: isize = pp | pZ; // a graphical character according to the Unicode definition.
const pLo: isize = pLl | pLu; // a letter that is neither upper nor lower case.
const pLmask: isize = pLo;

const properties: [u8; MaxLatin1 as usize + 1] = [
    PC as u8,         // '\x00'
    PC as u8,         // '\x01'
    PC as u8,         // '\x02'
    PC as u8,         // '\x03'
    PC as u8,         // '\x04'
    PC as u8,         // '\x05'
    PC as u8,         // '\x06'
    PC as u8,         // '\a'
    PC as u8,         // '\b'
    PC as u8,         // '\t'
    PC as u8,         // '\n'
    PC as u8,         // '\v'
    PC as u8,         // '\f'
    PC as u8,         // '\r'
    PC as u8,         // '\x0e'
    PC as u8,         // '\x0f'
    PC as u8,         // '\x10'
    PC as u8,         // '\x11'
    PC as u8,         // '\x12'
    PC as u8,         // '\x13'
    PC as u8,         // '\x14'
    PC as u8,         // '\x15'
    PC as u8,         // '\x16'
    PC as u8,         // '\x17'
    PC as u8,         // '\x18'
    PC as u8,         // '\x19'
    PC as u8,         // '\x1a'
    PC as u8,         // '\x1b'
    PC as u8,         // '\x1c'
    PC as u8,         // '\x1d'
    PC as u8,         // '\x1e'
    PC as u8,         // '\x1f'
    (pZ | pp) as u8,  // ' '
    (pP | pp) as u8,  // '!'
    (pP | pp) as u8,  // '"'
    (pP | pp) as u8,  // '#'
    (pS | pp) as u8,  // '$'
    (pP | pp) as u8,  // '%'
    (pP | pp) as u8,  // '&'
    (pP | pp) as u8,  // '\''
    (pP | pp) as u8,  // '('
    (pP | pp) as u8,  // ')'
    (pP | pp) as u8,  // '*'
    (pS | pp) as u8,  // '+'
    (pP | pp) as u8,  // ','
    (pP | pp) as u8,  // '-'
    (pP | pp) as u8,  // '.'
    (pP | pp) as u8,  // '/'
    (pN | pp) as u8,  // '0'
    (pN | pp) as u8,  // '1'
    (pN | pp) as u8,  // '2'
    (pN | pp) as u8,  // '3'
    (pN | pp) as u8,  // '4'
    (pN | pp) as u8,  // '5'
    (pN | pp) as u8,  // '6'
    (pN | pp) as u8,  // '7'
    (pN | pp) as u8,  // '8'
    (pN | pp) as u8,  // '9'
    (pP | pp) as u8,  // ':'
    (pP | pp) as u8,  // ';'
    (pS | pp) as u8,  // '<'
    (pS | pp) as u8,  // '='
    (pS | pp) as u8,  // '>'
    (pP | pp) as u8,  // '?'
    (pP | pp) as u8,  // '@'
    (pLu | pp) as u8, // 'A'
    (pLu | pp) as u8, // 'B'
    (pLu | pp) as u8, // 'C'
    (pLu | pp) as u8, // 'D'
    (pLu | pp) as u8, // 'E'
    (pLu | pp) as u8, // 'F'
    (pLu | pp) as u8, // 'G'
    (pLu | pp) as u8, // 'H'
    (pLu | pp) as u8, // 'I'
    (pLu | pp) as u8, // 'J'
    (pLu | pp) as u8, // 'K'
    (pLu | pp) as u8, // 'L'
    (pLu | pp) as u8, // 'M'
    (pLu | pp) as u8, // 'N'
    (pLu | pp) as u8, // 'O'
    (pLu | pp) as u8, // 'P'
    (pLu | pp) as u8, // 'Q'
    (pLu | pp) as u8, // 'R'
    (pLu | pp) as u8, // 'S'
    (pLu | pp) as u8, // 'T'
    (pLu | pp) as u8, // 'U'
    (pLu | pp) as u8, // 'V'
    (pLu | pp) as u8, // 'W'
    (pLu | pp) as u8, // 'X'
    (pLu | pp) as u8, // 'Y'
    (pLu | pp) as u8, // 'Z'
    (pP | pp) as u8,  // '['
    (pP | pp) as u8,  // '\\'
    (pP | pp) as u8,  // ']'
    (pS | pp) as u8,  // '^'
    (pP | pp) as u8,  // '_'
    (pS | pp) as u8,  // '`'
    (pLl | pp) as u8, // 'a'
    (pLl | pp) as u8, // 'b'
    (pLl | pp) as u8, // 'c'
    (pLl | pp) as u8, // 'd'
    (pLl | pp) as u8, // 'e'
    (pLl | pp) as u8, // 'f'
    (pLl | pp) as u8, // 'g'
    (pLl | pp) as u8, // 'h'
    (pLl | pp) as u8, // 'i'
    (pLl | pp) as u8, // 'j'
    (pLl | pp) as u8, // 'k'
    (pLl | pp) as u8, // 'l'
    (pLl | pp) as u8, // 'm'
    (pLl | pp) as u8, // 'n'
    (pLl | pp) as u8, // 'o'
    (pLl | pp) as u8, // 'p'
    (pLl | pp) as u8, // 'q'
    (pLl | pp) as u8, // 'r'
    (pLl | pp) as u8, // 's'
    (pLl | pp) as u8, // 't'
    (pLl | pp) as u8, // 'u'
    (pLl | pp) as u8, // 'v'
    (pLl | pp) as u8, // 'w'
    (pLl | pp) as u8, // 'x'
    (pLl | pp) as u8, // 'y'
    (pLl | pp) as u8, // 'z'
    (pP | pp) as u8,  // '{'
    (pS | pp) as u8,  // '|'
    (pP | pp) as u8,  // '}'
    (pS | pp) as u8,  // '~'
    PC as u8,         // '\x7f'
    PC as u8,         // '\u0080'
    PC as u8,         // '\u0081'
    PC as u8,         // '\u0082'
    PC as u8,         // '\u0083'
    PC as u8,         // '\u0084'
    PC as u8,         // '\u0085'
    PC as u8,         // '\u0086'
    PC as u8,         // '\u0087'
    PC as u8,         // '\u0088'
    PC as u8,         // '\u0089'
    PC as u8,         // '\u008a'
    PC as u8,         // '\u008b'
    PC as u8,         // '\u008c'
    PC as u8,         // '\u008d'
    PC as u8,         // '\u008e'
    PC as u8,         // '\u008f'
    PC as u8,         // '\u0090'
    PC as u8,         // '\u0091'
    PC as u8,         // '\u0092'
    PC as u8,         // '\u0093'
    PC as u8,         // '\u0094'
    PC as u8,         // '\u0095'
    PC as u8,         // '\u0096'
    PC as u8,         // '\u0097'
    PC as u8,         // '\u0098'
    PC as u8,         // '\u0099'
    PC as u8,         // '\u009a'
    PC as u8,         // '\u009b'
    PC as u8,         // '\u009c'
    PC as u8,         // '\u009d'
    PC as u8,         // '\u009e'
    PC as u8,         // '\u009f'
    pZ as u8,         // '\u00a0'
    (pP | pp) as u8,  // '¡'
    (pS | pp) as u8,  // '¢'
    (pS | pp) as u8,  // '£'
    (pS | pp) as u8,  // '¤'
    (pS | pp) as u8,  // '¥'
    (pS | pp) as u8,  // '¦'
    (pP | pp) as u8,  // '§'
    (pS | pp) as u8,  // '¨'
    (pS | pp) as u8,  // '©'
    (pLo | pp) as u8, // 'ª'
    (pP | pp) as u8,  // '«'
    (pS | pp) as u8,  // '¬'
    0,                // '\u00ad'
    (pS | pp) as u8,  // '®'
    (pS | pp) as u8,  // '¯'
    (pS | pp) as u8,  // '°'
    (pS | pp) as u8,  // '±'
    (pN | pp) as u8,  // '²'
    (pN | pp) as u8,  // '³'
    (pS | pp) as u8,  // '´'
    (pLl | pp) as u8, // 'µ'
    (pP | pp) as u8,  // '¶'
    (pP | pp) as u8,  // '·'
    (pS | pp) as u8,  // '¸'
    (pN | pp) as u8,  // '¹'
    (pLo | pp) as u8, // 'º'
    (pP | pp) as u8,  // '»'
    (pN | pp) as u8,  // '¼'
    (pN | pp) as u8,  // '½'
    (pN | pp) as u8,  // '¾'
    (pP | pp) as u8,  // '¿'
    (pLu | pp) as u8, // 'À'
    (pLu | pp) as u8, // 'Á'
    (pLu | pp) as u8, // 'Â'
    (pLu | pp) as u8, // 'Ã'
    (pLu | pp) as u8, // 'Ä'
    (pLu | pp) as u8, // 'Å'
    (pLu | pp) as u8, // 'Æ'
    (pLu | pp) as u8, // 'Ç'
    (pLu | pp) as u8, // 'È'
    (pLu | pp) as u8, // 'É'
    (pLu | pp) as u8, // 'Ê'
    (pLu | pp) as u8, // 'Ë'
    (pLu | pp) as u8, // 'Ì'
    (pLu | pp) as u8, // 'Í'
    (pLu | pp) as u8, // 'Î'
    (pLu | pp) as u8, // 'Ï'
    (pLu | pp) as u8, // 'Ð'
    (pLu | pp) as u8, // 'Ñ'
    (pLu | pp) as u8, // 'Ò'
    (pLu | pp) as u8, // 'Ó'
    (pLu | pp) as u8, // 'Ô'
    (pLu | pp) as u8, // 'Õ'
    (pLu | pp) as u8, // 'Ö'
    (pS | pp) as u8,  // '×'
    (pLu | pp) as u8, // 'Ø'
    (pLu | pp) as u8, // 'Ù'
    (pLu | pp) as u8, // 'Ú'
    (pLu | pp) as u8, // 'Û'
    (pLu | pp) as u8, // 'Ü'
    (pLu | pp) as u8, // 'Ý'
    (pLu | pp) as u8, // 'Þ'
    (pLl | pp) as u8, // 'ß'
    (pLl | pp) as u8, // 'à'
    (pLl | pp) as u8, // 'á'
    (pLl | pp) as u8, // 'â'
    (pLl | pp) as u8, // 'ã'
    (pLl | pp) as u8, // 'ä'
    (pLl | pp) as u8, // 'å'
    (pLl | pp) as u8, // 'æ'
    (pLl | pp) as u8, // 'ç'
    (pLl | pp) as u8, // 'è'
    (pLl | pp) as u8, // 'é'
    (pLl | pp) as u8, // 'ê'
    (pLl | pp) as u8, // 'ë'
    (pLl | pp) as u8, // 'ì'
    (pLl | pp) as u8, // 'í'
    (pLl | pp) as u8, // 'î'
    (pLl | pp) as u8, // 'ï'
    (pLl | pp) as u8, // 'ð'
    (pLl | pp) as u8, // 'ñ'
    (pLl | pp) as u8, // 'ò'
    (pLl | pp) as u8, // 'ó'
    (pLl | pp) as u8, // 'ô'
    (pLl | pp) as u8, // 'õ'
    (pLl | pp) as u8, // 'ö'
    (pS | pp) as u8,  // '÷'
    (pLl | pp) as u8, // 'ø'
    (pLl | pp) as u8, // 'ù'
    (pLl | pp) as u8, // 'ú'
    (pLl | pp) as u8, // 'û'
    (pLl | pp) as u8, // 'ü'
    (pLl | pp) as u8, // 'ý'
    (pLl | pp) as u8, // 'þ'
    (pLl | pp) as u8, // 'ÿ'
];

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
