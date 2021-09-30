//! Package time provides functionality for measuring and displaying time.
//! The calendrical calculations always assume a Gregorian calendar, with no leap seconds.
//!
//! <details class="rustdoc-toggle top-doc">
//! <summary class="docblock">zh-cn</summary>
//! time包提供了时间的显示和测量用的函数。日历的计算采用的是公历。
//! </details>
// #![allow(unused_assignments)]
#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#[macro_use]
use crate::builtin::*;
use gostd_derive::Fmt;
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

pub const Nanosecond: int64 = 1;
pub const Microsecond: int64 = 1000 * Nanosecond;
pub const Millisecond: int64 = 1000 * Microsecond;
pub const Second: int64 = 1000 * Millisecond;
pub const Minute: int64 = 60 * Second;
pub const Hour: int64 = 60 * Minute;
#[derive(Default, PartialEq, PartialOrd, Fmt)]
pub struct Duration(int64); // 由于类型别名不能绑定方法通过元组类型结构体实现,访问元组内容用d.0数字下标访问，go源码是 type Duration int64

const minDuration: int64 = int64!(-1) << 63;
const maxDuration: int64 = int64!((uint64!(1) << 63) - 1);

impl Duration {
    pub fn new(i: int64) -> Duration {
        Duration(i)
    }
    pub fn String(&self) -> string {
        let d = self.0;
        let mut buf: [byte; 32] = [0; 32];
        let mut w = buf.len();

        let mut u: uint64 = 0;
        let neg = d < 0;
        if neg {
            u = uint64!(-d);
        } else {
            u = uint64!(d);
        }

        if u < uint64!(Second) {
            let prec: int;
            w -= 1;
            buf[w] = byte!('s');
            w -= 1;
            if u == 0 {
                return "0s".to_string();
            } else if u < uint64!(Microsecond) {
                prec = 0;
                buf[w] = byte!('n');
            } else if u < uint64!(Millisecond) {
                prec = 3;
                w -= 1;
                let s = "µ";
                buf[w..].copy_from_slice(s.as_bytes());
            } else {
                prec = 6;
                buf[w] = byte!('m');
            }
            let (w2, u2) = fmtFrac(&mut buf[..w], uint64!(u), prec);
            u = uint64!(u2);
            w = fmtInt(&mut buf[..w2], u);
        } else {
            w -= 1;
            buf[w] = byte!('s');

            let (w3, u3) = fmtFrac(&mut buf[..w], u, 9);
            u = u3;
            w = fmtInt(&mut buf[..w3], u3 % 60);
            u /= 60;
            if u > 0 {
                w -= 1;
                buf[w] = byte!('m');
                w = fmtInt(&mut buf[..w], u % 60);
                u /= 60;

                if u > 0 {
                    w -= 1;
                    buf[w] = byte!('h');
                    w = fmtInt(&mut buf[..w], u);
                }
            }
        }
        if neg {
            w -= 1;
            buf[w] = byte!('_');
        }
        string(&buf[w..])
    }

    pub fn Nanoseconds(&self) -> int64 {
        self.0
    }

    pub fn Microseconds(&self) -> int64 {
        self.0 / 1000
    }

    pub fn Milliseconds(&self) -> int64 {
        self.0 / 1000_000
    }

    pub fn Seconds(&self) -> float64 {
        let d = self.0;
        let sec = d / Second;
        let nsec = d % Second;
        float64!(sec) + float64!(nsec) / 1e9
    }

    pub fn Minutes(&self) -> float64 {
        let d = self.0;
        let min = d / Minute;
        let nsec = d % Minute;
        float64!(min) + float64!(nsec) / (60.0 * 1e9)
    }

    pub fn Hours(&self) -> float64 {
        let d = self.0;
        let hour = d / Hour;
        let nsec = d % Hour;
        float64!(hour) + float64!(nsec) / (60.0 * 60.0 * 1e9)
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
            if lessThanHalf(r, m) {
                return Duration(d + r);
            }
            let d1 = d - m + r;
            if d1 < d {
                return Duration(d1);
            }
            return Duration(minDuration);
        }
        if lessThanHalf(r, m) {
            return Duration(d - r);
        }
        let d1 = d + m - r;
        if d1 > d {
            return Duration(d1);
        }
        Duration(maxDuration)
    }
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
    int64!(float64!(absoluteZeroYear - internalYear) * 365.2425 * float64!(secondsPerDay));
const internalToAbsolute: int64 = -absoluteToInternal;
const unixToInternal: int64 = (1969 * 365 + 1969 / 4 - 1969 / 100 + 1969 / 400) * secondsPerDay;
const internalToUnix: int64 = -unixToInternal;
const wallToInternal: int64 = (1884 * 365 + 1884 / 4 - 1884 / 100 + 1884 / 400) * secondsPerDay;
const internalToWall: int64 = -wallToInternal;

const hasMonotonic: int64 = 1 << 63;
const maxWall: int64 = wallToInternal + (1 << 33 - 1); // year 2157
const minWall: int64 = wallToInternal; // year 1885
const nsecMask: int32 = (1 << 30) - 1;
const nsecShift: int32 = 30;

#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct Time {
    wall: uint64,
    ext: int64,
    loc: Location,
}

impl Time {
    fn new() -> Time {
        Time::default()
    }
    // nsec returns the time's nanoseconds.
    fn nsec(&self) -> int32 {
        int32!(self.wall & uint64!(nsecMask))
    }

    // sec returns the time's seconds since Jan 1 year 1.
    fn sec(&self) -> int64 {
        if int64!(self.wall) & hasMonotonic != 0 {
            return wallToInternal + int64!(self.wall << 1 >> (nsecShift + 1));
        }
        self.ext
    }

    // unixSec returns the time's seconds since Jan 1 1970 (Unix time).
    fn unixSec(&self) -> int64 {
        return self.sec() + internalToUnix;
    }

    // addSec adds d seconds to the time.
    fn addSec(&mut self, d: int64) {
        if int64!(self.wall) & hasMonotonic != 0 {
            let sec = int64!(self.wall << 1 >> (nsecShift + 1));
            let dsec = sec + d;
            if 0 <= dsec && dsec <= (1 << 33) - 1 {
                self.wall = self.wall & uint64!(nsecMask)
                    | uint64!(dsec) << nsecShift
                    | uint64!(hasMonotonic);
                return;
            }
            // Wall second now out of range for packed field.
            // Move to ext.
            self.stripMono()
        }

        // Check if the sum of t.ext and d overflows and handle it properly.
        let mut sum = self.ext + d;
        if (sum > self.ext) == (d > 0) {
            self.ext = sum;
        } else if d > 0 {
            self.ext = int64::MAX; //int64!((uint64!(1) << 63) - 1);
        } else {
            self.ext = int64::MIN;
        }
    }

    fn setLoc(&mut self, mut loc: Location) {
        if loc.name == "UTC" {
            loc = Location::new();
        }
        self.stripMono();
        self.loc = loc;
    }

    fn stripMono(&mut self) {
        if self.wall & uint64!(hasMonotonic) != 0 {
            self.ext = self.sec();
            self.wall &= uint64!(nsecMask)
        }
    }

    fn setMono(&mut self, m: int64) {
        if self.wall & uint64!(hasMonotonic) == 0 {
            let sec = self.ext;
            if sec < minWall || maxWall < sec {
                return;
            }
            self.wall |= uint64!(hasMonotonic) | uint64!(sec - minWall) << nsecShift;
        }
        self.ext = m
    }

    fn mono(&self) -> int64 {
        if self.wall & uint64!(hasMonotonic) == 0 {
            return 0;
        }
        self.ext
    }

    pub fn After(&self, u: Time) -> bool {
        if self.wall & u.wall & uint64!(hasMonotonic) != 0 {
            return self.ext > u.ext;
        }
        let ts = self.sec();
        let us = u.sec();
        ts > us || ts == us && self.nsec() > u.nsec()
    }
    /// Before reports whether the time instant t is before u.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 如果t代表的时间点在u之前，返回真；否则返回假。
    /// </details>
    ///
    /// # Example
    ///
    /// ```rust
    /// use gostd::time;
    /// let first = time::Now();
    /// let second = time::Now();
    /// let is_befor = first.Before(&second);
    /// let not_befor = second.Before(&first);
    /// assert_eq!(true, is_befor);
    /// assert_eq!(false, not_befor);
    /// ```
    pub fn Before(&self, u: &Time) -> bool {
        if self.wall & u.wall & uint64!(hasMonotonic) != 0 {
            return self.ext < u.ext;
        }
        let ts = self.sec();
        let us = u.sec();
        ts < us || ts == us && self.nsec() < u.nsec()
    }

    pub fn Equal(&self, u: &Time) -> bool {
        if self.wall & u.wall & uint64!(hasMonotonic) != 0 {
            return self.ext == u.ext;
        }
        self.sec() == u.sec() && self.nsec() == u.nsec()
    }

    pub fn IsZero(&self) -> bool {
        self.sec() == 0 && self.nsec() == 0
    }

    fn abs(&self) -> uint64 {
        let mut l = self.loc.clone();
        if l.name.len() == 0 || l.name.as_str() == "Local" {
            l = l.get();
        }
        let mut sec = self.unixSec();
        if l.name.as_str() != "UTC" {
            if l.cacheZone.name.len() != 0 && l.cacheStart <= sec && sec < l.cacheEnd {
                sec += int64!(l.cacheZone.offset);
            } else {
                let (_, offset, _, _, _) = l.lookup(sec);
                sec += int64!(offset);
            }
        }
        uint64!(sec + (unixToInternal + internalToAbsolute))
    }
    /// 待完善
    fn locabs(&self) -> (string, int, uint64) {
        let sec = self.unixSec();
        let name = "UTC".to_string();
        let abs = uint64!(sec + (unixToInternal + internalToAbsolute));
        (name, 0, abs)
    }

    fn date(&self, full: bool) -> (int, Month, int, int) {
        absDate(self.abs(), full)
    }

    /// Add returns the time t+d.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Add返回时间点t+d。
    /// </details>
    ///
    /// # Example
    ///
    /// ```rust
    /// use gostd::time;
    /// use gostd::time::Duration;
    ///
    /// let mut start = time::Date(2009, 1, 1, 12, 0, 0, 0, time::UTC.clone());
    ///
    ///	let afterTenSeconds = start.Add(&Duration::new(time::Second * 10));
    ///	let afterTenMinutes = start.Add(&Duration::new(time::Minute * 10));
    ///	let afterTenHours = start.Add(&Duration::new(time::Hour * 10));
    ///	let afterTenDays = start.Add(&Duration::new(time::Hour * 24 * 10));

    ///	println!("start = {:?}\n", start);
    ///	println!("start.Add(time.Second * 10) = {:?}\n", afterTenSeconds);
    ///	println!("start.Add(time.Minute * 10) = {:?}\n", afterTenMinutes);
    ///	println!("start.Add(time.Hour * 10) = {:?}\n", afterTenHours);
    ///	println!("start.Add(time.Hour * 24 * 10) = {:?}\n", afterTenDays);
    ///
    /// ```
    pub fn Add(&mut self, d: &Duration) -> Time {
        let mut t = self;
        let mut dsec = d.0 / 1_000_000_000;
        let mut nsec = t.nsec() + int32!(d.0 % 1_000_000_000);
        if nsec >= 1_000_000_000 {
            dsec += 1;
            nsec -= 1_000_000_000;
        } else if nsec < 0 {
            dsec -= 1;
            nsec += 1_000_000_000;
        }
        t.wall = t.wall ^ (t.wall & uint64!(nsecMask)) | uint64!(nsec);
        t.addSec(dsec);
        if t.wall & uint64!(hasMonotonic) != 0 {
            let te = t.ext + d.0;
            if d.0 < 0 && te > t.ext || d.0 > 0 && te < t.ext {
                t.stripMono();
            } else {
                t.ext = te;
            }
        }
        let mut result = Time::new();
        result.wall = t.wall;
        result.ext = t.ext;
        result.loc = t.loc.clone();
        result
    }
    /// Sub returns the duration t-u. If the result exceeds the maximum (or minimum) value that can be stored in a Duration, the maximum (or minimum) duration will be returned. To compute t-d for a duration d, use t.Add(-d).
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 返回一个时间段t-u。如果结果超出了Duration可以表示的最大值/最小值，将返回最大值/最小值。要获取时间点t-d（d为Duration），可以使用t.Add(-d)。
    /// </details>
    ///
    /// # Example
    ///
    /// ```rust
    /// use gostd::time;
    /// let loc = time::UTC.clone();
    /// let mut start = time::Date(2000, 1, 1, 0, 0, 0, 0, loc.clone());
    /// let end = time::Date(2000, 1, 1, 12, 0, 0, 0, loc.clone());
    ///
    /// let difference = end.Sub(&mut start);
    /// println!("difference: {}",difference);
    /// assert_eq!(12_f64,difference.Hours());
    /// ```
    pub fn Sub(&self, u: &mut Time) -> Duration {
        if self.wall & u.wall & uint64!(hasMonotonic) != 0 {
            let te = self.ext;
            let ue = self.ext;
            let d = Duration(te - ue);
            if d.0 < 0 && te > ue {
                return Duration(maxDuration);
            }
            if d.0 > 0 && te < ue {
                return Duration(minDuration);
            }
            return d;
        }
        let d = Duration((self.sec() - u.sec()) * Second + int64!(self.nsec() - u.nsec()));
        if u.Add(&d).Equal(self) {
            d
        } else if self.Before(u) {
            Duration(minDuration)
        } else {
            Duration(maxDuration)
        }
    }

    pub fn Date(&self) -> (int, Month, int) {
        let (year, month, day, _) = self.date(true);
        (year, month, day)
    }

    pub fn Year(&self) -> int {
        let (year, _, _, _) = self.date(false);
        year
    }

    /// Month returns the month of the year specified by t.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 返回时间点t对应那一年的第几月。
    /// </details>
    ///
    /// #Example
    ///
    /// ```rust
    /// use gostd::time;
    /// let t = time::Date(2009, 11, 1, 1, 1, 1, 1, time::UTC.clone());
    ///
    /// assert_eq!(time::Month::November,t.Month());
    /// assert_eq!("November",t.Month().String());
    /// ```
    pub fn Month(&self) -> Month {
        let (_, month, _, _) = self.date(true);
        month
    }

    pub fn Clock(&self) -> (int, int, int) {
        absClock(self.abs())
    }

    /// Day returns the day of the month specified by t.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 返回时间点t对应那一月的第几日。
    /// </details>
    ///
    /// # Example
    ///
    /// ```rust
    /// use gostd::time;
    /// let t = time::Date(2009, 11, 1, 1, 1, 1, 1, time::UTC.clone());
    ///
    /// assert_eq!(1,t.Day());
    /// ```
    pub fn Day(&self) -> int {
        let (_, _, day, _) = self.date(true);
        day
    }

    /// Weekday returns the day of the week specified by t.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 返回时间点t对应的那一周的周几。
    /// </details>
    ///
    /// # Example
    ///
    /// ```rust
    /// use gostd::time;
    /// let t = time::Date(2009, 11, 1, 1, 1, 1, 1, time::UTC.clone());
    ///
    /// assert_eq!(time::Weekday::Sunday,t.Weekday());
    /// assert_eq!("Sunday",t.Weekday().String());
    /// ```
    pub fn Weekday(&self) -> Weekday {
        absWeekday(self.abs())
    }

    pub fn ISOWeek(&self) -> (int, int) {
        let mut abs = self.abs();
        let mut d = int!(Weekday::Thursday) - int!(absWeekday(abs));
        if d == 4 {
            d = -3;
        }
        abs += uint64!(d) * uint64!(secondsPerDay);
        let (year, _, _, yday) = absDate(abs, false);
        (year, yday / 7 + 1)
    }

    pub fn UTC(&mut self) -> Time {
        self.setLoc(utcLoc.clone());
        let mut t = Time::new();
        t.wall = self.wall;
        t.ext = self.ext;
        t.loc = self.loc.clone();
        t
    }

    pub fn Local(&mut self) -> Time {
        self.setLoc(Local.clone());
        let mut t = Time::new();
        t.wall = self.wall;
        t.ext = self.ext;
        t.loc = self.loc.clone();
        t
    }

    pub fn In(&mut self, loc: Location) -> Time {
        self.setLoc(loc);
        let mut t = Time::new();
        t.wall = self.wall;
        t.ext = self.ext;
        t.loc = self.loc.clone();
        t
    }

    pub fn Location(&self) -> Location {
        let mut l = self.loc.clone();
        if l.name.len() == 0 {
            l = utcLoc.clone();
        }
        l
    }

    pub fn Zone(&self) -> (string, int) {
        let (name, offset, _, _, _) = self.loc.lookup(self.unixSec());
        (name, offset)
    }

    pub fn Unix(&self) -> int64 {
        self.unixSec()
    }

    pub fn UnixMilli(&self) -> int64 {
        self.unixSec() * 1000 + int64!(self.nsec()) / 1000_000
    }

    pub fn UnixMicro(&self) -> int64 {
        self.unixSec() * 1000_000 + int64!(self.nsec()) / 1000
    }

    pub fn UnixNano(&self) -> int64 {
        self.unixSec() * 1000_000_000 + int64!(self.nsec())
    }

    pub fn AddDate(&self, years: int, months: int, days: int) -> Time {
        let (year, month, day) = self.Date();
        let (hour, min, sec) = self.Clock();
        return Date(
            year + years,
            uint!(months) + uint!(month),
            day + days,
            hour,
            min,
            sec,
            int!(self.nsec()),
            self.loc.clone(),
        );
    }

    pub fn IsDST(&self) -> bool {
        let (_, _, _, _, isDST) = self.loc.lookup(self.Unix());
        isDST
    }
}

pub fn Unix(sec: int64, nsec: int64) -> Time {
    let mut sec = sec;
    let mut nsec = nsec;
    if nsec < 0 || nsec >= 100_000_000 {
        let n = nsec / 1000_000_000;
        sec += n;
        nsec -= n * 1000_000_000;
        if nsec < 0 {
            nsec += 1000_000_000;
            sec -= 1;
        }
    }
    unixTime(sec, int32!(nsec))
}

pub fn UnixMilli(msec: int64) -> Time {
    Unix(msec / 1000, (msec % 1000) / 1000_000)
}

pub fn UnixMicro(usec: int64) -> Time {
    Unix(usec / 1000_000, (usec % 1000_000) * 1000)
}

/// # Example
///
/// ```rust
/// use gostd::time;
/// let d = time::Date(2000, 2, 1, 12, 30, 0, 0, time::UTC.clone());
/// let (year, month, day) = d.Date();
/// println!("year = {}",year);
/// println!("month = {}",month.String());
/// println!("day = {}",day);
/// ```
pub fn Date(
    year: int,
    month: uint,
    day: int,
    hour: int,
    min: int,
    sec: int,
    nsec: int,
    location: Location,
) -> Time {
    let loc = Some(location);
    if loc.is_none() {
        panic!("time: missing Location in call to Date")
    }

    let mut m = month - 1;
    let (year, m) = norm(year, int!(m), 12);
    let month = Month::indexOf(uint!(m + 1));

    let (sec, nsec) = norm(sec, nsec, 1000_000_000);
    let (min, sec) = norm(min, sec, 60);
    let (hour, min) = norm(hour, min, 60);
    let (day, hour) = norm(day, hour, 24);

    let mut d = daySinceEpoch(year);

    d += uint64!(daysBefore[uint!(month) - 1]);

    if isLeap(year) && month > Month::March {
        d += 1;
    }

    d += uint64!(day - 1);

    let mut abs = d * uint64!(secondsPerDay);
    abs += uint64!(hour) * uint64!(secondsPerHour)
        + uint64!(min) * uint64!(secondsPerMinute)
        + uint64!(sec);

    let mut unix = int64!(abs) + (absoluteToInternal + internalToUnix);
    let l = loc.unwrap();
    let (_, offset, start, end, _) = l.lookup(unix);
    if offset != 0 {
        let utc = unix - int64!(offset);

        if utc < start {
            let (_, offset, _, _, _) = l.lookup(start - 1);
        }

        if utc >= end {
            let (_, offset, _, _, _) = l.lookup(end);
        }
        unix -= int64!(offset);
    }
    let mut t = unixTime(unix, int32!(nsec));
    t.loc = l.clone();
    t
}

fn absClock(abs: uint64) -> (int, int, int) {
    let mut sec = int!(abs % uint64!(secondsPerDay));
    let hour = int!(sec) / int!(secondsPerHour);
    sec -= hour * int!(secondsPerHour);
    let min = sec / int!(secondsPerMinute);
    sec -= min * int!(secondsPerMinute);
    (hour, min, sec)
}

fn absWeekday(abs: uint64) -> Weekday {
    let sec = (int64!(abs) + int64!(Weekday::Monday) * secondsPerDay) % secondsPerWeek;
    Weekday::indexOf(uint!(sec / secondsPerDay))
}

#[derive(Default, PartialEq, PartialOrd, Clone, Debug, Fmt)]
pub struct Location {
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

impl Location {
    fn new() -> Location {
        Location::default()
    }

    fn get(&self) -> Location {
        if self.name.len() == 0 {
            return utcLoc.clone();
        }

        if self.name.as_str() == "Local" {
            return UTC.clone(); //待完善
        }
        self.clone()
    }

    pub fn String(&self) -> string {
        self.get().name
    }

    fn lookup(&self, sec: int64) -> (string, int, int64, int64, bool) {
        let l = self.get();

        let mut name: string;
        let mut offset: int;
        let mut start: int64;
        let mut end: int64;
        let mut isDST: bool;

        if l.zone.len() == 0 {
            name = "UTC".to_string();
            offset = 0;
            start = alpha;
            end = omega;
            isDST = false;
            return (name, offset, start, end, isDST);
        }
        let zone = l.cacheZone.clone();
        if l.cacheStart <= sec && sec < l.cacheEnd {
            name = zone.name;
            offset = zone.offset;
            start = l.cacheStart;
            end = l.cacheEnd;
            isDST = zone.isDST;
            return (name, offset, start, end, isDST);
        }
        if l.tx.len() == 0 || sec < l.tx[0].when {
            let mut zone = l.zone[(l.lookupFirstZone())].clone();
            name = zone.name;
            offset = zone.offset;
            start = alpha;
            if l.tx.len() > 0 {
                end = l.tx[0].when;
            } else {
                end = omega;
            }
            isDST = zone.isDST;
            return (name, offset, start, end, isDST);
        }

        let tx = l.tx;
        end = omega;
        let mut lo = 0;
        let mut hi = tx.len();
        while (hi - lo > 1) {
            let m = lo + (hi - lo) / 2;
            let lim = tx[m].when;
            if sec < lim {
                end = lim;
                hi = m;
            } else {
                lo = m;
            }
        }
        let zone = l.zone[tx[lo].index as uint].clone();
        name = zone.name;
        offset = zone.offset;
        start = tx[lo].when;
        isDST = zone.isDST;

        //        if lo == tx.len() - 1 && l.extend != "" {
        // let (ename, eoffset, estart, eend, eisDST, ok) = tzset(l.extend, end, sec);
        // if ok {
        //     return (ename, eoffset, estart, eend, eisDST);
        // }
        // }

        (name, offset, start, end, isDST)
    }

    fn lookupFirstZone(&self) -> uint {
        if !self.firstZoneUsed() {
            return 0;
        }
        let index = uint!(self.tx[0].index);
        if self.tx.len() > 0 && self.zone[index].isDST {
            let mut zi = index - 1;
            while int32!(zi) >= 0 {
                zi -= 1;
                if self.zone[zi].isDST {
                    return zi;
                }
            }
        }

        for (zi, v) in self.zone.iter().enumerate() {
            if !v.isDST {
                return zi;
            }
        }
        0
    }

    fn firstZoneUsed(&self) -> bool {
        for v in self.tx.iter() {
            if v.index == 0 {
                return true;
            }
        }
        false
    }
}

// A zone represents a single time zone such as CET.
#[derive(Default, PartialEq, PartialOrd, Clone, Debug)]
struct zone {
    name: string, // abbreviated name, "CET"
    offset: int,  // seconds east of UTC
    isDST: bool,  // is this zone Daylight Savings Time?
}

// A zoneTrans represents a single time zone transition.
#[derive(Default, PartialEq, PartialOrd, Clone, Copy, Debug)]
struct zoneTrans {
    when: int64,  // transition time, in seconds since 1970 GMT
    index: uint8, // the index of the zone that goes into effect at that time
    isstd: bool,
    isutc: bool, // ignored - no idea what these mean
}

const alpha: int64 = (-1) << 63; // math.MinInt64
const omega: int64 = int64!((uint64!(1) << 63) - 1); // math.MaxInt64

pub fn FixedZone(name: string, offset: int) -> Location {
    let zo = vec![zone {
        name: name.to_string(),
        offset: offset,
        isDST: false,
    }];
    let loc = Location {
        name: name.to_string(),
        zone: zo.clone(),
        tx: vec![zoneTrans {
            when: alpha,
            index: 0,
            isstd: false,
            isutc: false,
        }],
        cacheStart: alpha,
        cacheEnd: omega,
        cacheZone: zo.get(0).unwrap().clone(),
        extend: "".to_string(),
    };
    loc
}
mod sys;
use lazy_static;
lazy_static::lazy_static! {
   pub  static ref startNano:int64 =runtimeNano() - 1;
   pub  static ref Local:Location = Location::new();
   pub  static ref utcLoc:Location = {
    let mut l = Location::new();
    l.name="UTC".to_string();
     l
    };
    pub static ref UTC:Location =utcLoc.clone() ;
}

fn runtimeNano() -> int64 {
    sys::monotonic_now() as int64
}

fn now() -> (int64, int32, int64) {
    let (sec, nsec) = sys::real_time_now();
    let mono = sys::monotonic_now();
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
            loc: Local.clone(),
        };
    }
    Time {
        wall: (hasMonotonic as int64
            | (uint64!(sec) << uint64!(nsecShift)) as int64
            | nsec as int64) as uint64,
        ext: mono,
        loc: Local.clone(),
    }
}

fn unixTime(sec: int64, nsec: int32) -> Time {
    let mut t = Time::new();
    t.wall = uint64!(nsec);
    t.ext = sec + unixToInternal;
    t
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
        if day > 31 + 29 - 1 {
            day = day - 1;
        } else if day == 31 + 29 - 1 {
            month = Month::February;
            day = 29;
            return (year, month, day, yday);
        }
    }

    let mut m = uint!(day / 31);
    let end = int!(daysBefore[m + 1]);
    let begin: int;
    if day >= end {
        m += 1;
        begin = end;
    } else {
        begin = int!(daysBefore[m]);
    }

    m += 1; // because January is 1
    day = day - begin + 1;
    month = Month::indexOf(m);
    (year, month, day, yday)
}

const daysBefore: [int32; 13] = [
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

fn norm(mut hi: int, mut lo: int, mut base: int) -> (int, int) {
    if lo < 0 {
        let n = (-lo - 1) / base + 1;
        hi -= n;
        lo += n * base;
    }

    if lo >= base {
        let n = lo / base;
        hi += n;
        lo -= n * base;
    }
    (hi, lo)
}

fn daysIn(m: Month, year: int) -> int {
    if m == Month::February && isLeap(year) {
        return 29;
    }
    let index = uint!(m);
    int!(daysBefore[index] - daysBefore[index - 1])
}

fn daySinceEpoch(year: int) -> uint64 {
    let mut y = int64!(year) - absoluteZeroYear;

    let mut n = y / 400;
    y -= 400 * n;
    let mut d = daysPer400Years * n;

    n = y / 100;
    y -= 100 * n;
    d += daysPer100Years * n;

    n = y / 4;
    y -= 4 * n;
    d += daysPer4Years * n;

    n = y;
    d += 365 * n;
    uint64!(d)
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

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug, Fmt)]
pub enum Month {
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
    pub fn indexOf(i: uint) -> Month {
        match i {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => Month::December,
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug, Fmt)]
pub enum Weekday {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

impl Weekday {
    pub fn String(&self) -> string {
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
    pub fn indexOf(i: uint) -> Weekday {
        match i {
            1 => Weekday::Monday,
            2 => Weekday::Tuesday,
            3 => Weekday::Wednesday,
            4 => Weekday::Thursday,
            5 => Weekday::Friday,
            6 => Weekday::Saturday,
            7 => Weekday::Sunday,
            _ => Weekday::Sunday,
        }
    }
}

// 函数
pub fn Since(mut t: Time) -> Duration {
    let mut now = Time::new();
    if t.wall & uint64!(hasMonotonic) != 0 {
        now.wall = uint64!(hasMonotonic);
        now.ext = runtimeNano() - *startNano;
    } else {
        now = Now();
    }
    now.Sub(&mut t)
}

pub fn Until(t: Time) -> Duration {
    let mut now = Time::new();
    if t.wall & uint64!(hasMonotonic) != 0 {
        now.wall = uint64!(hasMonotonic);
        now.ext = runtimeNano() - *startNano;
    } else {
        now = Now();
    }
    t.Sub(&mut now)
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
            buf[w] = byte!(digit) + byte!('0');
        }
        v /= 10;
    }
    if print {
        w -= 1;
        buf[w] = byte!('.');
    }
    (w, v)
}

fn fmtInt(buf: &mut [byte], v: uint64) -> uint {
    let mut v = v;
    let mut w = buf.len();
    if v == 0 {
        w -= 1;
        buf[w] = byte!('0');
    } else {
        while v > 0 {
            w -= 1;
            buf[w] = byte!(v % 10) + byte!('0');
            v /= 10;
        }
    }
    w
}

fn lessThanHalf(x: int64, y: int64) -> bool {
    x + x < y
}

fn div(t: Time, d: Duration) -> (int, Duration) {
    let mut qmod2: int = 0;
    let mut r: Duration;
    let mut neg = false;
    let mut nsec = t.nsec();
    let mut sec = t.sec();

    if sec < 0 {
        neg = true;
        sec = -sec;
        nsec = -nsec;
        if nsec < 0 {
            nsec += 1_000_000_000;
            sec -= 1;
        }
    }

    if d.0 < Second && Second % (d.0 + d.0) == 0 {
        qmod2 = int!(nsec / (int32!(d.0))) & 1;
        r = Duration(int64!(nsec % int32!(d.0)));
        return (qmod2, r);
    } else if d.0 % Second == 0 {
        let d1 = int64!(d.0 / Second);
        qmod2 = int!((sec / d1) & 1);
        r = Duration((sec % d1) * Second + int64!(nsec));
        return (qmod2, r);
    } else {
        let sec = uint64!(sec);
        let mut tmp = (sec >> 32) * 1000_0000_000;
        let mut u1 = tmp >> 32;
        let u0 = tmp << 32;
        tmp = (sec & 0xFFFFFFFF) * 1000_0000_000;

        let mut u0x = u0;
        let mut u0 = u0 + tmp;

        if u0 < u0x {
            u1 += 1;
        }

        u0x = u0;
        u0 = uint64!(nsec);
        if u0 < u0x {
            u1 += 1;
        }

        let mut d1 = uint64!(d.0);
        while ((d1 >> 63) != 1) {
            d1 <<= 1
        }
        let mut d0 = uint64!(0);

        loop {
            if u1 > d1 || u1 == d1 && u0 >= d0 {
                qmod2 = 1;
                u0x = u0;
                u0 = u0 - d0;
                if u0 > u0x {
                    u1 -= 1;
                }
                u1 -= d1;
            }

            if d1 == 0 && d0 == uint64!(0) {
                break;
            }
            d0 >>= 1;
            d0 |= (d1 & 1) << 63;
            d1 >>= 1
        }

        r = Duration(int64!(u0));
        return (qmod2, r);
    }

    if neg && r.0 != 0 {
        qmod2 ^= 1;
        r = Duration(d.0 - r.0)
    }
    (qmod2, r)
}

// format.go -start

// private fn

// match报告s1和s2是否匹配，忽略大小写。
// 假设s1和s2的长度相同。
// 注意：因为match是rust的保留字，这里改成isMatch。
fn isMatch(s1: &str, s2: &str) -> bool {
    //检查长度一致
    if s1.len() != s2.len() {
        return false;
    }
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();

    for i in 0..s1.len() {
        let mut c1 = s1[i];
        let mut c2 = s2[i];
        if c1 != c2 {
            c1 |= byte!('a') - byte!('A');
            c2 |= byte!('a') - byte!('A');
            if c1 != c2 || c1 < byte!('a') || c1 > byte!('z') {
                return false;
            }
        }
    }
    true
}

const errBad: &'static str = "bad value for field";

fn lookup(tab: Vec<string>, val: string) -> Result<(int, string), &'static str> {
    for (i, v) in tab.iter().enumerate() {
        if val.len() >= v.len() && isMatch(&val[0..v.len()], v) {
            let index = uint!(v.len());
            let vstr = v.as_str();
            return Ok((int!(i), vstr[index..].to_string()));
        }
    }

    Err(errBad) // go源码:return -1, val, errBad
}

fn appendInt(b: Vec<byte>, x: int, width: int) -> Vec<byte> {
    let mut b = b;
    let width = uint!(width);
    let mut u = uint!(x);

    if x < 0 {
        b.push(byte!('-'));
        u = uint!(-x);
    }

    let mut buf: [byte; 20] = [0; 20];
    let mut i = buf.len();
    while (u >= 10) {
        i -= 1;
        let q = u / 10;
        buf[i] = byte!(uint!(byte!('0')) + u - q * 10);
        u = q;
    }
    i -= 1;
    buf[i] = byte!(uint!('0') + u);

    let mut w = buf.len() - i;
    for _ in w..width {
        b.push(byte!('0'));
    }

    // b.append(buf[i..])
    b.extend_from_slice(&buf[i..]);
    b
}

// format.go -end
