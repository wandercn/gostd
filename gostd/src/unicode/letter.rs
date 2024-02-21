// #![allow(unused_assignments)]
#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::unicode::graphic::*;
use crate::unicode::tables::*;

use std::sync::Arc;

// Is reports whether the rune is in the specified table of ranges.
pub fn Is(range_tab: Arc<RangeTable>, r: i32) -> bool {
    let r16 = &range_tab.R16;
    // Compare as uint32 to correctly handle negative runes.
    if r16.len() > 0 && r as u32 <= r16[r16.len() - 1].Hi as u32 {
        return is16(r16, r as u16);
    }
    let r32 = &range_tab.R32;
    if r32.len() > 0 && r as u32 >= r32[0].Lo {
        return is32(r32, r as u32);
    }
    return false;
}

const linearMax: usize = 18;

// is16 reports whether r is in the sorted slice of 16-bit ranges.
fn is16(ranges: &[Range16], r: u16) -> bool {
    if ranges.len() <= linearMax || r <= MaxLatin1 as u16 {
        for range_ in ranges {
            if r < range_.Lo {
                return false;
            }
            if r <= range_.Hi {
                return range_.Stride == 1 || (r - range_.Lo) % range_.Stride == 0;
            }
        }
        return false;
    }

    // binary search over ranges
    let mut lo = 0;
    let mut hi = ranges.len();
    while lo < hi {
        let m = lo + (hi - lo) / 2;
        let range_ = &ranges[m];
        if range_.Lo <= r && r <= range_.Hi {
            return range_.Stride == 1 || (r - range_.Lo) % range_.Stride == 0;
        }
        if r < range_.Lo {
            hi = m;
        } else {
            lo = m + 1;
        }
    }
    return false;
}

// is32 reports whether r is in the sorted slice of 32-bit ranges.
fn is32(ranges: &[Range32], r: u32) -> bool {
    if ranges.len() <= linearMax {
        for range_ in ranges {
            if r < range_.Lo {
                return false;
            }
            if r <= range_.Hi {
                return range_.Stride == 1 || (r - range_.Lo) % range_.Stride == 0;
            }
        }
        return false;
    }

    // binary search over ranges
    let mut lo = 0;
    let mut hi = ranges.len();
    while lo < hi {
        let m = lo + (hi - lo) / 2;
        let range_ = &ranges[m];
        if range_.Lo <= r && r <= range_.Hi {
            return range_.Stride == 1 || (r - range_.Lo) % range_.Stride == 0;
        }
        if r < range_.Lo {
            hi = m;
        } else {
            lo = m + 1;
        }
    }
    return false;
}

pub(crate) fn isExcludingLatin(range_tab: Arc<RangeTable>, r: i32) -> bool {
    let r16 = &range_tab.R16;
    // Compare as uint32 to correctly handle negative runes.
    if r16.len() > range_tab.LatinOffset && r as u32 <= r16[r16.len() - 1].Hi as u32 {
        return is16(&r16[range_tab.LatinOffset..], r as u16);
    }
    let r32 = &range_tab.R32;
    if r32.len() > 0 && r as u32 >= r32[0].Lo {
        return is32(r32, r as u32);
    }
    return false;
}

#[test]
fn test_negative_rune() {
    // Issue 43254
    // These tests cover negative rune handling by testing values which,
    // when cast to uint8 or uint16, look like a particular valid rune.
    // This package has Latin-1-specific optimizations, so we test all of
    // Latin-1 and representative non-Latin-1 values in the character
    // categories covered by IsGraphic, etc.
    let non_latin1: Vec<u32> = vec![
        0x0100, // Lu: LATIN CAPITAL LETTER A WITH MACRON
        0x0101, // Ll: LATIN SMALL LETTER A WITH MACRON
        0x01C5, // Lt: LATIN CAPITAL LETTER D WITH SMALL LETTER Z WITH CARON
        0x0300, // M: COMBINING GRAVE ACCENT
        0x0660, // Nd: ARABIC-INDIC DIGIT ZERO
        0x037E, // P: GREEK QUESTION MARK
        0x02C2, // S: MODIFIER LETTER LEFT ARROWHEAD
        0x1680, // Z: OGHAM SPACE MARK
    ];
    for i in 0..(MaxLatin1 as isize + non_latin1.len() as isize) {
        let base: u32 = if i >= MaxLatin1 as isize {
            non_latin1[(i - MaxLatin1 as isize) as usize]
        } else {
            i as u32
        };
        println!("base(u32): {}", base);
        let r: i32 = (base as i64 - ((1 << 31) as i64)) as i32;
        assert_eq!(Is(Letter.clone(), r), false);
        assert_eq!(IsLetter(r), false);
        assert_eq!(IsPrint(r), false);
    }
}
