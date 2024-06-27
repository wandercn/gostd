# gostd_time

Package time provides functionality for measuring and displaying time.
The calendrical calculations always assume a Gregorian calendar, with no leap seconds.

time包提供了时间的显示和测量用的函数。日历的计算采用的是公历。

- [x] time库在rust实现 gostd::time
- [x] time库支持macOSX 和linux平台，通过libc库调用C函数实现 time::Now()
- [x] time，支持各种格式显示时间。
- [x] docs.rs文档增加例子程序"RUN"按钮,但是要复制代码本地运行,在rust play运行不了(因为下载量没到前100)。
- [x] v1.0.3,开始time支持local时区信息自动从系统读取，可以用time::Now()获取本地时间。
- [x] v1.1.0,开始支持windo10平台的正常编译运行，由于平台差异，暂时获取不到本地时区信息，默认取UTC时间，可以传入时区格式化本地时间。

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
