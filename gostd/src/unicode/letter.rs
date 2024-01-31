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
