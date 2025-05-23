// #![allow(unused_assignments)]
#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::graphic::*;
use std::{rc::Rc, sync::Arc};

// Range16 represents of a range of 16-bit Unicode code points. The range runs from Lo to Hi
// inclusive and has the specified stride.
pub struct Range16 {
    pub Lo: u16,
    pub Hi: u16,
    pub Stride: u16,
}

// Range32 represents of a range of Unicode code points and is used when one or
// more of the values will not fit in 16 bits. The range runs from Lo to Hi
// inclusive and has the specified stride. Lo and Hi must always be >= 1<<16.
pub struct Range32 {
    pub Lo: u32,
    pub Hi: u32,
    pub Stride: u32,
}

// RangeTable defines a set of Unicode code points by listing the ranges of
// code points within the set. The ranges are listed in two slices
// to save space: a slice of 16-bit ranges and a slice of 32-bit ranges.
// The two slices must be in sorted order and non-overlapping.
// Also, R32 should contain only values >= 0x10000 (1<<16).
pub struct RangeTable {
    pub R16: Arc<Vec<Range16>>,
    pub R32: Arc<Vec<Range32>>,
    pub LatinOffset: usize,
}

lazy_static::lazy_static! {
    pub static ref _L: Arc<RangeTable> = Arc::new(RangeTable{
        R16:_L_RangeTable_R16.clone(),
        R32:_L_RangeTable_R32.clone(),
        LatinOffset: 6,
    });
}

lazy_static::lazy_static! {
    pub static ref _M: Arc<RangeTable> = Arc::new(RangeTable{
        R16:_M_RangeTable_R16.clone(),
        R32:_M_RangeTable_R32.clone(),
        LatinOffset: 0,
    });
}
lazy_static::lazy_static! {
    pub static ref _N: Arc<RangeTable> = Arc::new(RangeTable{
        R16:_N_RangeTable_R16.clone(),
        R32:_N_RangeTable_R32.clone(),
        LatinOffset: 4,
    });
}

lazy_static::lazy_static! {
    pub static ref _P: Arc<RangeTable> = Arc::new(RangeTable{
        R16:_P_RangeTable_R16.clone(),
        R32:_P_RangeTable_R32.clone(),
        LatinOffset: 11,
    });
}
lazy_static::lazy_static! {
    pub static ref _S: Arc<RangeTable> = Arc::new(RangeTable{
        R16:_S_RangeTable_R16.clone(),
        R32:_S_RangeTable_R32.clone(),
        LatinOffset: 10,
    });
}
lazy_static::lazy_static! {
    pub static ref Letter:Arc<RangeTable> = Arc::clone(&_L);
}

lazy_static::lazy_static! {
    pub static ref _S_RangeTable_R16: Arc<Vec<Range16>> = Arc::new(vec![
        Range16{Lo:0x0024,Hi: 0x002b,Stride: 7},
        Range16{Lo:0x003c,Hi: 0x003e,Stride: 1},
        Range16{Lo:0x005e,Hi: 0x0060,Stride: 2},
        Range16{Lo:0x007c,Hi: 0x007e,Stride: 2},
        Range16{Lo:0x00a2,Hi: 0x00a6,Stride: 1},
        Range16{Lo:0x00a8,Hi: 0x00a9,Stride: 1},
        Range16{Lo:0x00ac,Hi: 0x00ae,Stride: 2},
        Range16{Lo:0x00af,Hi: 0x00b1,Stride: 1},
        Range16{Lo:0x00b4,Hi: 0x00b8,Stride: 4},
        Range16{Lo:0x00d7,Hi: 0x00f7,Stride: 32},
        Range16{Lo:0x02c2,Hi: 0x02c5,Stride: 1},
        Range16{Lo:0x02d2,Hi: 0x02df,Stride: 1},
        Range16{Lo:0x02e5,Hi: 0x02eb,Stride: 1},
        Range16{Lo:0x02ed,Hi: 0x02ef,Stride: 2},
        Range16{Lo:0x02f0,Hi: 0x02ff,Stride: 1},
        Range16{Lo:0x0375,Hi: 0x0384,Stride: 15},
        Range16{Lo:0x0385,Hi: 0x03f6,Stride: 113},
        Range16{Lo:0x0482,Hi: 0x058d,Stride: 267},
        Range16{Lo:0x058e,Hi: 0x058f,Stride: 1},
        Range16{Lo:0x0606,Hi: 0x0608,Stride: 1},
        Range16{Lo:0x060b,Hi: 0x060e,Stride: 3},
        Range16{Lo:0x060f,Hi: 0x06de,Stride: 207},
        Range16{Lo:0x06e9,Hi: 0x06fd,Stride: 20},
        Range16{Lo:0x06fe,Hi: 0x07f6,Stride: 248},
        Range16{Lo:0x07fe,Hi: 0x07ff,Stride: 1},
        Range16{Lo:0x0888,Hi: 0x09f2,Stride: 362},
        Range16{Lo:0x09f3,Hi: 0x09fa,Stride: 7},
        Range16{Lo:0x09fb,Hi: 0x0af1,Stride: 246},
        Range16{Lo:0x0b70,Hi: 0x0bf3,Stride: 131},
        Range16{Lo:0x0bf4,Hi: 0x0bfa,Stride: 1},
        Range16{Lo:0x0c7f,Hi: 0x0d4f,Stride: 208},
        Range16{Lo:0x0d79,Hi: 0x0e3f,Stride: 198},
        Range16{Lo:0x0f01,Hi: 0x0f03,Stride: 1},
        Range16{Lo:0x0f13,Hi: 0x0f15,Stride: 2},
        Range16{Lo:0x0f16,Hi: 0x0f17,Stride: 1},
        Range16{Lo:0x0f1a,Hi: 0x0f1f,Stride: 1},
        Range16{Lo:0x0f34,Hi: 0x0f38,Stride: 2},
        Range16{Lo:0x0fbe,Hi: 0x0fc5,Stride: 1},
        Range16{Lo:0x0fc7,Hi: 0x0fcc,Stride: 1},
        Range16{Lo:0x0fce,Hi: 0x0fcf,Stride: 1},
        Range16{Lo:0x0fd5,Hi: 0x0fd8,Stride: 1},
        Range16{Lo:0x109e,Hi: 0x109f,Stride: 1},
        Range16{Lo:0x1390,Hi: 0x1399,Stride: 1},
        Range16{Lo:0x166d,Hi: 0x17db,Stride: 366},
        Range16{Lo:0x1940,Hi: 0x19de,Stride: 158},
        Range16{Lo:0x19df,Hi: 0x19ff,Stride: 1},
        Range16{Lo:0x1b61,Hi: 0x1b6a,Stride: 1},
        Range16{Lo:0x1b74,Hi: 0x1b7c,Stride: 1},
        Range16{Lo:0x1fbd,Hi: 0x1fbf,Stride: 2},
        Range16{Lo:0x1fc0,Hi: 0x1fc1,Stride: 1},
        Range16{Lo:0x1fcd,Hi: 0x1fcf,Stride: 1},
        Range16{Lo:0x1fdd,Hi: 0x1fdf,Stride: 1},
        Range16{Lo:0x1fed,Hi: 0x1fef,Stride: 1},
        Range16{Lo:0x1ffd,Hi: 0x1ffe,Stride: 1},
        Range16{Lo:0x2044,Hi: 0x2052,Stride: 14},
        Range16{Lo:0x207a,Hi: 0x207c,Stride: 1},
        Range16{Lo:0x208a,Hi: 0x208c,Stride: 1},
        Range16{Lo:0x20a0,Hi: 0x20c0,Stride: 1},
        Range16{Lo:0x2100,Hi: 0x2101,Stride: 1},
        Range16{Lo:0x2103,Hi: 0x2106,Stride: 1},
        Range16{Lo:0x2108,Hi: 0x2109,Stride: 1},
        Range16{Lo:0x2114,Hi: 0x2116,Stride: 2},
        Range16{Lo:0x2117,Hi: 0x2118,Stride: 1},
        Range16{Lo:0x211e,Hi: 0x2123,Stride: 1},
        Range16{Lo:0x2125,Hi: 0x2129,Stride: 2},
        Range16{Lo:0x212e,Hi: 0x213a,Stride: 12},
        Range16{Lo:0x213b,Hi: 0x2140,Stride: 5},
        Range16{Lo:0x2141,Hi: 0x2144,Stride: 1},
        Range16{Lo:0x214a,Hi: 0x214d,Stride: 1},
        Range16{Lo:0x214f,Hi: 0x218a,Stride: 59},
        Range16{Lo:0x218b,Hi: 0x2190,Stride: 5},
        Range16{Lo:0x2191,Hi: 0x2307,Stride: 1},
        Range16{Lo:0x230c,Hi: 0x2328,Stride: 1},
        Range16{Lo:0x232b,Hi: 0x2426,Stride: 1},
        Range16{Lo:0x2440,Hi: 0x244a,Stride: 1},
        Range16{Lo:0x249c,Hi: 0x24e9,Stride: 1},
        Range16{Lo:0x2500,Hi: 0x2767,Stride: 1},
        Range16{Lo:0x2794,Hi: 0x27c4,Stride: 1},
        Range16{Lo:0x27c7,Hi: 0x27e5,Stride: 1},
        Range16{Lo:0x27f0,Hi: 0x2982,Stride: 1},
        Range16{Lo:0x2999,Hi: 0x29d7,Stride: 1},
        Range16{Lo:0x29dc,Hi: 0x29fb,Stride: 1},
        Range16{Lo:0x29fe,Hi: 0x2b73,Stride: 1},
        Range16{Lo:0x2b76,Hi: 0x2b95,Stride: 1},
        Range16{Lo:0x2b97,Hi: 0x2bff,Stride: 1},
        Range16{Lo:0x2ce5,Hi: 0x2cea,Stride: 1},
        Range16{Lo:0x2e50,Hi: 0x2e51,Stride: 1},
        Range16{Lo:0x2e80,Hi: 0x2e99,Stride: 1},
        Range16{Lo:0x2e9b,Hi: 0x2ef3,Stride: 1},
        Range16{Lo:0x2f00,Hi: 0x2fd5,Stride: 1},
        Range16{Lo:0x2ff0,Hi: 0x2ffb,Stride: 1},
        Range16{Lo:0x3004,Hi: 0x3012,Stride: 14},
        Range16{Lo:0x3013,Hi: 0x3020,Stride: 13},
        Range16{Lo:0x3036,Hi: 0x3037,Stride: 1},
        Range16{Lo:0x303e,Hi: 0x303f,Stride: 1},
        Range16{Lo:0x309b,Hi: 0x309c,Stride: 1},
        Range16{Lo:0x3190,Hi: 0x3191,Stride: 1},
        Range16{Lo:0x3196,Hi: 0x319f,Stride: 1},
        Range16{Lo:0x31c0,Hi: 0x31e3,Stride: 1},
        Range16{Lo:0x3200,Hi: 0x321e,Stride: 1},
        Range16{Lo:0x322a,Hi: 0x3247,Stride: 1},
        Range16{Lo:0x3250,Hi: 0x3260,Stride: 16},
        Range16{Lo:0x3261,Hi: 0x327f,Stride: 1},
        Range16{Lo:0x328a,Hi: 0x32b0,Stride: 1},
        Range16{Lo:0x32c0,Hi: 0x33ff,Stride: 1},
        Range16{Lo:0x4dc0,Hi: 0x4dff,Stride: 1},
        Range16{Lo:0xa490,Hi: 0xa4c6,Stride: 1},
        Range16{Lo:0xa700,Hi: 0xa716,Stride: 1},
        Range16{Lo:0xa720,Hi: 0xa721,Stride: 1},
        Range16{Lo:0xa789,Hi: 0xa78a,Stride: 1},
        Range16{Lo:0xa828,Hi: 0xa82b,Stride: 1},
        Range16{Lo:0xa836,Hi: 0xa839,Stride: 1},
        Range16{Lo:0xaa77,Hi: 0xaa79,Stride: 1},
        Range16{Lo:0xab5b,Hi: 0xab6a,Stride: 15},
        Range16{Lo:0xab6b,Hi: 0xfb29,Stride: 20414},
        Range16{Lo:0xfbb2,Hi: 0xfbc2,Stride: 1},
        Range16{Lo:0xfd40,Hi: 0xfd4f,Stride: 1},
        Range16{Lo:0xfdcf,Hi: 0xfdfc,Stride: 45},
        Range16{Lo:0xfdfd,Hi: 0xfdff,Stride: 1},
        Range16{Lo:0xfe62,Hi: 0xfe64,Stride: 2},
        Range16{Lo:0xfe65,Hi: 0xfe66,Stride: 1},
        Range16{Lo:0xfe69,Hi: 0xff04,Stride: 155},
        Range16{Lo:0xff0b,Hi: 0xff1c,Stride: 17},
        Range16{Lo:0xff1d,Hi: 0xff1e,Stride: 1},
        Range16{Lo:0xff3e,Hi: 0xff40,Stride: 2},
        Range16{Lo:0xff5c,Hi: 0xff5e,Stride: 2},
        Range16{Lo:0xffe0,Hi: 0xffe6,Stride: 1},
        Range16{Lo:0xffe8,Hi: 0xffee,Stride: 1},
        Range16{Lo:0xfffc,Hi: 0xfffd,Stride: 1},
    ]);
}
lazy_static::lazy_static! {
    pub static ref _S_RangeTable_R32: Arc<Vec<Range32>> = Arc::new(vec![
        Range32{Lo:0x10137,Hi: 0x1013f,Stride: 1},
        Range32{Lo:0x10179,Hi: 0x10189,Stride: 1},
        Range32{Lo:0x1018c,Hi: 0x1018e,Stride: 1},
        Range32{Lo:0x10190,Hi: 0x1019c,Stride: 1},
        Range32{Lo:0x101a0,Hi: 0x101d0,Stride: 48},
        Range32{Lo:0x101d1,Hi: 0x101fc,Stride: 1},
        Range32{Lo:0x10877,Hi: 0x10878,Stride: 1},
        Range32{Lo:0x10ac8,Hi: 0x1173f,Stride: 3191},
        Range32{Lo:0x11fd5,Hi: 0x11ff1,Stride: 1},
        Range32{Lo:0x16b3c,Hi: 0x16b3f,Stride: 1},
        Range32{Lo:0x16b45,Hi: 0x1bc9c,Stride: 20823},
        Range32{Lo:0x1cf50,Hi: 0x1cfc3,Stride: 1},
        Range32{Lo:0x1d000,Hi: 0x1d0f5,Stride: 1},
        Range32{Lo:0x1d100,Hi: 0x1d126,Stride: 1},
        Range32{Lo:0x1d129,Hi: 0x1d164,Stride: 1},
        Range32{Lo:0x1d16a,Hi: 0x1d16c,Stride: 1},
        Range32{Lo:0x1d183,Hi: 0x1d184,Stride: 1},
        Range32{Lo:0x1d18c,Hi: 0x1d1a9,Stride: 1},
        Range32{Lo:0x1d1ae,Hi: 0x1d1ea,Stride: 1},
        Range32{Lo:0x1d200,Hi: 0x1d241,Stride: 1},
        Range32{Lo:0x1d245,Hi: 0x1d300,Stride: 187},
        Range32{Lo:0x1d301,Hi: 0x1d356,Stride: 1},
        Range32{Lo:0x1d6c1,Hi: 0x1d6db,Stride: 26},
        Range32{Lo:0x1d6fb,Hi: 0x1d715,Stride: 26},
        Range32{Lo:0x1d735,Hi: 0x1d74f,Stride: 26},
        Range32{Lo:0x1d76f,Hi: 0x1d789,Stride: 26},
        Range32{Lo:0x1d7a9,Hi: 0x1d7c3,Stride: 26},
        Range32{Lo:0x1d800,Hi: 0x1d9ff,Stride: 1},
        Range32{Lo:0x1da37,Hi: 0x1da3a,Stride: 1},
        Range32{Lo:0x1da6d,Hi: 0x1da74,Stride: 1},
        Range32{Lo:0x1da76,Hi: 0x1da83,Stride: 1},
        Range32{Lo:0x1da85,Hi: 0x1da86,Stride: 1},
        Range32{Lo:0x1e14f,Hi: 0x1e2ff,Stride: 432},
        Range32{Lo:0x1ecac,Hi: 0x1ecb0,Stride: 4},
        Range32{Lo:0x1ed2e,Hi: 0x1eef0,Stride: 450},
        Range32{Lo:0x1eef1,Hi: 0x1f000,Stride: 271},
        Range32{Lo:0x1f001,Hi: 0x1f02b,Stride: 1},
        Range32{Lo:0x1f030,Hi: 0x1f093,Stride: 1},
        Range32{Lo:0x1f0a0,Hi: 0x1f0ae,Stride: 1},
        Range32{Lo:0x1f0b1,Hi: 0x1f0bf,Stride: 1},
        Range32{Lo:0x1f0c1,Hi: 0x1f0cf,Stride: 1},
        Range32{Lo:0x1f0d1,Hi: 0x1f0f5,Stride: 1},
        Range32{Lo:0x1f10d,Hi: 0x1f1ad,Stride: 1},
        Range32{Lo:0x1f1e6,Hi: 0x1f202,Stride: 1},
        Range32{Lo:0x1f210,Hi: 0x1f23b,Stride: 1},
        Range32{Lo:0x1f240,Hi: 0x1f248,Stride: 1},
        Range32{Lo:0x1f250,Hi: 0x1f251,Stride: 1},
        Range32{Lo:0x1f260,Hi: 0x1f265,Stride: 1},
        Range32{Lo:0x1f300,Hi: 0x1f6d7,Stride: 1},
        Range32{Lo:0x1f6dc,Hi: 0x1f6ec,Stride: 1},
        Range32{Lo:0x1f6f0,Hi: 0x1f6fc,Stride: 1},
        Range32{Lo:0x1f700,Hi: 0x1f776,Stride: 1},
        Range32{Lo:0x1f77b,Hi: 0x1f7d9,Stride: 1},
        Range32{Lo:0x1f7e0,Hi: 0x1f7eb,Stride: 1},
        Range32{Lo:0x1f7f0,Hi: 0x1f800,Stride: 16},
        Range32{Lo:0x1f801,Hi: 0x1f80b,Stride: 1},
        Range32{Lo:0x1f810,Hi: 0x1f847,Stride: 1},
        Range32{Lo:0x1f850,Hi: 0x1f859,Stride: 1},
        Range32{Lo:0x1f860,Hi: 0x1f887,Stride: 1},
        Range32{Lo:0x1f890,Hi: 0x1f8ad,Stride: 1},
        Range32{Lo:0x1f8b0,Hi: 0x1f8b1,Stride: 1},
        Range32{Lo:0x1f900,Hi: 0x1fa53,Stride: 1},
        Range32{Lo:0x1fa60,Hi: 0x1fa6d,Stride: 1},
        Range32{Lo:0x1fa70,Hi: 0x1fa7c,Stride: 1},
        Range32{Lo:0x1fa80,Hi: 0x1fa88,Stride: 1},
        Range32{Lo:0x1fa90,Hi: 0x1fabd,Stride: 1},
        Range32{Lo:0x1fabf,Hi: 0x1fac5,Stride: 1},
        Range32{Lo:0x1face,Hi: 0x1fadb,Stride: 1},
        Range32{Lo:0x1fae0,Hi: 0x1fae8,Stride: 1},
        Range32{Lo:0x1faf0,Hi: 0x1faf8,Stride: 1},
        Range32{Lo:0x1fb00,Hi: 0x1fb92,Stride: 1},
        Range32{Lo:0x1fb94,Hi: 0x1fbca,Stride: 1},

    ]);
}

lazy_static::lazy_static! {
    static ref _P_RangeTable_R16: Arc<Vec<Range16>> = Arc::new(vec![
        Range16{Lo:0x0021,Hi: 0x0023,Stride: 1},
        Range16{Lo:0x0025,Hi: 0x002a,Stride: 1},
        Range16{Lo:0x002c,Hi: 0x002f,Stride: 1},
        Range16{Lo:0x003a,Hi: 0x003b,Stride: 1},
        Range16{Lo:0x003f,Hi: 0x0040,Stride: 1},
        Range16{Lo:0x005b,Hi: 0x005d,Stride: 1},
        Range16{Lo:0x005f,Hi: 0x007b,Stride: 28},
        Range16{Lo:0x007d,Hi: 0x00a1,Stride: 36},
        Range16{Lo:0x00a7,Hi: 0x00ab,Stride: 4},
        Range16{Lo:0x00b6,Hi: 0x00b7,Stride: 1},
        Range16{Lo:0x00bb,Hi: 0x00bf,Stride: 4},
        Range16{Lo:0x037e,Hi: 0x0387,Stride: 9},
        Range16{Lo:0x055a,Hi: 0x055f,Stride: 1},
        Range16{Lo:0x0589,Hi: 0x058a,Stride: 1},
        Range16{Lo:0x05be,Hi: 0x05c0,Stride: 2},
        Range16{Lo:0x05c3,Hi: 0x05c6,Stride: 3},
        Range16{Lo:0x05f3,Hi: 0x05f4,Stride: 1},
        Range16{Lo:0x0609,Hi: 0x060a,Stride: 1},
        Range16{Lo:0x060c,Hi: 0x060d,Stride: 1},
        Range16{Lo:0x061b,Hi: 0x061d,Stride: 2},
        Range16{Lo:0x061e,Hi: 0x061f,Stride: 1},
        Range16{Lo:0x066a,Hi: 0x066d,Stride: 1},
        Range16{Lo:0x06d4,Hi: 0x0700,Stride: 44},
        Range16{Lo:0x0701,Hi: 0x070d,Stride: 1},
        Range16{Lo:0x07f7,Hi: 0x07f9,Stride: 1},
        Range16{Lo:0x0830,Hi: 0x083e,Stride: 1},
        Range16{Lo:0x085e,Hi: 0x0964,Stride: 262},
        Range16{Lo:0x0965,Hi: 0x0970,Stride: 11},
        Range16{Lo:0x09fd,Hi: 0x0a76,Stride: 121},
        Range16{Lo:0x0af0,Hi: 0x0c77,Stride: 391},
        Range16{Lo:0x0c84,Hi: 0x0df4,Stride: 368},
        Range16{Lo:0x0e4f,Hi: 0x0e5a,Stride: 11},
        Range16{Lo:0x0e5b,Hi: 0x0f04,Stride: 169},
        Range16{Lo:0x0f05,Hi: 0x0f12,Stride: 1},
        Range16{Lo:0x0f14,Hi: 0x0f3a,Stride: 38},
        Range16{Lo:0x0f3b,Hi: 0x0f3d,Stride: 1},
        Range16{Lo:0x0f85,Hi: 0x0fd0,Stride: 75},
        Range16{Lo:0x0fd1,Hi: 0x0fd4,Stride: 1},
        Range16{Lo:0x0fd9,Hi: 0x0fda,Stride: 1},
        Range16{Lo:0x104a,Hi: 0x104f,Stride: 1},
        Range16{Lo:0x10fb,Hi: 0x1360,Stride: 613},
        Range16{Lo:0x1361,Hi: 0x1368,Stride: 1},
        Range16{Lo:0x1400,Hi: 0x166e,Stride: 622},
        Range16{Lo:0x169b,Hi: 0x169c,Stride: 1},
        Range16{Lo:0x16eb,Hi: 0x16ed,Stride: 1},
        Range16{Lo:0x1735,Hi: 0x1736,Stride: 1},
        Range16{Lo:0x17d4,Hi: 0x17d6,Stride: 1},
        Range16{Lo:0x17d8,Hi: 0x17da,Stride: 1},
        Range16{Lo:0x1800,Hi: 0x180a,Stride: 1},
        Range16{Lo:0x1944,Hi: 0x1945,Stride: 1},
        Range16{Lo:0x1a1e,Hi: 0x1a1f,Stride: 1},
        Range16{Lo:0x1aa0,Hi: 0x1aa6,Stride: 1},
        Range16{Lo:0x1aa8,Hi: 0x1aad,Stride: 1},
        Range16{Lo:0x1b5a,Hi: 0x1b60,Stride: 1},
        Range16{Lo:0x1b7d,Hi: 0x1b7e,Stride: 1},
        Range16{Lo:0x1bfc,Hi: 0x1bff,Stride: 1},
        Range16{Lo:0x1c3b,Hi: 0x1c3f,Stride: 1},
        Range16{Lo:0x1c7e,Hi: 0x1c7f,Stride: 1},
        Range16{Lo:0x1cc0,Hi: 0x1cc7,Stride: 1},
        Range16{Lo:0x1cd3,Hi: 0x2010,Stride: 829},
        Range16{Lo:0x2011,Hi: 0x2027,Stride: 1},
        Range16{Lo:0x2030,Hi: 0x2043,Stride: 1},
        Range16{Lo:0x2045,Hi: 0x2051,Stride: 1},
        Range16{Lo:0x2053,Hi: 0x205e,Stride: 1},
        Range16{Lo:0x207d,Hi: 0x207e,Stride: 1},
        Range16{Lo:0x208d,Hi: 0x208e,Stride: 1},
        Range16{Lo:0x2308,Hi: 0x230b,Stride: 1},
        Range16{Lo:0x2329,Hi: 0x232a,Stride: 1},
        Range16{Lo:0x2768,Hi: 0x2775,Stride: 1},
        Range16{Lo:0x27c5,Hi: 0x27c6,Stride: 1},
        Range16{Lo:0x27e6,Hi: 0x27ef,Stride: 1},
        Range16{Lo:0x2983,Hi: 0x2998,Stride: 1},
        Range16{Lo:0x29d8,Hi: 0x29db,Stride: 1},
        Range16{Lo:0x29fc,Hi: 0x29fd,Stride: 1},
        Range16{Lo:0x2cf9,Hi: 0x2cfc,Stride: 1},
        Range16{Lo:0x2cfe,Hi: 0x2cff,Stride: 1},
        Range16{Lo:0x2d70,Hi: 0x2e00,Stride: 144},
        Range16{Lo:0x2e01,Hi: 0x2e2e,Stride: 1},
        Range16{Lo:0x2e30,Hi: 0x2e4f,Stride: 1},
        Range16{Lo:0x2e52,Hi: 0x2e5d,Stride: 1},
        Range16{Lo:0x3001,Hi: 0x3003,Stride: 1},
        Range16{Lo:0x3008,Hi: 0x3011,Stride: 1},
        Range16{Lo:0x3014,Hi: 0x301f,Stride: 1},
        Range16{Lo:0x3030,Hi: 0x303d,Stride: 13},
        Range16{Lo:0x30a0,Hi: 0x30fb,Stride: 91},
        Range16{Lo:0xa4fe,Hi: 0xa4ff,Stride: 1},
        Range16{Lo:0xa60d,Hi: 0xa60f,Stride: 1},
        Range16{Lo:0xa673,Hi: 0xa67e,Stride: 11},
        Range16{Lo:0xa6f2,Hi: 0xa6f7,Stride: 1},
        Range16{Lo:0xa874,Hi: 0xa877,Stride: 1},
        Range16{Lo:0xa8ce,Hi: 0xa8cf,Stride: 1},
        Range16{Lo:0xa8f8,Hi: 0xa8fa,Stride: 1},
        Range16{Lo:0xa8fc,Hi: 0xa92e,Stride: 50},
        Range16{Lo:0xa92f,Hi: 0xa95f,Stride: 48},
        Range16{Lo:0xa9c1,Hi: 0xa9cd,Stride: 1},
        Range16{Lo:0xa9de,Hi: 0xa9df,Stride: 1},
        Range16{Lo:0xaa5c,Hi: 0xaa5f,Stride: 1},
        Range16{Lo:0xaade,Hi: 0xaadf,Stride: 1},
        Range16{Lo:0xaaf0,Hi: 0xaaf1,Stride: 1},
        Range16{Lo:0xabeb,Hi: 0xfd3e,Stride: 20819},
        Range16{Lo:0xfd3f,Hi: 0xfe10,Stride: 209},
        Range16{Lo:0xfe11,Hi: 0xfe19,Stride: 1},
        Range16{Lo:0xfe30,Hi: 0xfe52,Stride: 1},
        Range16{Lo:0xfe54,Hi: 0xfe61,Stride: 1},
        Range16{Lo:0xfe63,Hi: 0xfe68,Stride: 5},
        Range16{Lo:0xfe6a,Hi: 0xfe6b,Stride: 1},
        Range16{Lo:0xff01,Hi: 0xff03,Stride: 1},
        Range16{Lo:0xff05,Hi: 0xff0a,Stride: 1},
        Range16{Lo:0xff0c,Hi: 0xff0f,Stride: 1},
        Range16{Lo:0xff1a,Hi: 0xff1b,Stride: 1},
        Range16{Lo:0xff1f,Hi: 0xff20,Stride: 1},
        Range16{Lo:0xff3b,Hi: 0xff3d,Stride: 1},
        Range16{Lo:0xff3f,Hi: 0xff5b,Stride: 28},
        Range16{Lo:0xff5d,Hi: 0xff5f,Stride: 2},
        Range16{Lo:0xff60,Hi: 0xff65,Stride: 1},
    ]);
}

lazy_static::lazy_static! {
    static ref _P_RangeTable_R32: Arc<Vec<Range32>> = Arc::new(vec![
        Range32{Lo:0x10100,Hi: 0x10102,Stride: 1},
        Range32{Lo:0x1039f,Hi: 0x103d0,Stride: 49},
        Range32{Lo:0x1056f,Hi: 0x10857,Stride: 744},
        Range32{Lo:0x1091f,Hi: 0x1093f,Stride: 32},
        Range32{Lo:0x10a50,Hi: 0x10a58,Stride: 1},
        Range32{Lo:0x10a7f,Hi: 0x10af0,Stride: 113},
        Range32{Lo:0x10af1,Hi: 0x10af6,Stride: 1},
        Range32{Lo:0x10b39,Hi: 0x10b3f,Stride: 1},
        Range32{Lo:0x10b99,Hi: 0x10b9c,Stride: 1},
        Range32{Lo:0x10ead,Hi: 0x10f55,Stride: 168},
        Range32{Lo:0x10f56,Hi: 0x10f59,Stride: 1},
        Range32{Lo:0x10f86,Hi: 0x10f89,Stride: 1},
        Range32{Lo:0x11047,Hi: 0x1104d,Stride: 1},
        Range32{Lo:0x110bb,Hi: 0x110bc,Stride: 1},
        Range32{Lo:0x110be,Hi: 0x110c1,Stride: 1},
        Range32{Lo:0x11140,Hi: 0x11143,Stride: 1},
        Range32{Lo:0x11174,Hi: 0x11175,Stride: 1},
        Range32{Lo:0x111c5,Hi: 0x111c8,Stride: 1},
        Range32{Lo:0x111cd,Hi: 0x111db,Stride: 14},
        Range32{Lo:0x111dd,Hi: 0x111df,Stride: 1},
        Range32{Lo:0x11238,Hi: 0x1123d,Stride: 1},
        Range32{Lo:0x112a9,Hi: 0x1144b,Stride: 418},
        Range32{Lo:0x1144c,Hi: 0x1144f,Stride: 1},
        Range32{Lo:0x1145a,Hi: 0x1145b,Stride: 1},
        Range32{Lo:0x1145d,Hi: 0x114c6,Stride: 105},
        Range32{Lo:0x115c1,Hi: 0x115d7,Stride: 1},
        Range32{Lo:0x11641,Hi: 0x11643,Stride: 1},
        Range32{Lo:0x11660,Hi: 0x1166c,Stride: 1},
        Range32{Lo:0x116b9,Hi: 0x1173c,Stride: 131},
        Range32{Lo:0x1173d,Hi: 0x1173e,Stride: 1},
        Range32{Lo:0x1183b,Hi: 0x11944,Stride: 265},
        Range32{Lo:0x11945,Hi: 0x11946,Stride: 1},
        Range32{Lo:0x119e2,Hi: 0x11a3f,Stride: 93},
        Range32{Lo:0x11a40,Hi: 0x11a46,Stride: 1},
        Range32{Lo:0x11a9a,Hi: 0x11a9c,Stride: 1},
        Range32{Lo:0x11a9e,Hi: 0x11aa2,Stride: 1},
        Range32{Lo:0x11b00,Hi: 0x11b09,Stride: 1},
        Range32{Lo:0x11c41,Hi: 0x11c45,Stride: 1},
        Range32{Lo:0x11c70,Hi: 0x11c71,Stride: 1},
        Range32{Lo:0x11ef7,Hi: 0x11ef8,Stride: 1},
        Range32{Lo:0x11f43,Hi: 0x11f4f,Stride: 1},
        Range32{Lo:0x11fff,Hi: 0x12470,Stride: 1137},
        Range32{Lo:0x12471,Hi: 0x12474,Stride: 1},
        Range32{Lo:0x12ff1,Hi: 0x12ff2,Stride: 1},
        Range32{Lo:0x16a6e,Hi: 0x16a6f,Stride: 1},
        Range32{Lo:0x16af5,Hi: 0x16b37,Stride: 66},
        Range32{Lo:0x16b38,Hi: 0x16b3b,Stride: 1},
        Range32{Lo:0x16b44,Hi: 0x16e97,Stride: 851},
        Range32{Lo:0x16e98,Hi: 0x16e9a,Stride: 1},
        Range32{Lo:0x16fe2,Hi: 0x1bc9f,Stride: 19645},
        Range32{Lo:0x1da87,Hi: 0x1da8b,Stride: 1},
        Range32{Lo:0x1e95e,Hi: 0x1e95f,Stride: 1},
    ]);
}

lazy_static::lazy_static! {
    static ref _N_RangeTable_R16: Arc<Vec<Range16>> = Arc::new(vec![
        Range16{Lo:0x0030,Hi: 0x0039,Stride: 1},
        Range16{Lo:0x00b2,Hi: 0x00b3,Stride: 1},
        Range16{Lo:0x00b9,Hi: 0x00bc,Stride: 3},
        Range16{Lo:0x00bd,Hi: 0x00be,Stride: 1},
        Range16{Lo:0x0660,Hi: 0x0669,Stride: 1},
        Range16{Lo:0x06f0,Hi: 0x06f9,Stride: 1},
        Range16{Lo:0x07c0,Hi: 0x07c9,Stride: 1},
        Range16{Lo:0x0966,Hi: 0x096f,Stride: 1},
        Range16{Lo:0x09e6,Hi: 0x09ef,Stride: 1},
        Range16{Lo:0x09f4,Hi: 0x09f9,Stride: 1},
        Range16{Lo:0x0a66,Hi: 0x0a6f,Stride: 1},
        Range16{Lo:0x0ae6,Hi: 0x0aef,Stride: 1},
        Range16{Lo:0x0b66,Hi: 0x0b6f,Stride: 1},
        Range16{Lo:0x0b72,Hi: 0x0b77,Stride: 1},
        Range16{Lo:0x0be6,Hi: 0x0bf2,Stride: 1},
        Range16{Lo:0x0c66,Hi: 0x0c6f,Stride: 1},
        Range16{Lo:0x0c78,Hi: 0x0c7e,Stride: 1},
        Range16{Lo:0x0ce6,Hi: 0x0cef,Stride: 1},
        Range16{Lo:0x0d58,Hi: 0x0d5e,Stride: 1},
        Range16{Lo:0x0d66,Hi: 0x0d78,Stride: 1},
        Range16{Lo:0x0de6,Hi: 0x0def,Stride: 1},
        Range16{Lo:0x0e50,Hi: 0x0e59,Stride: 1},
        Range16{Lo:0x0ed0,Hi: 0x0ed9,Stride: 1},
        Range16{Lo:0x0f20,Hi: 0x0f33,Stride: 1},
        Range16{Lo:0x1040,Hi: 0x1049,Stride: 1},
        Range16{Lo:0x1090,Hi: 0x1099,Stride: 1},
        Range16{Lo:0x1369,Hi: 0x137c,Stride: 1},
        Range16{Lo:0x16ee,Hi: 0x16f0,Stride: 1},
        Range16{Lo:0x17e0,Hi: 0x17e9,Stride: 1},
        Range16{Lo:0x17f0,Hi: 0x17f9,Stride: 1},
        Range16{Lo:0x1810,Hi: 0x1819,Stride: 1},
        Range16{Lo:0x1946,Hi: 0x194f,Stride: 1},
        Range16{Lo:0x19d0,Hi: 0x19da,Stride: 1},
        Range16{Lo:0x1a80,Hi: 0x1a89,Stride: 1},
        Range16{Lo:0x1a90,Hi: 0x1a99,Stride: 1},
        Range16{Lo:0x1b50,Hi: 0x1b59,Stride: 1},
        Range16{Lo:0x1bb0,Hi: 0x1bb9,Stride: 1},
        Range16{Lo:0x1c40,Hi: 0x1c49,Stride: 1},
        Range16{Lo:0x1c50,Hi: 0x1c59,Stride: 1},
        Range16{Lo:0x2070,Hi: 0x2074,Stride: 4},
        Range16{Lo:0x2075,Hi: 0x2079,Stride: 1},
        Range16{Lo:0x2080,Hi: 0x2089,Stride: 1},
        Range16{Lo:0x2150,Hi: 0x2182,Stride: 1},
        Range16{Lo:0x2185,Hi: 0x2189,Stride: 1},
        Range16{Lo:0x2460,Hi: 0x249b,Stride: 1},
        Range16{Lo:0x24ea,Hi: 0x24ff,Stride: 1},
        Range16{Lo:0x2776,Hi: 0x2793,Stride: 1},
        Range16{Lo:0x2cfd,Hi: 0x3007,Stride: 778},
        Range16{Lo:0x3021,Hi: 0x3029,Stride: 1},
        Range16{Lo:0x3038,Hi: 0x303a,Stride: 1},
        Range16{Lo:0x3192,Hi: 0x3195,Stride: 1},
        Range16{Lo:0x3220,Hi: 0x3229,Stride: 1},
        Range16{Lo:0x3248,Hi: 0x324f,Stride: 1},
        Range16{Lo:0x3251,Hi: 0x325f,Stride: 1},
        Range16{Lo:0x3280,Hi: 0x3289,Stride: 1},
        Range16{Lo:0x32b1,Hi: 0x32bf,Stride: 1},
        Range16{Lo:0xa620,Hi: 0xa629,Stride: 1},
        Range16{Lo:0xa6e6,Hi: 0xa6ef,Stride: 1},
        Range16{Lo:0xa830,Hi: 0xa835,Stride: 1},
        Range16{Lo:0xa8d0,Hi: 0xa8d9,Stride: 1},
        Range16{Lo:0xa900,Hi: 0xa909,Stride: 1},
        Range16{Lo:0xa9d0,Hi: 0xa9d9,Stride: 1},
        Range16{Lo:0xa9f0,Hi: 0xa9f9,Stride: 1},
        Range16{Lo:0xaa50,Hi: 0xaa59,Stride: 1},
        Range16{Lo:0xabf0,Hi: 0xabf9,Stride: 1},
        Range16{Lo:0xff10,Hi: 0xff19,Stride: 1},
    ]);
}

lazy_static::lazy_static! {
    static ref _N_RangeTable_R32: Arc<Vec<Range32>> = Arc::new(vec![
        Range32{Lo:0x10107,Hi: 0x10133,Stride: 1},
        Range32{Lo:0x10140,Hi: 0x10178,Stride: 1},
        Range32{Lo:0x1018a,Hi: 0x1018b,Stride: 1},
        Range32{Lo:0x102e1,Hi: 0x102fb,Stride: 1},
        Range32{Lo:0x10320,Hi: 0x10323,Stride: 1},
        Range32{Lo:0x10341,Hi: 0x1034a,Stride: 9},
        Range32{Lo:0x103d1,Hi: 0x103d5,Stride: 1},
        Range32{Lo:0x104a0,Hi: 0x104a9,Stride: 1},
        Range32{Lo:0x10858,Hi: 0x1085f,Stride: 1},
        Range32{Lo:0x10879,Hi: 0x1087f,Stride: 1},
        Range32{Lo:0x108a7,Hi: 0x108af,Stride: 1},
        Range32{Lo:0x108fb,Hi: 0x108ff,Stride: 1},
        Range32{Lo:0x10916,Hi: 0x1091b,Stride: 1},
        Range32{Lo:0x109bc,Hi: 0x109bd,Stride: 1},
        Range32{Lo:0x109c0,Hi: 0x109cf,Stride: 1},
        Range32{Lo:0x109d2,Hi: 0x109ff,Stride: 1},
        Range32{Lo:0x10a40,Hi: 0x10a48,Stride: 1},
        Range32{Lo:0x10a7d,Hi: 0x10a7e,Stride: 1},
        Range32{Lo:0x10a9d,Hi: 0x10a9f,Stride: 1},
        Range32{Lo:0x10aeb,Hi: 0x10aef,Stride: 1},
        Range32{Lo:0x10b58,Hi: 0x10b5f,Stride: 1},
        Range32{Lo:0x10b78,Hi: 0x10b7f,Stride: 1},
        Range32{Lo:0x10ba9,Hi: 0x10baf,Stride: 1},
        Range32{Lo:0x10cfa,Hi: 0x10cff,Stride: 1},
        Range32{Lo:0x10d30,Hi: 0x10d39,Stride: 1},
        Range32{Lo:0x10e60,Hi: 0x10e7e,Stride: 1},
        Range32{Lo:0x10f1d,Hi: 0x10f26,Stride: 1},
        Range32{Lo:0x10f51,Hi: 0x10f54,Stride: 1},
        Range32{Lo:0x10fc5,Hi: 0x10fcb,Stride: 1},
        Range32{Lo:0x11052,Hi: 0x1106f,Stride: 1},
        Range32{Lo:0x110f0,Hi: 0x110f9,Stride: 1},
        Range32{Lo:0x11136,Hi: 0x1113f,Stride: 1},
        Range32{Lo:0x111d0,Hi: 0x111d9,Stride: 1},
        Range32{Lo:0x111e1,Hi: 0x111f4,Stride: 1},
        Range32{Lo:0x112f0,Hi: 0x112f9,Stride: 1},
        Range32{Lo:0x11450,Hi: 0x11459,Stride: 1},
        Range32{Lo:0x114d0,Hi: 0x114d9,Stride: 1},
        Range32{Lo:0x11650,Hi: 0x11659,Stride: 1},
        Range32{Lo:0x116c0,Hi: 0x116c9,Stride: 1},
        Range32{Lo:0x11730,Hi: 0x1173b,Stride: 1},
        Range32{Lo:0x118e0,Hi: 0x118f2,Stride: 1},
        Range32{Lo:0x11950,Hi: 0x11959,Stride: 1},
        Range32{Lo:0x11c50,Hi: 0x11c6c,Stride: 1},
        Range32{Lo:0x11d50,Hi: 0x11d59,Stride: 1},
        Range32{Lo:0x11da0,Hi: 0x11da9,Stride: 1},
        Range32{Lo:0x11f50,Hi: 0x11f59,Stride: 1},
        Range32{Lo:0x11fc0,Hi: 0x11fd4,Stride: 1},
        Range32{Lo:0x12400,Hi: 0x1246e,Stride: 1},
        Range32{Lo:0x16a60,Hi: 0x16a69,Stride: 1},
        Range32{Lo:0x16ac0,Hi: 0x16ac9,Stride: 1},
        Range32{Lo:0x16b50,Hi: 0x16b59,Stride: 1},
        Range32{Lo:0x16b5b,Hi: 0x16b61,Stride: 1},
        Range32{Lo:0x16e80,Hi: 0x16e96,Stride: 1},
        Range32{Lo:0x1d2c0,Hi: 0x1d2d3,Stride: 1},
        Range32{Lo:0x1d2e0,Hi: 0x1d2f3,Stride: 1},
        Range32{Lo:0x1d360,Hi: 0x1d378,Stride: 1},
        Range32{Lo:0x1d7ce,Hi: 0x1d7ff,Stride: 1},
        Range32{Lo:0x1e140,Hi: 0x1e149,Stride: 1},
        Range32{Lo:0x1e2f0,Hi: 0x1e2f9,Stride: 1},
        Range32{Lo:0x1e4f0,Hi: 0x1e4f9,Stride: 1},
        Range32{Lo:0x1e8c7,Hi: 0x1e8cf,Stride: 1},
        Range32{Lo:0x1e950,Hi: 0x1e959,Stride: 1},
        Range32{Lo:0x1ec71,Hi: 0x1ecab,Stride: 1},
        Range32{Lo:0x1ecad,Hi: 0x1ecaf,Stride: 1},
        Range32{Lo:0x1ecb1,Hi: 0x1ecb4,Stride: 1},
        Range32{Lo:0x1ed01,Hi: 0x1ed2d,Stride: 1},
        Range32{Lo:0x1ed2f,Hi: 0x1ed3d,Stride: 1},
        Range32{Lo:0x1f100,Hi: 0x1f10c,Stride: 1},
        Range32{Lo:0x1fbf0,Hi: 0x1fbf9,Stride: 1},
    ]);
}

lazy_static::lazy_static! {
    static ref _M_RangeTable_R32: Arc<Vec<Range32>> = Arc::new(vec![
        Range32{Lo:0x101fd,Hi: 0x102e0,Stride: 227},
        Range32{Lo:0x10376,Hi: 0x1037a,Stride: 1},
        Range32{Lo:0x10a01,Hi: 0x10a03,Stride: 1},
        Range32{Lo:0x10a05,Hi: 0x10a06,Stride: 1},
        Range32{Lo:0x10a0c,Hi: 0x10a0f,Stride: 1},
        Range32{Lo:0x10a38,Hi: 0x10a3a,Stride: 1},
        Range32{Lo:0x10a3f,Hi: 0x10ae5,Stride: 166},
        Range32{Lo:0x10ae6,Hi: 0x10d24,Stride: 574},
        Range32{Lo:0x10d25,Hi: 0x10d27,Stride: 1},
        Range32{Lo:0x10eab,Hi: 0x10eac,Stride: 1},
        Range32{Lo:0x10efd,Hi: 0x10eff,Stride: 1},
        Range32{Lo:0x10f46,Hi: 0x10f50,Stride: 1},
        Range32{Lo:0x10f82,Hi: 0x10f85,Stride: 1},
        Range32{Lo:0x11000,Hi: 0x11002,Stride: 1},
        Range32{Lo:0x11038,Hi: 0x11046,Stride: 1},
        Range32{Lo:0x11070,Hi: 0x11073,Stride: 3},
        Range32{Lo:0x11074,Hi: 0x1107f,Stride: 11},
        Range32{Lo:0x11080,Hi: 0x11082,Stride: 1},
        Range32{Lo:0x110b0,Hi: 0x110ba,Stride: 1},
        Range32{Lo:0x110c2,Hi: 0x11100,Stride: 62},
        Range32{Lo:0x11101,Hi: 0x11102,Stride: 1},
        Range32{Lo:0x11127,Hi: 0x11134,Stride: 1},
        Range32{Lo:0x11145,Hi: 0x11146,Stride: 1},
        Range32{Lo:0x11173,Hi: 0x11180,Stride: 13},
        Range32{Lo:0x11181,Hi: 0x11182,Stride: 1},
        Range32{Lo:0x111b3,Hi: 0x111c0,Stride: 1},
        Range32{Lo:0x111c9,Hi: 0x111cc,Stride: 1},
        Range32{Lo:0x111ce,Hi: 0x111cf,Stride: 1},
        Range32{Lo:0x1122c,Hi: 0x11237,Stride: 1},
        Range32{Lo:0x1123e,Hi: 0x11241,Stride: 3},
        Range32{Lo:0x112df,Hi: 0x112ea,Stride: 1},
        Range32{Lo:0x11300,Hi: 0x11303,Stride: 1},
        Range32{Lo:0x1133b,Hi: 0x1133c,Stride: 1},
        Range32{Lo:0x1133e,Hi: 0x11344,Stride: 1},
        Range32{Lo:0x11347,Hi: 0x11348,Stride: 1},
        Range32{Lo:0x1134b,Hi: 0x1134d,Stride: 1},
        Range32{Lo:0x11357,Hi: 0x11362,Stride: 11},
        Range32{Lo:0x11363,Hi: 0x11366,Stride: 3},
        Range32{Lo:0x11367,Hi: 0x1136c,Stride: 1},
        Range32{Lo:0x11370,Hi: 0x11374,Stride: 1},
        Range32{Lo:0x11435,Hi: 0x11446,Stride: 1},
        Range32{Lo:0x1145e,Hi: 0x114b0,Stride: 82},
        Range32{Lo:0x114b1,Hi: 0x114c3,Stride: 1},
        Range32{Lo:0x115af,Hi: 0x115b5,Stride: 1},
        Range32{Lo:0x115b8,Hi: 0x115c0,Stride: 1},
        Range32{Lo:0x115dc,Hi: 0x115dd,Stride: 1},
        Range32{Lo:0x11630,Hi: 0x11640,Stride: 1},
        Range32{Lo:0x116ab,Hi: 0x116b7,Stride: 1},
        Range32{Lo:0x1171d,Hi: 0x1172b,Stride: 1},
        Range32{Lo:0x1182c,Hi: 0x1183a,Stride: 1},
        Range32{Lo:0x11930,Hi: 0x11935,Stride: 1},
        Range32{Lo:0x11937,Hi: 0x11938,Stride: 1},
        Range32{Lo:0x1193b,Hi: 0x1193e,Stride: 1},
        Range32{Lo:0x11940,Hi: 0x11942,Stride: 2},
        Range32{Lo:0x11943,Hi: 0x119d1,Stride: 142},
        Range32{Lo:0x119d2,Hi: 0x119d7,Stride: 1},
        Range32{Lo:0x119da,Hi: 0x119e0,Stride: 1},
        Range32{Lo:0x119e4,Hi: 0x11a01,Stride: 29},
        Range32{Lo:0x11a02,Hi: 0x11a0a,Stride: 1},
        Range32{Lo:0x11a33,Hi: 0x11a39,Stride: 1},
        Range32{Lo:0x11a3b,Hi: 0x11a3e,Stride: 1},
        Range32{Lo:0x11a47,Hi: 0x11a51,Stride: 10},
        Range32{Lo:0x11a52,Hi: 0x11a5b,Stride: 1},
        Range32{Lo:0x11a8a,Hi: 0x11a99,Stride: 1},
        Range32{Lo:0x11c2f,Hi: 0x11c36,Stride: 1},
        Range32{Lo:0x11c38,Hi: 0x11c3f,Stride: 1},
        Range32{Lo:0x11c92,Hi: 0x11ca7,Stride: 1},
        Range32{Lo:0x11ca9,Hi: 0x11cb6,Stride: 1},
        Range32{Lo:0x11d31,Hi: 0x11d36,Stride: 1},
        Range32{Lo:0x11d3a,Hi: 0x11d3c,Stride: 2},
        Range32{Lo:0x11d3d,Hi: 0x11d3f,Stride: 2},
        Range32{Lo:0x11d40,Hi: 0x11d45,Stride: 1},
        Range32{Lo:0x11d47,Hi: 0x11d8a,Stride: 67},
        Range32{Lo:0x11d8b,Hi: 0x11d8e,Stride: 1},
        Range32{Lo:0x11d90,Hi: 0x11d91,Stride: 1},
        Range32{Lo:0x11d93,Hi: 0x11d97,Stride: 1},
        Range32{Lo:0x11ef3,Hi: 0x11ef6,Stride: 1},
        Range32{Lo:0x11f00,Hi: 0x11f01,Stride: 1},
        Range32{Lo:0x11f03,Hi: 0x11f34,Stride: 49},
        Range32{Lo:0x11f35,Hi: 0x11f3a,Stride: 1},
        Range32{Lo:0x11f3e,Hi: 0x11f42,Stride: 1},
        Range32{Lo:0x13440,Hi: 0x13447,Stride: 7},
        Range32{Lo:0x13448,Hi: 0x13455,Stride: 1},
        Range32{Lo:0x16af0,Hi: 0x16af4,Stride: 1},
        Range32{Lo:0x16b30,Hi: 0x16b36,Stride: 1},
        Range32{Lo:0x16f4f,Hi: 0x16f51,Stride: 2},
        Range32{Lo:0x16f52,Hi: 0x16f87,Stride: 1},
        Range32{Lo:0x16f8f,Hi: 0x16f92,Stride: 1},
        Range32{Lo:0x16fe4,Hi: 0x16ff0,Stride: 12},
        Range32{Lo:0x16ff1,Hi: 0x1bc9d,Stride: 19628},
        Range32{Lo:0x1bc9e,Hi: 0x1cf00,Stride: 4706},
        Range32{Lo:0x1cf01,Hi: 0x1cf2d,Stride: 1},
        Range32{Lo:0x1cf30,Hi: 0x1cf46,Stride: 1},
        Range32{Lo:0x1d165,Hi: 0x1d169,Stride: 1},
        Range32{Lo:0x1d16d,Hi: 0x1d172,Stride: 1},
        Range32{Lo:0x1d17b,Hi: 0x1d182,Stride: 1},
        Range32{Lo:0x1d185,Hi: 0x1d18b,Stride: 1},
        Range32{Lo:0x1d1aa,Hi: 0x1d1ad,Stride: 1},
        Range32{Lo:0x1d242,Hi: 0x1d244,Stride: 1},
        Range32{Lo:0x1da00,Hi: 0x1da36,Stride: 1},
        Range32{Lo:0x1da3b,Hi: 0x1da6c,Stride: 1},
        Range32{Lo:0x1da75,Hi: 0x1da84,Stride: 15},
        Range32{Lo:0x1da9b,Hi: 0x1da9f,Stride: 1},
        Range32{Lo:0x1daa1,Hi: 0x1daaf,Stride: 1},
        Range32{Lo:0x1e000,Hi: 0x1e006,Stride: 1},
        Range32{Lo:0x1e008,Hi: 0x1e018,Stride: 1},
        Range32{Lo:0x1e01b,Hi: 0x1e021,Stride: 1},
        Range32{Lo:0x1e023,Hi: 0x1e024,Stride: 1},
        Range32{Lo:0x1e026,Hi: 0x1e02a,Stride: 1},
        Range32{Lo:0x1e08f,Hi: 0x1e130,Stride: 161},
        Range32{Lo:0x1e131,Hi: 0x1e136,Stride: 1},
        Range32{Lo:0x1e2ae,Hi: 0x1e2ec,Stride: 62},
        Range32{Lo:0x1e2ed,Hi: 0x1e2ef,Stride: 1},
        Range32{Lo:0x1e4ec,Hi: 0x1e4ef,Stride: 1},
        Range32{Lo:0x1e8d0,Hi: 0x1e8d6,Stride: 1},
        Range32{Lo:0x1e944,Hi: 0x1e94a,Stride: 1},
        Range32{Lo:0xe0100,Hi: 0xe01ef,Stride: 1},
    ]);
}

lazy_static::lazy_static! {
    static ref _M_RangeTable_R16: Arc<Vec<Range16>> = Arc::new(vec![
        Range16{Lo:0x0300,Hi: 0x036f,Stride: 1},
        Range16{Lo:0x0483,Hi: 0x0489,Stride: 1},
        Range16{Lo:0x0591,Hi: 0x05bd,Stride: 1},
        Range16{Lo:0x05bf,Hi: 0x05c1,Stride: 2},
        Range16{Lo:0x05c2,Hi: 0x05c4,Stride: 2},
        Range16{Lo:0x05c5,Hi: 0x05c7,Stride: 2},
        Range16{Lo:0x0610,Hi: 0x061a,Stride: 1},
        Range16{Lo:0x064b,Hi: 0x065f,Stride: 1},
        Range16{Lo:0x0670,Hi: 0x06d6,Stride: 102},
        Range16{Lo:0x06d7,Hi: 0x06dc,Stride: 1},
        Range16{Lo:0x06df,Hi: 0x06e4,Stride: 1},
        Range16{Lo:0x06e7,Hi: 0x06e8,Stride: 1},
        Range16{Lo:0x06ea,Hi: 0x06ed,Stride: 1},
        Range16{Lo:0x0711,Hi: 0x0730,Stride: 31},
        Range16{Lo:0x0731,Hi: 0x074a,Stride: 1},
        Range16{Lo:0x07a6,Hi: 0x07b0,Stride: 1},
        Range16{Lo:0x07eb,Hi: 0x07f3,Stride: 1},
        Range16{Lo:0x07fd,Hi: 0x0816,Stride: 25},
        Range16{Lo:0x0817,Hi: 0x0819,Stride: 1},
        Range16{Lo:0x081b,Hi: 0x0823,Stride: 1},
        Range16{Lo:0x0825,Hi: 0x0827,Stride: 1},
        Range16{Lo:0x0829,Hi: 0x082d,Stride: 1},
        Range16{Lo:0x0859,Hi: 0x085b,Stride: 1},
        Range16{Lo:0x0898,Hi: 0x089f,Stride: 1},
        Range16{Lo:0x08ca,Hi: 0x08e1,Stride: 1},
        Range16{Lo:0x08e3,Hi: 0x0903,Stride: 1},
        Range16{Lo:0x093a,Hi: 0x093c,Stride: 1},
        Range16{Lo:0x093e,Hi: 0x094f,Stride: 1},
        Range16{Lo:0x0951,Hi: 0x0957,Stride: 1},
        Range16{Lo:0x0962,Hi: 0x0963,Stride: 1},
        Range16{Lo:0x0981,Hi: 0x0983,Stride: 1},
        Range16{Lo:0x09bc,Hi: 0x09be,Stride: 2},
        Range16{Lo:0x09bf,Hi: 0x09c4,Stride: 1},
        Range16{Lo:0x09c7,Hi: 0x09c8,Stride: 1},
        Range16{Lo:0x09cb,Hi: 0x09cd,Stride: 1},
        Range16{Lo:0x09d7,Hi: 0x09e2,Stride: 11},
        Range16{Lo:0x09e3,Hi: 0x09fe,Stride: 27},
        Range16{Lo:0x0a01,Hi: 0x0a03,Stride: 1},
        Range16{Lo:0x0a3c,Hi: 0x0a3e,Stride: 2},
        Range16{Lo:0x0a3f,Hi: 0x0a42,Stride: 1},
        Range16{Lo:0x0a47,Hi: 0x0a48,Stride: 1},
        Range16{Lo:0x0a4b,Hi: 0x0a4d,Stride: 1},
        Range16{Lo:0x0a51,Hi: 0x0a70,Stride: 31},
        Range16{Lo:0x0a71,Hi: 0x0a75,Stride: 4},
        Range16{Lo:0x0a81,Hi: 0x0a83,Stride: 1},
        Range16{Lo:0x0abc,Hi: 0x0abe,Stride: 2},
        Range16{Lo:0x0abf,Hi: 0x0ac5,Stride: 1},
        Range16{Lo:0x0ac7,Hi: 0x0ac9,Stride: 1},
        Range16{Lo:0x0acb,Hi: 0x0acd,Stride: 1},
        Range16{Lo:0x0ae2,Hi: 0x0ae3,Stride: 1},
        Range16{Lo:0x0afa,Hi: 0x0aff,Stride: 1},
        Range16{Lo:0x0b01,Hi: 0x0b03,Stride: 1},
        Range16{Lo:0x0b3c,Hi: 0x0b3e,Stride: 2},
        Range16{Lo:0x0b3f,Hi: 0x0b44,Stride: 1},
        Range16{Lo:0x0b47,Hi: 0x0b48,Stride: 1},
        Range16{Lo:0x0b4b,Hi: 0x0b4d,Stride: 1},
        Range16{Lo:0x0b55,Hi: 0x0b57,Stride: 1},
        Range16{Lo:0x0b62,Hi: 0x0b63,Stride: 1},
        Range16{Lo:0x0b82,Hi: 0x0bbe,Stride: 60},
        Range16{Lo:0x0bbf,Hi: 0x0bc2,Stride: 1},
        Range16{Lo:0x0bc6,Hi: 0x0bc8,Stride: 1},
        Range16{Lo:0x0bca,Hi: 0x0bcd,Stride: 1},
        Range16{Lo:0x0bd7,Hi: 0x0c00,Stride: 41},
        Range16{Lo:0x0c01,Hi: 0x0c04,Stride: 1},
        Range16{Lo:0x0c3c,Hi: 0x0c3e,Stride: 2},
        Range16{Lo:0x0c3f,Hi: 0x0c44,Stride: 1},
        Range16{Lo:0x0c46,Hi: 0x0c48,Stride: 1},
        Range16{Lo:0x0c4a,Hi: 0x0c4d,Stride: 1},
        Range16{Lo:0x0c55,Hi: 0x0c56,Stride: 1},
        Range16{Lo:0x0c62,Hi: 0x0c63,Stride: 1},
        Range16{Lo:0x0c81,Hi: 0x0c83,Stride: 1},
        Range16{Lo:0x0cbc,Hi: 0x0cbe,Stride: 2},
        Range16{Lo:0x0cbf,Hi: 0x0cc4,Stride: 1},
        Range16{Lo:0x0cc6,Hi: 0x0cc8,Stride: 1},
        Range16{Lo:0x0cca,Hi: 0x0ccd,Stride: 1},
        Range16{Lo:0x0cd5,Hi: 0x0cd6,Stride: 1},
        Range16{Lo:0x0ce2,Hi: 0x0ce3,Stride: 1},
        Range16{Lo:0x0cf3,Hi: 0x0d00,Stride: 13},
        Range16{Lo:0x0d01,Hi: 0x0d03,Stride: 1},
        Range16{Lo:0x0d3b,Hi: 0x0d3c,Stride: 1},
        Range16{Lo:0x0d3e,Hi: 0x0d44,Stride: 1},
        Range16{Lo:0x0d46,Hi: 0x0d48,Stride: 1},
        Range16{Lo:0x0d4a,Hi: 0x0d4d,Stride: 1},
        Range16{Lo:0x0d57,Hi: 0x0d62,Stride: 11},
        Range16{Lo:0x0d63,Hi: 0x0d81,Stride: 30},
        Range16{Lo:0x0d82,Hi: 0x0d83,Stride: 1},
        Range16{Lo:0x0dca,Hi: 0x0dcf,Stride: 5},
        Range16{Lo:0x0dd0,Hi: 0x0dd4,Stride: 1},
        Range16{Lo:0x0dd6,Hi: 0x0dd8,Stride: 2},
        Range16{Lo:0x0dd9,Hi: 0x0ddf,Stride: 1},
        Range16{Lo:0x0df2,Hi: 0x0df3,Stride: 1},
        Range16{Lo:0x0e31,Hi: 0x0e34,Stride: 3},
        Range16{Lo:0x0e35,Hi: 0x0e3a,Stride: 1},
        Range16{Lo:0x0e47,Hi: 0x0e4e,Stride: 1},
        Range16{Lo:0x0eb1,Hi: 0x0eb4,Stride: 3},
        Range16{Lo:0x0eb5,Hi: 0x0ebc,Stride: 1},
        Range16{Lo:0x0ec8,Hi: 0x0ece,Stride: 1},
        Range16{Lo:0x0f18,Hi: 0x0f19,Stride: 1},
        Range16{Lo:0x0f35,Hi: 0x0f39,Stride: 2},
        Range16{Lo:0x0f3e,Hi: 0x0f3f,Stride: 1},
        Range16{Lo:0x0f71,Hi: 0x0f84,Stride: 1},
        Range16{Lo:0x0f86,Hi: 0x0f87,Stride: 1},
        Range16{Lo:0x0f8d,Hi: 0x0f97,Stride: 1},
        Range16{Lo:0x0f99,Hi: 0x0fbc,Stride: 1},
        Range16{Lo:0x0fc6,Hi: 0x102b,Stride: 101},
        Range16{Lo:0x102c,Hi: 0x103e,Stride: 1},
        Range16{Lo:0x1056,Hi: 0x1059,Stride: 1},
        Range16{Lo:0x105e,Hi: 0x1060,Stride: 1},
        Range16{Lo:0x1062,Hi: 0x1064,Stride: 1},
        Range16{Lo:0x1067,Hi: 0x106d,Stride: 1},
        Range16{Lo:0x1071,Hi: 0x1074,Stride: 1},
        Range16{Lo:0x1082,Hi: 0x108d,Stride: 1},
        Range16{Lo:0x108f,Hi: 0x109a,Stride: 11},
        Range16{Lo:0x109b,Hi: 0x109d,Stride: 1},
        Range16{Lo:0x135d,Hi: 0x135f,Stride: 1},
        Range16{Lo:0x1712,Hi: 0x1715,Stride: 1},
        Range16{Lo:0x1732,Hi: 0x1734,Stride: 1},
        Range16{Lo:0x1752,Hi: 0x1753,Stride: 1},
        Range16{Lo:0x1772,Hi: 0x1773,Stride: 1},
        Range16{Lo:0x17b4,Hi: 0x17d3,Stride: 1},
        Range16{Lo:0x17dd,Hi: 0x180b,Stride: 46},
        Range16{Lo:0x180c,Hi: 0x180d,Stride: 1},
        Range16{Lo:0x180f,Hi: 0x1885,Stride: 118},
        Range16{Lo:0x1886,Hi: 0x18a9,Stride: 35},
        Range16{Lo:0x1920,Hi: 0x192b,Stride: 1},
        Range16{Lo:0x1930,Hi: 0x193b,Stride: 1},
        Range16{Lo:0x1a17,Hi: 0x1a1b,Stride: 1},
        Range16{Lo:0x1a55,Hi: 0x1a5e,Stride: 1},
        Range16{Lo:0x1a60,Hi: 0x1a7c,Stride: 1},
        Range16{Lo:0x1a7f,Hi: 0x1ab0,Stride: 49},
        Range16{Lo:0x1ab1,Hi: 0x1ace,Stride: 1},
        Range16{Lo:0x1b00,Hi: 0x1b04,Stride: 1},
        Range16{Lo:0x1b34,Hi: 0x1b44,Stride: 1},
        Range16{Lo:0x1b6b,Hi: 0x1b73,Stride: 1},
        Range16{Lo:0x1b80,Hi: 0x1b82,Stride: 1},
        Range16{Lo:0x1ba1,Hi: 0x1bad,Stride: 1},
        Range16{Lo:0x1be6,Hi: 0x1bf3,Stride: 1},
        Range16{Lo:0x1c24,Hi: 0x1c37,Stride: 1},
        Range16{Lo:0x1cd0,Hi: 0x1cd2,Stride: 1},
        Range16{Lo:0x1cd4,Hi: 0x1ce8,Stride: 1},
        Range16{Lo:0x1ced,Hi: 0x1cf4,Stride: 7},
        Range16{Lo:0x1cf7,Hi: 0x1cf9,Stride: 1},
        Range16{Lo:0x1dc0,Hi: 0x1dff,Stride: 1},
        Range16{Lo:0x20d0,Hi: 0x20f0,Stride: 1},
        Range16{Lo:0x2cef,Hi: 0x2cf1,Stride: 1},
        Range16{Lo:0x2d7f,Hi: 0x2de0,Stride: 97},
        Range16{Lo:0x2de1,Hi: 0x2dff,Stride: 1},
        Range16{Lo:0x302a,Hi: 0x302f,Stride: 1},
        Range16{Lo:0x3099,Hi: 0x309a,Stride: 1},
        Range16{Lo:0xa66f,Hi: 0xa672,Stride: 1},
        Range16{Lo:0xa674,Hi: 0xa67d,Stride: 1},
        Range16{Lo:0xa69e,Hi: 0xa69f,Stride: 1},
        Range16{Lo:0xa6f0,Hi: 0xa6f1,Stride: 1},
        Range16{Lo:0xa802,Hi: 0xa806,Stride: 4},
        Range16{Lo:0xa80b,Hi: 0xa823,Stride: 24},
        Range16{Lo:0xa824,Hi: 0xa827,Stride: 1},
        Range16{Lo:0xa82c,Hi: 0xa880,Stride: 84},
        Range16{Lo:0xa881,Hi: 0xa8b4,Stride: 51},
        Range16{Lo:0xa8b5,Hi: 0xa8c5,Stride: 1},
        Range16{Lo:0xa8e0,Hi: 0xa8f1,Stride: 1},
        Range16{Lo:0xa8ff,Hi: 0xa926,Stride: 39},
        Range16{Lo:0xa927,Hi: 0xa92d,Stride: 1},
        Range16{Lo:0xa947,Hi: 0xa953,Stride: 1},
        Range16{Lo:0xa980,Hi: 0xa983,Stride: 1},
        Range16{Lo:0xa9b3,Hi: 0xa9c0,Stride: 1},
        Range16{Lo:0xa9e5,Hi: 0xaa29,Stride: 68},
        Range16{Lo:0xaa2a,Hi: 0xaa36,Stride: 1},
        Range16{Lo:0xaa43,Hi: 0xaa4c,Stride: 9},
        Range16{Lo:0xaa4d,Hi: 0xaa7b,Stride: 46},
        Range16{Lo:0xaa7c,Hi: 0xaa7d,Stride: 1},
        Range16{Lo:0xaab0,Hi: 0xaab2,Stride: 2},
        Range16{Lo:0xaab3,Hi: 0xaab4,Stride: 1},
        Range16{Lo:0xaab7,Hi: 0xaab8,Stride: 1},
        Range16{Lo:0xaabe,Hi: 0xaabf,Stride: 1},
        Range16{Lo:0xaac1,Hi: 0xaaeb,Stride: 42},
        Range16{Lo:0xaaec,Hi: 0xaaef,Stride: 1},
        Range16{Lo:0xaaf5,Hi: 0xaaf6,Stride: 1},
        Range16{Lo:0xabe3,Hi: 0xabea,Stride: 1},
        Range16{Lo:0xabec,Hi: 0xabed,Stride: 1},
        Range16{Lo:0xfb1e,Hi: 0xfe00,Stride: 738},
        Range16{Lo:0xfe01,Hi: 0xfe0f,Stride: 1},
        Range16{Lo:0xfe20,Hi: 0xfe2f,Stride: 1},
    ]);
}

lazy_static::lazy_static! {
    static ref _L_RangeTable_R32: Arc<Vec<Range32>> = Arc::new(vec![
        Range32{Lo:0x10000,Hi: 0x1000b, Stride:1},
        Range32{Lo:0x1000d,Hi: 0x10026,Stride: 1},
        Range32{Lo:0x10028,Hi: 0x1003a,Stride: 1},
        Range32{Lo:0x1003c,Hi: 0x1003d,Stride: 1},
        Range32{Lo:0x1003f,Hi: 0x1004d,Stride: 1},
        Range32{Lo:0x10050,Hi: 0x1005d,Stride: 1},
        Range32{Lo:0x10080,Hi: 0x100fa,Stride: 1},
        Range32{Lo:0x10280,Hi: 0x1029c,Stride: 1},
        Range32{Lo:0x102a0,Hi: 0x102d0,Stride: 1},
        Range32{Lo:0x10300,Hi: 0x1031f,Stride: 1},
        Range32{Lo:0x1032d,Hi: 0x10340,Stride: 1},
        Range32{Lo:0x10342,Hi: 0x10349,Stride: 1},
        Range32{Lo:0x10350,Hi: 0x10375,Stride: 1},
        Range32{Lo:0x10380,Hi: 0x1039d,Stride: 1},
        Range32{Lo:0x103a0,Hi: 0x103c3,Stride: 1},
        Range32{Lo:0x103c8,Hi: 0x103cf,Stride: 1},
        Range32{Lo:0x10400,Hi: 0x1049d,Stride: 1},
        Range32{Lo:0x104b0,Hi: 0x104d3,Stride: 1},
        Range32{Lo:0x104d8,Hi: 0x104fb,Stride: 1},
        Range32{Lo:0x10500,Hi: 0x10527,Stride: 1},
        Range32{Lo:0x10530,Hi: 0x10563,Stride: 1},
        Range32{Lo:0x10570,Hi: 0x1057a,Stride: 1},
        Range32{Lo:0x1057c,Hi: 0x1058a,Stride: 1},
        Range32{Lo:0x1058c,Hi: 0x10592,Stride: 1},
        Range32{Lo:0x10594,Hi: 0x10595,Stride: 1},
        Range32{Lo:0x10597,Hi: 0x105a1,Stride: 1},
        Range32{Lo:0x105a3,Hi: 0x105b1,Stride: 1},
        Range32{Lo:0x105b3,Hi: 0x105b9,Stride: 1},
        Range32{Lo:0x105bb,Hi: 0x105bc,Stride: 1},
        Range32{Lo:0x10600,Hi: 0x10736,Stride: 1},
        Range32{Lo:0x10740,Hi: 0x10755,Stride: 1},
        Range32{Lo:0x10760,Hi: 0x10767,Stride: 1},
        Range32{Lo:0x10780,Hi: 0x10785,Stride: 1},
        Range32{Lo:0x10787,Hi: 0x107b0,Stride: 1},
        Range32{Lo:0x107b2,Hi: 0x107ba,Stride: 1},
        Range32{Lo:0x10800,Hi: 0x10805,Stride: 1},
        Range32{Lo:0x10808,Hi: 0x1080a,Stride: 2},
        Range32{Lo:0x1080b,Hi: 0x10835,Stride: 1},
        Range32{Lo:0x10837,Hi: 0x10838,Stride: 1},
        Range32{Lo:0x1083c,Hi: 0x1083f,Stride: 3},
        Range32{Lo:0x10840,Hi: 0x10855,Stride: 1},
        Range32{Lo:0x10860,Hi: 0x10876,Stride: 1},
        Range32{Lo:0x10880,Hi: 0x1089e,Stride: 1},
        Range32{Lo:0x108e0,Hi: 0x108f2,Stride: 1},
        Range32{Lo:0x108f4,Hi: 0x108f5,Stride: 1},
        Range32{Lo:0x10900,Hi: 0x10915,Stride: 1},
        Range32{Lo:0x10920,Hi: 0x10939,Stride: 1},
        Range32{Lo:0x10980,Hi: 0x109b7,Stride: 1},
        Range32{Lo:0x109be,Hi: 0x109bf,Stride: 1},
        Range32{Lo:0x10a00,Hi: 0x10a10,Stride: 16},
        Range32{Lo:0x10a11,Hi: 0x10a13,Stride: 1},
        Range32{Lo:0x10a15,Hi: 0x10a17,Stride: 1},
        Range32{Lo:0x10a19,Hi: 0x10a35,Stride: 1},
        Range32{Lo:0x10a60,Hi: 0x10a7c,Stride: 1},
        Range32{Lo:0x10a80,Hi: 0x10a9c,Stride: 1},
        Range32{Lo:0x10ac0,Hi: 0x10ac7,Stride: 1},
        Range32{Lo:0x10ac9,Hi: 0x10ae4,Stride: 1},
        Range32{Lo:0x10b00,Hi: 0x10b35,Stride: 1},
        Range32{Lo:0x10b40,Hi: 0x10b55,Stride: 1},
        Range32{Lo:0x10b60,Hi: 0x10b72,Stride: 1},
        Range32{Lo:0x10b80,Hi: 0x10b91,Stride: 1},
        Range32{Lo:0x10c00,Hi: 0x10c48,Stride: 1},
        Range32{Lo:0x10c80,Hi: 0x10cb2,Stride: 1},
        Range32{Lo:0x10cc0,Hi: 0x10cf2,Stride: 1},
        Range32{Lo:0x10d00,Hi: 0x10d23,Stride: 1},
        Range32{Lo:0x10e80,Hi: 0x10ea9,Stride: 1},
        Range32{Lo:0x10eb0,Hi: 0x10eb1,Stride: 1},
        Range32{Lo:0x10f00,Hi: 0x10f1c,Stride: 1},
        Range32{Lo:0x10f27,Hi: 0x10f30,Stride: 9},
        Range32{Lo:0x10f31,Hi: 0x10f45,Stride: 1},
        Range32{Lo:0x10f70,Hi: 0x10f81,Stride: 1},
        Range32{Lo:0x10fb0,Hi: 0x10fc4,Stride: 1},
        Range32{Lo:0x10fe0,Hi: 0x10ff6,Stride: 1},
        Range32{Lo:0x11003,Hi: 0x11037,Stride: 1},
        Range32{Lo:0x11071,Hi: 0x11072,Stride: 1},
        Range32{Lo:0x11075,Hi: 0x11083,Stride: 14},
        Range32{Lo:0x11084,Hi: 0x110af,Stride: 1},
        Range32{Lo:0x110d0,Hi: 0x110e8,Stride: 1},
        Range32{Lo:0x11103,Hi: 0x11126,Stride: 1},
        Range32{Lo:0x11144,Hi: 0x11147,Stride: 3},
        Range32{Lo:0x11150,Hi: 0x11172,Stride: 1},
        Range32{Lo:0x11176,Hi: 0x11183,Stride: 13},
        Range32{Lo:0x11184,Hi: 0x111b2,Stride: 1},
        Range32{Lo:0x111c1,Hi: 0x111c4,Stride: 1},
        Range32{Lo:0x111da,Hi: 0x111dc,Stride: 2},
        Range32{Lo:0x11200,Hi: 0x11211,Stride: 1},
        Range32{Lo:0x11213,Hi: 0x1122b,Stride: 1},
        Range32{Lo:0x1123f,Hi: 0x11240,Stride: 1},
        Range32{Lo:0x11280,Hi: 0x11286,Stride: 1},
        Range32{Lo:0x11288,Hi: 0x1128a,Stride: 2},
        Range32{Lo:0x1128b,Hi: 0x1128d,Stride: 1},
        Range32{Lo:0x1128f,Hi: 0x1129d,Stride: 1},
        Range32{Lo:0x1129f,Hi: 0x112a8,Stride: 1},
        Range32{Lo:0x112b0,Hi: 0x112de,Stride: 1},
        Range32{Lo:0x11305,Hi: 0x1130c,Stride: 1},
        Range32{Lo:0x1130f,Hi: 0x11310,Stride: 1},
        Range32{Lo:0x11313,Hi: 0x11328,Stride: 1},
        Range32{Lo:0x1132a,Hi: 0x11330,Stride: 1},
        Range32{Lo:0x11332,Hi: 0x11333,Stride: 1},
        Range32{Lo:0x11335,Hi: 0x11339,Stride: 1},
        Range32{Lo:0x1133d,Hi: 0x11350,Stride: 19},
        Range32{Lo:0x1135d,Hi: 0x11361,Stride: 1},
        Range32{Lo:0x11400,Hi: 0x11434,Stride: 1},
        Range32{Lo:0x11447,Hi: 0x1144a,Stride: 1},
        Range32{Lo:0x1145f,Hi: 0x11461,Stride: 1},
        Range32{Lo:0x11480,Hi: 0x114af,Stride: 1},
        Range32{Lo:0x114c4,Hi: 0x114c5,Stride: 1},
        Range32{Lo:0x114c7,Hi: 0x11580,Stride: 185},
        Range32{Lo:0x11581,Hi: 0x115ae,Stride: 1},
        Range32{Lo:0x115d8,Hi: 0x115db,Stride: 1},
        Range32{Lo:0x11600,Hi: 0x1162f,Stride: 1},
        Range32{Lo:0x11644,Hi: 0x11680,Stride: 60},
        Range32{Lo:0x11681,Hi: 0x116aa,Stride: 1},
        Range32{Lo:0x116b8,Hi: 0x11700,Stride: 72},
        Range32{Lo:0x11701,Hi: 0x1171a,Stride: 1},
        Range32{Lo:0x11740,Hi: 0x11746,Stride: 1},
        Range32{Lo:0x11800,Hi: 0x1182b,Stride: 1},
        Range32{Lo:0x118a0,Hi: 0x118df,Stride: 1},
        Range32{Lo:0x118ff,Hi: 0x11906,Stride: 1},
        Range32{Lo:0x11909,Hi: 0x1190c,Stride: 3},
        Range32{Lo:0x1190d,Hi: 0x11913,Stride: 1},
        Range32{Lo:0x11915,Hi: 0x11916,Stride: 1},
        Range32{Lo:0x11918,Hi: 0x1192f,Stride: 1},
        Range32{Lo:0x1193f,Hi: 0x11941,Stride: 2},
        Range32{Lo:0x119a0,Hi: 0x119a7,Stride: 1},
        Range32{Lo:0x119aa,Hi: 0x119d0,Stride: 1},
        Range32{Lo:0x119e1,Hi: 0x119e3,Stride: 2},
        Range32{Lo:0x11a00,Hi: 0x11a0b,Stride: 11},
        Range32{Lo:0x11a0c,Hi: 0x11a32,Stride: 1},
        Range32{Lo:0x11a3a,Hi: 0x11a50,Stride: 22},
        Range32{Lo:0x11a5c,Hi: 0x11a89,Stride: 1},
        Range32{Lo:0x11a9d,Hi: 0x11ab0,Stride: 19},
        Range32{Lo:0x11ab1,Hi: 0x11af8,Stride: 1},
        Range32{Lo:0x11c00,Hi: 0x11c08,Stride: 1},
        Range32{Lo:0x11c0a,Hi: 0x11c2e,Stride: 1},
        Range32{Lo:0x11c40,Hi: 0x11c72,Stride: 50},
        Range32{Lo:0x11c73,Hi: 0x11c8f,Stride: 1},
        Range32{Lo:0x11d00,Hi: 0x11d06,Stride: 1},
        Range32{Lo:0x11d08,Hi: 0x11d09,Stride: 1},
        Range32{Lo:0x11d0b,Hi: 0x11d30,Stride: 1},
        Range32{Lo:0x11d46,Hi: 0x11d60,Stride: 26},
        Range32{Lo:0x11d61,Hi: 0x11d65,Stride: 1},
        Range32{Lo:0x11d67,Hi: 0x11d68,Stride: 1},
        Range32{Lo:0x11d6a,Hi: 0x11d89,Stride: 1},
        Range32{Lo:0x11d98,Hi: 0x11ee0,Stride: 328},
        Range32{Lo:0x11ee1,Hi: 0x11ef2,Stride: 1},
        Range32{Lo:0x11f02,Hi: 0x11f04,Stride: 2},
        Range32{Lo:0x11f05,Hi: 0x11f10,Stride: 1},
        Range32{Lo:0x11f12,Hi: 0x11f33,Stride: 1},
        Range32{Lo:0x11fb0,Hi: 0x12000,Stride: 80},
        Range32{Lo:0x12001,Hi: 0x12399,Stride: 1},
        Range32{Lo:0x12480,Hi: 0x12543,Stride: 1},
        Range32{Lo:0x12f90,Hi: 0x12ff0,Stride: 1},
        Range32{Lo:0x13000,Hi: 0x1342f,Stride: 1},
        Range32{Lo:0x13441,Hi: 0x13446,Stride: 1},
        Range32{Lo:0x14400,Hi: 0x14646,Stride: 1},
        Range32{Lo:0x16800,Hi: 0x16a38,Stride: 1},
        Range32{Lo:0x16a40,Hi: 0x16a5e,Stride: 1},
        Range32{Lo:0x16a70,Hi: 0x16abe,Stride: 1},
        Range32{Lo:0x16ad0,Hi: 0x16aed,Stride: 1},
        Range32{Lo:0x16b00,Hi: 0x16b2f,Stride: 1},
        Range32{Lo:0x16b40,Hi: 0x16b43,Stride: 1},
        Range32{Lo:0x16b63,Hi: 0x16b77,Stride: 1},
        Range32{Lo:0x16b7d,Hi: 0x16b8f,Stride: 1},
        Range32{Lo:0x16e40,Hi: 0x16e7f,Stride: 1},
        Range32{Lo:0x16f00,Hi: 0x16f4a,Stride: 1},
        Range32{Lo:0x16f50,Hi: 0x16f93,Stride: 67},
        Range32{Lo:0x16f94,Hi: 0x16f9f,Stride: 1},
        Range32{Lo:0x16fe0,Hi: 0x16fe1,Stride: 1},
        Range32{Lo:0x16fe3,Hi: 0x17000,Stride: 29},
        Range32{Lo:0x17001,Hi: 0x187f7,Stride: 1},
        Range32{Lo:0x18800,Hi: 0x18cd5,Stride: 1},
        Range32{Lo:0x18d00,Hi: 0x18d08,Stride: 1},
        Range32{Lo:0x1aff0,Hi: 0x1aff3,Stride: 1},
        Range32{Lo:0x1aff5,Hi: 0x1affb,Stride: 1},
        Range32{Lo:0x1affd,Hi: 0x1affe,Stride: 1},
        Range32{Lo:0x1b000,Hi: 0x1b122,Stride: 1},
        Range32{Lo:0x1b132,Hi: 0x1b150,Stride: 30},
        Range32{Lo:0x1b151,Hi: 0x1b152,Stride: 1},
        Range32{Lo:0x1b155,Hi: 0x1b164,Stride: 15},
        Range32{Lo:0x1b165,Hi: 0x1b167,Stride: 1},
        Range32{Lo:0x1b170,Hi: 0x1b2fb,Stride: 1},
        Range32{Lo:0x1bc00,Hi: 0x1bc6a,Stride: 1},
        Range32{Lo:0x1bc70,Hi: 0x1bc7c,Stride: 1},
        Range32{Lo:0x1bc80,Hi: 0x1bc88,Stride: 1},
        Range32{Lo:0x1bc90,Hi: 0x1bc99,Stride: 1},
        Range32{Lo:0x1d400,Hi: 0x1d454,Stride: 1},
        Range32{Lo:0x1d456,Hi: 0x1d49c,Stride: 1},
        Range32{Lo:0x1d49e,Hi: 0x1d49f,Stride: 1},
        Range32{Lo:0x1d4a2,Hi: 0x1d4a5,Stride: 3},
        Range32{Lo:0x1d4a6,Hi: 0x1d4a9,Stride: 3},
        Range32{Lo:0x1d4aa,Hi: 0x1d4ac,Stride: 1},
        Range32{Lo:0x1d4ae,Hi: 0x1d4b9,Stride: 1},
        Range32{Lo:0x1d4bb,Hi: 0x1d4bd,Stride: 2},
        Range32{Lo:0x1d4be,Hi: 0x1d4c3,Stride: 1},
        Range32{Lo:0x1d4c5,Hi: 0x1d505,Stride: 1},
        Range32{Lo:0x1d507,Hi: 0x1d50a,Stride: 1},
        Range32{Lo:0x1d50d,Hi: 0x1d514,Stride: 1},
        Range32{Lo:0x1d516,Hi: 0x1d51c,Stride: 1},
        Range32{Lo:0x1d51e,Hi: 0x1d539,Stride: 1},
        Range32{Lo:0x1d53b,Hi: 0x1d53e,Stride: 1},
        Range32{Lo:0x1d540,Hi: 0x1d544,Stride: 1},
        Range32{Lo:0x1d546,Hi: 0x1d54a,Stride: 4},
        Range32{Lo:0x1d54b,Hi: 0x1d550,Stride: 1},
        Range32{Lo:0x1d552,Hi: 0x1d6a5,Stride: 1},
        Range32{Lo:0x1d6a8,Hi: 0x1d6c0,Stride: 1},
        Range32{Lo:0x1d6c2,Hi: 0x1d6da,Stride: 1},
        Range32{Lo:0x1d6dc,Hi: 0x1d6fa,Stride: 1},
        Range32{Lo:0x1d6fc,Hi: 0x1d714,Stride: 1},
        Range32{Lo:0x1d716,Hi: 0x1d734,Stride: 1},
        Range32{Lo:0x1d736,Hi: 0x1d74e,Stride: 1},
        Range32{Lo:0x1d750,Hi: 0x1d76e,Stride: 1},
        Range32{Lo:0x1d770,Hi: 0x1d788,Stride: 1},
        Range32{Lo:0x1d78a,Hi: 0x1d7a8,Stride: 1},
        Range32{Lo:0x1d7aa,Hi: 0x1d7c2,Stride: 1},
        Range32{Lo:0x1d7c4,Hi: 0x1d7cb,Stride: 1},
        Range32{Lo:0x1df00,Hi: 0x1df1e,Stride: 1},
        Range32{Lo:0x1df25,Hi: 0x1df2a,Stride: 1},
        Range32{Lo:0x1e030,Hi: 0x1e06d,Stride: 1},
        Range32{Lo:0x1e100,Hi: 0x1e12c,Stride: 1},
        Range32{Lo:0x1e137,Hi: 0x1e13d,Stride: 1},
        Range32{Lo:0x1e14e,Hi: 0x1e290,Stride: 322},
        Range32{Lo:0x1e291,Hi: 0x1e2ad,Stride: 1},
        Range32{Lo:0x1e2c0,Hi: 0x1e2eb,Stride: 1},
        Range32{Lo:0x1e4d0,Hi: 0x1e4eb,Stride: 1},
        Range32{Lo:0x1e7e0,Hi: 0x1e7e6,Stride: 1},
        Range32{Lo:0x1e7e8,Hi: 0x1e7eb,Stride: 1},
        Range32{Lo:0x1e7ed,Hi: 0x1e7ee,Stride: 1},
        Range32{Lo:0x1e7f0,Hi: 0x1e7fe,Stride: 1},
        Range32{Lo:0x1e800,Hi: 0x1e8c4,Stride: 1},
        Range32{Lo:0x1e900,Hi: 0x1e943,Stride: 1},
        Range32{Lo:0x1e94b,Hi: 0x1ee00,Stride: 1205},
        Range32{Lo:0x1ee01,Hi: 0x1ee03,Stride: 1},
        Range32{Lo:0x1ee05,Hi: 0x1ee1f,Stride: 1},
        Range32{Lo:0x1ee21,Hi: 0x1ee22,Stride: 1},
        Range32{Lo:0x1ee24,Hi: 0x1ee27,Stride: 3},
        Range32{Lo:0x1ee29,Hi: 0x1ee32,Stride: 1},
        Range32{Lo:0x1ee34,Hi: 0x1ee37,Stride: 1},
        Range32{Lo:0x1ee39,Hi: 0x1ee3b,Stride: 2},
        Range32{Lo:0x1ee42,Hi: 0x1ee47,Stride: 5},
        Range32{Lo:0x1ee49,Hi: 0x1ee4d,Stride: 2},
        Range32{Lo:0x1ee4e,Hi: 0x1ee4f,Stride: 1},
        Range32{Lo:0x1ee51,Hi: 0x1ee52,Stride: 1},
        Range32{Lo:0x1ee54,Hi: 0x1ee57,Stride: 3},
        Range32{Lo:0x1ee59,Hi: 0x1ee61,Stride: 2},
        Range32{Lo:0x1ee62,Hi: 0x1ee64,Stride: 2},
        Range32{Lo:0x1ee67,Hi: 0x1ee6a,Stride: 1},
        Range32{Lo:0x1ee6c,Hi: 0x1ee72,Stride: 1},
        Range32{Lo:0x1ee74,Hi: 0x1ee77,Stride: 1},
        Range32{Lo:0x1ee79,Hi: 0x1ee7c,Stride: 1},
        Range32{Lo:0x1ee7e,Hi: 0x1ee80,Stride: 2},
        Range32{Lo:0x1ee81,Hi: 0x1ee89,Stride: 1},
        Range32{Lo:0x1ee8b,Hi: 0x1ee9b,Stride: 1},
        Range32{Lo:0x1eea1,Hi: 0x1eea3,Stride: 1},
        Range32{Lo:0x1eea5,Hi: 0x1eea9,Stride: 1},
        Range32{Lo:0x1eeab,Hi: 0x1eebb,Stride: 1},
        Range32{Lo:0x20000,Hi: 0x2a6df,Stride: 1},
        Range32{Lo:0x2a700,Hi: 0x2b739,Stride: 1},
        Range32{Lo:0x2b740,Hi: 0x2b81d,Stride: 1},
        Range32{Lo:0x2b820,Hi: 0x2cea1,Stride: 1},
        Range32{Lo:0x2ceb0,Hi: 0x2ebe0,Stride: 1},
        Range32{Lo:0x2f800,Hi: 0x2fa1d,Stride: 1},
        Range32{Lo:0x30000,Hi: 0x3134a,Stride: 1},
        Range32{Lo:0x31350,Hi: 0x323af,Stride: 1},
    ]);
}

lazy_static::lazy_static! {
    static ref _L_RangeTable_R16: Arc<Vec<Range16>> = Arc::new(vec![
        Range16{Lo: 0x0061, Hi: 0x007a, Stride: 1},
        Range16{Lo:0x0061,Hi: 0x007a,Stride: 1},
        Range16{Lo:0x00aa,Hi: 0x00b5,Stride: 11},
        Range16{Lo:0x00ba,Hi: 0x00c0,Stride: 6},
        Range16{Lo:0x00c1,Hi: 0x00d6,Stride: 1},
        Range16{Lo:0x00d8,Hi: 0x00f6,Stride: 1},
        Range16{Lo:0x00f8,Hi: 0x02c1,Stride: 1},
        Range16{Lo:0x02c6,Hi: 0x02d1,Stride: 1},
        Range16{Lo:0x02e0,Hi: 0x02e4,Stride: 1},
        Range16{Lo:0x02ec,Hi: 0x02ee,Stride: 2},
        Range16{Lo:0x0370,Hi: 0x0374,Stride: 1},
        Range16{Lo:0x0376,Hi: 0x0377,Stride: 1},
        Range16{Lo:0x037a,Hi: 0x037d,Stride: 1},
        Range16{Lo:0x037f,Hi: 0x0386,Stride: 7},
        Range16{Lo:0x0388,Hi: 0x038a,Stride: 1},
        Range16{Lo:0x038c,Hi: 0x038e,Stride: 2},
        Range16{Lo:0x038f,Hi: 0x03a1,Stride: 1},
        Range16{Lo:0x03a3,Hi: 0x03f5,Stride: 1},
        Range16{Lo:0x03f7,Hi: 0x0481,Stride: 1},
        Range16{Lo:0x048a,Hi: 0x052f,Stride: 1},
        Range16{Lo:0x0531,Hi: 0x0556,Stride: 1},
        Range16{Lo:0x0559,Hi: 0x0560,Stride: 7},
        Range16{Lo:0x0561,Hi: 0x0588,Stride: 1},
        Range16{Lo:0x05d0,Hi: 0x05ea,Stride: 1},
        Range16{Lo:0x05ef,Hi: 0x05f2,Stride: 1},
        Range16{Lo:0x0620,Hi: 0x064a,Stride: 1},
        Range16{Lo:0x066e,Hi: 0x066f,Stride: 1},
        Range16{Lo:0x0671,Hi: 0x06d3,Stride: 1},
        Range16{Lo:0x06d5,Hi: 0x06e5,Stride: 16},
        Range16{Lo:0x06e6,Hi: 0x06ee,Stride: 8},
        Range16{Lo:0x06ef,Hi: 0x06fa,Stride: 11},
        Range16{Lo:0x06fb,Hi: 0x06fc,Stride: 1},
        Range16{Lo:0x06ff,Hi: 0x0710,Stride: 17},
        Range16{Lo:0x0712,Hi: 0x072f,Stride: 1},
        Range16{Lo:0x074d,Hi: 0x07a5,Stride: 1},
        Range16{Lo:0x07b1,Hi: 0x07ca,Stride: 25},
        Range16{Lo:0x07cb,Hi: 0x07ea,Stride: 1},
        Range16{Lo:0x07f4,Hi: 0x07f5,Stride: 1},
        Range16{Lo:0x07fa,Hi: 0x0800,Stride: 6},
        Range16{Lo:0x0801,Hi: 0x0815,Stride: 1},
        Range16{Lo:0x081a,Hi: 0x0824,Stride: 10},
        Range16{Lo:0x0828,Hi: 0x0840,Stride: 24},
        Range16{Lo:0x0841,Hi: 0x0858,Stride: 1},
        Range16{Lo:0x0860,Hi: 0x086a,Stride: 1},
        Range16{Lo:0x0870,Hi: 0x0887,Stride: 1},
        Range16{Lo:0x0889,Hi: 0x088e,Stride: 1},
        Range16{Lo:0x08a0,Hi: 0x08c9,Stride: 1},
        Range16{Lo:0x0904,Hi: 0x0939,Stride: 1},
        Range16{Lo:0x093d,Hi: 0x0950,Stride: 19},
        Range16{Lo:0x0958,Hi: 0x0961,Stride: 1},
        Range16{Lo:0x0971,Hi: 0x0980,Stride: 1},
        Range16{Lo:0x0985,Hi: 0x098c,Stride: 1},
        Range16{Lo:0x098f,Hi: 0x0990,Stride: 1},
        Range16{Lo:0x0993,Hi: 0x09a8,Stride: 1},
        Range16{Lo:0x09aa,Hi: 0x09b0,Stride: 1},
        Range16{Lo:0x09b2,Hi: 0x09b6,Stride: 4},
        Range16{Lo:0x09b7,Hi: 0x09b9,Stride: 1},
        Range16{Lo:0x09bd,Hi: 0x09ce,Stride: 17},
        Range16{Lo:0x09dc,Hi: 0x09dd,Stride: 1},
        Range16{Lo:0x09df,Hi: 0x09e1,Stride: 1},
        Range16{Lo:0x09f0,Hi: 0x09f1,Stride: 1},
        Range16{Lo:0x09fc,Hi: 0x0a05,Stride: 9},
        Range16{Lo:0x0a06,Hi: 0x0a0a,Stride: 1},
        Range16{Lo:0x0a0f,Hi: 0x0a10,Stride: 1},
        Range16{Lo:0x0a13,Hi: 0x0a28,Stride: 1},
        Range16{Lo:0x0a2a,Hi: 0x0a30,Stride: 1},
        Range16{Lo:0x0a32,Hi: 0x0a33,Stride: 1},
        Range16{Lo:0x0a35,Hi: 0x0a36,Stride: 1},
        Range16{Lo:0x0a38,Hi: 0x0a39,Stride: 1},
        Range16{Lo:0x0a59,Hi: 0x0a5c,Stride: 1},
        Range16{Lo:0x0a5e,Hi: 0x0a72,Stride: 20},
        Range16{Lo:0x0a73,Hi: 0x0a74,Stride: 1},
        Range16{Lo:0x0a85,Hi: 0x0a8d,Stride: 1},
        Range16{Lo:0x0a8f,Hi: 0x0a91,Stride: 1},
        Range16{Lo:0x0a93,Hi: 0x0aa8,Stride: 1},
        Range16{Lo:0x0aaa,Hi: 0x0ab0,Stride: 1},
        Range16{Lo:0x0ab2,Hi: 0x0ab3,Stride: 1},
        Range16{Lo:0x0ab5,Hi: 0x0ab9,Stride: 1},
        Range16{Lo:0x0abd,Hi: 0x0ad0,Stride: 19},
        Range16{Lo:0x0ae0,Hi: 0x0ae1,Stride: 1},
        Range16{Lo:0x0af9,Hi: 0x0b05,Stride: 12},
        Range16{Lo:0x0b06,Hi: 0x0b0c,Stride: 1},
        Range16{Lo:0x0b0f,Hi: 0x0b10,Stride: 1},
        Range16{Lo:0x0b13,Hi: 0x0b28,Stride: 1},
        Range16{Lo:0x0b2a,Hi: 0x0b30,Stride: 1},
        Range16{Lo:0x0b32,Hi: 0x0b33,Stride: 1},
        Range16{Lo:0x0b35,Hi: 0x0b39,Stride: 1},
        Range16{Lo:0x0b3d,Hi: 0x0b5c,Stride: 31},
        Range16{Lo:0x0b5d,Hi: 0x0b5f,Stride: 2},
        Range16{Lo:0x0b60,Hi: 0x0b61,Stride: 1},
        Range16{Lo:0x0b71,Hi: 0x0b83,Stride: 18},
        Range16{Lo:0x0b85,Hi: 0x0b8a,Stride: 1},
        Range16{Lo:0x0b8e,Hi: 0x0b90,Stride: 1},
        Range16{Lo:0x0b92,Hi: 0x0b95,Stride: 1},
        Range16{Lo:0x0b99,Hi: 0x0b9a,Stride: 1},
        Range16{Lo:0x0b9c,Hi: 0x0b9e,Stride: 2},
        Range16{Lo:0x0b9f,Hi: 0x0ba3,Stride: 4},
        Range16{Lo:0x0ba4,Hi: 0x0ba8,Stride: 4},
        Range16{Lo:0x0ba9,Hi: 0x0baa,Stride: 1},
        Range16{Lo:0x0bae,Hi: 0x0bb9,Stride: 1},
        Range16{Lo:0x0bd0,Hi: 0x0c05,Stride: 53},
        Range16{Lo:0x0c06,Hi: 0x0c0c,Stride: 1},
        Range16{Lo:0x0c0e,Hi: 0x0c10,Stride: 1},
        Range16{Lo:0x0c12,Hi: 0x0c28,Stride: 1},
        Range16{Lo:0x0c2a,Hi: 0x0c39,Stride: 1},
        Range16{Lo:0x0c3d,Hi: 0x0c58,Stride: 27},
        Range16{Lo:0x0c59,Hi: 0x0c5a,Stride: 1},
        Range16{Lo:0x0c5d,Hi: 0x0c60,Stride: 3},
        Range16{Lo:0x0c61,Hi: 0x0c80,Stride: 31},
        Range16{Lo:0x0c85,Hi: 0x0c8c,Stride: 1},
        Range16{Lo:0x0c8e,Hi: 0x0c90,Stride: 1},
        Range16{Lo:0x0c92,Hi: 0x0ca8,Stride: 1},
        Range16{Lo:0x0caa,Hi: 0x0cb3,Stride: 1},
        Range16{Lo:0x0cb5,Hi: 0x0cb9,Stride: 1},
        Range16{Lo:0x0cbd,Hi: 0x0cdd,Stride: 32},
        Range16{Lo:0x0cde,Hi: 0x0ce0,Stride: 2},
        Range16{Lo:0x0ce1,Hi: 0x0cf1,Stride: 16},
        Range16{Lo:0x0cf2,Hi: 0x0d04,Stride: 18},
        Range16{Lo:0x0d05,Hi: 0x0d0c,Stride: 1},
        Range16{Lo:0x0d0e,Hi: 0x0d10,Stride: 1},
        Range16{Lo:0x0d12,Hi: 0x0d3a,Stride: 1},
        Range16{Lo:0x0d3d,Hi: 0x0d4e,Stride: 17},
        Range16{Lo:0x0d54,Hi: 0x0d56,Stride: 1},
        Range16{Lo:0x0d5f,Hi: 0x0d61,Stride: 1},
        Range16{Lo:0x0d7a,Hi: 0x0d7f,Stride: 1},
        Range16{Lo:0x0d85,Hi: 0x0d96,Stride: 1},
        Range16{Lo:0x0d9a,Hi: 0x0db1,Stride: 1},
        Range16{Lo:0x0db3,Hi: 0x0dbb,Stride: 1},
        Range16{Lo:0x0dbd,Hi: 0x0dc0,Stride: 3},
        Range16{Lo:0x0dc1,Hi: 0x0dc6,Stride: 1},
        Range16{Lo:0x0e01,Hi: 0x0e30,Stride: 1},
        Range16{Lo:0x0e32,Hi: 0x0e33,Stride: 1},
        Range16{Lo:0x0e40,Hi: 0x0e46,Stride: 1},
        Range16{Lo:0x0e81,Hi: 0x0e82,Stride: 1},
        Range16{Lo:0x0e84,Hi: 0x0e86,Stride: 2},
        Range16{Lo:0x0e87,Hi: 0x0e8a,Stride: 1},
        Range16{Lo:0x0e8c,Hi: 0x0ea3,Stride: 1},
        Range16{Lo:0x0ea5,Hi: 0x0ea7,Stride: 2},
        Range16{Lo:0x0ea8,Hi: 0x0eb0,Stride: 1},
        Range16{Lo:0x0eb2,Hi: 0x0eb3,Stride: 1},
        Range16{Lo:0x0ebd,Hi: 0x0ec0,Stride: 3},
        Range16{Lo:0x0ec1,Hi: 0x0ec4,Stride: 1},
        Range16{Lo:0x0ec6,Hi: 0x0edc,Stride: 22},
        Range16{Lo:0x0edd,Hi: 0x0edf,Stride: 1},
        Range16{Lo:0x0f00,Hi: 0x0f40,Stride: 64},
        Range16{Lo:0x0f41,Hi: 0x0f47,Stride: 1},
        Range16{Lo:0x0f49,Hi: 0x0f6c,Stride: 1},
        Range16{Lo:0x0f88,Hi: 0x0f8c,Stride: 1},
        Range16{Lo:0x1000,Hi: 0x102a,Stride: 1},
        Range16{Lo:0x103f,Hi: 0x1050,Stride: 17},
        Range16{Lo:0x1051,Hi: 0x1055,Stride: 1},
        Range16{Lo:0x105a,Hi: 0x105d,Stride: 1},
        Range16{Lo:0x1061,Hi: 0x1065,Stride: 4},
        Range16{Lo:0x1066,Hi: 0x106e,Stride: 8},
        Range16{Lo:0x106f,Hi: 0x1070,Stride: 1},
        Range16{Lo:0x1075,Hi: 0x1081,Stride: 1},
        Range16{Lo:0x108e,Hi: 0x10a0,Stride: 18},
        Range16{Lo:0x10a1,Hi: 0x10c5,Stride: 1},
        Range16{Lo:0x10c7,Hi: 0x10cd,Stride: 6},
        Range16{Lo:0x10d0,Hi: 0x10fa,Stride: 1},
        Range16{Lo:0x10fc,Hi: 0x1248,Stride: 1},
        Range16{Lo:0x124a,Hi: 0x124d,Stride: 1},
        Range16{Lo:0x1250,Hi: 0x1256,Stride: 1},
        Range16{Lo:0x1258,Hi: 0x125a,Stride: 2},
        Range16{Lo:0x125b,Hi: 0x125d,Stride: 1},
        Range16{Lo:0x1260,Hi: 0x1288,Stride: 1},
        Range16{Lo:0x128a,Hi: 0x128d,Stride: 1},
        Range16{Lo:0x1290,Hi: 0x12b0,Stride: 1},
        Range16{Lo:0x12b2,Hi: 0x12b5,Stride: 1},
        Range16{Lo:0x12b8,Hi: 0x12be,Stride: 1},
        Range16{Lo:0x12c0,Hi: 0x12c2,Stride: 2},
        Range16{Lo:0x12c3,Hi: 0x12c5,Stride: 1},
        Range16{Lo:0x12c8,Hi: 0x12d6,Stride: 1},
        Range16{Lo:0x12d8,Hi: 0x1310,Stride: 1},
        Range16{Lo:0x1312,Hi: 0x1315,Stride: 1},
        Range16{Lo:0x1318,Hi: 0x135a,Stride: 1},
        Range16{Lo:0x1380,Hi: 0x138f,Stride: 1},
        Range16{Lo:0x13a0,Hi: 0x13f5,Stride: 1},
        Range16{Lo:0x13f8,Hi: 0x13fd,Stride: 1},
        Range16{Lo:0x1401,Hi: 0x166c,Stride: 1},
        Range16{Lo:0x166f,Hi: 0x167f,Stride: 1},
        Range16{Lo:0x1681,Hi: 0x169a,Stride: 1},
        Range16{Lo:0x16a0,Hi: 0x16ea,Stride: 1},
        Range16{Lo:0x16f1,Hi: 0x16f8,Stride: 1},
        Range16{Lo:0x1700,Hi: 0x1711,Stride: 1},
        Range16{Lo:0x171f,Hi: 0x1731,Stride: 1},
        Range16{Lo:0x1740,Hi: 0x1751,Stride: 1},
        Range16{Lo:0x1760,Hi: 0x176c,Stride: 1},
        Range16{Lo:0x176e,Hi: 0x1770,Stride: 1},
        Range16{Lo:0x1780,Hi: 0x17b3,Stride: 1},
        Range16{Lo:0x17d7,Hi: 0x17dc,Stride: 5},
        Range16{Lo:0x1820,Hi: 0x1878,Stride: 1},
        Range16{Lo:0x1880,Hi: 0x1884,Stride: 1},
        Range16{Lo:0x1887,Hi: 0x18a8,Stride: 1},
        Range16{Lo:0x18aa,Hi: 0x18b0,Stride: 6},
        Range16{Lo:0x18b1,Hi: 0x18f5,Stride: 1},
        Range16{Lo:0x1900,Hi: 0x191e,Stride: 1},
        Range16{Lo:0x1950,Hi: 0x196d,Stride: 1},
        Range16{Lo:0x1970,Hi: 0x1974,Stride: 1},
        Range16{Lo:0x1980,Hi: 0x19ab,Stride: 1},
        Range16{Lo:0x19b0,Hi: 0x19c9,Stride: 1},
        Range16{Lo:0x1a00,Hi: 0x1a16,Stride: 1},
        Range16{Lo:0x1a20,Hi: 0x1a54,Stride: 1},
        Range16{Lo:0x1aa7,Hi: 0x1b05,Stride: 94},
        Range16{Lo:0x1b06,Hi: 0x1b33,Stride: 1},
        Range16{Lo:0x1b45,Hi: 0x1b4c,Stride: 1},
        Range16{Lo:0x1b83,Hi: 0x1ba0,Stride: 1},
        Range16{Lo:0x1bae,Hi: 0x1baf,Stride: 1},
        Range16{Lo:0x1bba,Hi: 0x1be5,Stride: 1},
        Range16{Lo:0x1c00,Hi: 0x1c23,Stride: 1},
        Range16{Lo:0x1c4d,Hi: 0x1c4f,Stride: 1},
        Range16{Lo:0x1c5a,Hi: 0x1c7d,Stride: 1},
        Range16{Lo:0x1c80,Hi: 0x1c88,Stride: 1},
        Range16{Lo:0x1c90,Hi: 0x1cba,Stride: 1},
        Range16{Lo:0x1cbd,Hi: 0x1cbf,Stride: 1},
        Range16{Lo:0x1ce9,Hi: 0x1cec,Stride: 1},
        Range16{Lo:0x1cee,Hi: 0x1cf3,Stride: 1},
        Range16{Lo:0x1cf5,Hi: 0x1cf6,Stride: 1},
        Range16{Lo:0x1cfa,Hi: 0x1d00,Stride: 6},
        Range16{Lo:0x1d01,Hi: 0x1dbf,Stride: 1},
        Range16{Lo:0x1e00,Hi: 0x1f15,Stride: 1},
        Range16{Lo:0x1f18,Hi: 0x1f1d,Stride: 1},
        Range16{Lo:0x1f20,Hi: 0x1f45,Stride: 1},
        Range16{Lo:0x1f48,Hi: 0x1f4d,Stride: 1},
        Range16{Lo:0x1f50,Hi: 0x1f57,Stride: 1},
        Range16{Lo:0x1f59,Hi: 0x1f5f,Stride: 2},
        Range16{Lo:0x1f60,Hi: 0x1f7d,Stride: 1},
        Range16{Lo:0x1f80,Hi: 0x1fb4,Stride: 1},
        Range16{Lo:0x1fb6,Hi: 0x1fbc,Stride: 1},
        Range16{Lo:0x1fbe,Hi: 0x1fc2,Stride: 4},
        Range16{Lo:0x1fc3,Hi: 0x1fc4,Stride: 1},
        Range16{Lo:0x1fc6,Hi: 0x1fcc,Stride: 1},
        Range16{Lo:0x1fd0,Hi: 0x1fd3,Stride: 1},
        Range16{Lo:0x1fd6,Hi: 0x1fdb,Stride: 1},
        Range16{Lo:0x1fe0,Hi: 0x1fec,Stride: 1},
        Range16{Lo:0x1ff2,Hi: 0x1ff4,Stride: 1},
        Range16{Lo:0x1ff6,Hi: 0x1ffc,Stride: 1},
        Range16{Lo:0x2071,Hi: 0x207f,Stride: 14},
        Range16{Lo:0x2090,Hi: 0x209c,Stride: 1},
        Range16{Lo:0x2102,Hi: 0x2107,Stride: 5},
        Range16{Lo:0x210a,Hi: 0x2113,Stride: 1},
        Range16{Lo:0x2115,Hi: 0x2119,Stride: 4},
        Range16{Lo:0x211a,Hi: 0x211d,Stride: 1},
        Range16{Lo:0x2124,Hi: 0x212a,Stride: 2},
        Range16{Lo:0x212b,Hi: 0x212d,Stride: 1},
        Range16{Lo:0x212f,Hi: 0x2139,Stride: 1},
        Range16{Lo:0x213c,Hi: 0x213f,Stride: 1},
        Range16{Lo:0x2145,Hi: 0x2149,Stride: 1},
        Range16{Lo:0x214e,Hi: 0x2183,Stride: 53},
        Range16{Lo:0x2184,Hi: 0x2c00,Stride: 2684},
        Range16{Lo:0x2c01,Hi: 0x2ce4,Stride: 1},
        Range16{Lo:0x2ceb,Hi: 0x2cee,Stride: 1},
        Range16{Lo:0x2cf2,Hi: 0x2cf3,Stride: 1},
        Range16{Lo:0x2d00,Hi: 0x2d25,Stride: 1},
        Range16{Lo:0x2d27,Hi: 0x2d2d,Stride: 6},
        Range16{Lo:0x2d30,Hi: 0x2d67,Stride: 1},
        Range16{Lo:0x2d6f,Hi: 0x2d80,Stride: 17},
        Range16{Lo:0x2d81,Hi: 0x2d96,Stride: 1},
        Range16{Lo:0x2da0,Hi: 0x2da6,Stride: 1},
        Range16{Lo:0x2da8,Hi: 0x2dae,Stride: 1},
        Range16{Lo:0x2db0,Hi: 0x2db6,Stride: 1},
        Range16{Lo:0x2db8,Hi: 0x2dbe,Stride: 1},
        Range16{Lo:0x2dc0,Hi: 0x2dc6,Stride: 1},
        Range16{Lo:0x2dc8,Hi: 0x2dce,Stride: 1},
        Range16{Lo:0x2dd0,Hi: 0x2dd6,Stride: 1},
        Range16{Lo:0x2dd8,Hi: 0x2dde,Stride: 1},
        Range16{Lo:0x2e2f,Hi: 0x3005,Stride: 470},
        Range16{Lo:0x3006,Hi: 0x3031,Stride: 43},
        Range16{Lo:0x3032,Hi: 0x3035,Stride: 1},
        Range16{Lo:0x303b,Hi: 0x303c,Stride: 1},
        Range16{Lo:0x3041,Hi: 0x3096,Stride: 1},
        Range16{Lo:0x309d,Hi: 0x309f,Stride: 1},
        Range16{Lo:0x30a1,Hi: 0x30fa,Stride: 1},
        Range16{Lo:0x30fc,Hi: 0x30ff,Stride: 1},
        Range16{Lo:0x3105,Hi: 0x312f,Stride: 1},
        Range16{Lo:0x3131,Hi: 0x318e,Stride: 1},
        Range16{Lo:0x31a0,Hi: 0x31bf,Stride: 1},
        Range16{Lo:0x31f0,Hi: 0x31ff,Stride: 1},
        Range16{Lo:0x3400,Hi: 0x4dbf,Stride: 1},
        Range16{Lo:0x4e00,Hi: 0xa48c,Stride: 1},
        Range16{Lo:0xa4d0,Hi: 0xa4fd,Stride: 1},
        Range16{Lo:0xa500,Hi: 0xa60c,Stride: 1},
        Range16{Lo:0xa610,Hi: 0xa61f,Stride: 1},
        Range16{Lo:0xa62a,Hi: 0xa62b,Stride: 1},
        Range16{Lo:0xa640,Hi: 0xa66e,Stride: 1},
        Range16{Lo:0xa67f,Hi: 0xa69d,Stride: 1},
        Range16{Lo:0xa6a0,Hi: 0xa6e5,Stride: 1},
        Range16{Lo:0xa717,Hi: 0xa71f,Stride: 1},
        Range16{Lo:0xa722,Hi: 0xa788,Stride: 1},
        Range16{Lo:0xa78b,Hi: 0xa7ca,Stride: 1},
        Range16{Lo:0xa7d0,Hi: 0xa7d1,Stride: 1},
        Range16{Lo:0xa7d3,Hi: 0xa7d5,Stride: 2},
        Range16{Lo:0xa7d6,Hi: 0xa7d9,Stride: 1},
        Range16{Lo:0xa7f2,Hi: 0xa801,Stride: 1},
        Range16{Lo:0xa803,Hi: 0xa805,Stride: 1},
        Range16{Lo:0xa807,Hi: 0xa80a,Stride: 1},
        Range16{Lo:0xa80c,Hi: 0xa822,Stride: 1},
        Range16{Lo:0xa840,Hi: 0xa873,Stride: 1},
        Range16{Lo:0xa882,Hi: 0xa8b3,Stride: 1},
        Range16{Lo:0xa8f2,Hi: 0xa8f7,Stride: 1},
        Range16{Lo:0xa8fb,Hi: 0xa8fd,Stride: 2},
        Range16{Lo:0xa8fe,Hi: 0xa90a,Stride: 12},
        Range16{Lo:0xa90b,Hi: 0xa925,Stride: 1},
        Range16{Lo:0xa930,Hi: 0xa946,Stride: 1},
        Range16{Lo:0xa960,Hi: 0xa97c,Stride: 1},
        Range16{Lo:0xa984,Hi: 0xa9b2,Stride: 1},
        Range16{Lo:0xa9cf,Hi: 0xa9e0,Stride: 17},
        Range16{Lo:0xa9e1,Hi: 0xa9e4,Stride: 1},
        Range16{Lo:0xa9e6,Hi: 0xa9ef,Stride: 1},
        Range16{Lo:0xa9fa,Hi: 0xa9fe,Stride: 1},
        Range16{Lo:0xaa00,Hi: 0xaa28,Stride: 1},
        Range16{Lo:0xaa40,Hi: 0xaa42,Stride: 1},
        Range16{Lo:0xaa44,Hi: 0xaa4b,Stride: 1},
        Range16{Lo:0xaa60,Hi: 0xaa76,Stride: 1},
        Range16{Lo:0xaa7a,Hi: 0xaa7e,Stride: 4},
        Range16{Lo:0xaa7f,Hi: 0xaaaf,Stride: 1},
        Range16{Lo:0xaab1,Hi: 0xaab5,Stride: 4},
        Range16{Lo:0xaab6,Hi: 0xaab9,Stride: 3},
        Range16{Lo:0xaaba,Hi: 0xaabd,Stride: 1},
        Range16{Lo:0xaac0,Hi: 0xaac2,Stride: 2},
        Range16{Lo:0xaadb,Hi: 0xaadd,Stride: 1},
        Range16{Lo:0xaae0,Hi: 0xaaea,Stride: 1},
        Range16{Lo:0xaaf2,Hi: 0xaaf4,Stride: 1},
        Range16{Lo:0xab01,Hi: 0xab06,Stride: 1},
        Range16{Lo:0xab09,Hi: 0xab0e,Stride: 1},
        Range16{Lo:0xab11,Hi: 0xab16,Stride: 1},
        Range16{Lo:0xab20,Hi: 0xab26,Stride: 1},
        Range16{Lo:0xab28,Hi: 0xab2e,Stride: 1},
        Range16{Lo:0xab30,Hi: 0xab5a,Stride: 1},
        Range16{Lo:0xab5c,Hi: 0xab69,Stride: 1},
        Range16{Lo:0xab70,Hi: 0xabe2,Stride: 1},
        Range16{Lo:0xac00,Hi: 0xd7a3,Stride: 1},
        Range16{Lo:0xd7b0,Hi: 0xd7c6,Stride: 1},
        Range16{Lo:0xd7cb,Hi: 0xd7fb,Stride: 1},
        Range16{Lo:0xf900,Hi: 0xfa6d,Stride: 1},
        Range16{Lo:0xfa70,Hi: 0xfad9,Stride: 1},
        Range16{Lo:0xfb00,Hi: 0xfb06,Stride: 1},
        Range16{Lo:0xfb13,Hi: 0xfb17,Stride: 1},
        Range16{Lo:0xfb1d,Hi: 0xfb1f,Stride: 2},
        Range16{Lo:0xfb20,Hi: 0xfb28,Stride: 1},
        Range16{Lo:0xfb2a,Hi: 0xfb36,Stride: 1},
        Range16{Lo:0xfb38,Hi: 0xfb3c,Stride: 1},
        Range16{Lo:0xfb3e,Hi: 0xfb40,Stride: 2},
        Range16{Lo:0xfb41,Hi: 0xfb43,Stride: 2},
        Range16{Lo:0xfb44,Hi: 0xfb46,Stride: 2},
        Range16{Lo:0xfb47,Hi: 0xfbb1,Stride: 1},
        Range16{Lo:0xfbd3,Hi: 0xfd3d,Stride: 1},
        Range16{Lo:0xfd50,Hi: 0xfd8f,Stride: 1},
        Range16{Lo:0xfd92,Hi: 0xfdc7,Stride: 1},
        Range16{Lo:0xfdf0,Hi: 0xfdfb,Stride: 1},
        Range16{Lo:0xfe70,Hi: 0xfe74,Stride: 1},
        Range16{Lo:0xfe76,Hi: 0xfefc,Stride: 1},
        Range16{Lo:0xff21,Hi: 0xff3a,Stride: 1},
        Range16{Lo:0xff41,Hi: 0xff5a,Stride: 1},
        Range16{Lo:0xff66,Hi: 0xffbe,Stride: 1},
        Range16{Lo:0xffc2,Hi: 0xffc7,Stride: 1},
        Range16{Lo:0xffca,Hi: 0xffcf,Stride: 1},
        Range16{Lo:0xffd2,Hi: 0xffd7,Stride: 1},
        Range16{Lo:0xffda,Hi: 0xffdc,Stride: 1},
    ]);

}

pub(crate) const properties: [u8; MaxLatin1 as usize + 1] = [
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
