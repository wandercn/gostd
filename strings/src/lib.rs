//! Package strings implements simple functions to manipulate UTF-8 encoded strings.
//!
//! <details class="rustdoc-toggle top-doc">
//! <summary class="docblock">zh-cn</summary>
//! strings 实现了操作UTF-8编码字符串的简单函数。
//! </details>

// #![allow(unused_assignments)]
#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#[cfg(test)]
mod tests;
#[macro_use]
use gostd_builtin::*;
use gostd_derive::Fmt;
use gostd_io as io;
use gostd_unicode::utf8;
/// Compare returns an integer comparing two strings lexicographically. The result will be 0 if a==b, -1 if a < b, and +1 if a > b.
/// Compare is included only for symmetry with package bytes. It is usually clearer and always faster to use the built-in string comparison operators ==, <, >, and so on.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Compare返回一个按字典顺序比较两个字符串的整数。如果a == b，结果为0；如果a < b，结果为-1；如果a>b，结果为+1
/// 比较仅用于与包字节对称。使用内置的字符串比较运算符==、<、>，等等通常更清晰，而且总是更快。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
///    assert_eq!(-1, strings::Compare("a", "b"));
///    assert_eq!(0, strings::Compare("a", "a".to_string()));
///    assert_eq!(1, strings::Compare("b", "a"));
///
/// ```
pub fn Compare(a: impl AsRef<str>, b: impl AsRef<str>) -> int {
    if a.as_ref() == b.as_ref() {
        return 0;
    }
    if a.as_ref() < b.as_ref() {
        return -1;
    }
    1
}
/// Contains reports whether substr is within s.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 判断字符串s是否包含子串substr。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!(true,strings::Contains("seafood", "foo"));
/// assert_eq!(false,strings::Contains("seafood", "bar"));
/// assert_eq!(true,strings::Contains("seafood", ""));
/// assert_eq!(true,strings::Contains("", ""));
/// ```
pub fn Contains(s: impl AsRef<str>, substr: impl AsRef<str>) -> bool {
    s.as_ref().contains(substr.as_ref())
}

/// ContainsAny reports whether any Unicode code points in chars are within s.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 判断字符串s是否包含字符串chars中的任一字符。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
///    assert_eq!(false, strings::ContainsAny("team", "i"));
///    assert_eq!(true, strings::ContainsAny("failure", "u & i"));
///    assert_eq!(false, strings::ContainsAny("foo", ""));
///    assert_eq!(false, strings::ContainsAny("", ""));
///    assert_eq!(true, strings::ContainsAny("你好,中国", "hello,好"));
///
/// ```
pub fn ContainsAny(s: impl AsRef<str>, chars: impl AsRef<str>) -> bool {
    for c in chars.as_ref().chars() {
        if s.as_ref().contains(c) {
            return true;
        }
    }
    false
}

/// ContainsRune reports whether the Unicode code point r is within s.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 判断字符串s是否包含utf-8码值r
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// // '中' as rune = 20013 or 0x4e2d
/// assert_eq!(true, strings::ContainsRune("hello中国!", 20013));
/// assert_eq!(true, strings::ContainsRune("hello中国!", 0x4e2d));
/// assert_eq!(false, strings::ContainsRune("hello世界!", 0x4e2d));
/// ```
pub fn ContainsRune(s: impl AsRef<str>, r: rune) -> bool {
    s.as_ref().contains(char::from_u32(r).unwrap())
}

/// Count counts the number of non-overlapping instances of substr in s. If substr is an empty string, Count returns 1 + the number of Unicode code points in s.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回字符串s中有几个不重复的substr子串。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///     assert_eq!(3, strings::Count("cheese", "e"));
///     assert_eq!(5, strings::Count("five", ""));
///     assert_eq!(4, strings::Count("台湾人香港人澳门人都是中国人", "人"));
/// ```
pub fn Count(mut s: &str, substr: impl AsRef<str>) -> int {
    if len!(substr.as_ref()) == 0 {
        return int!(s.chars().count() + 1);
    }

    if len!(substr.as_ref()) == 1 {
        let mut c: int = 0;
        let s1 = substr.as_ref().bytes().nth(0).unwrap();
        for v in s.bytes() {
            if v == s1 {
                c += 1
            }
        }
        return c;
    }

    let mut n: int = 0;
    loop {
        let i = Index(s, substr.as_ref());
        if i == -1 {
            return n;
        }
        n += 1;
        s = &s[uint!(i) + len!(substr.as_ref())..];
    }
}

/// EqualFold reports whether s and t, interpreted as UTF-8 strings, are equal under Unicode case-folding, which is a more general form of case-insensitivity.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 判断两个utf-8编码字符串（将unicode大写、小写、标题三种格式字符视为相同）是否相同
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
///    assert_eq!(true, strings::EqualFold("Hello, 世界", "heLLo, 世界"));
///    assert_eq!(false, strings::EqualFold("hello,world", "hello, 世界"));
///    assert_eq!(true, strings::EqualFold("RUST-LANG", "rust-lang"));
///    assert_eq!(true, strings::EqualFold("Go", "go"));
/// ```
pub fn EqualFold(s: impl AsRef<str>, t: impl AsRef<str>) -> bool {
    s.as_ref().to_lowercase() == t.as_ref().to_lowercase()
}

/// Fields splits the string s around each instance of one or more consecutive white space characters, as defined by unicode.IsSpace, returning a slice of substrings of s or an empty slice if s contains only white space.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将字符串按照空白（unicode.IsSpace确定，可以是一到多个连续的空白字符）分割的多个字符串。如果字符串全部是空白或者是空字符串的话，会返回空切片。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///  assert_eq!(vec!["foo","bar","baz"],strings::Fields("  foo bar  baz   "));
///  assert_eq!(
///     vec!["aaa", "bbb", "cccc", "ddd"],
///     strings::Fields("  \taaa bbb\t  cccc\r ddd  \r"));
/// ```
pub fn Fields(s: &str) -> Vec<&str> {
    s.trim().split_whitespace().collect()
}

/// FieldsFunc splits the string s at each run of Unicode code points c satisfying f(c) and returns an array of slices of s. If all code points in s satisfy f(c) or the string is empty, an empty slice is returned.
///
/// FieldsFunc makes no guarantees about the order in which it calls f(c) and assumes that f always returns the same value for a given c.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
///    /* fn f(c: u32) -> bool {
///        let s = char::from_u32(c).unwrap();
///        !s.is_numeric() && !s.is_alphabetic()
///    } */
///    // f 用函数或者匿名函数都可以
///    let f = |c: u32| {
///        let s = char::from_u32(c).unwrap();
///        !s.is_numeric() && !s.is_alphabetic()
///    };
///    assert_eq!(
///        vec!["foo1", "bar2", "baz3"],
///        strings::FieldsFunc("  foo1;bar2,baz3...", f)
///    )
/// ```
pub fn FieldsFunc(s: &str, f: fn(rune) -> bool) -> Vec<&str> {
    #[derive(Default, PartialEq, PartialOrd, Debug, Clone)]
    struct span {
        start: int,
        end: int,
    }
    let mut spans = Vec::with_capacity(32);
    let mut start = -1;
    for (end, rune) in s.chars().enumerate() {
        if f(rune as u32) {
            if start >= 0 {
                spans.push(span {
                    start,
                    end: end as int,
                });

                start = !start; // go中一元运算符^ ,在rust中对应的是!,都是按位取反。
            }
        } else {
            if start < 0 {
                start = end as int;
            }
        }
    }

    if start >= 0 {
        spans.push(span {
            start,
            end: len!(s) as int,
        });
    }

    let mut a = vec![];
    a.resize(len!(spans), "");
    for (i, span) in spans.iter().enumerate() {
        a[i] = &s[span.start as usize..span.end as usize];
    }
    a
}

/// HasPrefix tests whether the string s begins with prefix.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 判断s是否有前缀字符串prefix。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!(true,strings::HasPrefix("Rustacean","Rust"));
/// assert_eq!(false,strings::HasPrefix("Rustacean","c"));
/// assert_eq!(true,strings::HasPrefix("Rustacean",""));
/// assert_eq!(true,strings::HasPrefix("Gopher","Go"));
///
/// ```
pub fn HasPrefix(s: impl AsRef<str>, prefix: impl AsRef<str>) -> bool {
    s.as_ref().starts_with(prefix.as_ref())
}

/// HasSuffix tests whether the string s ends with suffix.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 判断s是否有后缀字符串suffix。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!(true,strings::HasSuffix("Amirust","rust"));
/// assert_eq!(false,strings::HasSuffix("Amirust","R"));
/// assert_eq!(false,strings::HasSuffix("Amirust","Ami"));
/// assert_eq!(true,strings::HasSuffix("Amirust",""));
/// ```
pub fn HasSuffix(s: impl AsRef<str>, suffix: impl AsRef<str>) -> bool {
    s.as_ref().ends_with(suffix.as_ref())
}

/// Cut slices s around the first instance of sep,
/// returning the text before and after sep.
/// The found result reports whether sep appears in s.
/// If sep does not appear in s, cut returns s, "", false.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 在s中的第一个分隔字符串sep处切分字符串，返回sep前面部分before和sep后面的部分字符串after。found 值表示在s字符串中是否找到sep字符串。如果seq在s中找不到，切割结果返回(s,"",false)。
/// </details>
///
/// # Example
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!(strings::Cut("127.0.0.1:8080",":"),("127.0.0.1","8080",true));
/// ```
pub fn Cut<'a>(s: &'a str, sep: &str) -> (&'a str, &'a str, bool) {
    let before: &'a str;
    let after: &'a str;
    let found: bool;
    let i = Index(s, sep);
    if i >= 0 {
        before = &s[..uint!(i)];
        after = &s[(uint!(i) + len!(sep))..];
        found = true;
        return (before, after, found);
    }
    before = s;
    after = "";
    found = false;
    (before, after, found)
}

/// Index returns the index of the first instance of substr in s, or -1 if substr is not present in s.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 子串substr在字符串s中第一次出现的位置，不存在则返回-1
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!(4,strings::Index("chicken", "ken"));
/// assert_eq!(-1,strings::Index("chicken", "dmr"));
/// ```
pub fn Index(s: impl AsRef<str>, substr: impl AsRef<str>) -> int {
    if substr.as_ref() == "" {
        return -1;
    }
    if let Some(i) = s.as_ref().find(substr.as_ref()) {
        return int!(i);
    } else {
        -1
    }
}

/// IndexAny returns the index of the first instance of any Unicode code point from chars in s, or -1 if no Unicode code point from chars is present in s.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 字符串chars中的任一utf-8码值在s中第一次出现的位置，如果不存在或者chars为空字符串则返回-1。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!(strings::IndexAny("chicken", "aeiouy"),2);
/// assert_eq!(strings::IndexAny("crwth", "aeiouy"),-1);
///
/// ```
pub fn IndexAny(s: impl AsRef<str>, chars: impl AsRef<str>) -> int {
    if chars.as_ref() == "" || s.as_ref() == "" {
        return -1;
    }
    for (idx, r) in s.as_ref().chars().enumerate() {
        if IndexRune(chars.as_ref(), r as u32) != -1 {
            return int!(idx);
        }
    }

    -1
}

/// IndexByte returns the index of the first instance of c in s, or -1 if c is not present in s.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 字符c在s中第一次出现的位置，不存在则返回-1。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!(0,strings::IndexByte("rustlang",b'r'));
/// assert_eq!(3,strings::IndexByte("gophers",b'h'));
/// assert_eq!(-1,strings::IndexByte("gophers".to_string(),b'x'));
///
/// ```
pub fn IndexByte(s: impl AsRef<str>, c: byte) -> int {
    for (i, v) in s.as_ref().bytes().enumerate() {
        if v == c {
            return int!(i);
        }
    }
    -1
}

/// IndexFunc returns the index into s of the first Unicode code point satisfying f(c), or -1 if none do.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// IndexFunc将满足f（rune）的第一个Unicode代码点的索引返回到s，如果没有，则返回-1。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
///    /* fn f(c: u32) -> bool {
///        let s = char::from_u32(c).unwrap();
///        !s.is_ascii()
///    } */
///    // 用上面注释掉的函数也可以，用下面的匿名函数也可以。
///    let f = |c: u32| {
///        let s = char::from_u32(c).unwrap();
///        !s.is_ascii()
///    };
///    assert_eq!(7, strings::IndexFunc("Hello, 世界", f));
///    assert_eq!(-1, strings::IndexFunc("Hello, world", f));
///
/// ```
pub fn IndexFunc(s: impl AsRef<str>, f: fn(rune) -> bool) -> int {
    for (i, r) in s.as_ref().chars().enumerate() {
        if f(r as u32) == true {
            return int!(i);
        }
    }
    -1
}

/// IndexRune returns the index of the first instance of the Unicode code point r, or -1 if rune is not present in s. If r is utf8.RuneError, it returns the first instance of any invalid UTF-8 byte sequence.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// unicode码值r在s中第一次出现的位置，不存在则返回-1。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!(4,strings::IndexRune("chicken", 'k' as u32));
/// assert_eq!(4,strings::IndexRune("chicken", 0x6b));
/// assert_eq!(4,strings::IndexRune("chicken", 107_u32));
/// assert_eq!(-1,strings::IndexRune("chicken", 'd' as u32));
/// assert_eq!(-1,strings::IndexRune("chicken", 0x64));
/// assert_eq!(-1,strings::IndexRune("chicken", 100_u32));
///
/// ```
pub fn IndexRune(s: impl AsRef<str>, r: rune) -> int {
    if let Some(i) = s.as_ref().find(|c: char| c == char::from_u32(r).unwrap()) {
        return int!(i);
    }
    -1
}

/// Join concatenates the elements of its first argument to create a single string. The separator string sep is placed between elements in the resulting string.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 将一系列字符串连接为一个字符串，之间用sep来分隔。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// let s = vec!["foo", "bar", "baz"];
///
/// assert_eq!("foo, bar, baz",strings::Join(s,", "));
///
/// ```
pub fn Join(elems: Vec<impl AsRef<str>>, sep: impl AsRef<str>) -> String {
    match len!(elems) {
        0 => return "".to_owned(),
        1 => return elems[0].as_ref().to_string(),
        _ => (),
    }

    let mut n = len!(sep.as_ref()) * (len!(elems) - 1);
    for i in 0..len!(elems) {
        n += len!(elems[i].as_ref());
    }

    let mut b = Builder::new();

    b.Grow(int!(n));
    b.WriteString(elems[0].as_ref());
    for s in elems.get(1..).unwrap() {
        b.WriteString(sep.as_ref());
        b.WriteString(s.as_ref());
    }
    b.String()
}

/// LastIndex returns the index of the last instance of substr in s, or -1 if substr is not present in s.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 子串substr在字符串s中最后一次出现的位置，不存在则返回-1。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!(0,strings::Index("rust rustacean","rust"));
/// assert_eq!(5,strings::LastIndex("rust rustacean","rust"));
/// assert_eq!(-1,strings::LastIndex("rust rustacean","go"));
///
/// ```
pub fn LastIndex(s: impl AsRef<str>, substr: impl AsRef<str>) -> int {
    if substr.as_ref() == "" {
        return -1;
    }
    if let Some(i) = s.as_ref().rfind(substr.as_ref()) {
        return int!(i);
    }
    -1
}

/// LastIndexAny returns the index of the last instance of any Unicode code point from chars in s, or -1 if no Unicode code point from chars is present in s.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 字符串chars中的任一utf-8码值在s中最后一次出现的位置，如不存在或者chars为空字符串则返回-1。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!(4,strings::LastIndexAny("go gopher", "go"));
/// assert_eq!(8,strings::LastIndexAny("go gopher", "ordent"));
/// assert_eq!(-1,strings::LastIndexAny("go gopher", "fail"));
///
/// ```
pub fn LastIndexAny(s: impl AsRef<str>, chars: impl AsRef<str>) -> int {
    let max = len!(s.as_ref()) - 1;
    if chars.as_ref() == "" {
        return -1;
    }
    for (rindex, r) in s.as_ref().chars().rev().enumerate() {
        if chars.as_ref().contains(r) {
            return int!(max - rindex);
        }
    }
    -1
}

/// LastIndexByte returns the index of the last instance of c in s, or -1 if c is not present in s.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 字符c在s中最后一次出现的位置，如不存在返回-1。

/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!(10,strings::LastIndexByte("Hello, world", b'l'));
/// assert_eq!(8,strings::LastIndexByte("Hello, world", b'o'));
/// assert_eq!(-1,strings::LastIndexByte("Hello, world", b'x'));
/// ```
pub fn LastIndexByte(s: impl AsRef<str>, c: byte) -> int {
    if let Some(i) = s.as_ref().rfind(c as char) {
        return int!(i);
    }
    -1
}

/// LastIndexFunc returns the index into s of the last Unicode code point satisfying f(c), or -1 if none do.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// s中最后一个满足函数f的unicode码值的位置i，不存在则返回-1。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
///    let f = |x: u32| char::from_u32(x).unwrap().is_ascii_digit();
///
///    assert_eq!(5, strings::LastIndexFunc("go 123", f));
///    assert_eq!(2, strings::LastIndexFunc("123 go", f));
///    assert_eq!(-1, strings::LastIndexFunc("go", f));
/// ```
pub fn LastIndexFunc(s: impl AsRef<str>, f: fn(rune) -> bool) -> int {
    let max = len!(s.as_ref()) - 1;
    for (rindex, r) in s.as_ref().chars().rev().enumerate() {
        if f(r as u32) == true {
            return int!(max - rindex);
        }
    }
    -1
}

/// Map returns a copy of the string s with all its characters modified according to the mapping function. If mapping returns a negative value, the character is dropped from the string with no replacement.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 将s的每一个unicode码值r都替换为mapping(r)，返回这些新码值组成的字符串拷贝。如果mapping返回一个负值，将会丢弃该码值而不会被替换。（返回值中对应位置将没有码值）
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
///    let rot13 = |r: u32| -> u32 {
///        if r >= 'A' as u32 && r < 'Z' as u32 {
///            return 'A' as u32 + (r - 'A' as u32 + 13) % 26;
///        }
///        if r >= 'a' as u32 && r <= 'z' as u32 {
///            return 'a' as u32 + (r - 'a' as u32 + 13) % 26;
///        }
///        r
///    };
///    let s = "'Twas brillig and the slithy gopher...";
///    assert_eq!(
///        "'Gjnf oevyyvt naq gur fyvgul tbcure...",
///        strings::Map(rot13, s)
///    );
///
/// ```
pub fn Map(mapping: fn(rune) -> rune, mut s: impl AsRef<str>) -> String {
    let mut b = Builder::new();
    b.Grow(int!(len!(s.as_ref())));
    for (idx, v) in s.as_ref().chars().enumerate() {
        let r = mapping(v as u32);
        if r > 0 {
            b.WriteRune(r);
        } else {
            b.WriteRune(v as u32);
        }
    }
    b.String()
}

/// Repeat returns a new string consisting of count copies of the string s.
///
/// It panics if count is negative or if the result of (len!(s) * count) overflows.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回count个s串联的字符串。
/// </details>
///
/// # Example
///
/// ```
///  use gostd_strings as strings;
///
///  println!("{}",strings::Repeat("12",3));
///
/// ```
/// # Output
///
/// ```text
/// 121212
/// ```
pub fn Repeat(s: impl AsRef<str>, count: uint) -> String {
    if count == 0 {
        return "".to_owned();
    }

    if len!(s.as_ref()) * count / count != len!(s.as_ref()) {
        panic!("strings: Repeat count causes overflow")
    }
    let mut n = len!(s.as_ref()) * count;
    let mut b = Builder::new();
    b.Grow(int!(n));
    b.WriteString(s.as_ref());
    while b.Len() < int!(n) {
        if b.Len() <= int!(n) / 2 {
            b.WriteString(b.String().as_str());
        } else {
            b.WriteString(b.String().get(..(n - b.Len() as usize)).unwrap());
            break;
        }
    }
    b.String()
}

/// Replace returns a copy of the string s with the first n non-overlapping instances of old replaced by new. If old is empty, it matches at the beginning of the string and after each UTF-8 sequence, yielding up to k+1 replacements for a k-rune string. If n < 0, there is no limit on the number of replacements.
/// It panics if count is negative or if the result of (len!(s) * count) overflows.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将s中前n个不重叠old子串都替换为new的新字符串，如果n<0会替换所有old子串
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!("oinky oinky oink",strings::Replace("oink oink oink", "k", "ky", 2));
/// assert_eq!("moo moo moo",strings::Replace("oink oink oink", "oink", "moo", -1));
///
/// ```
pub fn Replace(s: impl AsRef<str>, old: impl AsRef<str>, new: impl AsRef<str>, n: int) -> String {
    if n < 0 {
        return s.as_ref().replace(old.as_ref(), new.as_ref());
    }
    s.as_ref().replacen(old.as_ref(), new.as_ref(), uint!(n))
}

/// ReplaceAll returns a copy of the string s with all non-overlapping instances of old replaced by new. If old is empty, it matches at the beginning of the string and after each UTF-8 sequence, yielding up to k+1 replacements for a k-rune string.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将s中不重叠old子串都替换为new的新字符串。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!("moo moo moo",strings::ReplaceAll("oink oink oink", "oink", "moo"));
///
/// ```
pub fn ReplaceAll(s: impl AsRef<str>, old: impl AsRef<str>, new: impl AsRef<str>) -> String {
    s.as_ref().replace(old.as_ref(), new.as_ref())
}

/// Split slices s into all substrings separated by sep and returns a slice of the substrings between those separators.
///
/// If s does not contain sep and sep is not empty, Split returns a slice of length 1 whose only element is s.
///
/// If sep is empty, Split splits after each UTF-8 sequence. If both s and sep are empty, Split returns an empty slice.
///
/// It is equivalent to SplitN with a count of -1.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 用去掉s中出现的sep的方式进行分割，会分割到结尾，并返回生成的所有片段组成的切片（每一个sep都会进行一次切割，即使两个sep相邻，也会进行两次切割）。如果sep为空字符，Split会将s切分成每一个unicode码值一个字符串。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!(vec!["a","b","c"],strings::Split("a,b,c", ","));
/// assert_eq!(vec!["", "man ", "plan ", "canal panama"],strings::Split("a man a plan a canal panama", "a "));
/// assert_eq!(vec![""," ", "x", "y", "z", " ",""],strings::Split(" xyz ", ""));
/// assert_eq!(vec![""],strings::Split("", "Bernardo O'Higgins"));
/// ```
pub fn Split(s: &str, sep: impl AsRef<str>) -> Vec<&str> {
    s.split(sep.as_ref()).collect()
}

/// SplitAfter slices s into all substrings after each instance of sep and returns a slice of those substrings.
///
/// If s does not contain sep and sep is not empty, SplitAfter returns a slice of length 1 whose only element is s.
///
/// If sep is empty, SplitAfter splits after each UTF-8 sequence. If both s and sep are empty, SplitAfter returns an empty slice.
///
/// It is equivalent to SplitAfterN with a count of -1.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 用从s中出现的sep后面切断的方式进行分割，会分割到结尾，并返回生成的所有片段组成的切片（每一个sep都会进行一次切割，即使两个sep相邻，也会进行两次切割）。如果sep为空字符，Split会将s切分成每一个unicode码值一个字符串。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!(vec!["a,", "b,", "c"],strings::SplitAfter("a,b,c", ","));
/// ```
pub fn SplitAfter(s: &str, sep: impl AsRef<str>) -> Vec<&str> {
    s.split_inclusive(sep.as_ref()).collect()
}

/// SplitAfterN slices s into substrings after each instance of sep and returns a slice of those substrings.
///
/// The count determines the number of substrings to return:
/// ```text
/// n > 0: at most n substrings; the last substring will be the unsplit remainder.
/// n == 0: the result is [] (zero substrings)
/// n < 0: all substrings
/// ```
/// Edge cases for s and sep (for example, empty strings) are handled as described in the documentation for SplitAfter.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 用从s中出现的sep后面切断的方式进行分割，会分割到结尾，并返回生成的所有片段组成的切片（每一个sep都会进行一次切割，即使两个sep相邻，也会进行两次切割）。如果sep为空字符，Split会将s切分成每一个unicode码值一个字符串。
///
/// 参数n决定返回的切片的数目：
///
/// ```text
///
/// n > 0 : 返回的切片最多n个子字符串；最后一个子字符串包含未进行切割的部分(当n大于最大子串数量，也只返回最大值)。
/// n == 0: 返回[]
/// n < 0 : 返回所有的子字符
///
/// ```
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
/// // n == 0 返回 []
///    assert_eq!(Vec::<String>::new(), strings::SplitAfterN(",a,b2,c", ",", 0));
///    assert_eq!(vec![",a,b2,c"], strings::SplitAfterN(",a,b2,c", ",", 1));
///    assert_eq!(vec![",", "a,", "b2,", "c"],strings::SplitAfterN(",a,b2,c", ",", -1));
///    assert_eq!(vec![",", "a,b2,c"], strings::SplitAfterN(",a,b2,c", ",", 2));
///    assert_eq!(vec![",", "a,", "b2,c"],strings::SplitAfterN(",a,b2,c", ",", 3));
///    assert_eq!(vec![",", "a,", "b2,", "c"],strings::SplitAfterN(",a,b2,c", ",", 4));
/// // 当n大于最大子串数量，也只返回最大值。
///    assert_eq!(vec![",", "a,", "b2,", "c"],strings::SplitAfterN(",a,b2,c", ",", 5));
///    assert_eq!(vec![",", "a,", "b2,", "c"],strings::SplitAfterN(",a,b2,c", ",", 10));
/// ```
pub fn SplitAfterN(s: impl AsRef<str>, sep: impl AsRef<str>, n: int) -> Vec<String> {
    let mut n = n;
    let length = len!(s.as_ref());
    if n < 0 || uint!(n) > length - 1 {
        return s
            .as_ref()
            .split_inclusive(sep.as_ref())
            .map(|x| x.to_string())
            .collect();
    }

    if n == 0 {
        return vec![];
    }

    if n == 1 {
        return vec![s.as_ref().to_string()];
    }

    let mut list: Vec<String> = s
        .as_ref()
        .splitn(n as usize, sep.as_ref())
        .map(|x| x.to_string())
        .collect();

    let list_len = list.len();
    if n > int!(list_len - 1) {
        n = int!(list_len - 1);
    }
    for i in 0..n as usize {
        list.get_mut(i).unwrap().push_str(sep.as_ref());
    }
    list
}

/// SplitN slices s into substrings separated by sep and returns a slice of the substrings between those separators.
///
/// The count determines the number of substrings to return:
/// ```text
/// n > 0: at most n substrings; the last substring will be the unsplit remainder.
/// n == 0: the result is nil (zero substrings)
/// n < 0: all substrings
/// ```
/// Edge cases for s and sep (for example, empty strings) are handled as described in the documentation for Split.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 用去掉s中出现的sep的方式进行分割，会分割到结尾，并返回生成的所有片段组成的切片（每一个sep都会进行一次切割，即使两个sep相邻，也会进行两次切割）。如果sep为空字符，Split会将s切分成每一个unicode码值一个字符串。参数n决定返回的切片的数目：
///
/// ```text
/// n > 0 : 返回的切片最多n个子字符串；最后一个子字符串包含未进行切割的部分。
/// n == 0: 返回[]
/// n < 0 : 返回所有的子字符串组成的切片
/// ```
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
///    assert_eq!(Vec::<String>::new(), strings::SplitN(",a,b2,c", ",", 0));
///    assert_eq!(vec![",a,b2,c"], strings::SplitN(",a,b2,c", ",", 1));
///    assert_eq!(
///        vec!["", "a", "b2", "c"],
///        strings::SplitN(",a,b2,c", ",", -1)
///    );
///    assert_eq!(vec!["", "a,b2,c"], strings::SplitN(",a,b2,c", ",", 2));
///    assert_eq!(vec!["", "a", "b2,c"], strings::SplitN(",a,b2,c", ",", 3));
///    assert_eq!(
///        vec!["", "a", "b2", "c"],
///        strings::SplitN(",a,b2,c", ",", 4)
///    );
///    assert_eq!(
///        vec!["", "a", "b2", "c"],
///        strings::SplitN(",a,b2,c", ",", 5)
///    );
///    assert_eq!(
///        vec!["", "a", "b2", "c"],
///        strings::SplitN(",a,b2,c", ",", 10)
///    );
///
/// ```
pub fn SplitN<'a>(s: &'a str, sep: impl AsRef<str>, n: int) -> Vec<&'a str> {
    let mut n = n;
    if n == 0 {
        return vec![];
    }
    if n < 0 {
        return s.split(sep.as_ref()).collect();
    }

    let list: Vec<&'a str> = s.split(sep.as_ref()).collect();
    let length = list.len();
    if n > int!(length) {
        n = int!(length);
    }
    s.splitn(n as usize, sep.as_ref()).collect()
}

/// ToLower returns s with all Unicode letters mapped to their lower case.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将所有字母都转为对应的小写版本的拷贝。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!("gopher",strings::ToLower("Gopher"));
/// ```
pub fn ToLower(s: impl AsRef<str>) -> String {
    s.as_ref().to_lowercase()
}

/// ToTitle returns a copy of the string s with all Unicode letters mapped to their Unicode title case.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将所有字母都转为对应的标题版本的拷贝。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!("LOUD NOISES",strings::ToTitle("loud noises"));
/// assert_eq!("ХЛЕБ",strings::ToTitle("хлеб"));
/// ```
pub fn ToTitle(s: impl AsRef<str>) -> String {
    s.as_ref().to_uppercase()
}

/// ToUpper returns s with all Unicode letters mapped to their upper case.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将所有字母都转为对应的大写版本的拷贝。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!("GOPHER",strings::ToUpper("Gopher"));
/// ```
pub fn ToUpper(s: impl AsRef<str>) -> String {
    s.as_ref().to_uppercase()
}

/// Trim returns a slice of the string s with all leading and trailing Unicode code points contained in cutset removed.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将s前后端所有cutset包含的utf-8码值都去掉的字符串。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!("Hello, 中国",strings::Trim("¡¡¡Hello, 中国!!!", "!¡"));
///
/// ```
pub fn Trim(mut s: &str, cutset: impl AsRef<str>) -> &str {
    s.trim_matches(|x| cutset.as_ref().contains(x))
}

/// TrimFunc returns a slice of the string s with all leading and trailing Unicode code points c satisfying f(c) removed.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// let f = |x| x >= '1' as u32 && x <= '9' as u32;
/// assert_eq!("Hello, Rust",strings::TrimFunc("2211345Hello, Rust1122345", f));
/// ```
pub fn TrimFunc(s: &str, f: fn(rune) -> bool) -> &str {
    s.trim_matches(|x| f(x as rune))
}

/// TrimLeft returns a slice of the string s with all leading Unicode code points contained in cutset removed.
///
/// To remove a prefix, use TrimPrefix instead.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将s前端所有cutset包含的utf-8码值都去掉的字符串。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!("Hello, Gophers!!!",strings::TrimLeft("¡¡¡Hello, Gophers!!!", "!¡"))
/// ```
pub fn TrimLeft<'a>(s: &'a str, cutset: impl AsRef<str>) -> &'a str {
    s.trim_start_matches(|x| cutset.as_ref().contains(x))
}

/// TrimLeftFunc returns a slice of the string s with all leading Unicode code points c satisfying f(c) removed.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将s前端所有满足f的unicode码值都去掉的字符串
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// let f = |x| x >= '1' as u32 && x <= '9' as u32;
/// assert_eq!("Hello, Rust654321",strings::TrimLeftFunc("123456Hello, Rust654321", f));
///
///
/// ```
pub fn TrimLeftFunc(s: &str, f: fn(rune) -> bool) -> &str {
    s.trim_start_matches(|x| f(x as rune))
}

/// TrimPrefix returns s without the provided leading prefix string. If s doesn't start with prefix, s is returned unchanged.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回去除s可能的前缀prefix的字符串。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!("Hello, Rust!xxx",strings::TrimPrefix("xxxHello, Rust!xxx","xxx"));
///
/// ```
pub fn TrimPrefix<'a>(s: &'a str, prefix: impl AsRef<str>) -> &'a str {
    s.trim_start_matches(prefix.as_ref())
}

/// TrimRight returns a slice of the string s, with all trailing Unicode code points contained in cutset removed.
///
/// To remove a suffix, use TrimSuffix instead.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将s后端所有cutset包含的utf-8码值都去掉的字符串。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
/// assert_eq!("¡¡¡Hello, Gophers",strings::TrimRight("¡¡¡Hello, Gophers!!!", "!¡"));
///
/// ```
pub fn TrimRight<'a>(s: &'a str, cutset: impl AsRef<str>) -> &'a str {
    s.trim_end_matches(|x| cutset.as_ref().contains(x))
}

/// TrimRightFunc returns a slice of the string s with all trailing Unicode code points c satisfying f(c) removed.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将s后端所有满足f的unicode码值都去掉的字符串。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// let f = |x| x >= '1' as u32 && x <= '9' as u32;
/// assert_eq!("123456Hello, Rust",strings::TrimRightFunc("123456Hello, Rust654321", f));
///
/// ```
pub fn TrimRightFunc(s: &str, f: fn(rune) -> bool) -> &str {
    s.trim_end_matches(|x| f(x as rune))
}

/// TrimSpace returns a slice of the string s, with all leading and trailing white space removed, as defined by Unicode.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将s前后端所有空白（is_whitespace()指定）都去掉的字符串。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!("Hello, Rust!",strings::TrimSpace("  Hello, Rust!  "));
/// assert_eq!("Hello, Rust!",strings::TrimSpace("\nHello, Rust! \t "));
/// assert_eq!("Hello, Rust!",strings::TrimSpace("\n\t Hello, Rust! \t\r "));
///
/// ```
pub fn TrimSpace(s: &str) -> &str {
    s.trim()
}

/// TrimSuffix returns s without the provided trailing suffix string. If s doesn't end with suffix, s is returned unchanged.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回去除s可能的后缀suffix的字符串。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_strings as strings;
///
/// assert_eq!("test",strings::TrimSuffix("test.rs",".rs"))
///
/// ```
pub fn TrimSuffix<'a>(s: &'a str, suffix: impl AsRef<str>) -> &'a str {
    s.trim_end_matches(suffix.as_ref())
}

// Builder Begin

/// A Builder is used to efficiently build a string using Write methods. It minimizes memory copying. The zero value is ready to use. Do not copy a non-zero Builder.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Builder 生成器用于使用写方法高效地构建字符串。它最大限度地减少了内存复制。零值已准备好使用。不要复制非零生成器。
/// </details>
///
/// # Example
///
/// ```
///  use gostd_io::*;
///  use gostd_strings::Builder;
///
///  let mut buf = Builder::new();
///  buf.WriteString("hello");
///  buf.WriteByte(b' ');
///  buf.WriteString("world");
///  buf.WriteByte(b'!');
///
///  assert_eq!("hello world!", buf.String());
///
///  buf.Reset(); // clear 清空数据
///  for i in 'a'..='z' {
///   buf.WriteByte(i as u8);
///  }
///  assert_eq!("abcdefghijklmnopqrstuvwxyz", buf.String());
///
///
/// ```
#[derive(Default, PartialEq, PartialOrd, Debug, Clone, Fmt)]
pub struct Builder {
    addr: Box<Option<Builder>>,
    buf: Vec<byte>,
}

impl Builder {
    /// initialization a Builder
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 初始化生成器
    /// </details>
    pub fn new() -> Builder {
        let mut b = Builder::default();
        b.addr = Box::new(Some(b.clone()));
        b
    }

    /// Cap returns the capacity of the builder's underlying byte slice. It is the total space allocated for the string being built and includes any bytes already written.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Cap返回构建器底层字节切片的容量。它是为正在生成的字符串分配的总空间，包括已写入的所有字节。
    /// </details>
    pub fn Cap(&self) -> int {
        int!(self.buf.capacity())
    }

    // grow copies the buffer to a new, larger buffer so that there are at least n
    // bytes of capacity beyond len(b.buf).
    fn grow(&mut self, n: int) {
        let mut buf: Vec<byte> = Vec::with_capacity(2 * self.buf.capacity() + uint!(n));
        buf.extend_from_slice(self.buf.as_slice());
        self.buf = buf
    }
    /// Grow grows b's capacity, if necessary, to guarantee space for another n bytes. After Grow(n), at least n bytes can be written to b without another allocation. If n is negative, Grow panics.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 如果需要的话，Grow会增加b的容量，以保证另一个n字节的空间。在Grow（n）之后，至少可以将n个字节写入b而无需另一次分配。如果n为负，则增加恐慌。
    /// </details>
    pub fn Grow(&mut self, n: int) {
        // self.copyCheck();
        if n < 0 {
            panic!("strings.Builder.Grow: negative count")
        }
        if self.buf.capacity() - len!(self.buf) < uint!(n.abs()) {
            self.grow(n)
        }
    }

    /// Len returns the number of accumulated bytes; b.Len() == len!(b.String()).
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Len返回累计字节数；b.Len() == len!(b.String()).
    /// </details>
    pub fn Len(&self) -> int {
        int!(len!(self.buf))
    }

    /// Reset resets the Builder to be empty.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 重置将生成器重置为空。
    /// </details>
    pub fn Reset(&mut self) {
        self.addr = Box::new(None);
        self.buf.clear()
    }
    /// String returns the accumulated string.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 返回累积的字节序列
    /// </details>
    pub fn Bytes(self) -> Vec<byte> {
        self.buf.clone()
    }

    /// String returns the accumulated string.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 返回累积的字符串。
    /// </details>
    pub fn String(&self) -> String {
        String::from_utf8(self.buf.clone()).unwrap()
    }

    /// WriteRune appends the UTF-8 encoding of Unicode code point r to b's buffer. It returns the length of r.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 将Unicode代码点r的UTF-8编码附加到b的缓冲区。它返回r的长度。
    /// </details>
    pub fn WriteRune<'a>(&mut self, r: rune) -> Result<int, &'a str> {
        if uint32!(r) < utf8::RuneSelf {
            self.buf.push(byte!(r));
            return Ok(1);
        }

        let l = len!(self.buf);
        if self.buf.capacity() - l < utf8::UTFMax {
            self.grow(utf8::UTFMax as int);
        }
        let n = utf8::EncodeRune(self.buf.get(l..l + utf8::UTFMax).unwrap().to_vec(), r);
        self.buf = self.buf.get(..l + (n.abs() as uint)).unwrap().to_vec();
        return Ok(n);
    }
}

use crate::io::ByteWriter;
impl ByteWriter for Builder {
    /// WriteByte appends the byte c to b's buffer.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// WriteByte将字节c追加到b的缓冲区。
    /// </details>
    fn WriteByte(&mut self, c: byte) -> Result<(), Error> {
        self.buf.push(c);
        Ok(())
    }
}

use crate::io::StringWriter;
impl StringWriter for Builder {
    /// WriteString appends the contents of s to b's buffer. It returns the length of s.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// WriteString将s的内容附加到b的缓冲区。它返回s的长度。
    /// </details>
    fn WriteString(&mut self, s: &str) -> Result<int, Error> {
        self.buf.append(s.as_bytes().to_vec().as_mut());
        return Ok(len!(self.buf) as int);
    }
}

use crate::io::Writer;
impl Writer for Builder {
    /// Write appends the contents of p to b's buffer. Write always returns len!(p).
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Write将p的内容附加到b的缓冲区, 写总是返回len!(p).
    /// </details>
    fn Write<'a>(&mut self, p: Vec<byte>) -> Result<int, Error> {
        self.buf.extend_from_slice(p.as_slice());
        Ok(int!(len!(p)))
    }
}

// Builder End
/// A Reader implements the io.Reader, io.ReaderAt, io.ByteReader, io.ByteScanner,
/// io.RuneReader, io.RuneScanner, io.Seeker, and io.WriterTo interfaces by reading
/// from a string.
/// The zero value for Reader operates like a Reader of an empty string.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
#[derive(Default, PartialEq, PartialOrd, Debug, Clone)]
pub struct Reader {
    s: String,
    i: int64,      // current reading index
    prevRune: int, // index of previous rune; or < 0
}

impl Reader {
    /// new returns a new Reader reading from s.  more efficient and read-only.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// NewReader创建一个从s读取数据的Reader。更有效率，且为只读的。
    /// </details>
    pub fn new(s: &str) -> Reader {
        Reader {
            s: s.into(),
            i: 0,
            prevRune: -1,
        }
    }

    /// Len returns the number of bytes of the unread portion of the string.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Len返回self包含的字符串还没有被读取的部分。
    /// </details>
    pub fn Len(&self) -> int {
        if self.i >= int64!(len!(self.s)) {
            return 0;
        }

        int!(int64!(len!(self.s)) - self.i)
    }

    ///  Reset resets the Reader to be reading from s.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 重置将Reader重置为从s读取。
    /// </details>
    pub fn Reset(&mut self, s: &str) {
        self.s = s.into();
        self.i = 0;
        self.prevRune = -1;
    }

    /// Size returns the original length of the underlying string. Size is the number of bytes available for reading via ReadAt. The returned value is always the same and is not affected by calls to any other method.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Size返回基础字符串的原始长度。Size是可通过ReadAt读取的字节数。返回的值始终相同，并且不受对任何其他方法的调用的影响。
    /// </details>
    pub fn Size(&self) -> int64 {
        int64!(len!(self.s))
    }
}

use std::io::Error;
use std::io::ErrorKind;
impl io::Reader for Reader {
    /// Read implements the io.Reader interface.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    ///
    /// </details>
    fn Read(&mut self, mut b: Vec<byte>) -> Result<int, Error> {
        if self.i >= int64!(len!(self.s)) {
            return Err(Error::new(ErrorKind::UnexpectedEof, "EOF"));
        }
        self.prevRune = -1;
        let n = int!(len!(self.s.as_bytes()[uint!(self.i)..]));
        b.copy_from_slice(self.s.as_bytes()[uint!(self.i)..].as_ref());
        self.i += int64!(n);
        Ok(n)
    }
}

impl io::ReaderAt for Reader {
    fn ReadAt(&mut self, mut b: Vec<byte>, off: int64) -> Result<int, Error> {
        if off < 0 {
            return Err(Error::new(
                ErrorKind::Other,
                "strings.Reader.ReadAt: negative offset",
            ));
        }
        if off >= int64!(len!(self.s)) {
            return Err(Error::new(ErrorKind::UnexpectedEof, "EOF")); //todo io.EOF在rust中怎么表示
        }
        let n = len!(self.s.as_bytes()[uint!(off)..]);
        b.copy_from_slice(self.s.as_bytes()[uint!(off)..].as_ref());
        if n < len!(b) {
            return Err(Error::new(ErrorKind::UnexpectedEof, "EOF"));
        }
        Ok(int!(n))
    }
}

impl io::ByteReader for Reader {
    fn ReadByte(&mut self) -> Result<byte, Error> {
        self.prevRune = -1;
        if self.i >= int64!(len!(self.s)) {
            return Err(Error::new(ErrorKind::UnexpectedEof, "EOF"));
        }
        let b = self.s.as_bytes()[uint!(self.i)];
        self.i += 1;
        Ok(b)
    }
}

impl io::RuneReader for Reader {
    fn ReadRune(&mut self) -> Result<(rune, int), Error> {
        if self.i >= int64!(len!(self.s)) {
            self.prevRune = -1;
            return Err(Error::new(ErrorKind::UnexpectedEof, "EOF"));
        }
        self.prevRune = self.i as int;
        if let Some(c) = self.s.chars().nth(uint!(self.i)) {
            if rune!(c) < utf8::RuneSelf {
                self.i += 1;
                return Ok((rune!(c), 1));
            }
            let size = c.len_utf8() as int;
            self.i += 1;
            return Ok((c as rune, size as isize));
        } else {
            self.prevRune = -1;
            return Err(Error::new(ErrorKind::UnexpectedEof, "EOF"));
        }
    }
}

use io::Whence;
impl io::Seeker for Reader {
    fn Seek(&mut self, offset: int64, whence: Whence) -> int64 {
        self.prevRune = -1;
        let abs: int64;
        match whence {
            Whence::SeekStat => abs = offset,
            Whence::SeekCurrent => abs = self.i + offset,
            Whence::SeekEnd => abs = int64!(len!(self.s)) + offset,
        }
        self.i = abs;
        abs
    }
}

impl io::ByteScanner for Reader {
    fn UnreadByte(&mut self) -> Result<(), Error> {
        if self.i <= 0 {
            return Err(Error::new(
                ErrorKind::Other,
                "strings.Reader.UnreadByte: at beginning of string",
            ));
        }
        self.prevRune = -1;
        self.i -= 1;
        Ok(())
    }
}

impl io::WriterTo for Reader {
    fn WriteTo(&mut self, w: Box<dyn io::Writer>) -> Result<int64, Error> {
        self.prevRune = -1;
        if self.i >= int64!(len!(self.s)) {
            return Ok(0);
        }
        let s = self.s.get(self.i as usize..).unwrap();
        if let Ok(m) = io::WriteString(w, s) {
            if m > int!(len!(s)) {
                panic!("gostd_strings::Reader::WriteTo: invalid WriteString count")
            }
            if m != int!(len!(s)) {
                return Err(Error::new(ErrorKind::Other, "short write"));
            }
            self.i += int64!(m);
            let n = int64!(m);
            return Ok(n);
        } else {
            return Err(Error::new(ErrorKind::Other, "short write"));
        }
    }
}

/// Replacer replaces a list of strings with replacements. It is safe for concurrent use by multiple goroutines.
///
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Replacer类型进行一系列字符串的替换。
/// </details>
pub struct Replacer<'a> {
    oldnew: Vec<(&'a str, &'a str)>,
}

impl<'a> Replacer<'a> {
    /// new returns a new Replacer from a list of old, new string pairs. Replacements are performed in the order they appear in the target string, without overlapping matches. The old string comparisons are done in argument order.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 使用提供的多组old、new字符串对创建并返回一个*Replacer。替换是依次进行的，匹配时不会重叠。
    /// </details>
    ///
    /// # Example
    ///
    /// ```
    /// use gostd_strings as strings;
    ///
    ///    let p = vec![("<", "&lt;"), (">", "&gt;")];
    ///    let r = strings::Replacer::new(p);
    ///    let s = r.Replace("This is <b>HTML</b>!");
    ///    println!("{}", s);
    ///
    /// ```
    /// # Output
    ///
    /// ```text
    /// This is &lt;b&gt;HTML&lt;/b&gt;!
    /// ```
    pub fn new(pairs: Vec<(&'a str, &'a str)>) -> Replacer<'a> {
        Replacer { oldnew: pairs }
    }
    /// Replace returns a copy of s with all replacements performed.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Replace返回s的所有替换进行完后的拷贝.
    /// </details>
    pub fn Replace(self, s: &str) -> String {
        let mut new_str = s.to_owned();
        for pair in self.oldnew.clone() {
            new_str = ReplaceAll(new_str.as_str(), pair.0, pair.1);
        }
        new_str
    }
    /// WriteString writes s to w with all replacements performed.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// WriteString向w中写入s的所有替换进行完后的拷贝
    /// </details>
    pub fn WriteString(&self, mut w: Box<dyn io::Writer>, s: &str) -> Result<int, Error> {
        w.Write(s.as_bytes().to_vec())
    }
}
