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
/// Time Format - Layout
pub const Layout: &str = "01/02 03:04:05PM '06 -0700"; // The reference time, in numerical order.
/// Time Format - ANSIC
pub const ANSIC: &str = "Mon Jan _2 15:04:05 2006";
/// Time Format - UnixDate
pub const UnixDate: &str = "Mon Jan _2 15:04:05 MST 2006";
/// Time Format - RubyDate
pub const RubyDate: &str = "Mon Jan 02 15:04:05 -0700 2006";
/// Time Format - RFC822
pub const RFC822: &str = "02 Jan 06 15:04 MST";
/// Time Format - RFC822Z
pub const RFC822Z: &str = "02 Jan 06 15:04 -0700"; // RFC822 with numeric zone
/// Time Format - RFC850
pub const RFC850: &str = "Monday, 02-Jan-06 15:04:05 MST";
/// Time Format - RFC1123
pub const RFC1123: &str = "Mon, 02 Jan 2006 15:04:05 MST";
/// Time Format - RFC1123Z
pub const RFC1123Z: &str = "Mon, 02 Jan 2006 15:04:05 -0700"; // RFC1123 with numeric zone
/// Time Format - RFC3339
pub const RFC3339: &str = "2006-01-02T15:04:05Z07:00";
/// Time Format - RFC3339Nano
pub const RFC3339Nano: &str = "2006-01-02T15:04:05.999999999Z07:00";
/// Time Format - Kitchen
pub const Kitchen: &str = "3:04PM";
/// Handy time stamps.
pub const Stamp: &str = "Jan _2 15:04:05";
/// Handy time StampMilli.
pub const StampMilli: &str = "Jan _2 15:04:05.000";
/// Handy time StampMicro.
pub const StampMicro: &str = "Jan _2 15:04:05.000000";
/// Handy time StampNano.
pub const StampNano: &str = "Jan _2 15:04:05.000000000";
/// Time Unit Nanosecond
pub const Nanosecond: int64 = 1;
/// Time Unit Microsecond
pub const Microsecond: int64 = 1000 * Nanosecond;
/// Time Unit Millisecond
pub const Millisecond: int64 = 1000 * Microsecond;
/// Time Unit Second
pub const Second: int64 = 1000 * Millisecond;
/// Time Unit Minute
pub const Minute: int64 = 60 * Second;
/// Time Unit Hour
pub const Hour: int64 = 60 * Minute;

/// A Duration represents the elapsed time between two instants as an int64 nanosecond count. The representation limits the largest representable duration to approximately 290 years.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Duration类型代表两个时间点之间经过的时间，以纳秒为单位。可表示的最长时间段大约290年。
/// </details>
#[derive(PartialEq, PartialOrd, Debug, Copy, Clone, Fmt)]
pub struct Duration(int64); // 由于类型别名不能绑定方法通过元组类型结构体实现,访问元组内容用d.0数字下标访问，go源码是 type Duration int64

const minDuration: int64 = int64!(-1) << 63;
const maxDuration: int64 = int64!((uint64!(1) << 63) - 1);

impl Duration {
    /// Specifies the int64 number i to create for a period of time.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 指定int64 数字i 创建一段时间。
    /// </details>
    pub fn new(i: int64) -> Duration {
        Duration(i)
    }
    /// String returns a string representing the duration in the form "72h3m0.5s". Leading zero units are omitted. As a special case, durations less than one second format use a smaller unit (milli-, micro-, or nanoseconds) to ensure that the leading digit is non-zero. The zero duration formats as 0s.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 返回时间段采用"72h3m0.5s"格式的字符串表示。最前面可以有符号，数字+单位为一个单元，开始部分的0值单元会被省略；如果时间段<1s，会使用"ms"、"us"、"ns"来保证第一个单元的数字不是0；如果时间段为0，会返回"0"
    /// </details>
    ///
    /// # Example
    /// ```
    /// use gostd::time;
    ///
    /// let t0 = time::Duration::new(1 * time::Hour + 2 * time::Minute + 300 * time::Millisecond);
    /// let t1 = time::Duration::new(300 * time::Millisecond);
    /// println!("{}", t0);
    /// println!("{}", t1);
    /// assert_eq!(t0.String().as_str(), "1h2m0.3s");
    /// assert_eq!(t1.String().as_str(), "300ms");
    ///
    /// ```
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
                buf[w..(w + 2)].copy_from_slice(s.as_bytes()); // go中slice copy()不要求dst必须与source长度一致，转换成rust后，copy_from_slice函数要求dst和source长度必须一致导致bug，所以修正为w.. w+2
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

    /// Nanoseconds returns the duration as an integer nanosecond count.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Nanoseconds以整数纳秒计数的形式返回持续时间。
    /// </details>
    ///
    /// # Example
    /// ```rust
    /// use gostd::time;
    ///
    /// let u = time::ParseDuration("1µs").ok().unwrap();
    /// assert_eq!(u.Nanoseconds(),1000);
    /// println!("One microsecond is {} nanoseconds.", u.Nanoseconds())
    /// ```
    pub fn Nanoseconds(&self) -> int64 {
        self.0
    }

    /// Microseconds returns the duration as an integer microsecond count.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Microseconds 以整数微秒计数的形式返回持续时间。
    /// </details>
    ///
    /// # Example
    /// ```
    /// use gostd::time;
    ///
    /// let u = time::ParseDuration("1s").ok().unwrap();
    /// assert_eq!(u.Microseconds(),1000000);
    /// println!("One second is {} microseconds.", u.Microseconds())
    /// ```
    pub fn Microseconds(&self) -> int64 {
        self.0 / 1000
    }

    /// Milliseconds returns the duration as an integer millisecond count.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Milliseconds以整数毫秒的形式返回持续时间。
    /// </details>
    ///
    /// # Example
    /// ```
    /// use gostd::time;
    ///
    /// let u = time::ParseDuration("1s").ok().unwrap();
    /// assert_eq!(u.Milliseconds(),1000);
    /// println!("One second is {} milliseconds.", u.Milliseconds())
    /// ```
    pub fn Milliseconds(&self) -> int64 {
        self.0 / 1000_000
    }

    /// Seconds returns the duration as a floating point number of seconds.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Seconds返回持续时间的浮点数为秒。
    /// </details>
    ///
    /// # Example
    /// ```
    /// use gostd::time;
    ///
    /// let m = time::ParseDuration("1m30s").ok().unwrap();
    /// assert_eq!(m.Seconds(),90.0);
    /// println!("Take off in {} seconds.", m.Seconds())
    /// ```
    pub fn Seconds(&self) -> float64 {
        let d = self.0;
        let sec = d / Second;
        let nsec = d % Second;
        float64!(sec) + float64!(nsec) / 1e9
    }

    /// Minutes returns the duration as a floating point number of minutes.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Minutes以分钟的浮点数返回持续时间。
    /// </details>
    ///
    /// # Example
    /// ```
    /// use gostd::time;
    ///
    /// let m = time::ParseDuration("1h30m").ok().unwrap();
    /// assert_eq!(m.Minutes(),90.0);
    /// println!("The movie is {} minutes long.", m.Minutes())
    /// ```
    pub fn Minutes(&self) -> float64 {
        let d = self.0;
        let min = d / Minute;
        let nsec = d % Minute;
        float64!(min) + float64!(nsec) / (60.0 * 1e9)
    }

    /// Hours returns the duration as a floating point number of hours.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// hours以浮点数的形式返回持续时间。
    /// </details>
    ///
    /// # Example
    /// ```
    /// use gostd::time;
    ///
    /// let h = time::ParseDuration("4h30m").ok().unwrap();
    /// assert_eq!(h.Hours(),4.5);
    /// println!("I've got {} hours of work left.",h.Hours());
    /// ```
    pub fn Hours(&self) -> float64 {
        let d = self.0;
        let hour = d / Hour;
        let nsec = d % Hour;
        float64!(hour) + float64!(nsec) / (60.0 * 60.0 * 1e9)
    }

    /// Truncate returns the result of rounding d toward zero to a multiple of m. If m <= 0, Truncate returns d unchanged.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Truncate返回d向0舍入到m的倍数的结果。如果m<=0，Truncate返回d不变。
    /// </details>
    pub fn Truncate(&self, m: Duration) -> Duration {
        let d = self.0;
        let m = m.0;
        if m <= 0 {
            return Duration::new(d);
        }
        Duration::new(d - d % m)
    }

    /// Round returns the result of rounding d to the nearest multiple of m. The rounding behavior for halfway values is to round away from zero. If the result exceeds the maximum (or minimum) value that can be stored in a Duration, Round returns the maximum (or minimum) duration. If m <= 0, Round returns d unchanged.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Round返回将d四舍五入到最接近的m的倍数的结果，对于半数的值，四舍五入的行为是从零开始。如果结果超过了可以存储在持续时间中的最大（或最小）值，Round返回最大（或最小）持续时间。如果m <= 0，Round返回d，不作任何改变。
    /// </details>
    pub fn Round(&self, m: Duration) -> Duration {
        let d = self.0;
        let m = m.0;
        if m <= 0 {
            return Duration::new(d);
        }
        let mut r = d % m;

        if d < 0 {
            r = -r;
            if lessThanHalf(r, m) {
                return Duration::new(d + r);
            }
            let d1 = d - m + r;
            if d1 < d {
                return Duration::new(d1);
            }
            return Duration::new(minDuration);
        }
        if lessThanHalf(r, m) {
            return Duration::new(d - r);
        }
        let d1 = d + m - r;
        if d1 > d {
            return Duration::new(d1);
        }
        Duration::new(maxDuration)
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
const absoluteToInternal: int64 = int64!(
    (int64!(absoluteZeroYear - internalYear) * int64!(365)
        + int64!(float64!(absoluteZeroYear - internalYear) * float64!(0.2425)))
        * int64!(secondsPerDay)
);
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
/// A Time represents an instant in time with nanosecond precision.
/// Programs using times should typically store and pass them as values, not pointers. That is, time variables and struct fields should be of type time.Time, not *time.Time.
///
/// A Time value can be used by multiple goroutines simultaneously except that the methods GobDecode, UnmarshalBinary, UnmarshalJSON and UnmarshalText are not concurrency-safe.
///
/// Time instants can be compared using the Before, After, and Equal methods. The Sub method subtracts two instants, producing a Duration. The Add method adds a Time and a Duration, producing a Time.
///
/// The zero value of type Time is January 1, year 1, 00:00:00.000000000 UTC. As this time is unlikely to come up in practice, the IsZero method gives a simple way of detecting a time that has not been initialized explicitly.
///
/// Each Time has associated with it a Location, consulted when computing the presentation form of the time, such as in the Format, Hour, and Year methods. The methods Local, UTC, and In return a Time with a specific location. Changing the location in this way changes only the presentation; it does not change the instant in time being denoted and therefore does not affect the computations described in earlier paragraphs.
///
/// Representations of a Time value saved by the GobEncode, MarshalBinary, MarshalJSON, and MarshalText methods store the Time.Location's offset, but not the location name. They therefore lose information about Daylight Saving Time.
///
/// In addition to the required “wall clock” reading, a Time may contain an optional reading of the current process's monotonic clock, to provide additional precision for comparison or subtraction. See the “Monotonic Clocks” section in the package documentation for details.
///
/// Note that the Go == operator compares not just the time instant but also the Location and the monotonic clock reading. Therefore, Time values should not be used as map or database keys without first guaranteeing that the identical Location has been set for all values, which can be achieved through use of the UTC or Local method, and that the monotonic clock reading has been stripped by setting t = t.Round(0). In general, prefer t.Equal(u) to t == u, since t.Equal uses the most accurate comparison available and correctly handles the case when only one of its arguments has a monotonic clock reading.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Time代表一个纳秒精度的时间点。
/// 程序中应使用Time类型值来保存和传递时间，而不能用指针。就是说，表示时间的变量和字段，应为time.Time类型，而不是*time.Time.类型。一个Time类型值可以被多个go程同时使用。时间点可以使用Before、After和Equal方法进行比较。Sub方法让两个时间点相减，生成一个Duration类型值（代表时间段）。Add方法给一个时间点加上一个时间段，生成一个新的Time类型时间点。
/// Time零值代表时间点January 1, year 1, 00:00:00.000000000 UTC。因为本时间点一般不会出现在使用中，IsZero方法提供了检验时间是否显式初始化的一个简单途径。
/// 每一个时间都具有一个地点信息（及对应地点的时区信息），当计算时间的表示格式时，如Format、Hour和Year等方法，都会考虑该信息。Local、UTC和In方法返回一个指定时区（但指向同一时间点）的Time。修改地点/时区信息只是会改变其表示；不会修改被表示的时间点，因此也不会影响其计算。
/// </details>
#[derive(Default, PartialEq, PartialOrd, Debug, Clone, Fmt)]
pub struct Time {
    wall: uint64,
    ext: int64,
    loc: Location,
}

impl Time {
    fn new() -> Time {
        Time::default()
    }

    /// Format returns a textual representation of the time value formatted according to the layout defined by the argument. See the documentation for the constant called Layout to see how to represent the layout format.
    ///
    /// The executable example for Time.Format demonstrates the working of the layout string in detail and is a good reference.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Format根据layout指定的格式返回t代表的时间点的格式化文本表示。layout定义了参考时间：
    ///
    /// `Mon Jan 2 15:04:05 -0700 MST 2006`
    ///
    /// 格式化后的字符串表示，它作为期望输出的例子。同样的格式规则会被用于格式化时间。
    ///
    /// 预定义的ANSIC、UnixDate、RFC3339和其他版式描述了参考时间的标准或便捷表示。要获得更多参考时间的定义和格式，参见本包的ANSIC和其他版式常量。
    /// </details>
    ///
    /// # Example
    /// ```
    /// use gostd::time;
    ///
    /// let t = time::Date(2009, 11, 10, 14, 30, 12, 13, time::UTC.clone());
    /// assert_eq!(t.String(),"2009-11-10 14:30:12.000000013 +0000 UTC".to_string());
    /// assert_eq!(t.Format(time::RFC822),"10 Nov 09 14:30 UTC".to_string());
    /// assert_eq!(t.Format(time::RFC1123),"Tue, 10 Nov 2009 14:30:12 UTC".to_string());
    /// println!("default: {}", t);
    /// println!("RFC822: {}", t.Format(time::RFC822));
    /// println!("RFC1123: {}", t.Format(time::RFC1123));
    /// // output:
    /// // default: 2009-11-10 14:30:12.000000013 +0000 UTC
    /// // RFC822: 11/10 02:30:12PM '09 +0000
    /// // RFC1123: Tue, 10 Nov 2009 14:30:12 UTC
    /// ```
    pub fn Format(&self, layout: &str) -> string {
        const bufSize: uint = 64;
        let mut b: Vec<byte> = vec![];
        let max = layout.len() + 10;
        if max < bufSize {
            let buf: [byte; bufSize] = [0; bufSize];
            b = buf[..0].to_vec()
        } else {
            b = Vec::with_capacity(max);
        }

        b = self.AppendFormat(b, layout);
        string(&b)
    }

    /// String returns the time formatted using the format string
    ///
    /// `2006-01-02 15:04:05.999999999 -0700 MST`
    ///
    /// If the time has a monotonic clock reading, the returned string includes a final field `m=±<value>`, where value is the monotonic clock reading formatted as a decimal number of seconds.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// String 返回使用格式化字符串的时间
    ///
    /// `2006-01-02 15:04:05.999999999 -0700 MST`
    ///
    /// 如果时间有一个单调的时钟读数，返回的字符串包括最后一个字段 `m=±<value>`，其中value是单调的时钟读数，格式为十进制的秒数。
    /// </details>
    pub fn String(&self) -> string {
        let mut s = self.Format("2006-01-02 15:04:05.999999999 -0700 MST");
        if self.wall & uint64!(hasMonotonic) != 0 {
            let mut m2 = int64!(self.ext);
            let mut sign = byte!('+');
            if self.ext < 0 {
                sign = byte!('+');
                m2 = int64!(-m2);
            }
            let mut m1 = m2 / 1000_000_000;
            m2 = m2 % 1000_000_000;
            let m0 = m1 / 100_000_000;
            m1 = m1 % 1000_000_000;
            let mut buf = Vec::with_capacity(24);
            buf.extend_from_slice((" m=".as_bytes()));
            buf.push(sign);
            let mut wid = 0;

            if m0 != 0 {
                buf = appendInt(buf, int!(m0), wid);
                wid = 9;
            }

            buf = appendInt(buf, int!(m1), wid);
            buf.push(byte!('.'));
            buf = appendInt(buf, int!(m2), 9);
            s += string::from_utf8(buf.to_vec())
                .expect("time String failed from_utf8")
                .as_str();
        }
        s
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

    /// After reports whether the time instant self is after u.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 如果self 代表的时间点在u之后，返回真；否则返回假。
    /// </details>
    ///
    /// # Example
    ///
    /// ```
    /// use gostd::time;
    ///
    /// let year2000 = time::Date(2000, 1, 1, 0, 0, 0, 0, time::UTC.clone());
    ///	let year3000 = time::Date(3000, 1, 1, 0, 0, 0, 0, time::UTC.clone());
    ///
    ///	assert_eq!(true,year3000.After(&year2000));// True
    ///	assert_eq!(false,year2000.After(&year3000)); // False
    ///
    /// ```
    pub fn After(&self, u: &Time) -> bool {
        if self.wall & u.wall & uint64!(hasMonotonic) != 0 {
            return self.ext > u.ext;
        }
        let ts = self.sec();
        let us = u.sec();
        ts > us || ts == us && self.nsec() > u.nsec()
    }
    /// Before reports whether the time instant self is before u.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 如果self代表的时间点在u之前，返回真；否则返回假。
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

    /// Equal reports whether t and u represent the same time instant. Two times can be equal even if they are in different locations. For example, 6:00 +0200 and 4:00 UTC are Equal. See the documentation on the Time type for the pitfalls of using == with Time values; most code should use Equal instead.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 判断两个时间是否相同，会考虑时区的影响，因此不同时区标准的时间也可以正确比较。本方法和用t==u不同，这种方法还会比较地点和时区信息。
    /// </details>
    ///
    /// # Example
    ///
    /// ```
    /// use gostd::builtin::*;
    /// use gostd::time;
    ///
    ///    let secondsEastOfUTC = int!(time::Duration::new(8 * time::Hour).Seconds());
    ///    let beijing = time::FixedZone("Beijing Time", secondsEastOfUTC);///
    ///
    ///    // Unlike the equal operator, Equal is aware that d1 and d2 are the
    ///    // same instant but in different time zones.
    ///    let d1 = time::Date(2000, 2, 1, 12, 30, 0, 0, time::UTC.clone());
    ///    let d2 = time::Date(2000, 2, 1, 20, 30, 0, 0, beijing);///
    ///
    ///    let datesEqualUsingEqualOperator = d1 == d2;
    ///    let datesEqualUsingFunction = d1.Equal(&d2);
    ///
    ///    assert_eq!(false,datesEqualUsingEqualOperator);
    ///    assert_eq!(true,datesEqualUsingFunction);
    ///
    ///    println!("datesEqualUsingEqualOperator = {}",datesEqualUsingEqualOperator);
    ///    println!("datesEqualUsingFunction = {}", datesEqualUsingFunction);
    ///
    /// // output:
    /// // datesEqualUsingEqualOperator = false
    /// // datesEqualUsingFunction = true
    /// ```
    pub fn Equal(&self, u: &Time) -> bool {
        if self.wall & u.wall & uint64!(hasMonotonic) != 0 {
            return self.ext == u.ext;
        }
        self.sec() == u.sec() && self.nsec() == u.nsec()
    }

    /// IsZero reports whether t represents the zero time instant, January 1, year 1, 00:00:00 UTC.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// IsZero报告t是否代表零点时刻，1年1月1日，00:00:00 UTC。
    /// </details>
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

    // locabs是Zone和abs方法的组合。
    // 从一个区的查询中提取两个返回值。
    fn locabs(&self) -> (string, int, uint64) {
        let mut name: string = "".to_string();
        let mut offset: int = 0;
        let mut l = self.loc.clone();
        if l.name.len() == 0 || l.name.as_str() == "Local" {
            l = l.get();
        }

        let mut sec = self.unixSec();
        if l.name.as_str() != "UTC" {
            if l.cacheZone.name.len() != 0 && l.cacheStart <= sec && sec < l.cacheEnd {
                name = l.cacheZone.name;
                offset = l.cacheZone.offset;
            } else {
                let (name, offset, _, _, _) = l.lookup(sec);
            }
            sec += int64!(offset);
        } else {
            name = "UTC".to_string();
        }
        let abs = uint64!(sec + int64!(unixToInternal + internalToAbsolute));
        (name, offset, abs)
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

    ///	println!("start = {}", start);
    ///	println!("start.Add(time.Second * 10) = {}", afterTenSeconds);
    ///	println!("start.Add(time.Minute * 10) = {}", afterTenMinutes);
    ///	println!("start.Add(time.Hour * 10) = {}", afterTenHours);
    ///	println!("start.Add(time.Hour * 24 * 10) = {}", afterTenDays);
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
            let d = Duration::new(te - ue);
            if d.0 < 0 && te > ue {
                return Duration::new(maxDuration);
            }
            if d.0 > 0 && te < ue {
                return Duration::new(minDuration);
            }
            return d;
        }
        let d = Duration::new((self.sec() - u.sec()) * Second + int64!(self.nsec() - u.nsec()));
        if u.Add(&d).Equal(self) {
            d
        } else if self.Before(u) {
            Duration::new(minDuration)
        } else {
            Duration::new(maxDuration)
        }
    }

    /// Date returns the year, month, and day in which self occurs.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 返回时间点self对应的年、月、日。
    /// </details>
    ///
    /// # Example
    ///
    /// ```
    /// use gostd::time;
    ///
    ///     let d = time::Date(2000, 2, 1, 12, 30, 0, 0, time::UTC.clone());
    ///     let (year, month, day) = d.Date();
    ///     assert_eq!((2000, time::Month::February, 1), d.Date());
    ///     println!("year = {}", year);
    ///     println!("month = {}", month);
    ///     println!("day = {}", day);
    /// // output:
    /// // year = 2000
    /// // month = February
    /// // day = 1
    /// ```
    pub fn Date(&self) -> (int, Month, int) {
        let (year, month, day, _) = self.date(true);
        (year, month, day)
    }

    /// Year returns the year in which self occurs.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 返回时间点self对应的年份。
    /// </details>
    pub fn Year(&self) -> int {
        let (year, _, _, _) = self.date(false);
        year
    }

    /// Month returns the month of the year specified by self.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 返回时间点self对应那一年的第几月。
    /// </details>
    ///
    /// # Example
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

    /// Clock returns the hour, minute, and second within the day specified by self.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Clock返回由self指定的一天中的时、分、秒。
    /// </details>
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
    /// let day = t.Day();
    /// assert_eq!(1,day);
    /// println!("day = {}",day);
    /// // output:
    /// // day = 1
    /// ```
    pub fn Day(&self) -> int {
        let (_, _, day, _) = self.date(true);
        day
    }

    /// Hour returns the hour within the day specified by self, in the range [0, 23].
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Hour 返回由self指定的一天中的第几小时，范围为[0, 23]。
    /// </details>
    pub fn Hour(&self) -> int {
        int!(int64!(self.abs()) % secondsPerDay / secondsPerHour)
    }

    /// Minute returns the minute offset within the hour specified by self, in the range [0, 59].
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Minute 返回由self指定的小时内的第几分钟，范围为[0, 59]。
    /// </details>
    pub fn Minute(&self) -> int {
        int!(int64!(self.abs()) % secondsPerHour / secondsPerMinute)
    }

    /// Second returns the second offset within the minute specified by self, in the range [0, 59].
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Second 返回由self指定的分钟内的第几秒，范围为[0, 59]。
    /// </details>
    pub fn Second(&self) -> int {
        int!(int64!(self.abs()) % secondsPerMinute)
    }

    /// Nanosecond returns the nanosecond offset within the second specified by t, in the range [0, 999999999].
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Nanosecond 返回self对应的那一秒内的纳秒偏移量，范围[0, 999999999]。
    /// </details>
    pub fn Nanosecond(&self) -> int {
        int!(self.nsec())
    }

    /// Round returns the result of rounding t to the nearest multiple of d (since the zero time).
    /// The rounding behavior for halfway values is to round up.
    /// If d <= 0, Round returns self stripped of any monotonic clock reading but otherwise unchanged.
    ///
    /// Round operates on the time as an absolute duration since the
    /// zero time; it does not operate on the presentation form of the
    /// time. Thus, Round(Hour) may return a time with a non-zero
    /// minute, depending on the time's Location.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 返回距离self最近的时间点，该时间点应该满足从Time零值到该时间点的时间段能整除d；如果有两个满足要求的时间点，距离self相同，会向上舍入；如果d <= 0，会返回self的拷贝。
    /// </details>
    pub fn Round(&self, d: Duration) -> Time {
        let mut t = self.to_owned();
        t.stripMono();
        let d1 = d.0;
        if d1 <= 0 {
            return t;
        }
        let (_, r) = div(t.to_owned(), d);
        if lessThanHalf(r.0, d1) {
            return t.Add(&Duration::new(-(r.0)));
        }
        return t.Add(&Duration::new(d1 - r.0));
    }

    /// YearDay returns the day of the year specified by t, in the range `[1, 365]` for non-leap years, and `[1, 366]` in leap years.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// YearDay 返回时间点self对应的那一年的第几天，平年的返回值范围`[1, 365]`，闰年`[1, 366]`。
    /// </details>
    pub fn YeayDay(&self) -> int {
        let (_, _, _, yday) = self.date(false);
        yday + 1
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

    /// ISOWeek returns the ISO 8601 year and week number in which t occurs. Week ranges from 1 to 53. Jan 01 to Jan 03 of year n might belong to week 52 or 53 of year n-1, and Dec 29 to Dec 31 might belong to week 1 of year n+1.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// ISOWeek 返回时间点t对应的ISO 9601标准下的年份和星期编号。星期编号范围[1,53]，1月1号到1月3号可能属于上一年的最后一周，12月29号到12月31号可能属于下一年的第一周。
    /// </details>
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

    /// UTC returns self with the location set to UTC.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// UTC返回采用UTC和零时区，但指向同一时间点的Time。
    /// </details>
    pub fn UTC(&mut self) -> Time {
        self.setLoc(utcLoc.clone());
        let mut t = Time::new();
        t.wall = self.wall;
        t.ext = self.ext;
        t.loc = self.loc.clone();
        t
    }

    /// Local returns t with the location set to local time.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Local 返回采用本地和本地时区，但指向同一时间点的Time。
    /// </details>
    pub fn Local(&mut self) -> Time {
        self.setLoc(Local.clone());
        let mut t = Time::new();
        t.wall = self.wall;
        t.ext = self.ext;
        t.loc = self.loc.clone();
        t
    }

    /// In returns a copy of t representing the same time instant, but with the copy's location information set to loc for display purposes.
    ///
    /// In panics if loc is nil.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// In 返回采用loc指定的地点和时区，但指向同一时间点的Time。
    ///
    /// 如果loc为nil会panic
    /// </details>
    pub fn In(&mut self, loc: Location) -> Time {
        self.setLoc(loc);
        let mut t = Time::new();
        t.wall = self.wall;
        t.ext = self.ext;
        t.loc = self.loc.clone();
        t
    }

    /// Location returns the time zone information associated with self.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Location 返回与self相关的时区信息。
    /// </details>
    pub fn Location(&self) -> Location {
        let mut l = self.loc.clone();
        if l.name.len() == 0 {
            l = utcLoc.clone();
        }
        l
    }

    /// Zone computes the time zone in effect at time self, returning the abbreviated name of the zone (such as "CET") and its offset in seconds east of UTC.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Zone计算t所在的时区，返回该时区的规范名（如"CET"）和该时区相对于UTC的时间偏移量（单位秒）。
    /// </details>
    pub fn Zone(&self) -> (string, int) {
        let (name, offset, _, _, _) = self.loc.lookup(self.unixSec());
        (name, offset)
    }

    /// Unix returns t as a Unix time, the number of seconds elapsed since January 1, 1970 UTC. The result does not depend on the location associated with t. Unix-like operating systems often record time as a 32-bit count of seconds, but since the method here returns a 64-bit value it is valid for billions of years into the past or future.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Unix将t作为Unix时间返回，即从1970年1月1日UTC开始经过的秒数。类似Unix的操作系统通常将时间记录为32位的秒数，但由于这里的方法返回的是64位的值，所以对过去或未来的数十亿年都有效。
    /// </details>
    ///
    /// # Example
    /// ```
    /// use gostd::time;
    ///
    /// let t = time::Date(2001, 9, 9, 1, 46, 40, 0, time::UTC.clone());
    /// println!("{}",t.Unix());     // seconds since 1970
    /// println!("{}",t.UnixNano()); // nanoseconds since 1970
    /// assert_eq!(1000000000,t.Unix());
    /// assert_eq!(1000000000000000000,t.UnixNano());
    ///
    /// ```
    pub fn Unix(&self) -> int64 {
        self.unixSec()
    }

    /// UnixMilli returns self as a Unix time, the number of milliseconds elapsed since January 1, 1970 UTC.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// UnixMilli返回self作为Unix时间，即从1970年1月1日UTC开始经过的毫秒数。
    /// </details>
    pub fn UnixMilli(&self) -> int64 {
        self.unixSec() * 1000 + int64!(self.nsec()) / 1000_000
    }
    /// UnixMicro returns t as a Unix time, the number of microseconds elapsed since January 1, 1970 UTC.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// UnixMicro将t作为Unix时间返回，即从1970年1月1日UTC开始经过的微秒数。
    /// </detail>
    pub fn UnixMicro(&self) -> int64 {
        self.unixSec() * 1000_000 + int64!(self.nsec()) / 1000
    }

    /// UnixNano returns t as a Unix time, the number of nanoseconds elapsed since January 1, 1970 UTC.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// UnixNano返回t作为Unix时间，即从1970年1月1日UTC开始经过的纳秒数。
    /// </details>
    pub fn UnixNano(&self) -> int64 {
        self.unixSec() * 1000_000_000 + int64!(self.nsec())
    }

    /// AddDate returns the time corresponding to adding the given number of years, months, and days to t. For example, AddDate(-1, 2, 3) applied to January 1, 2011 returns March 4, 2010.
    ///  
    /// AddDate normalizes its result in the same way that Date does, so, for example, adding one month to October 31 yields December 1, the normalized form for November 31.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// AddDate返回与给定的年、月、日数相加的时间。例如，AddDate(-1, 2, 3)应用于2011年1月1日，返回2010年3月4日。
    ///
    /// AddDate以与Date相同的方式对其结果进行规范化处理，因此，例如，在10月31日的基础上增加一个月，得到12月1日，即11月31日的规范化形式。
    /// </details>
    ///
    /// # Example
    /// ```
    /// use gostd::time;
    ///
    /// let start = time::Date(2009, 1, 1, 0, 0, 0, 0, time::UTC.clone());
    /// let oneDayLater = start.AddDate(0, 0, 1);
    /// let oneMonthLater = start.AddDate(0, 1, 0);
    /// let oneYearLater = start.AddDate(1, 0, 0);
    /// assert_eq!(oneDayLater.String(), "2009-01-02 00:00:00 +0000 UTC");
    /// assert_eq!(oneMonthLater.String(), "2009-02-01 00:00:00 +0000 UTC");
    /// assert_eq!(oneYearLater.String(), "2010-01-01 00:00:00 +0000 UTC");
    ///
    /// println!("oneDayLater: start.AddDate(0, 0, 1) = {}", oneDayLater);
    /// println!("oneMonthLater: start.AddDate(0, 1, 0) = {}", oneMonthLater);
    /// println!("oneYearLater: start.AddDate(1, 0, 0) = {}", oneYearLater);
    /// ```
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

    /// IsDST reports whether the time in the configured location is in Daylight Savings Time.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// IsDST报告配置地点的时间是否处于夏令时。
    /// </details>
    pub fn IsDST(&self) -> bool {
        let (_, _, _, _, isDST) = self.loc.lookup(self.Unix());
        isDST
    }
}

/// Unix returns the local Time corresponding to the given Unix time, sec seconds and nsec nanoseconds since January 1, 1970 UTC. It is valid to pass nsec outside the range [0, 999999999]. Not all sec values have a corresponding time value. One such value is 1<<63-1 (the largest int64 value).
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Unix返回与给定的Unix时间相对应的本地时间，自1970年1月1日UTC以来的秒数和纳秒数。在[0, 999999999]范围之外传递nsec是有效的。不是所有的sec值都有相应的时间值。一个这样的值是1<<63-1（最大的int64值）。
/// </details>
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

/// UnixMilli returns the local Time corresponding to the given Unix time, msec milliseconds since January 1, 1970 UTC.
///
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// UnixMilli返回与给定的Unix时间相对应的本地时间，自1970年1月1日UTC以来的毫秒数。
/// </details>
pub fn UnixMilli(msec: int64) -> Time {
    Unix(msec / 1000, (msec % 1000) / 1000_000)
}

/// UnixMicro returns the local Time corresponding to the given Unix time, usec microseconds since January 1, 1970 UTC.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// UnixMicro返回与给定的Unix时间相对应的本地时间，自1970年1月1日UTC以来的微秒数。
/// </details>
pub fn UnixMicro(usec: int64) -> Time {
    Unix(usec / 1000_000, (usec % 1000_000) * 1000)
}
/// Date returns the Time corresponding to
///
/// `yyyy-mm-dd hh:mm:ss + nsec nanoseconds`
///
/// in the appropriate zone for that time in the given location.
///
/// The month, day, hour, min, sec, and nsec values may be outside their usual ranges and will be normalized during the conversion. For example, October 32 converts to November 1.
///
/// A daylight savings time transition skips or repeats times. For example, in the United States, March 13, 2011 2:15am never occurred, while November 6, 2011 1:15am occurred twice. In such cases, the choice of time zone, and therefore the time, is not well-defined. Date returns a time that is correct in one of the two zones involved in the transition, but it does not guarantee which.
///
/// Date panics if loc is nil.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Date返回一个时区为loc、当地时间为：
///
/// `year-month-day hour:min:sec + nsec nanoseconds`
///
/// 的时间点。
///
/// month、day、hour、min、sec和nsec的值可能会超出它们的正常范围，在转换前函数会自动将之规范化。如October 32被修正为November 1。
///
/// 夏时制的时区切换会跳过或重复时间。如，在美国，March 13, 2011 2:15am从来不会出现，而November 6, 2011 1:15am 会出现两次。此时，时区的选择和时间是没有良好定义的。Date会返回在时区切换的两个时区其中一个时区
/// 正确的时间，但本函数不会保证在哪一个时区正确。
/// 如果loc为nil会panic。
/// </details>
///
/// # Example
///
/// ```rust
/// use gostd::time;
/// let d = time::Date(2009, 11, 10, 14, 30, 12, 13, time::UTC.clone());
/// let (year, month, day) = d.Date();
/// assert_eq!(year, 2009);
/// assert_eq!(month.String(), "November");
/// assert_eq!(day, 10);
/// assert_eq!(d.String(),"2009-11-10 14:30:12.000000013 +0000 UTC".to_string());
/// println!("year = {}",year);
/// println!("month = {}",month.String());
/// println!("day = {}",day);
/// println!("{}",d);
///
///  //  output:
///  //  year = 2009
///  //  month = November
///  //  day = 10
///  //  2009-11-10 14:30:12.000000013 +0000 UTC
/// ```
pub fn Date(
    year: int,
    month: uint,
    day: int,
    hour: int,
    min: int,
    sec: int,
    nsec: int,
    loc: Location,
) -> Time {
    let loc = Some(loc);
    if loc.is_none() {
        panic!("time: missing Location in call to Date")
    }

    let mut m: int = int!(month) - 1;
    let (year, m) = norm(year, int!(m), 12);
    let month = Month::IndexOf(uint!(m + 1));

    let (sec, nsec) = norm(sec, nsec, 1000_000_000);
    let (min, sec) = norm(min, sec, 60);
    let (hour, min) = norm(hour, min, 60);
    let (day, hour) = norm(day, hour, 24);

    let mut d = daySinceEpoch(year);

    d += uint64!(daysBefore[uint!(month) - 1]);

    if isLeap(year) && month > Month::March {
        d += 1;
    }

    d += uint64!((day - 1).abs());

    let mut abs: uint64 = d * uint64!(secondsPerDay);
    abs += uint64!(hour) * uint64!(secondsPerHour)
        + uint64!(min) * uint64!(secondsPerMinute)
        + uint64!(sec);

    let mut unix: int64 = 0;
    if uint64!((absoluteToInternal + internalToUnix).abs()) > abs {
        unix = -int64!(uint64!((absoluteToInternal + internalToUnix).abs()) - abs)
    } else {
        unix = int64!(abs - uint64!((absoluteToInternal + internalToUnix).abs()));
    }
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

///
/// Parse parses a formatted string and returns the time value it represents. See the documentation for the constant called Layout to see how to represent the format. The second argument must be parseable using the format string (layout) provided as the first argument.
///
/// The example for Time.Format demonstrates the working of the layout string in detail and is a good reference.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Parse解析一个格式化的时间字符串并返回它代表的时间。layout定义了参考时间：
///
/// `Mon Jan 2 15:04:05 -0700 MST 2006`
/// 在输入格式下的字符串表示，作为输入的格式的示例。同样的格式规则会被用于输入字符串。
///
// 预定义的ANSIC、UnixDate、RFC3339和其他版式描述了参考时间的标准或便捷表示。要获得更多参考时间的定义和格式，参/// 见本包的ANSIC和其他版式常量。
/// </details>
///
/// # Example:
///```
/// use gostd::time;
///
/// let layout = "Jan 2, 2006 at 3:04pm (MST)";
/// let t = time::Parse(layout, "Feb 3, 2013 at 7:54pm (PST)")
///     .ok()
///     .expect("Parse faile:");
/// assert_eq!( t.String() ,"2013-02-03 19:54:00 +0000 PST".to_string());
/// println!("{}", t);
/// // output:
/// // 2013-02-03 19:54:00 +0000 PST
///```
pub fn Parse(layout: &str, value: &str) -> Result<Time, string> {
    parse(layout, value, UTC.clone(), Local.clone())
}

fn parse(
    layout: &str,
    value: &str,
    defaultLocation: Location,
    local: Location,
) -> Result<Time, string> {
    let mut layout = layout;
    let mut value = value;
    let mut alayout = layout;
    let mut avalue = value;
    let mut rangeErrString = "";
    let mut amSet = false;
    let mut pmSet = false;

    let mut year: int = 0;
    let mut month: int = -1;
    let mut day: int = -1;
    let mut yday: int = -1;
    let mut hour: int = 0;
    let mut min: int = 0;
    let mut sec: int = 0;
    let mut nsec: int = 0;
    let mut z = Location::new().clone();
    let mut zoneOffset: int = -1;
    let mut zoneName: &str = "";

    // Each iteration processes one std value.
    loop {
        let mut err: &str = "";
        let (prefix, mut std, suffix) = nextStdChunk(layout);
        let stdstr = &layout[len!(prefix)..(len!(layout) - len!(suffix))];
        let res = skip(value, prefix)?;
        value = res;

        if std == 0 {
            if len!(value) != 0 {
                return Err(format!("{} {}", "extra text:", value));
            }
            break;
        }
        layout = suffix;
        let mut p: &str = "";
        match (std & stdMask) {
            stdYear => {
                if len!(value) < 2 {
                    err = "stdYear err1";
                    break;
                }
                let hold = value;
                p = &value[0..2];
                value = &value[2..];
                let res = atoi(p);
                if res.is_err() {
                    value = hold;
                }
                if res.is_ok() {
                    year = res.ok().unwrap();
                }
                if year >= 69 {
                    // Unix time starts Dec 31 1969 in some time zones
                    year += 1900
                } else {
                    year += 2000
                }
            }
            stdLongYear => {
                if len!(value) < 4 || !isDigit(value, 0) {
                    err = "stdLongYear err1";
                    break;
                }
                p = &value[0..4];
                value = &value[4..];
                let res = atoi(p);
                if res.is_err() {
                    err = res.err().unwrap();
                }
                if res.is_ok() {
                    year = res.ok().unwrap();
                }
            }
            stdMonth => {
                let res = lookup(shortMonthNames.to_vec(), value);
                if res.is_err() {
                    err = res.err().unwrap();
                }

                if res.is_ok() {
                    let r = res.ok().unwrap();
                    month = r.0;
                    value = r.1;
                }
                month += 1;
            }
            stdLongMonth => {
                let res = lookup(longMonthNames.to_vec(), value);
                if res.is_err() {
                    err = res.err().unwrap();
                }
                if res.is_ok() {
                    let r = res.ok().unwrap();
                    month = r.0;
                    value = r.1;
                }
                month += 1;
            }
            stdNumMonth | stdZeroMonth => {
                let res = getnum(value, std == stdZeroMonth);
                if res.is_ok() {
                    let r = res.ok().unwrap();
                    month = r.0;
                    value = r.1;
                    if (month <= 0 || 12 < month) {
                        rangeErrString = "month";
                    }
                }
            }
            stdWeekDay => {
                // Ignore weekday except for error checking.
                let res = lookup(shortDayNames.to_vec(), value);
                if res.is_err() {
                    err = res.err().unwrap();
                }

                if res.is_ok() {
                    let r = res.ok().unwrap();
                    value = r.1;
                }
            }
            stdLongWeekDay => {
                let res = lookup(longDayNames.to_vec(), value);
                if res.is_err() {
                    err = res.err().unwrap();
                }
                if res.is_ok() {
                    let r = res.ok().unwrap();
                    value = r.1;
                }
            }
            stdDay | stdUnderDay | stdZeroDay => {
                if std == stdUnderDay && len!(value) > 0 && value.bytes().nth(0) == Some(b' ') {
                    value = &value[1..];
                }
                let res = getnum(value, std == stdZeroDay);
                if res.is_err() {
                    err = res.err().unwrap();
                }

                if res.is_ok() {
                    let r = res.ok().unwrap();
                    day = r.0;
                    value = r.1;
                }
                // Note that we allow any one- or two-digit day here.
                // The month, day, year combination is validated after we've completed parsing.
            }
            stdUnderYearDay | stdZeroYearDay => {
                for i in 0..2 {
                    if std == stdUnderYearDay
                        && len!(value) > 0
                        && value.bytes().nth(0) == Some(b' ')
                    {
                        value = &value[1..]
                    }
                }
                let res = getnum3(value, std == stdZeroYearDay);
                if res.is_err() {
                    err = res.err().unwrap();
                }
                if res.is_ok() {
                    let r = res.ok().unwrap();
                    yday = r.0;
                    value = r.1;
                }
                // Note that we allow any one-, two-, or three-digit year-day here.
                // The year-day, year combination is validated after we've completed parsing.
            }
            stdHour => {
                let res = getnum(value, false);
                if res.is_err() {
                    err = res.err().unwrap();
                }
                if res.is_ok() {
                    let r = res.ok().unwrap();
                    hour = r.0;
                    value = r.1;
                }

                if hour < 0 || 24 <= hour {
                    rangeErrString = "hour"
                }
            }
            stdHour12 | stdZeroHour12 => {
                let res = getnum(value, std == stdZeroHour12);
                if res.is_err() {
                    err = res.err().unwrap();
                }
                if res.is_ok() {
                    let r = res.ok().unwrap();
                    hour = r.0;
                    value = r.1;
                }
                if hour < 0 || 12 < hour {
                    rangeErrString = "hour"
                }
            }
            stdMinute | stdZeroMinute => {
                let res = getnum(value, std == stdZeroMinute);
                if res.is_err() {
                    err = res.err().unwrap();
                }
                if res.is_ok() {
                    let r = res.ok().unwrap();
                    min = r.0;
                    value = r.1;
                }

                if min < 0 || 60 <= min {
                    rangeErrString = "minute"
                }
            }
            stdSecond | stdZeroSecond => {
                let res = getnum(value, std == stdZeroSecond);
                if res.is_err() {
                    err = res.err().unwrap();
                }
                if res.is_ok() {
                    let r = res.ok().unwrap();
                    sec = r.0;
                    value = r.1;
                }
                if sec < 0 || 60 <= sec {
                    rangeErrString = "second";
                    break;
                }
                // Special case: do we have a fractional second but no
                // fractional second in the format?
                if len!(value) >= 2
                    && commaOrPeriod(value.bytes().nth(0).unwrap())
                    && isDigit(value, 1)
                {
                    let stdResult = nextStdChunk(layout);
                    std = stdResult.1;
                    std &= stdMask;
                    if std == stdFracSecond0 || std == stdFracSecond9 {
                        // Fractional second in the layout; proceed normally
                        break;
                    }
                    // No fractional second in the layout but we have one in the input.
                    let mut n = 2;
                    while (n < len!(value) && isDigit(value, int!(n))) {
                        n += 1;
                    }
                    let res = parseNanoseconds(value, n);
                    if res.is_err() {
                        err = res.err().unwrap();
                    }
                    if res.is_ok() {
                        nsec = res.ok().unwrap();
                    }
                    value = &value[n..];
                }
            }
            stdPM => {
                if len!(value) < 2 {
                    err = errBad;
                    break;
                }
                p = &value[0..2];
                value = &value[2..];
                match p {
                    "PM" => pmSet = true,
                    "AM" => amSet = true,
                    _ => err = errBad,
                }
            }
            stdpm => {
                if len!(value) < 2 {
                    err = errBad;
                    break;
                }
                p = &value[0..2];
                value = &value[2..];
                match p {
                    "pm" => pmSet = true,
                    "am" => amSet = true,

                    _ => err = errBad,
                }
            }
            stdISO8601TZ
            | stdISO8601ColonTZ
            | stdISO8601SecondsTZ
            | stdISO8601ShortTZ
            | stdISO8601ColonSecondsTZ
            | stdNumTZ
            | stdNumShortTZ
            | stdNumColonTZ
            | stdNumSecondsTz
            | stdNumColonSecondsTZ => {
                if (std == stdISO8601TZ || std == stdISO8601ShortTZ || std == stdISO8601ColonTZ)
                    && len!(value) >= 1
                    && value.bytes().nth(0) == Some(b'Z')
                {
                    value = &value[1..];
                    z = UTC.clone();
                    break;
                }
                let mut sign: &str;
                let mut hour: &str;
                let mut min: &str;
                let mut seconds: &str;
                if std == stdISO8601ColonTZ || std == stdNumColonTZ {
                    if len!(value) < 6 {
                        err = errBad;
                        break;
                    }
                    if value.bytes().nth(3) != Some(b':') {
                        err = errBad;
                        break;
                    }

                    sign = &value[0..1];
                    hour = &value[1..3];
                    min = &value[4..6];
                    seconds = "00";
                    value = &value[6..];
                } else if std == stdNumShortTZ || std == stdISO8601ShortTZ {
                    if len!(value) < 3 {
                        err = errBad;
                        break;
                    }
                    sign = &value[0..1];
                    hour = &value[1..3];
                    min = "00";
                    seconds = "00";
                    value = &value[3..];
                } else if std == stdISO8601ColonSecondsTZ || std == stdNumColonSecondsTZ {
                    if len!(value) < 9 {
                        err = errBad;
                        break;
                    }
                    if value.bytes().nth(3) != Some(b':') || value.bytes().nth(6) != Some(b':') {
                        err = errBad;
                        break;
                    }
                    sign = &value[0..1];
                    hour = &value[1..3];
                    min = &value[4..6];
                    seconds = &value[7..9];
                    value = &value[9..];
                } else if std == stdISO8601SecondsTZ || std == stdNumSecondsTz {
                    if len!(value) < 7 {
                        err = errBad;
                        break;
                    }
                    sign = &value[0..1];
                    hour = &value[1..3];
                    min = &value[3..5];
                    seconds = &value[5..7];
                    value = &value[7..];
                } else {
                    if len!(value) < 5 {
                        err = errBad;
                        break;
                    }
                    sign = &value[0..1];
                    hour = &value[1..3];
                    min = &value[3..5];
                    seconds = "00";
                    value = &value[5..];
                }
                let mut hr: int = 0;
                let mut mm: int = 0;
                let mut ss: int = 0;
                let res = atoi(hour);
                if res.is_err() {
                    err = res.err().unwrap();
                }
                if res.is_ok() {
                    hr = res.ok().unwrap();
                    let res1 = atoi(min);
                    if res1.is_err() {
                        err = res1.err().unwrap();
                    }
                    if res1.is_ok() {
                        mm = res1.ok().unwrap();

                        let res2 = atoi(seconds);
                        if res2.is_err() {
                            err = res2.err().unwrap();
                        }

                        if res2.is_ok() {
                            ss = res2.ok().unwrap();
                        }
                    }
                }
                zoneOffset = (hr * 60 + mm) * 60 + ss; // offset is in seconds
                match sign.bytes().nth(0) {
                    Some(b'+') => (),
                    Some(b'-') => zoneOffset = -zoneOffset,
                    _ => err = errBad,
                }
            }
            stdTZ => {
                // Does it look like a time zone?
                if len!(value) >= 3 && &value[0..3] == "UTC" {
                    z = UTC.clone();
                    value = &value[3..];
                    break;
                }
                let res = parseTimeZone(value);
                let n = uint!(res.0);
                let ok = res.1;
                if !ok {
                    err = errBad;
                    break;
                }
                zoneName = &value[..n];
                value = &value[n..];
            }
            stdFracSecond0 => {
                // stdFracSecond0 requires the exact number of digits as specified in
                // the layout.
                let ndigit = uint!(1 + digitsLen(int!(std)));
                if len!(value) < ndigit {
                    err = errBad;
                    break;
                }
                let res = parseNanoseconds(value, ndigit);
                if res.is_err() {
                    err = res.err().unwrap();
                }
                if res.is_ok() {
                    nsec = res.ok().unwrap();
                }
                value = &value[ndigit..];
            }
            stdFracSecond9 => {
                if len!(value) < 2
                    || !commaOrPeriod(value.bytes().nth(0).unwrap())
                    || value.bytes().nth(1) < Some(b'0')
                    || Some(b'9') < value.bytes().nth(1)
                {
                    // Fractional second omitted.
                    break;
                }
                // Take any number of digits, even more than asked for,
                // because it is what the stdSecond case would do.
                let mut i = 0;
                while (i < 9
                    && i + 1 < len!(value)
                    && Some(b'0') <= value.bytes().nth(i + 1)
                    && value.bytes().nth(i + 1) <= Some(b'9'))
                {
                    i += 1;
                }
                let res = parseNanoseconds(value, 1 + i);
                if res.is_err() {
                    err = res.err().unwrap();
                }
                if res.is_ok() {
                    nsec = res.ok().unwrap();
                }
                value = &value[1 + i..];
            }
            _ => (),
        }
        if rangeErrString != "" {
            return Err(format!("{} {}", rangeErrString, " out of range"));
        }
        if err != "" {
            return Err(format!("{} {}", "ParseError:", err));
        }
    }
    if pmSet && hour < 12 {
        hour += 12;
    } else if amSet && hour == 12 {
        hour = 0;
    }

    // Convert yday to day, month.
    if yday >= 0 {
        let mut d: int = 0;
        let mut m: int = 0;
        if isLeap(year) {
            if yday == 31 + 29 {
                m = int!(Month::February);
                d = 29;
            } else if yday > 31 + 29 {
                yday -= 1;
            }
        }
        if yday < 1 || yday > 365 {
            return Err(format!(
                "{} {} {} {} {}",
                alayout, avalue, "", value, ": day-of-year out of range"
            ));
        }
        if m == 0 {
            m = (yday - 1) / 31 + 1;
            if int!(daysBefore[uint!(m)]) < yday {
                m += 1;
            }
            d = yday - int!(daysBefore[uint!(m) - 1]);
        }
        // If month, day already seen, yday's m, d must match.
        // Otherwise, set them from m, d.
        if month >= 0 && month != m {
            return Err(format!(
                "{} {} {} {} {}",
                alayout, avalue, "", value, ": day-of-year does not match month"
            ));
        }
        month = m;
        if day >= 0 && day != d {
            return Err(format!(
                "{} {} {} {} {}",
                alayout, avalue, "", value, ": day-of-year does not match day"
            ));
        }
        day = d;
    } else {
        if month < 0 {
            month = int!(Month::January);
        }
        if day < 0 {
            day = 1;
        }
    }

    // Validate the day of the month.
    if day < 1 || day > daysIn(Month::IndexOf(uint!(month)), year) {
        return Err(format!(
            "{} {} {} {} {}",
            alayout, avalue, "", value, ": day out of range"
        ));
    }

    if z.name != "" {
        return Ok(Date(
            year,
            uint!(month),
            day,
            hour,
            min,
            sec,
            nsec,
            z.clone(),
        ));
    }

    if zoneOffset != -1 {
        let mut t = Date(year, uint!(month), day, hour, min, sec, nsec, UTC.clone());
        t.addSec(-int64!(zoneOffset));

        // Look for local zone with the given offset.
        // If that zone was in effect at the given time, use it.
        let (name, offset, _, _, _) = local.lookup(t.unixSec());
        if offset == zoneOffset && (zoneName == "" || name == zoneName) {
            t.setLoc(local);
            return Ok(t);
        }

        // Otherwise create fake zone to record offset.

        t.setLoc(FixedZone(zoneName, zoneOffset));
        return Ok(t);
    }

    if zoneName != "" {
        let mut t = Date(year, uint!(month), day, hour, min, sec, nsec, UTC.clone());
        // Look for local zone with the given offset.
        // If that zone was in effect at the given time, use it.
        let (mut offset, ok) = local.lookupName(zoneName, t.unixSec());
        if ok {
            t.addSec(-int64!(offset));
            t.setLoc(local);
            return Ok(t);
        }

        // Otherwise, create fake zone with unknown offset.
        if len!(zoneName) > 3 && &zoneName[..3] == "GMT" {
            let res = atoi(&zoneName[3..]); // Guaranteed OK by parseGMT
            if res.is_ok() {
                offset = res.ok().unwrap();
            }
            offset *= 3600;
        }
        t.setLoc(FixedZone(zoneName, offset));
        return Ok(t);
    }

    // Otherwise, fall back to default.
    Ok(Date(
        year,
        uint!(month),
        day,
        hour,
        min,
        sec,
        nsec,
        defaultLocation,
    ))
}

fn cutspace(s: &str) -> &str {
    let mut s = s;
    while (len!(s) > 0 && s.bytes().nth(0) == Some(b' ')) {
        s = &s[1..];
    }
    s
}

fn getnum(s: &str, fixed: bool) -> Result<(int, &str), &str> {
    if !isDigit(s, 0) {
        return Err(errBad);
    }

    if !isDigit(s, 1) {
        if fixed {
            return Err(errBad);
        }
        return Ok((int!(s.bytes().nth(0).unwrap() - b'0'), &s[1..]));
    }

    Ok((
        int!((s.bytes().nth(0).unwrap() - b'0') * 10) + int!(s.bytes().nth(1).unwrap() - b'0'),
        &s[2..],
    ))
}

fn getnum3(s: &str, fixed: bool) -> Result<(int, &str), &str> {
    let mut n: int = 0;
    let mut i: uint = 0;
    while (i < 3 && isDigit(s, int!(i))) {
        i += 1;
        n = n * 10 + int!(s.bytes().nth(i).unwrap() - b'0');
    }
    if i == 0 || fixed && i != 3 {
        return Err(errBad);
    }
    Ok((n, &s[i..]))
}

fn atoi(s: &str) -> Result<int, &str> {
    let mut x = 0;
    let mut s = s;
    let mut neg = false;
    if s != "" && (s.bytes().nth(0) == Some(b'-') || s.bytes().nth(0) == Some(b'+')) {
        neg = (s.bytes().nth(0) == Some(b'-'));
        s = &s[1..];
    }
    let (q, rem) = leadingInt(s)?; //?相当于原来的try!()宏，错误提前返回错误信息,相当于golang中的 if err! = nil { return err }
    x = int!(q);
    if neg {
        x = -x;
    }

    Ok(x)
}

fn parseTimeZone(value: &str) -> (int, bool) {
    if len!(value) < 3 {
        return (0, false);
    }
    // Special case 1: ChST and MeST are the only zones with a lower-case letter.
    if len!(value) >= 4 && (&value[..4] == "ChST" || &value[..4] == "MeST") {
        return (4, true);
    }
    // Special case 2: GMT may have an hour offset; treat it specially.
    if &value[..3] == "GMT" {
        let length = parseGMT(value);
        return (length, true);
    }
    // Special Case 3: Some time zones are not named, but have +/-00 format
    if value.bytes().nth(0) == Some(b'+') || value.bytes().nth(0) == Some(b'-') {
        let length = parseSignedOffset(value);
        let ok = (length > 0); // parseSignedOffset returns 0 in case of bad input
        return (length, ok);
    }
    // How many upper-case letters are there? Need at least three, at most five.
    let mut nUpper: uint = 0;
    for n in 0..6 {
        nUpper = n;
        if n >= len!(value) {
            break;
        }
        let c = value.bytes().nth(n);
        if c < Some(b'A') || Some(b'Z') < c {
            break;
        }
    }
    match nUpper {
        0 | 1 | 2 | 6 => return (0, false),
        5 => {
            // Must end in T to match.
            if value.bytes().nth(4) == Some(b'T') {
                return (5, true);
            }
        }
        4 => {
            // Must end in T, except one special case.
            if value.bytes().nth(3) == Some(b'T') || &value[..4] == "WITA" {
                return (4, true);
            }
        }
        3 => return (3, true),
        _ => return (0, false),
    }
    (0, false)
}

fn parseGMT(value: &str) -> int {
    let value = &value[3..];
    if len!(value) == 0 {
        return 3;
    }

    3 + parseSignedOffset(value)
}

fn parseSignedOffset(value: &str) -> int {
    let sign = value.bytes().nth(0).unwrap();
    if sign != b'-' && sign != b'+' {
        return 0;
    }

    let mut res = leadingInt(&value[1..]);
    if res.is_err() {
        return 0;
    }
    let (mut x, mut rem) = res.ok().unwrap();
    if &value[1..] == rem {
        return 0;
    }
    if sign == b'-' {
        x = -x;
    }
    if x < -23 || 23 < x {
        return 0;
    }
    return int!(len!(value) - len!(rem));
}

fn commaOrPeriod(b: byte) -> bool {
    return b == b'.' || b == b',';
}

fn parseNanoseconds(value: &str, nbytes: uint) -> Result<int, &str> {
    if !commaOrPeriod(value.bytes().nth(0).unwrap()) {
        return Err(errBad);
    }

    let mut ns = atoi(&value[1..nbytes])?;
    if ns < 0 || 1000_0000_000 < ns {
        return Err("fractional second");
    }

    let scaleDigits = 10 - nbytes;
    for i in 0..scaleDigits {
        ns *= 10;
    }
    Ok(ns)
}

const errLeadingInt: &str = "time: bad [0-9]*";
fn leadingInt(s: &str) -> Result<(int64, &str), &str> {
    let mut x: int64 = 0;
    let rem: &str = "";
    let mut j = 0;
    for i in 0..len!(s) {
        j = i;
        let c = s.bytes().nth(i).unwrap();
        if c < b'0' || c > b'9' {
            break;
        }
        if x > (1 << 63 - 1) / 10 {
            // overflow
            return Err(errLeadingInt);
        }
        x = x * 10 + int64!(c - b'0');
        if x < 0 {
            // overflow
            return Err(errLeadingInt);
        }
    }
    Ok((x, &s[j..]))
}

// leadingFraction consumes the leading [0-9]* from s.
// It is used only for fractions, so does not return an error on overflow,
// it just stops accumulating precision.
fn leadingFraction(s: &str) -> (int64, float64, &str) {
    let mut x: int64 = 0;
    let mut index = 0;
    let mut scale: float64 = 1.0;
    let mut overflow = false;
    for (i, c) in s.bytes().enumerate() {
        index = i;
        if c < b'0' || c > b'9' {
            break;
        }
        if overflow {
            continue;
        }
        if x > (1 << 63 - 1) / 10 {
            // It's possible for overflow to give a positive number, so take care.
            overflow = true;
            continue;
        }
        let y = x * 10 + int64!(c) - int64!(b'0');
        if y < 0 {
            overflow = true;
            continue;
        }
        x = y;
        scale *= 10.0;
    }
    (x, scale, &s[index..])
}

fn skip<'a>(value: &'a str, prefix: &'a str) -> Result<&'a str, &'a str> {
    let mut value = value;
    let mut prefix = prefix;
    while (len!(prefix) > 0) {
        if prefix.bytes().nth(0) == Some(b' ') {
            if len!(value) > 0 && value.bytes().nth(0) != Some(b' ') {
                return Err("skip err1");
            }
            prefix = cutspace(prefix);
            value = cutspace(value);
            continue;
        }
        if len!(value) == 0 || (value.bytes().nth(0) != prefix.bytes().nth(0)) {
            return Err("skip err2");
        }
        prefix = &prefix[1..];
        value = &value[1..];
    }
    Ok(value)
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

/// A Location maps time instants to the zone in use at that time. Typically, the Location represents the collection of time offsets in use in a geographical area. For many Locations the time offset varies depending on whether daylight savings time is in use at the time instant.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Location代表一个（关联到某个时间点的）地点，以及该地点所在的时区。
/// </details>
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
    /// String returns a descriptive name for the time zone information, corresponding to the name argument to LoadLocation or FixedZone.
    ///
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// String返回对时区信息的描述，返回值绑定为LoadLocation或FixedZone函数创建l时的name参数。
    /// </details>
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

    fn lookupName(&self, name: &str, unix: int64) -> (int, bool) {
        let mut offset: int = 0;
        let ok: bool = false;
        let l = self.get();
        for zone in l.zone.as_slice() {
            if zone.name == name {
                let res = l.lookup(unix - int64!(zone.offset));
                let nam = res.0;
                offset = res.1;
                if nam == zone.name {
                    return (offset, true);
                }
            }
        }

        for zone in l.zone.as_slice() {
            if zone.name == name {
                return (zone.offset, true);
            }
        }
        (offset, ok)
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

/// FixedZone returns a Location that always uses the given zone name and offset (seconds east of UTC).
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// FixedZone使用给定的地点名name和时间偏移量offset（单位秒）创建并返回一个Location
/// </details>
///
/// # Example
///
/// ```
/// use gostd::time;
///
/// let mut t = time::Date(2009, 11, 10, 14, 30, 12, 13, time::UTC.clone());
/// assert_eq!(t.String(),"2009-11-10 14:30:12.000000013 +0000 UTC".to_string());
/// println!("UTC: {}", t);
/// let cst_zone = time::FixedZone("CST", 8 * 3600); //北京时区 CST = UTC时区+8小时
/// t.In(cst_zone);
/// assert_eq!(t.String(),"2009-11-10 22:30:12.000000013 +0800 CST".to_string());
/// println!("CST: {}", t);
/// // output:
/// // UTC: 2009-11-10 14:30:12.000000013 +0000 UTC
/// // CST: 2009-11-10 22:30:12.000000013 +0800 CST
/// ```
pub fn FixedZone(name: &str, offset: int) -> Location {
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
     static ref startNano:int64 =runtimeNano() - 1;
/// Local represents the system's local time zone. On Unix systems, Local consults the TZ environment variable to find the time zone to use. No TZ means use the system default /etc/localtime. TZ="" means use UTC. TZ="foo" means use file foo in the system timezone directory.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Local代表系统本地，对应本地时区。
/// </details>
     pub static ref Local:Location = Location::new();
     static ref utcLoc:Location = {
    let mut l = Location::new();
    l.name="UTC".to_string();
     l
    };


/// UTC represents Universal Coordinated Time (UTC).
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// UTC代表通用协调时间，对应零时区。
/// </details>
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

/// Now returns the current local time.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Now返回当前本地时间。
/// </details>
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
    // t.loc = Local.clone();
    t
}

fn absDate(abs: uint64, full: bool) -> (int, Month, int, int) {
    let mut year: int = 0;
    let mut month = Month::Default;
    let mut day: int = 0;
    let mut yday: int = 0;
    // Split into time and day.
    let mut d = int64!(abs / uint64!(secondsPerDay.abs())); // rust需要人工提升到最高精度计算。go这里会自动转换成uint64最高精度记录

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
    month = Month::IndexOf(m);
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

/// A Month specifies a month of the year (January = 1, ...).
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Month代表一年的某个月。
/// </details>
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug, Fmt)]
pub enum Month {
    ///初始0值，实际没有这个月份
    Default = 0,
    /// 一月
    January = 1,
    /// 二月
    February = 2,
    /// 三月
    March = 3,
    /// 四月
    April = 4,
    /// 五月
    May = 5,
    /// 六月
    June = 6,
    /// 七月
    July = 7,
    /// 八月
    August = 8,
    /// 九月
    September = 9,
    /// 十月
    October = 10,
    /// 十一月
    November = 11,
    /// 十二月
    December = 12,
}

impl Month {
    /// String returns the English name of the month ("January", "February", ...).
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// String返回月份的英文名（"January"，"February"，……）
    /// </details>
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
    /// IndexOf returns the month according to the numeric index such as the numbers 1 to 12.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 根据数字1~12等数字索引返回月份。
    /// </details>
    pub fn IndexOf(i: uint) -> Month {
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

/// A Weekday specifies a day of the week (Sunday = 0, ...).
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Weekday代表一周的某一天。
/// </details>
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug, Fmt)]
pub enum Weekday {
    /// 星期日
    Sunday = 0,
    /// 星期一
    Monday = 1,
    /// 星期二
    Tuesday = 2,
    /// 星期三
    Wednesday = 3,
    /// 星期四
    Thursday = 4,
    /// 星期五
    Friday = 5,
    /// 星期六
    Saturday = 6,
}

impl Weekday {
    /// String returns the English name of the day ("Sunday", "Monday", ...).
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// String返回该日（周几）的英文名（"Sunday"、"Monday"，……）
    /// </details>
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

    /// IndexOf returns the day of the week according to the numerical index such as the number 0 to 6.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 根据数字0~6等数字索引返周几。
    /// </details>
    pub fn indexOf(i: uint) -> Weekday {
        match i {
            1 => Weekday::Monday,
            2 => Weekday::Tuesday,
            3 => Weekday::Wednesday,
            4 => Weekday::Thursday,
            5 => Weekday::Friday,
            6 => Weekday::Saturday,
            0 => Weekday::Sunday,
            _ => Weekday::Sunday,
        }
    }
}

fn unitToInt64(unit: &str) -> Option<int64> {
    match unit {
        "ns" => Some(Nanosecond),
        "us" => Some(Microsecond),
        "µs" => Some(Microsecond), // U+00B5 = micro symbol
        "μs" => Some(Microsecond), // U+03BC = Greek letter mu
        "ms" => Some(Millisecond),
        "s" => Some(Second),
        "m" => Some(Minute),
        "h" => Some(Hour),
        _ => None,
    }
}
// 函数
/// ParseDuration parses a duration string.
/// A duration string is a possibly signed sequence of
/// decimal numbers, each with optional fraction and a unit suffix,
/// such as "300ms", "-1.5h" or "2h45m".
/// Valid time units are "ns", "us" (or "µs"), "ms", "s", "m", "h".
///
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// ParseDuration解析一个时间段字符串。一个时间段字符串是一个序列，每个片段包含可选的正负号、十进制数、可选的小数部分和单位后缀，如"300ms"、"-1.5h"、"2h45m"。合法的单位有"ns"、"us" /"µs"、"ms"、"s"、"m"、"h"。
/// </details>
///
/// # Example
/// ```
/// use gostd::time;
///
/// let hours = time::ParseDuration("10h").ok().unwrap();
///	let complex = time::ParseDuration("1h10m10s").ok().unwrap();
///	let micro = time::ParseDuration("1µs").ok().unwrap();
///	// The package also accepts the incorrect but common prefix u for micro.
///	let micro2 = time::ParseDuration("1us").ok().unwrap();
/// assert_eq!(hours,time::Duration::new(36000000000000));
/// assert_eq!(complex,time::Duration::new(4210000000000));
/// assert_eq!(micro,time::Duration::new(1000));
/// assert_eq!(micro2,time::Duration::new(1000));
///	println!("{}",hours);
///	println!("{}",complex);
///	println!("There are {} seconds in {}.", complex.Seconds(), complex);
///	println!("There are {} nanoseconds in {}.", micro.Nanoseconds(), micro);
///	println!("There are {} seconds in {}.", micro2.Seconds(), micro2);
///	// output:
///	// 10h0m0s
/// // 1h10m10s
/// // There are 4210 seconds in 1h10m10s.
/// // There are 1000 nanoseconds in 1µs.
/// // There are 0.000001 seconds in 1µs.
/// ```
pub fn ParseDuration(s: &str) -> Result<Duration, &str> {
    // [-+]?([0-9]*(\.[0-9]*)?[a-z]+)+
    let mut s = s;
    let orig = s;
    let mut d: int64 = 0;
    let mut neg = false;

    // Consume [-+]?
    if s != "" {
        let c = s.as_bytes()[0];
        if c == b'-' || c == b'+' {
            neg = (c == b'-');
            s = &s[1..];
        }
    }
    // Special case: if all that is left is "0", this is zero.
    if s == "0" {
        return Ok(Duration::new(0));
    }
    if s == "" {
        return Err("time: invalid duration ");
    }
    while s != "" {
        let mut v: int64 = 0;
        let mut f: int64 = 0;
        let mut scale: float64 = 1.0;

        let err: &str = "";

        // The next character must be [0-9.]
        if !(s.as_bytes()[0] == b'.' || b'0' <= s.as_bytes()[0] && s.as_bytes()[0] <= b'9') {
            return Err("time: invalid duration ");
        }
        // Consume [0-9]*
        let pl = len!(s);
        let res = leadingInt(s);
        if res.is_err() {
            return Err("time: invalid duration ");
        }

        if res.is_ok() {
            let r = res.ok().unwrap();
            v = r.0;
            s = r.1;
        }

        let mut pre = (pl != len!(s)); // whether we consumed anything before a period

        // Consume (\.[0-9]*)?
        let mut post = false;
        if s != "" && s.as_bytes()[0] == b'.' {
            s = &s[1..];
            let pl = len!(s);
            let res = leadingFraction(s);
            f = res.0;
            scale = res.1;
            s = res.2;
            post = (pl != len!(s));
        }
        if !pre && !post {
            // no digits (e.g. ".s" or "-.s")
            return Err("time: invalid duration ");
        }

        // Consume unit.
        let mut index = 0;
        for i in 0..len!(s) {
            let c = s.bytes().nth(i).expect("time: nht index error");
            if c == b'.' || (b'0' <= c && c <= b'9') {
                break;
            }
            index += 1; // 这里rust for中的i不能使用循环外的变量，变通实现，只有不break返回的情况下才累计index加1。
        }
        if index == 0 {
            return Err("time: missing unit in duration ");
        }
        let u = &s[..index];
        s = &s[index..];
        let res = unitToInt64(u);
        if res.is_none() {
            return Err("time: unknown unit ");
        }
        let mut unit = res.unwrap();
        if v > int64::MAX / unit {
            // overflow
            return Err("time: invalid duration ");
        }
        v *= unit;
        if f > 0 {
            // float64 is needed to be nanosecond accurate for fractions of hours.
            // v >= 0 && (f*unit/scale) <= 3.6e+12 (ns/h, h is the largest unit)
            v += int64!(float64!(f) * (float64!(unit) / scale));
            if v < 0 {
                // overflow
                return Err("time: invalid duration ");
            }
        }
        d += v;
        if d < 0 {
            // overflow
            return Err("time: invalid duration ");
        }
    }

    if neg {
        d = -d;
    }
    Ok(Duration::new(d))
}
/// Since returns the time elapsed since t. It is shorthand for time.Now().Sub(t).
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Since 是time.Now().Sub(t)的简写。
/// </details>
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
/// Until returns the duration until t. It is shorthand for t.Sub(time.Now()).
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Until 是t.Sub(time.Now())的简写。
/// </details>
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
        r = Duration::new(int64!(nsec % int32!(d.0)));
    } else if d.0 % Second == 0 {
        let d1 = int64!(d.0 / Second);
        qmod2 = int!((sec / d1) & 1);
        r = Duration::new((sec % d1) * Second + int64!(nsec));
    } else {
        let sec = uint64!(sec.abs());
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
        u0 = uint64!(nsec.abs());
        if u0 < u0x {
            u1 += 1;
        }

        let mut d1 = uint64!(d.0.abs());
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

        r = Duration::new(int64!(u0));
    }

    if neg && r.0 != 0 {
        qmod2 ^= 1;
        r = Duration::new(d.0 - r.0)
    }
    (qmod2, r)
}

// format.go -start
const stdLongMonth: int32 = stdNeedDate; // "January"
const stdMonth: int32 = stdNeedDate + 1; // "Jan"
const stdNumMonth: int32 = stdNeedDate + 2; // "1"
const stdZeroMonth: int32 = stdNeedDate + 3; // "01"
const stdLongWeekDay: int32 = stdNeedDate + 4; // "Monday"
const stdWeekDay: int32 = stdNeedDate + 5; // "Mon"
const stdDay: int32 = stdNeedDate + 6; // "2"
const stdUnderDay: int32 = stdNeedDate + 7; // "_2"
const stdZeroDay: int32 = stdNeedDate + 8; // "02"
const stdUnderYearDay: int32 = stdNeedDate + 9; // "__2"
const stdZeroYearDay: int32 = stdNeedDate + 10; // "002"
const stdHour: int32 = stdNeedClock + 12; // "15"
const stdHour12: int32 = stdHour + 1; // "3"
const stdZeroHour12: int32 = stdHour + 2; // "03"
const stdMinute: int32 = stdHour + 3; // "4"
const stdZeroMinute: int32 = stdHour + 4; // "04"
const stdSecond: int32 = stdHour + 5; // "5"
const stdZeroSecond: int32 = stdHour + 6; // "05"
const stdLongYear: int32 = stdNeedDate + 19; // "2006"
const stdYear: int32 = stdLongYear + 1; // "06"
const stdPM: int32 = stdNeedClock + 21; // "PM"
const stdpm: int32 = stdPM + 1; // "pm"

const stdTZ: int32 = 23; // "MST"
const stdISO8601TZ: int32 = 24; // "Z0700"  // prints Z for UTC
const stdISO8601SecondsTZ: int32 = 25; // "Z070000"
const stdISO8601ShortTZ: int32 = 26; // "Z07"
const stdISO8601ColonTZ: int32 = 27; // "Z07:00" // prints Z for UTC
const stdISO8601ColonSecondsTZ: int32 = 28; // "Z07:00:00"
const stdNumTZ: int32 = 29; // "-0700"  // always numeric
const stdNumSecondsTz: int32 = 30; // "-070000"
const stdNumShortTZ: int32 = 31; // "-07"    // always numeric
const stdNumColonTZ: int32 = 32; // "-07:00" // always numeric
const stdNumColonSecondsTZ: int32 = 33; // "-07:00:00"
const stdFracSecond0: int32 = 34; // ".0", ".00", ... , trailing zeros included
const stdFracSecond9: int32 = 35; // ".9", ".99", ..., trailing zeros omitted

const stdNeedDate: int32 = 1 << 8; // need month, day, year
const stdNeedClock: int32 = 2 << 8; // need hour, minute, second
const stdArgShift: int32 = 16; // extra argument in high bits, above low stdArgShift
const stdSeparatorShift: int32 = 28; // extra argument in high 4 bits for fractional second separators
const stdMask: int32 = (1 << stdArgShift) - 1; // mask out argument

static std0x: [int32; 6] = [
    stdZeroMonth,
    stdZeroDay,
    stdZeroHour12,
    stdZeroMinute,
    stdZeroSecond,
    stdYear,
];

impl Time {
    /// AppendFormat is like Format but appends the textual representation to b and returns the extended buffer.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// AppendFormat与Format类似，但将文本表示法追加到b，并返回扩展的缓冲区。
    /// </details>
    ///
    /// # Example
    /// ```
    /// use gostd::builtin::*;
    /// use gostd::time;
    ///
    ///     let t = time::Date(2017, 11, 4, 11, 0, 0, 0, time::UTC.clone());
    ///     let mut text: Vec<byte> = "Time: ".as_bytes().to_vec();
    ///     text = t.AppendFormat(text, time::Kitchen);
    ///     assert_eq!("Time: 11:00AM", string(&text));
    ///     println!("{}", string(&text))
    /// // output:
    /// // Time: 11:00AM
    ///
    /// ```
    pub fn AppendFormat(&self, b: Vec<byte>, layout: &str) -> Vec<byte> {
        let (mut name, mut offset, mut abs) = self.locabs();
        let mut year: int = -1;
        let mut month: Month = Month::Default;
        let mut day: int = 0;
        let mut yday: int = 0;
        let mut hour: int = -1;
        let mut min: int = 0;
        let mut sec: int = 0;

        let mut b = b;
        let mut layout = layout;

        while (layout != "") {
            let (prefix, std, suffix) = nextStdChunk(layout);
            if prefix != "" {
                b.extend_from_slice(prefix.as_bytes());
            }

            if std == 0 {
                break;
            }
            layout = suffix;

            if year < 0 && (std & stdNeedDate) != 0 {
                let abs_date = absDate(abs, true);
                year = abs_date.0;
                month = abs_date.1;
                day = abs_date.2;
                yday = abs_date.3;
                yday += 1;
            }

            if hour < 0 && (std & stdNeedClock) != 0 {
                let abs_clock = absClock(abs);
                hour = abs_clock.0;
                min = abs_clock.1;
                sec = abs_clock.2;
            }

            match (std & stdMask) {
                stdYear => {
                    let mut y = year;
                    if y < 0 {
                        y = -y;
                    }
                    b = appendInt(b, y % 100, 2);
                }

                stdLongYear => {
                    b = appendInt(b, year, 4);
                }

                stdMonth => {
                    b.extend_from_slice(&month.clone().String().as_bytes()[..3]);
                }

                stdLongMonth => {
                    b.extend_from_slice(&month.clone().String().as_bytes());
                }

                stdNumMonth => {
                    b = appendInt(b, int!(month), 0);
                }

                stdZeroMonth => b = appendInt(b, int!(month), 2),

                stdWeekDay => {
                    b.extend_from_slice(absWeekday(abs).String().as_bytes()[..3].as_ref())
                }

                stdLongWeekDay => {
                    let s = absWeekday(abs).String();
                    b.extend_from_slice(&s.as_bytes());
                }

                stdDay => {
                    b = appendInt(b, day, 0);
                }

                stdUnderDay => {
                    if day < 10 {
                        b.push(b' ');
                    }
                    b = appendInt(b, day, 0);
                }

                stdZeroDay => {
                    b = appendInt(b, day, 2);
                }

                stdUnderYearDay => {
                    if yday < 100 {
                        b.push(b' ');
                        if yday < 10 {
                            b.push(b' ');
                        }
                    }

                    b = appendInt(b, yday, 0);
                }

                stdZeroYearDay => {
                    b = appendInt(b, yday, 3);
                }

                stdHour => {
                    b = appendInt(b, hour, 2);
                }

                stdHour12 => {
                    let mut hr = hour % 12;
                    if hr == 0 {
                        hr = 12
                    }
                    b = appendInt(b, hr, 0);
                }

                stdZeroHour12 => {
                    let mut hr = hour % 12;
                    if hr == 0 {
                        hr = 12
                    }
                    b = appendInt(b, hr, 2);
                }

                stdMinute => {
                    b = appendInt(b, min, 0);
                }

                stdZeroMinute => {
                    b = appendInt(b, min, 2);
                }

                stdSecond => {
                    b = appendInt(b, sec, 0);
                }

                stdZeroSecond => {
                    b = appendInt(b, sec, 2);
                }

                stdPM => {
                    if hour >= 12 {
                        b.extend_from_slice("PM".as_bytes());
                    } else {
                        b.extend_from_slice("AM".as_bytes());
                    }
                }

                stdpm => {
                    if hour >= 12 {
                        b.extend_from_slice("pm".as_bytes());
                    } else {
                        b.extend_from_slice("am".as_bytes());
                    }
                }

                stdISO8601TZ
                | stdISO8601ColonTZ
                | stdISO8601SecondsTZ
                | stdISO8601ShortTZ
                | stdISO8601ColonSecondsTZ
                | stdNumTZ
                | stdNumColonTZ
                | stdNumSecondsTz
                | stdNumShortTZ
                | stdNumColonSecondsTZ => {
                    if offset == 0
                        && (std == stdISO8601TZ
                            || std == stdISO8601ColonTZ
                            || std == stdISO8601ColonSecondsTZ
                            || std == stdISO8601ShortTZ
                            || std == stdISO8601ColonSecondsTZ)
                    {
                        b.push(b'Z');
                        break;
                    }

                    let mut zone = offset / 60;
                    let mut absoffset = offset;
                    if zone < 0 {
                        b.push(b'-');
                        zone = -zone;
                        absoffset = -absoffset;
                    } else {
                        b.push(b'+');
                    }
                    b = appendInt(b, zone / 60, 2);

                    if std == stdISO8601ColonTZ
                        || std == stdNumColonTZ
                        || std == stdISO8601ColonSecondsTZ
                        || std == stdNumColonSecondsTZ
                    {
                        b.push(b':');
                    }

                    if std != stdNumShortTZ && std != stdISO8601ShortTZ {
                        b = appendInt(b, zone % 60, 2);
                    }

                    if std == stdISO8601SecondsTZ
                        || std == stdNumSecondsTz
                        || std == stdNumColonSecondsTZ
                        || std == stdISO8601ColonSecondsTZ
                    {
                        if std == stdNumColonSecondsTZ || std == stdISO8601ColonSecondsTZ {
                            b.push(b':');
                        }
                        b = appendInt(b, absoffset % 60, 2);
                    }
                }

                stdTZ => {
                    if name != "" {
                        b.extend_from_slice(name.as_bytes());
                        break;
                    }

                    let mut zone = offset / 60;
                    if zone < 0 {
                        b.push(b'-');
                        zone = -zone;
                    } else {
                        b.push(b'+');
                    }
                    b = appendInt(b, zone / 60, 2);
                    b = appendInt(b, zone % 60, 2);
                }

                stdFracSecond0 | stdFracSecond9 => {
                    b = formatNano(b, uint!(self.Nanosecond()), int!(std))
                }

                _ => (), // 匹配不到什么都不做
            }
        }
        b
    }
}

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
            c1 |= (byte!('a') - byte!('A'));
            c2 |= (byte!('a') - byte!('A'));
            if c1 != c2 || c1 < byte!('a') || c1 > byte!('z') {
                return false;
            }
        }
    }
    true
}

const errBad: &'static str = "bad value for field";

fn lookup<'a>(tab: Vec<&'a str>, val: &'a str) -> Result<(int, &'a str), &'a str> {
    for (i, v) in tab.iter().enumerate() {
        if val.len() >= v.len() && isMatch(&val[0..v.len()], v) {
            let index = uint!(v.len());
            return Ok((int!(i), &val[index..]));
        }
    }

    Err("lookup err1") // go源码:return -1, val, errBad
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

fn startWithLowerCase(s: &str) -> bool {
    if 0 == s.len() {
        return false;
    }
    let c = s.bytes().nth(0).unwrap();
    b'a' <= c && c <= b'z'
}

fn isDigit(s: &str, i: int) -> bool {
    if s.len() <= uint!(i) {
        return false;
    }
    let c = s.bytes().nth(uint!(i)).unwrap();
    return b'0' <= c && c <= b'9';
}

fn stdFracSecond(code: int, n: int, c: int) -> int {
    // Use 0xfff to make the failure case even more absurd.
    if uint8!(c) == b'.' {
        return code | ((n & 0xfff) << stdArgShift);
    }
    code | ((n & 0xfff) << stdArgShift) | 1 << stdSeparatorShift
}

fn nextStdChunk(layout: &str) -> (&str, int32, &str) {
    let length = layout.len();
    for (i, c) in layout.bytes().enumerate() {
        match c {
            b'J' => {
                if layout.len() >= i + 3 && &layout[i..i + 3] == "Jan" {
                    if layout.len() >= i + 7 && &layout[i..i + 7] == "January" {
                        return (
                            layout[0..i].as_ref(),
                            stdLongMonth,
                            layout[i + 7..].as_ref(),
                        );
                    }
                    if !startWithLowerCase(layout[i + 3..].as_ref()) {
                        return (layout[0..i].as_ref(), stdMonth, layout[i + 3..].as_ref());
                    }
                }
            }

            b'M' => {
                if layout.len() >= 3 {
                    if &layout[i..i + 3] == "Mon" {
                        if layout.len() >= i + 6 && &layout[i..i + 6] == "Monday" {
                            return (
                                layout[0..i].as_ref(),
                                stdLongWeekDay,
                                layout[i + 6..].as_ref(),
                            );
                        }
                        if !startWithLowerCase(layout[i + 3..].as_ref()) {
                            return (layout[0..i].as_ref(), stdWeekDay, layout[i + 3..].as_ref());
                        }
                    }
                    if &layout[i..i + 3] == "MST" {
                        return (layout[0..i].as_ref(), stdTZ, layout[i + 3..].as_ref());
                    }
                }
            }

            b'0' => {
                if layout.len() >= i + 2
                    && b'1' <= layout.bytes().nth(i + 1).unwrap()
                    && layout.bytes().nth(i + 1).unwrap() <= b'6'
                {
                    let idx: uint = uint!(layout.bytes().nth(i + 1).unwrap() - b'1');
                    return (layout[0..i].as_ref(), std0x[idx], layout[i + 2..].as_ref());
                }

                if layout.len() >= i + 3
                    && layout.bytes().nth(i + 1).unwrap() == b'0'
                    && layout.bytes().nth(i + 2).unwrap() == b'2'
                {
                    return (
                        layout[0..i].as_ref(),
                        stdZeroYearDay,
                        layout[i + 3..].as_ref(),
                    );
                }
            }

            b'1' => {
                if layout.len() >= i + 2 && layout.bytes().nth(i + 1).unwrap() == b'5' {
                    return (layout[0..i].as_ref(), stdHour, layout[i + 2..].as_ref());
                }
                return (layout[0..i].as_ref(), stdNumMonth, layout[i + 1..].as_ref());
            }

            b'2' => {
                if layout.len() >= i + 4 && &layout[i..i + 4] == "2006" {
                    return (layout[0..i].as_ref(), stdLongYear, layout[i + 4..].as_ref());
                }
                return (layout[0..i].as_ref(), stdDay, layout[i + 1..].as_ref());
            }

            b'_' => {
                if layout.len() >= i + 2 && layout.bytes().nth(i + 1).unwrap() == b'2' {
                    if layout.len() >= i + 5 && &layout[i + 1..i + 5] == "2006" {
                        return (
                            layout[0..i + 1].as_ref(),
                            stdLongYear,
                            layout[i + 5..].as_ref(),
                        );
                    }
                    return (layout[0..i].as_ref(), stdUnderDay, layout[i + 2..].as_ref());
                }
                if layout.len() >= i + 3
                    && layout.bytes().nth(i + 1).unwrap() == b'_'
                    && layout.bytes().nth(i + 2).unwrap() == b'2'
                {
                    return (
                        layout[0..i].as_ref(),
                        stdUnderYearDay,
                        layout[i + 3..].as_ref(),
                    );
                }
            }

            b'3' => {
                return (layout[0..i].as_ref(), stdHour12, layout[i + 1..].as_ref());
            }

            b'4' => {
                return (layout[0..i].as_ref(), stdMinute, layout[i + 1..].as_ref());
            }

            b'5' => {
                return (layout[0..i].as_ref(), stdSecond, layout[i + 1..].as_ref());
            }

            b'P' => {
                if layout.len() >= i + 2 && layout.bytes().nth(i + 1).unwrap() == b'M' {
                    return (layout[0..i].as_ref(), stdPM, layout[i + 2..].as_ref());
                }
            }

            b'p' => {
                if layout.len() >= i + 2 && layout.bytes().nth(i + 1).unwrap() == b'm' {
                    return (layout[0..i].as_ref(), stdpm, layout[i + 2..].as_ref());
                }
            }

            b'-' => {
                if layout.len() >= i + 7 && &layout[i..i + 7] == "-070000" {
                    return (
                        layout[0..i].as_ref(),
                        stdNumSecondsTz,
                        layout[i + 7..].as_ref(),
                    );
                }
                if layout.len() >= i + 9 && &layout[i..i + 9] == "-07:00:00" {
                    return (
                        layout[0..i].as_ref(),
                        stdNumColonSecondsTZ,
                        layout[i + 9..].as_ref(),
                    );
                }
                if layout.len() >= i + 5 && &layout[i..i + 5] == "-0700" {
                    return (layout[0..i].as_ref(), stdNumTZ, layout[i + 5..].as_ref());
                }
                if layout.len() >= i + 6 && &layout[i..i + 6] == "-07:00" {
                    return (
                        layout[0..i].as_ref(),
                        stdNumColonTZ,
                        layout[i + 6..].as_ref(),
                    );
                }
                if layout.len() >= i + 3 && &layout[i..i + 3] == "-07" {
                    return (
                        layout[0..i].as_ref(),
                        stdNumShortTZ,
                        layout[i + 3..].as_ref(),
                    );
                }
            }

            b'Z' => {
                if layout.len() >= i + 7 && &layout[i..i + 7] == "Z070000" {
                    return (
                        layout[0..i].as_ref(),
                        stdISO8601SecondsTZ,
                        layout[i + 7..].as_ref(),
                    );
                }
                if layout.len() >= i + 9 && &layout[i..i + 9] == "Z07:00:00" {
                    return (
                        layout[0..i].as_ref(),
                        stdISO8601ColonSecondsTZ,
                        layout[i + 9..].as_ref(),
                    );
                }
                if layout.len() >= i + 5 && &layout[i..i + 5] == "Z0700" {
                    return (
                        layout[0..i].as_ref(),
                        stdISO8601TZ,
                        layout[i + 5..].as_ref(),
                    );
                }
                if layout.len() >= i + 6 && &layout[i..i + 6] == "Z07:00" {
                    return (
                        layout[0..i].as_ref(),
                        stdISO8601ColonTZ,
                        layout[i + 6..].as_ref(),
                    );
                }
                if layout.len() >= i + 3 && &layout[i..i + 3] == "Z07" {
                    return (
                        layout[0..i].as_ref(),
                        stdISO8601ShortTZ,
                        layout[i + 3..].as_ref(),
                    );
                }
            }

            b'.' | b',' => {
                if i + 1 < layout.len()
                    && (layout.bytes().nth(i + 1).unwrap() == b'0'
                        || layout.bytes().nth(i + 1).unwrap() == b'9')
                {
                    let ch = layout.bytes().nth(i + 1).unwrap();
                    let mut j = i + 1;
                    while (j < layout.len() && layout.bytes().nth(j).unwrap() == ch) {
                        j += 1;
                    }
                    // String of digits must end here - only fractional second is all digits.
                    if !isDigit(layout, int!(j)) {
                        let mut code = stdFracSecond0;
                        if layout.bytes().nth(i + 1).unwrap() == b'9' {
                            code = stdFracSecond9;
                        }
                        let std = stdFracSecond(int!(code), int!(j - (i + 1)), int!(c));
                        return (layout[0..i].as_ref(), int32!(std), layout[j..].as_ref());
                    }
                }
            }

            _ => (), // match 必须穷举所有可能项,_=>()表示什么都不做
        }
    }
    (layout, 0, "")
}

fn formatNano(b: Vec<byte>, nanosec: uint, std: int) -> Vec<byte> {
    let mut b = b;
    let mut n = digitsLen(std);

    let separator = separator(std);
    let trim = (int32!(std) & stdMask == stdFracSecond9);

    let mut u = nanosec;
    let mut buf: [byte; 9] = [0; 9];
    for start in (0..buf.len()).rev() {
        buf[start] = byte!(u % 10) + b'0';
        u /= 10;
    }

    if n > 9 {
        n = 9
    }

    if trim {
        while (n > 0 && buf[uint!(n - 1)] == b'0') {
            n -= 1;
        }
        if n == 0 {
            return b;
        }
    }
    b.push(separator);
    b.extend_from_slice(&buf[..uint!(n)]);
    b
}

fn digitsLen(std: int) -> int {
    (std >> stdArgShift) & 0xfff
}

fn separator(std: int) -> byte {
    if (std >> stdSeparatorShift) == 0 {
        return b'.';
    }
    b','
}
// format.go -end
