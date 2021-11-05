# gostd_time

Package time provides functionality for measuring and displaying time.
The calendrical calculations always assume a Gregorian calendar, with no leap seconds.

time包提供了时间的显示和测量用的函数。日历的计算采用的是公历。

# Example

```
fn main() {
    use gostd_time as time;

    let t = time::Date(2009, 11, 10, 14, 30, 12, 13, time::UTC.clone());
    assert_eq!(
        t.String(),
        "2009-11-10 14:30:12.000000013 +0000 UTC".to_string()
    );
    assert_eq!(t.Format(time::RFC822), "10 Nov 09 14:30 UTC".to_string());
    assert_eq!(
        t.Format(time::RFC1123),
        "Tue, 10 Nov 2009 14:30:12 UTC".to_string()
    );
    println!("default: {}", t);
    println!("RFC822: {}", t.Format(time::RFC822));
    println!("RFC1123: {}", t.Format(time::RFC1123));
}


```
