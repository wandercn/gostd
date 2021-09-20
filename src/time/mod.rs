// #![allow(unused_assignments)]
#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#[macro_use]
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

#[derive(Default, PartialEq, PartialOrd)]
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

use std::cell::RefCell;
use std::rc::Rc;
#[derive(Default, PartialEq, PartialOrd)]
pub struct Time {
    wall: uint64,
    ext: int64,
    loc: Rc<RefCell<Location>>,
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

    fn setLoc(&mut self, mut loc: Location) {
        if loc.name == "UTC" {
            loc = Location::default();
        }
        self.stripMono();
        self.loc = Rc::new(RefCell::new(loc));
    }

    fn stripMono(&mut self) {
        if self.wall & hasMonotonic as uint64 != 0 {
            self.ext = self.sec();
            self.wall &= nsecMask as uint64;
        }
    }

    fn setMono(&mut self, m: int64) {
        if self.wall as int64 & hasMonotonic == 0 {
            let sec = self.ext;
            if sec < minWall || maxWall < sec {
                return;
            }
            self.wall |=
                uint64!(hasMonotonic) | (((sec - minWall) as int64) << int64!(nsecShift)) as uint64;
        }
        self.ext = m
    }

    fn mono(&self) -> int64 {
        if self.wall as int64 & hasMonotonic == 0 {
            return 0;
        }
        self.ext
    }

    pub fn After(&self, u: Time) -> bool {
        if self.wall as int64 & u.wall as int64 & hasMonotonic != 0 {
            return self.ext > u.ext;
        }
        let ts = self.sec();
        let us = u.sec();
        ts > us || ts == us && self.nsec() > u.nsec()
    }

    pub fn Before(&self, u: Time) -> bool {
        if self.wall as int64 & u.wall as int64 & hasMonotonic != 0 {
            return self.ext < u.ext;
        }
        let ts = self.sec();
        let us = u.sec();
        ts < us || ts == us && self.nsec() < u.nsec()
    }

    pub fn Equal(&self, u: Time) -> bool {
        if self.wall as int64 & u.wall as int64 & hasMonotonic != 0 {
            return self.ext == u.ext;
        }
        self.sec() == u.sec() && self.nsec() == u.nsec()
    }

    pub fn IsZero(&self) -> bool {
        self.sec() == 0 && self.nsec() == 0
    }

    /// 待完善
    fn abs(&self) -> uint64 {
        // let mut l = self.loc;
        // if l.take().name == Local.name {
        // l = l.get();
        // }
        let mut sec = self.unixSec();
        if self.loc.borrow().name != utcLoc.name {}
        (sec + (unixToInternal + internalToAbsolute)) as uint64
    }

    fn date(&self, full:bool )->(int ,Month,int,int){
        absDate(self.abs(),full)
    }

    fn Add(&self,d :Duration)->Time{
        let t=self;
        let dsec =d.0/1e9 as int64;
        let nsec =t.nsec() + d%1e9;
        if nsec >=1e9{
            dsec+=1;
            nsec -=1e9;
        }else if nsec<0{
            dsec-=1;
            nsec +=1e9;
        }
        t.wall = t.wall&^nsecMask |nsec;
        t.addSec(dsec);
        if t.wall & hasMonotonic !=0{
            let te = t.ext +d;
            if d<0 && te>t.ext || d>0 && te<t.ext{
                t.stripMono();
            }else{
                t.ext=te;
            }
        }
        t
    }
}

#[derive(Default, PartialEq, PartialOrd, Clone)]
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
    cacheZone: zone,
}

impl Location {}

// A zone represents a single time zone such as CET.
#[derive(Default, PartialEq, PartialOrd, Clone)]
struct zone {
    name: string, // abbreviated name, "CET"
    offset: int,  // seconds east of UTC
    isDST: bool,  // is this zone Daylight Savings Time?
}

// A zoneTrans represents a single time zone transition.
#[derive(Default, PartialEq, PartialOrd, Clone, Copy)]
struct zoneTrans {
    when: int64,  // transition time, in seconds since 1970 GMT
    index: uint8, // the index of the zone that goes into effect at that time
    isstd: bool,
    isutc: bool, // ignored - no idea what these mean
}

mod unix;
use lazy_static;
lazy_static::lazy_static! {
    static ref startNano:int64 =runtimeNano() - 1;
    static ref Local:Location = Location::default();
    static ref utcLoc:Location = {
    let mut l = Location::default();
    l.name="UTC".to_string();
     l
    };
}
fn runtimeNano() -> int64 {
    unix::monotonic_now() as int64
}
fn now() -> (int64, int32, int64) {
    let (sec, nsec) = unix::real_time_now();
    let mono = unix::monotonic_now();
    (int64!(sec), int32!(nsec), int64!(mono))
}

pub fn Now() -> Time {
    let (mut sec, mut nsec, mut mono) = now();
    mono -= *startNano;
    sec += unixToInternal - minWall;
    if sec >> 33 != 0 {
        return Time {
            wall: uint64!(nsec),
            ext: sec + minWall,
            loc: Rc::new(RefCell::new(Local.clone())),
        };
    }
    Time {
        wall: (hasMonotonic as int64
            | (uint64!(sec) << uint64!(nsecShift)) as int64
            | nsec as int64) as uint64,
        ext: mono,
        loc: Rc::new(RefCell::new(Local.clone())),
    }
}

fn absDate(abs: uint64, full: bool) -> (int, Month, int, int) {
    let mut year: int = 0;
    let mut month = Month::January;
    let mut day: int = 0;
    let mut yday: int = 0;
    // Split into time and day.
    let mut d = abs as int64 / secondsPerDay;

    // Account for 400 year cycles.
    let mut n = d / daysPer400Years;
    let mut y = 400 * n;
    d -= daysPer400Years * n;

    // Cut off 100-year cycles.
    // The last cycle has one extra leap year, so on the last day
    // of that year, day / daysPer100Years will be 4 instead of 3.
    // Cut it back down to 3 by subtracting n>>2.
    n = d / daysPer100Years;
    n -= n >> 2;
    y += 100 * n;
    d -= daysPer100Years * n;

    // Cut off 4-year cycles.
    // The last cycle has a missing leap year, which does not
    // affect the computation.
    n = d / daysPer4Years;
    y += 4 * n;
    d -= daysPer4Years * n;

    // Cut off years within a 4-year cycle.
    // The last year is a leap year, so on the last day of that year,
    // day / 365 will be 4 instead of 3. Cut it back down to 3
    // by subtracting n>>2.
    n = d / 365;
    n -= n >> 2;
    y += n;
    d -= 365 * n;

    year = (y + absoluteZeroYear) as int;
    yday = int!(d);

    if !full {
        return (year, month, day, yday);
    }

    day = yday;
    if isLeap(year) {
        if  day > 31+29-1{
            day=day -1;
        }else if day == 31+29-1 {
            month = Month::February;
            day = 29;

        }
    }

    let mut  m = day/31;
    let end = daysBefore[m as isize+1] as int;
    let  begin: int;
    if day >= end {
         m+=1;
        begin = end;
    } else {
        begin = daysBefore[m] as int;
    }

    day = day - begin + 1;
         (year, m, day, yday)
}

const daysBefore:[int32;13] = [
	0,
	31,
	31 + 28,
	31 + 28 + 31,
	31 + 28 + 31 + 30,
	31 + 28 + 31 + 30 + 31,
	31 + 28 + 31 + 30 + 31 + 30,
	31 + 28 + 31 + 30 + 31 + 30 + 31,
	31 + 28 + 31 + 30 + 31 + 30 + 31 + 31,
	31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30,
	31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31,
	31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30,
	31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30 + 31,
];

fn isLeap(year: int) -> bool {
     year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

const longDayNames: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

const shortDayNames: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

const shortMonthNames: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

const longMonthNames: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl Month {
    pub fn String(&self) -> string {
        let m = *self;
        if Month::January <= m && m <= Month::December {
            return longMonthNames[m as usize - 1].to_string();
        }
        let mut buf: [byte; 20] = [0; 20];
        let n = fmtInt(&mut buf[..], m as uint64);
        let mut mon = String::from("%!Month(");
        let s = string::from_utf8(buf[n..].to_vec()).unwrap();
        mon.push_str(s.as_str());
        mon.push(')');
        mon
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum Weekday {
    Sunday = 1,
    Monday = 2,
    Tuesday = 3,
    Wednesday = 4,
    Thursday = 5,
    Friday = 6,
    Saturday,
}

impl Weekday {
    fn String(&self) -> string {
        let d = *self;
        if Weekday::Sunday <= d && d <= Weekday::Saturday {
            return longDayNames[d as usize].to_string();
        }
        let mut buf: [byte; 20] = [0; 20];
        let n = fmtInt(&mut buf[..], d as uint64);
        let mut mon = String::from("%!Weekday(");
        let s = string::from_utf8(buf[n..].to_vec()).unwrap();
        mon.push_str(s.as_str());
        mon.push(')');
        mon
    }
}
