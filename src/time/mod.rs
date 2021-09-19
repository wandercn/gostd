// #![allow(unused_assignments)]
#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::builtin::*;
use std::boxed::Box;

const Layout: &str = "01/02 03:04:05PM '06 -0700"; // The reference time, in numerical order.
const ANSIC: &str = "Mon Jan _2 15:04:05 2006";
const UnixDate: &str = "Mon Jan _2 15:04:05 MST 2006";
const RubyDate: &str = "Mon Jan 02 15:04:05 -0700 2006";
const RFC822: &str = "02 Jan 06 15:04 MST";
const RFC822Z: &str = "02 Jan 06 15:04 -0700"; // RFC822 with numeric zone
const RFC850: &str = "Monday, 02-Jan-06 15:04:05 MST";
const RFC1123: &str = "Mon, 02 Jan 2006 15:04:05 MST";
const RFC1123Z: &str = "Mon, 02 Jan 2006 15:04:05 -0700"; // RFC1123 with numeric zone
const RFC3339: &str = "2006-01-02T15:04:05Z07:00";
const RFC3339Nano: &str = "2006-01-02T15:04:05.999999999Z07:00";
const Kitchen: &str = "3:04PM";
// Handy time stamps.
const Stamp: &str = "Jan _2 15:04:05";
const StampMilli: &str = "Jan _2 15:04:05.000";
const StampMicro: &str = "Jan _2 15:04:05.000000";
const StampNano: &str = "Jan _2 15:04:05.000000000";

const Nanosecond: int64 = 1;
const Microsecond: int64 = 1000 * Nanosecond;
const Millisecond: int64 = 1000 * Microsecond;
const Second: int64 = 1000 * Millisecond;
const Minute: int64 = 60 * Second;
const Hour: int64 = 60 * Minute;

#[derive(Default,PartialEq,PartialOrd)]
pub struct Duration(int64); // 由于类型别名不能绑定方法通过元组类型结构体实现,访问元组内容用d.0数字下标访问，go源码是 type Duration int64

const minDuration: int64 = -1 << 63;
const maxDuration: int64 = 1 << 63 - 1;

impl Duration {
    pub fn new() -> Duration {
        Duration::default()
    }
    pub fn String(&self) -> string {
        let mut buf: [byte; 32] = [0; 32];
        let mut w = buf.len();

        // let Duration(mut u) = self;
        let mut u = self.0;
        u as uint64;
        let neg: bool;
        neg = u < 0;
        if neg {
            u = -u;
        }

        Second as uint64;
        if u < Second {
            let prec: int;
            w -= 1;
            buf[w] = 's' as byte;
            w -= 1;
            if u == 0 {
                return "0s".to_string();
            } else if u < Microsecond {
                prec = 0;
                buf[w] = 'n' as byte;
            } else if u < Millisecond {
                prec = 3;
                w -= 1;
                buf.copy_within(w.., ('µ' as byte).into());
            } else {
                prec = 6;
                buf[w] = 'm' as byte;
            }
            let (w2, u2) = fmtFrac(&mut buf[w..], u as uint64, prec);
            w = fmtInt(&mut buf[w2..], u2 % 60);
            u = u2 as int64;
        } else {
            w -= 1;
            buf[w] = 's' as byte;

            let (w3, u3) = fmtFrac(&mut buf[w..], u as uint64, 9);
            w = fmtInt(&mut buf[w3..], u3 % 60);
            u = u3 as int64 / 60;
            if u > 0 {
                w -= 1;
                buf[w] = 'm' as byte;
                w = fmtInt(&mut buf[w..], (u % 60) as uint64);
                u /= 60;

                if u > 0 {
                    w -= 1;
                    buf[w] = 'h' as byte;
                    w = fmtInt(&mut buf[w..], u as uint64);
                }
            }
        }
        if neg {
            w -= 1;
            buf[w] = '_' as byte;
        }
        string::from_utf8(buf[w..].to_vec()).unwrap()
    }

    pub fn Nanosecond(&self) -> int64 {
        self.0
    }

    pub fn Microsecond(&self) -> int64 {
        self.0 / Microsecond
    }

    pub fn Millisecond(&self) -> int64 {
        self.0 / Millisecond
    }

    pub fn Second(&self) -> float64 {
        let d = self.0;
        let sec = d / Second;
        let nsec = d % Second;
        sec as float64 + nsec as float64 / 1e9
    }

    pub fn Hours(&self) -> float64 {
        let d = self.0;
        let hour = d / Hour;
        let nsec = d % Hour;
        hour as float64 + nsec as float64 / (60 as float64 * 60 as float64 * 1e9)
    }

    pub fn Truncate(&self, m: Duration) -> Duration {
        let d = self.0;
        let m = m.0;
        if m <= 0 {
            return Duration(d);
        }
        Duration(d - d % m)
    }

    pub fn Round(&self, m: Duration) -> Duration {
        let d = self.0;
        let m = m.0;
        if m <= 0 {
            return Duration(d);
        }
        let mut r = d % m;

        if d < 0 {
            r = -r;
            if lessThanHalf(r as uint64, m as uint64) {
                return Duration(d + r);
            }
            let d1 = d - m + r;
            if d1 < d {
                return Duration(d1);
            }
            return Duration(minDuration);
        }
        if lessThanHalf(r as uint64, m as uint64) {
            return Duration(d - r);
        }
        let d1 = d + m - r;
        if d1 > d {
            return Duration(d1);
        }
        Duration(maxDuration)
    }
}

fn fmtFrac(buf: &mut [byte], v: uint64, prec: int) -> (uint, uint64) {
    let mut v = v;
    let mut w = buf.len();
    let mut print = false;
    for _i in 0..prec {
        let digit = v % 10;
        print = print || digit != 0;
        if print {
            w -= 1;
            buf[w] = digit as byte + '0' as byte;
        }
        v /= 10;
    }
    if print {
        w -= 1;
        buf[w] = '.' as byte;
    }
    (w, v)
}

fn fmtInt(buf: &mut [byte], v: uint64) -> uint {
    let mut v = v;
    let mut w = buf.len();
    if v == 0 {
        w -= w;
        buf[w] = '0' as byte;
    } else {
        while v > 0 {
            w = w - 1;
            buf[w] = (v % 10) as byte + '0' as byte;
            v /= 10;
        }
    }
    w as uint
}

fn lessThanHalf(x: uint64, y: uint64) -> bool {
    x + x < y
}

const secondsPerMinute: int64 = 60;
const secondsPerHour: int64 = 60 * secondsPerMinute;
const secondsPerDay: int64 = 24 * secondsPerHour;
const secondsPerWeek: int64 = 7 * secondsPerDay;
const daysPer400Years: int64 = 365 * 400 + 97;
const daysPer100Years: int64 = 365 * 100 + 24;
const daysPer4Years: int64 = 365 * 4 + 1;

const absoluteZeroYear: int64 = -292277022399;
// The year of the zero Time.
// Assumed by the unixToInternal computation below.
const internalYear: int64 = 1;
// Offsets to convert between internal and absolute or Unix times.
const absoluteToInternal: int64 =
    ((absoluteZeroYear - internalYear) as float64 * 365.2425 * secondsPerDay as float64) as int64;
const internalToAbsolute: int64 = -absoluteToInternal;
const unixToInternal: int64 = (1969 * 365 + 1969 / 4 - 1969 / 100 + 1969 / 400) * secondsPerDay;
const internalToUnix: int64 = -unixToInternal;
const wallToInternal: int64 = (1884 * 365 + 1884 / 4 - 1884 / 100 + 1884 / 400) * secondsPerDay;
const internalToWall: int64 = -wallToInternal;

const hasMonotonic: int64 = 1 << 63;
const maxWall: int64 = wallToInternal + (1 << 33 - 1); // year 2157
const minWall: int64 = wallToInternal; // year 1885
const nsecMask: int64 = 1 << 30 - 1;
const nsecShift: int64 = 30;

#[derive(Default,PartialEq,PartialOrd)]
pub struct Time {
    wall: uint64,
    ext: int64,
    loc: Box<Location>,
}

impl Time {
    // nsec returns the time's nanoseconds.
    fn nsec(&self) -> int32 {
        (self.wall as int64 & nsecMask) as int32
    }

    // sec returns the time's seconds since Jan 1 year 1.
    fn sec(&self) -> int64 {
        if self.wall as int64 & hasMonotonic != 0 {
            return wallToInternal + (self.wall << 1 >> (nsecShift + 1)) as int64;
        }
        self.ext
    }

    // unixSec returns the time's seconds since Jan 1 1970 (Unix time).
    fn unixSec(&self) -> int64 {
        return self.sec() + internalToUnix;
    }

    // addSec adds d seconds to the time.
    fn addSec(&mut self, d: int64) {
        if self.wall as int64 & hasMonotonic != 0 {
            let sec = self.wall << 1 >> (nsecShift + 1) as int64;
            let dsec = sec as int64 + d;
            if 0 <= dsec && dsec <= 1 << 33 - 1 {
                self.wall =
                    (self.wall as int64 & nsecMask | dsec << nsecShift | hasMonotonic) as uint64;
                return;
            }
            // Wall second now out of range for packed field.
            // Move to ext.
            self.stripMono()
        }

        // TODO: Check for overflow.
        self.ext += d
    }

    fn setLoc(&mut self, loc: Location) {
        /* if loc == &utcLoc {
            loc = nil
        } */
        self.stripMono();
        self.loc = Box::new(loc);
    }

    fn stripMono(&mut self) {
        if self.wall & hasMonotonic as uint64 != 0 {
            self.ext = self.sec();
            self.wall &= nsecMask as uint64;
        }
    }
}

#[derive(Default,PartialEq,PartialOrd)]
struct Location {
    name: string,
    zone: Vec<zone>,
    tx: Vec<zoneTrans>,

    // The tzdata information can be followed by a string that describes
    // how to handle DST transitions not recorded in zoneTrans.
    // The format is the TZ environment variable without a colon; see
    // https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap08.html.
    // Example string, for America/Los_Angeles: PST8PDT,M3.2.0,M11.1.0
    extend: string,

    // Most lookups will be for the current time.
    // To avoid the binary search through tx, keep a
    // static one-element cache that gives the correct
    // zone for the time when the Location was created.
    // if cacheStart <= t < cacheEnd,
    // lookup can return cacheZone.
    // The units for cacheStart and cacheEnd are seconds
    // since January 1, 1970 UTC, to match the argument
    // to lookup.
    cacheStart: int64,
    cacheEnd: int64,
    cacheZone: Box<zone>,
}

// A zone represents a single time zone such as CET.
 #[derive(Default,PartialEq,PartialOrd)]
struct zone {
    name: string, // abbreviated name, "CET"
    offset: int,  // seconds east of UTC
    isDST: bool,  // is this zone Daylight Savings Time?
}

// A zoneTrans represents a single time zone transition.
#[derive(Default,PartialEq,PartialOrd)]
struct zoneTrans {
    when: int64,  // transition time, in seconds since 1970 GMT
    index: uint8, // the index of the zone that goes into effect at that time
    isstd: bool,
    isutc: bool, // ignored - no idea what these mean
}


use std::time::Instant;

fn runtimeNano()->int64{
    let now = Instant::now;
    now.into()
}
fn now()->(int64,int32,int64){
   (secs,nanos,0)
}
pub fn Now()-> Time {
	let (sec, nsec, mono) = now() ;
	mono -= startNano
	sec += unixToInternal - minWall
	if uint64(sec)>>33 != 0 {
		return Time{uint64(nsec), sec + minWall, Local}
	}
	return Time{hasMonotonic | uint64(sec)<<nsecShift | uint64(nsec), mono, Local}
}
