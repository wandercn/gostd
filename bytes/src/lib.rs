//! Package bytes implements functions for the manipulation of byte slices. It is analogous to the facilities of the strings package.
//!
//! <details class="rustdoc-toggle top-doc">
//! <summary class="docblock">zh-cn</summary>
//! bytes包实现了操作[]byte的常用函数。本包的函数和strings包的函数相当类似。
//! </details>
// #![allow(unused_assignments)]
#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// #[cfg(test)]
// mod tests;
#[macro_use]
use gostd_derive::Fmt;
use gostd_builtin::*;
use gostd_io as io;
use gostd_unicode::utf8;
use io::*;
/// Compare returns an integer comparing two byte slices lexicographically. The result will be 0 if a==b, -1 if a < b, and +1 if a > b.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Compare函数返回一个整数表示两个&[byte]切片按字典序比较的结果（类同C的strcmp）。如果a == b返回0；如果a < b返回-1；否则返回+1。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
///    assert_eq!(-1, bytes::Compare(&[b'a'], &[b'b']));
///    assert_eq!(0, bytes::Compare(b"a", [b'a']));
///    assert_eq!(1, bytes::Compare(b"b", b"a"));
///
/// ```
pub fn Compare(a: impl AsRef<[byte]>, b: impl AsRef<[byte]>) -> int {
    if a.as_ref() == b.as_ref() {
        return 0;
    }
    if a.as_ref() < b.as_ref() {
        return -1;
    }
    1
}
/// Contains reports whether subslice is within b.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 判断切片b是否包含子切片subslice。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// assert_eq!(true,bytes::Contains(b"seafood", b"foo"));
/// assert_eq!(false,bytes::Contains(b"seafood", b"bar"));
/// assert_eq!(true,bytes::Contains(b"seafood", b""));
/// assert_eq!(true,bytes::Contains(b"", b""));
/// ```
pub fn Contains(b: impl AsRef<[byte]>, subslice: impl AsRef<[byte]>) -> bool {
    if subslice.as_ref().is_empty() {
        return true;
    }
    Index(b, subslice) != -1
}

/// ContainsAny reports whether any of the UTF-8-encoded code points in chars are within b.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 判断chars字符中的任何UTF-8编码代码点是否在b切片内。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
///    assert_eq!(false, bytes::ContainsAny("team", "i"));
///    assert_eq!(true, bytes::ContainsAny("failure", "u & i"));
///    assert_eq!(false, bytes::ContainsAny("foo", ""));
///    assert_eq!(false, bytes::ContainsAny("", ""));
///    assert_eq!(true, bytes::ContainsAny("你好,中国", "hello,好"));
///
/// ```
pub fn ContainsAny(b: impl AsRef<[byte]>, chars: impl AsRef<[byte]>) -> bool {
    for c in chars.as_ref() {
        if b.as_ref().contains(c) {
            return true;
        }
    }
    false
}

/// ContainsRune reports whether the rune is contained in the UTF-8-encoded byte slice b.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 判断字节切片b是否包含utf-8码值r
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// // '中' as rune = 20013 or 0x4e2d
/// assert_eq!(true, bytes::ContainsRune("hello中国!".as_bytes(), 20013));
/// assert_eq!(true, bytes::ContainsRune("hello中国!".to_string(), 0x4e2d));
/// assert_eq!(true, bytes::ContainsRune("hello中国!", 0x4e2d));
/// assert_eq!(false, bytes::ContainsRune("hello世界!", 0x4e2d));
/// ```
pub fn ContainsRune(b: impl AsRef<[byte]>, r: rune) -> bool {
    let c = char::from_u32(r).unwrap();
    let bytes = c.to_string();
    let mut is_contain = false;
    let mut buf: Vec<byte> = Vec::new();
    for v in bytes.bytes() {
        if b.as_ref().contains(&v) {
            is_contain = true;
            buf.push(v);
            if buf.as_slice() == bytes.as_bytes() {
                return is_contain;
            }
        } else {
            buf.clear();
            is_contain = false;
        }
    }
    is_contain
}

/// Count counts the number of non-overlapping instances of sep in s. If sep is an empty slice, Count returns 1 + the number of UTF-8-encoded code points in s.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回s字节切片中有几个不重复的sep子切片。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///     assert_eq!(3, bytes::Count("cheese".as_bytes(), "e".as_bytes()));
///     assert_eq!(5, bytes::Count("five".as_bytes(), "".as_bytes()));
///     assert_eq!(4, bytes::Count("台湾人香港人澳门人都是中国人".as_bytes(), "人".as_bytes()));
/// ```
pub fn Count(mut s: &[byte], sep: impl AsRef<[byte]>) -> int {
    if len!(sep.as_ref()) == 0 {
        return int!(s.len() as isize + 1);
    }

    if len!(sep.as_ref()) == 1 {
        let mut c: int = 0;
        let s1 = sep.as_ref()[0];
        for v in s.as_ref() {
            if v == &s1 {
                c += 1
            }
        }
        return c;
    }

    let mut n: int = 0;
    loop {
        let i = Index(s, sep.as_ref());
        if i == -1 {
            return n;
        }
        n += 1;
        s = &s[uint!(i) + len!(sep.as_ref())..];
    }
}

/// Fields interprets s as a sequence of UTF-8-encoded code points. It splits the slice s around each instance of one or more consecutive white space characters, as defined by unicode.IsSpace, returning a slice of subslices of s or an empty slice if s contains only white space.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 字段将s解释为UTF-8编码的代码点序列。它围绕由unicode定义的一个或多个连续空白字符的每个实例分割成子切片。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///  assert_eq!(vec!["foo".as_bytes(),"bar".as_bytes(),"baz".as_bytes()],bytes::Fields("  foo bar  baz   ".as_bytes()));
///  assert_eq!(
///     vec!["aaa".as_bytes(), "bbb".as_bytes(), "cccc".as_bytes(), "ddd".as_bytes()],
///     bytes::Fields("  \taaa bbb\t  cccc\r ddd  \r".as_bytes()));
/// ```
pub fn Fields(s: &[byte]) -> Vec<&[byte]> {
    let f = |c: u32| {
        let s = char::from_u32(c).unwrap();
        s.is_whitespace()
    };

    FieldsFunc(s.as_ref(), f)
}
/// FieldsFunc interprets s as a sequence of UTF-8-encoded code points. It splits the slice s at each run of code points c satisfying f(c) and returns a slice of subslices of s. If all code points in s satisfy f(c), or len(s) == 0, an empty slice is returned.
///
/// FieldsFunc makes no guarantees about the order in which it calls f(c) and assumes that f always returns the same value for a given c.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 类似Fields，但使用函数f来确定分割符（满足f的utf-8码值）。如果字符串全部是分隔符或者是空字符串的话，会返回空切片。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
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
///        vec!["foo1".as_bytes(), "bar2".as_bytes(), "baz3".as_bytes()],
///        bytes::FieldsFunc("  foo1;bar2,baz3...".as_bytes(), f)
///    )
/// ```
pub fn FieldsFunc(s: &[byte], f: fn(rune) -> bool) -> Vec<&[byte]> {
    #[derive(Default, PartialEq, PartialOrd, Debug, Clone)]
    struct span {
        start: int,
        end: int,
    }
    let mut spans = Vec::with_capacity(32);
    let mut start = -1;
    for (end, rune) in s.iter().enumerate() {
        if f(rune.to_owned() as u32) {
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

    let mut a: Vec<&[byte]> = vec![];
    a.resize(len!(spans), "".as_bytes());
    for (i, span) in spans.iter().enumerate() {
        a[i] = &s[span.start as usize..span.end as usize];
    }
    a
}

/// HasPrefix tests whether the byte slice s begins with prefix.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 判断s是否有前缀切片prefix。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// assert_eq!(true,bytes::HasPrefix("Rustacean","Rust"));
/// assert_eq!(false,bytes::HasPrefix("Rustacean","c"));
/// assert_eq!(true,bytes::HasPrefix("Rustacean",""));
/// assert_eq!(true,bytes::HasPrefix("Gopher","Go"));
///
/// ```
pub fn HasPrefix(s: impl AsRef<[byte]>, prefix: impl AsRef<[byte]>) -> bool {
    s.as_ref().starts_with(prefix.as_ref())
}

/// HasSuffix tests whether the byte slice s ends with suffix.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 判断s是否有后缀切片suffix。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// assert_eq!(true,bytes::HasSuffix("Amirust","rust"));
/// assert_eq!(false,bytes::HasSuffix("Amirust","R"));
/// assert_eq!(false,bytes::HasSuffix("Amirust","Ami"));
/// assert_eq!(true,bytes::HasSuffix("Amirust",""));
/// ```
pub fn HasSuffix(s: impl AsRef<[byte]>, suffix: impl AsRef<[byte]>) -> bool {
    s.as_ref().ends_with(suffix.as_ref())
}

/// Cut slices s around the first instance of sep,
/// returning the text before and after sep.
/// The found result reports whether sep appears in s.
/// If sep does not appear in s, cut returns s, "", false.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 在s中的第一个分隔sep处切分子切片，返回sep前面部分子切片before和sep后面的部分切片after。found 值表示在s字符串中是否找到sep字节切片。如果seq在s中找不到，切割结果返回(s,"",false)。
/// </details>
///
/// # Example
/// ```
/// use gostd_bytes as bytes;
///
/// assert_eq!(bytes::Cut("127.0.0.1:8080".as_bytes(),":".as_bytes()),(b"127.0.0.1".as_slice(),b"8080".as_slice(),true));
/// ```
pub fn Cut<'a>(s: &'a [byte], sep: &[byte]) -> (&'a [byte], &'a [byte], bool) {
    let before: &'a [byte];
    let after: &'a [byte];
    let found: bool;
    let i = Index(s, sep);
    if i >= 0 {
        before = &s[..uint!(i)];
        after = &s[(uint!(i) + len!(sep))..];
        found = true;
        return (before, after, found);
    } else {
    }
    before = s;
    after = "".as_bytes();
    found = false;
    (before, after, found)
}

/// Index returns the index of the first instance of sep in s, or -1 if sep is not present in s.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 子切片sep在s中第一次出现的位置，不存在则返回-1。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
/// assert_eq!(4,bytes::Index("rust社区 rust社区acean".to_string(),"社区".to_string()));
/// assert_eq!(4,bytes::Index("rust社区 rust社区acean".as_bytes(),"社区".as_bytes()));
/// assert_eq!(4,bytes::Index("chkcken", "ken"));
/// assert_eq!(-1,bytes::Index("chicken", "dmr"));
/// ```
pub fn Index(s: impl AsRef<[byte]>, sep: impl AsRef<[byte]>) -> int {
    let n = len!(sep.as_ref());
    let length = len!(s.as_ref());
    match n {
        0 => {
            return 0 as int;
        }
        1 => return IndexByte(s.as_ref(), sep.as_ref()[0]),

        _ => {
            if length == n {
                if s.as_ref() == sep.as_ref() {
                    return 0 as int;
                }
                return -1;
            }
            if n > length {
                return -1;
            }
        }
    }
    index(s, sep)
}

fn index(s: impl AsRef<[byte]>, sep: impl AsRef<[byte]>) -> int {
    let n: usize = len!(sep.as_ref());
    let length = len!(s.as_ref());
    let start_byte = sep.as_ref()[0];
    for (i, &v) in s.as_ref().iter().enumerate() {
        if start_byte == v && i + n <= length && sep.as_ref() == &s.as_ref()[i..i + n] {
            return int!(i);
        }
    }
    -1
}

/// IndexAny interprets s as a sequence of UTF-8-encoded Unicode code points. It returns the byte index of the first occurrence in s of any of the Unicode code points in chars. It returns -1 if chars is empty or if there is no code point in common.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 字符串chars中的任一utf-8编码在s中第一次出现的位置，如不存在或者chars为空字符串则返回-1
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// assert_eq!(bytes::IndexAny("chicken", "aeiouy"),2);
/// assert_eq!(bytes::IndexAny("crwth", "aeiouy"),-1);
/// assert_eq!(bytes::IndexAny("chicken".as_bytes(), "aeiouy"),2);
/// assert_eq!(bytes::IndexAny("crwth".as_bytes(), "aeiouy"),-1);
///
/// ```
pub fn IndexAny(s: impl AsRef<[byte]>, chars: impl AsRef<str>) -> int {
    if chars.as_ref().is_empty() || s.as_ref().is_empty() {
        return -1;
    }
    for (i, v) in s.as_ref().iter().enumerate() {
        if IndexRune(chars.as_ref(), v.to_owned() as u32) != -1 {
            return int!(i);
        }
    }
    -1
}

/// IndexByte returns the index of the first instance of c in b, or -1 if c is not present in b.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 字符c在s切片中第一次出现的位置，不存在则返回-1。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// assert_eq!(0,bytes::IndexByte("rustlang",b'r'));
/// assert_eq!(3,bytes::IndexByte("gophers",b'h'));
/// assert_eq!(0,bytes::IndexByte("rustlang".as_bytes(),b'r'));
/// assert_eq!(3,bytes::IndexByte("gophers".as_bytes(),b'h'));
/// assert_eq!(-1,bytes::IndexByte("gophers".to_string(),b'x'));
///
/// ```
pub fn IndexByte(s: impl AsRef<[byte]>, c: byte) -> int {
    for (i, v) in s.as_ref().iter().enumerate() {
        if v == &c {
            return int!(i);
        }
    }
    -1
}

/// IndexFunc interprets s as a sequence of UTF-8-encoded code points. It returns the byte index in s of the first Unicode code point satisfying f(c), or -1 if none do.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// IndexFunc将满足f（rune）的第一个Unicode代码点的索引返回到s，如果没有，则返回-1。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
///    /* fn f(c: u32) -> bool {
///        let s = char::from_u32(c).unwrap();
///        !s.is_ascii()
///    } */
///    // 用上面注释掉的函数也可以，用下面的匿名函数也可以。
///    let f = |c: u32| {
///        let s = char::from_u32(c).expect("u32 to char failed!");
///        !s.is_ascii()
///    };
///    assert_eq!(7, bytes::IndexFunc("Hello, 世界", f));
///    assert_eq!(-1, bytes::IndexFunc("Hello, world", f));
///    assert_eq!(7, bytes::IndexFunc("Hello, 世界".as_bytes(), f));
///    assert_eq!(-1, bytes::IndexFunc("Hello, world".to_string(), f));
///
/// ```
pub fn IndexFunc(s: impl AsRef<[byte]>, f: fn(rune) -> bool) -> int {
    for (i, r) in s.as_ref().iter().enumerate() {
        if f(*r as u32) {
            return int!(i);
        }
    }
    -1
}

/// IndexRune interprets s as a sequence of UTF-8-encoded code points. It returns the byte index of the first occurrence in s of the given rune. It returns -1 if rune is not present in s. If r is utf8.RuneError, it returns the first instance of any invalid UTF-8 byte sequence.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// unicode码值r在s字节切片中第一次出现的位置，不存在则返回-1。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// assert_eq!(4,bytes::IndexRune("chicken", 'k' as u32));
/// assert_eq!(4,bytes::IndexRune("chicken", 0x6b));
/// assert_eq!(4,bytes::IndexRune("chicken", 107_u32));
/// assert_eq!(-1,bytes::IndexRune("chicken", 'd' as u32));
/// assert_eq!(-1,bytes::IndexRune("chicken", 0x64));
/// assert_eq!(-1,bytes::IndexRune("chicken", 100_u32));
///
/// ```
pub fn IndexRune(s: impl AsRef<[byte]>, r: rune) -> int {
    let c = char::from_u32(r).expect("IndexRune rune to char failed!");
    let b = c.to_string();
    let n = b.as_bytes().len();
    for (i, _) in s.as_ref().iter().enumerate() {
        if b.as_bytes()[0] == s.as_ref()[i] && &s.as_ref()[i..i + n] == b.as_bytes() {
            return int!(i);
        }
    }
    -1
}

/// Join concatenates the elements of s to create a new byte slice. The separator sep is placed between elements in the resulting slice.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 将一系列字符串连接为一个字符串，之间用sep来分隔。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// let s = vec!["foo".as_bytes(), "bar".as_bytes(), "baz".as_bytes()];
///
/// assert_eq!("foo, bar, baz".as_bytes(),bytes::Join(s,", "));
///
/// let list: Vec<&[u8]> = [[1, 2], [3, 4]].iter().map(|x|x.as_slice()).collect();
/// assert_eq!(bytes::Join(list.clone(),&[0, 0][..]), [1, 2, 0, 0, 3, 4]);
/// assert_eq!(bytes::Join(list,&[0, 0][..]), [1, 2, 0, 0, 3, 4].as_slice());
///
/// ```
pub fn Join(elems: Vec<&[byte]>, sep: impl AsRef<[byte]>) -> Vec<byte> {
    elems.join(sep.as_ref())
}

/// LastIndex returns the index of the last instance of sep in s, or -1 if sep is not present in s.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 切片sep在字符串s中最后一次出现的位置，不存在则返回-1。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// assert_eq!(4,bytes::Index("rust社区 rust社区acean","社区"));
/// assert_eq!(15,bytes::LastIndex("rust社区 rust社区acean","社区"));
/// assert_eq!(4,bytes::Index("rust社区 rust社区acean".to_string(),"社区".to_string()));
/// assert_eq!(15,bytes::LastIndex("rust社区 rust社区acean".as_bytes(),"社区".as_bytes()));
/// assert_eq!(5,bytes::LastIndex("rust rustacean","rust"));
/// assert_eq!(-1,bytes::LastIndex("rust rustacean","go"));
/// // 以下例子只适用于ASCII字符串
/// assert_eq!(0,bytes::Index(b"rust rustacean",b"rust"));
/// assert_eq!(5,bytes::LastIndex(b"rust rustacean",b"rust"));
/// assert_eq!(-1,bytes::LastIndex(b"rust rustacean",b"go"));
///
/// ```
pub fn LastIndex(s: impl AsRef<[byte]>, sep: impl AsRef<[byte]>) -> int {
    let n = len!(sep.as_ref());
    match n {
        0 => {
            return len!(s.as_ref()) as int;
        }
        1 => {
            return LastIndexByte(s.as_ref(), sep.as_ref()[0]);
        }
        _ => {
            if n == len!(s.as_ref()) {
                if sep.as_ref() == s.as_ref() {
                    return 0;
                }
            }
            if n > len!(s.as_ref()) {
                return -1;
            }
        }
    }
    let mut i = len!(s.as_ref()) - 1 - n;
    while i > 0 {
        if &s.as_ref()[i..i + n] == sep.as_ref() {
            return int!(i);
        }
        i -= 1;
    }

    -1
}

/// LastIndexAny interprets s as a sequence of UTF-8-encoded Unicode code points. It returns the byte index of the last occurrence in s of any of the Unicode code points in chars. It returns -1 if chars is empty or if there is no code point in common.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 字符串chars中的任一utf-8字符在s中最后一次出现的位置，如不存在或者chars为空字符串则返回-1。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// assert_eq!(4,bytes::LastIndexAny("go gopher", "go"));
/// assert_eq!(8,bytes::LastIndexAny("go gopher", "ordent"));
/// assert_eq!(-1,bytes::LastIndexAny("go gopher", "fail"));
/// assert_eq!(4,bytes::LastIndexAny("go gopher".as_bytes(), "go"));
/// assert_eq!(8,bytes::LastIndexAny("go gopher".as_bytes(), "ordent"));
/// assert_eq!(-1,bytes::LastIndexAny("go gopher".as_bytes(), "fail"));
///
/// ```
pub fn LastIndexAny(s: impl AsRef<[byte]>, chars: impl AsRef<str>) -> int {
    let n = len!(s.as_ref());
    if chars.as_ref().is_empty() {
        return -1;
    }
    for (i, &r) in s.as_ref().iter().enumerate().rev() {
        if chars.as_ref().contains(|x| x == (r as char)) {
            return int!(i);
        }
    }
    -1
}

/// LastIndexByte returns the index of the last instance of c in s, or -1 if c is not present in s.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回s中c的最后一个出现的位置，如果s中不存在c，则返回-1。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// assert_eq!(10,bytes::LastIndexByte("Hello, world", b'l'));
/// assert_eq!(8,bytes::LastIndexByte("Hello, world", b'o'));
/// assert_eq!(10,bytes::LastIndexByte("Hello, world".as_bytes(), b'l'));
/// assert_eq!(8,bytes::LastIndexByte(vec!['H','e','l','l','o',',',' ','w','o','r','l','d'].iter().map(|c| *c as u8).collect::<Vec<_>>(), b'o'));
/// assert_eq!(-1,bytes::LastIndexByte("Hello, world", b'x'));
/// ```
pub fn LastIndexByte(s: impl AsRef<[byte]>, c: byte) -> int {
    for (i, &v) in s.as_ref().iter().enumerate().rev() {
        if v == c {
            return int!(i);
        }
    }
    -1
}

/// LastIndexFunc interprets s as a sequence of UTF-8-encoded code points. It returns the byte index in s of the last Unicode code point satisfying f(c), or -1 if none do.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// s字节切片中最后一个满足函数f的unicode码值的位置，不存在则返回-1。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
///    let f = |x: u32| char::from_u32(x).unwrap().is_ascii_digit();
///
///    assert_eq!(5, bytes::LastIndexFunc("go 123", f));
///    assert_eq!(2, bytes::LastIndexFunc("123 go", f));
///    assert_eq!(-1, bytes::LastIndexFunc("go", f));
///    assert_eq!(5, bytes::LastIndexFunc("go 123".as_bytes(), f));
///    assert_eq!(2, bytes::LastIndexFunc("123 go".to_string(), f));
///    assert_eq!(-1, bytes::LastIndexFunc(vec![b'g',b'o'], f));
/// ```
pub fn LastIndexFunc(s: impl AsRef<[byte]>, f: fn(rune) -> bool) -> int {
    for (i, &r) in s.as_ref().iter().enumerate().rev() {
        if f(r as u32) == true {
            return int!(i);
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
/// use gostd_bytes as bytes;
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
///    let s = "'Twas brillig and the slithy gopher...".as_bytes();
///    assert_eq!(
///        "'Gjnf oevyyvt naq gur fyvgul tbcure...".as_bytes(),
///        bytes::Map(rot13, s)
///    );
///
/// ```
pub fn Map(mapping: fn(rune) -> rune, mut s: impl AsRef<[byte]>) -> Vec<byte> {
    let mut b = Buffer::new();
    b.Grow(int!(len!(s.as_ref())));
    for (idx, &v) in s.as_ref().iter().enumerate() {
        let r = mapping(v as u32);
        if r > 0 {
            b.WriteRune(r);
        } else {
            b.WriteRune(v as u32);
        }
    }
    b.Bytes()
}

/// Repeat returns a new string consisting of count copies of the string s.
///
/// It panics if count is negative or if the result of (len!(s) * count) overflows.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回count个b串联形成的新的切片。
/// </details>
///
/// # Example
///
/// ```
///  use gostd_bytes as bytes;
///
///  assert_eq!("121212".as_bytes(),bytes::Repeat("12",3));
///  println!("{:?}",bytes::Repeat("12",3).as_slice());
///
/// ```
/// # Output
///
/// ```text
/// 121212
/// ```

pub fn Repeat(s: impl AsRef<[byte]>, count: uint) -> Vec<byte> {
    s.as_ref().repeat(count)
}

/// Replace returns a copy of the slice s with the first n non-overlapping instances of old replaced by new. If old is empty, it matches at the beginning of the slice and after each UTF-8 sequence, yielding up to k+1 replacements for a k-rune slice. If n = -1, there is no limit on the number of replacements.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将s中前n个不重叠old切片序列都替换为new的新的切片拷贝，如果n=-1会替换所有old子切片。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// assert_eq!("oinky oinky oink".as_bytes(),bytes::Replace("oink oink oink".as_bytes(), "k".as_bytes(), "ky".as_bytes(), 2));
/// assert_eq!("moo moo moo".as_bytes(),bytes::Replace("oink oink oink".as_bytes(), "oink", "moo", -1));
///
/// ```
pub fn Replace(
    s: impl AsRef<[byte]>,
    old: impl AsRef<[byte]>,
    new: impl AsRef<[byte]>,
    n: int,
) -> Vec<byte> {
    let mut s = s.as_ref();
    let mut start: int = 0;
    let old_len = len!(old.as_ref());
    let mut result: Vec<byte> = vec![];
    let mut count = 0;
    while start > -1 && start < len!(s.as_ref()) as int && (count < n || n == -1) {
        start = Index(s, old.as_ref());
        result.extend_from_slice(&s[..start as uint]);
        result.extend_from_slice(new.as_ref());
        s = &s[start as usize + old_len..];
        count += 1;
    }
    result.extend_from_slice(&s[..]);
    result
}

/// ReplaceAll returns a copy of the slice s with all non-overlapping instances of old replaced by new. If old is empty, it matches at the beginning of the slice and after each UTF-8 sequence, yielding up to k+1 replacements for a k-rune slice.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将s中不重叠old切片序列都替换为new的新的切片拷贝。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// assert_eq!("moo moo moo".as_bytes(),bytes::ReplaceAll("oink oink oink".as_bytes(), "oink".as_bytes(), "moo".as_bytes()));
///
/// ```
///
pub fn ReplaceAll(
    s: impl AsRef<[byte]>,
    old: impl AsRef<[byte]>,
    new: impl AsRef<[byte]>,
) -> Vec<byte> {
    Replace(s, old, new, -1)
}

/// Split slices s into all subslices separated by sep and returns a slice of the subslices between those separators. If sep is empty, Split splits after each UTF-8 sequence. It is equivalent to SplitN with a count of -1.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 用去掉s中出现的sep的方式进行分割，会分割到结尾，并返回生成的所有&[byte]切片组成的切片（每一个sep都会进行一次切割，即使两个sep相邻，也会进行两次切割）。如果sep为空字符，Split会将s切分成每一个unicode码值一个[]byte切片。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// assert_eq!(vec![b"a".to_vec(),b"b".to_vec(),b"c".to_vec()],bytes::Split("a,b,c".as_bytes(),b","));
/// assert_eq!(vec![b"".to_vec(),b"man ".to_vec(),b"plan ".to_vec(),b"canal panama".to_vec()],bytes::Split("a man a plan a canal panama".as_bytes(),b"a "));
/// assert_eq!(vec![b" ".to_vec(), b"x".to_vec(), b"y".to_vec(), b"z".to_vec(), b" ".to_vec()],bytes::Split(" xyz ".as_bytes(),""));
/// assert_eq!(vec![b"".to_vec()],bytes::Split("".as_bytes(),"Bernardo O'Higgins".as_bytes()));
/// ```
pub fn Split(s: impl AsRef<[byte]>, sep: impl AsRef<[byte]>) -> Vec<Vec<byte>> {
    genSplit(s, sep, 0, -1)
}

fn genSplit(
    s: impl AsRef<[byte]>,
    sep: impl AsRef<[byte]>,
    sepSave: uint,
    n: int,
) -> Vec<Vec<byte>> {
    let mut n = n;
    let mut s = s.as_ref();
    let sep_len = len!(sep.as_ref());

    if sep.as_ref().is_empty() {
        return s.iter().map(|&x| vec![x]).collect();
    }
    if n < 0 {
        n = Count(s, sep.as_ref()) + 1;
    }
    let mut a: Vec<Vec<byte>> = Vec::new();
    let mut i: int = 0;
    while i < n - 1 {
        let m = Index(s, sep.as_ref());
        if m < 0 {
            break;
        }
        let v = &s[..m as usize + sepSave];
        a.push(v.to_vec());
        s = &s[m as usize + sep_len..];
        i += 1;
    }
    a.push(s.to_vec());
    a
}

/// SplitAfter slices s into all subslices after each instance of sep and returns a slice of those subslices. If sep is empty, SplitAfter splits after each UTF-8 sequence. It is equivalent to SplitAfterN with a count of -1.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 用从s中出现的sep后面切断的方式进行分割，会分割到结尾，并返回生成的所有&[byte]切片组成的切片（每一个sep都会进行一次切割，即使两个sep相邻，也会进行两次切割）。如果sep为空字符，Split会将s切分成每一个unicode码值一个&[byte]切片。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// assert_eq!(vec!["a,".as_bytes().to_vec(), "b,".as_bytes().to_vec(), "c".as_bytes().to_vec()],bytes::SplitAfter("a,b,c".as_bytes(), ",".as_bytes()));
/// ```
pub fn SplitAfter(s: impl AsRef<[byte]>, sep: impl AsRef<[byte]>) -> Vec<Vec<byte>> {
    genSplit(s, sep.as_ref(), len!(sep.as_ref()), -1)
}

/// SplitAfterN slices s into subslices after each instance of sep and returns a slice of those subslices. If sep is empty, SplitAfterN splits after each UTF-8 sequence.
/// The count determines the number of subslices to return:
/// ```text
/// n > 0: at most n subslices; the last subslice will be the unsplit remainder.
/// n == 0: the result is [] (zero subslices)
/// n < 0: all subslices
/// ```
/// Edge cases for s and sep (for example, empty strings) are handled as described in the documentation for SplitAfter.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 用从s中出现的sep后面切断的方式进行分割，会分割到最多n个子切片，并返回生成的所有 &[byte]切片组成的切片（每一个sep都会进行一次切割，即使两个sep相邻，也会进行两次切割）。如果sep为空字符，Split会将s切分成每一个unicode码值一个&[byte]切片
///
/// 参数n决定返回的切片的数目：
///
/// ```text
///
/// n > 0 : 返回的切片最多n个子切片；最后一个子切片包含未进行切割的部分。
/// n == 0: 返回[]
/// n < 0 : 返回所有的子切片组成的切片
///
/// ```
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
/// use gostd_builtin::byte;
/// // n == 0 返回 []
///    assert_eq!(Vec::<Vec<byte>>::new(), bytes::SplitAfterN(",a,b2,c".as_bytes(), ",".as_bytes(), 0));
///    assert_eq!(vec![",a,b2,c".as_bytes()], bytes::SplitAfterN(",a,b2,c".as_bytes(), ",".as_bytes(), 1));
///    assert_eq!(vec![",".as_bytes(), "a,".as_bytes(), b"b2,".as_ref(), b"c".as_ref()],bytes::SplitAfterN(",a,b2,c", ",", -1));
///    assert_eq!(vec![b",".to_vec(), b"a,b2,c".to_vec()], bytes::SplitAfterN(",a,b2,c", ",", 2));
///    assert_eq!(vec![",".as_bytes(), "a,".as_bytes(), "b2,c".as_bytes()],bytes::SplitAfterN(",a,b2,c".as_bytes(), ",".as_bytes(), 3));
///    assert_eq!(vec![b",".as_ref(), b"a,".as_ref(), b"b2,".as_ref(), b"c".as_ref()],bytes::SplitAfterN(",a,b2,c".as_bytes(), ",".as_bytes(), 4));
/// // 当n大于最大子串数量，也只返回最大值。
///    assert_eq!(vec![b",".to_vec(), b"a,".to_vec(), b"b2,".to_vec(), b"c".to_vec()],bytes::SplitAfterN(",a,b2,c".as_bytes(), ",".as_bytes(), 5));
///    assert_eq!(vec![",".as_bytes().to_vec(), "a,".as_bytes().to_vec(), "b2,".as_bytes().to_vec(), "c".as_bytes().to_vec()],bytes::SplitAfterN(",a,b2,c".as_bytes(), ",".as_bytes(), 10));
/// ```
pub fn SplitAfterN(s: impl AsRef<[byte]>, sep: impl AsRef<[byte]>, n: int) -> Vec<Vec<byte>> {
    if n == 0 {
        return vec![];
    }
    genSplit(s, sep.as_ref(), len!(sep.as_ref()), n)
}

/// SplitN slices s into subslices separated by sep and returns a slice of the subslices between those separators. If sep is empty, SplitN splits after each UTF-8 sequence.
///
/// The count determines the number of subslices to return:
/// ```text
/// n > 0: at most n subslices; the last subslices will be the unsplit remainder.
/// n == 0: the result is [] (zero subslices)
/// n < 0: all subslices
/// ```
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
/// use gostd_bytes as bytes;
/// use gostd_builtin::byte;
///
///    assert_eq!(Vec::<Vec<byte>>::new(), bytes::SplitN(",a,b2,c".as_bytes(), ",", 0));
///    assert_eq!(vec![",a,b2,c".as_bytes().to_vec()], bytes::SplitN(",a,b2,c".as_bytes(), ",", 1));
///    assert_eq!(
///        vec!["", "a", "b2", "c"].iter().map(|x|x.as_bytes().to_vec()).collect::<Vec<Vec<byte>>>(),  bytes::SplitN(",a,b2,c".as_bytes(), ",", -1)
///    );
///    assert_eq!(vec!["", "a,b2,c"].iter().map(|x|x.as_bytes().to_vec()).collect::<Vec<Vec<byte>>>(), bytes::SplitN(",a,b2,c".as_bytes(), ",", 2));
///    assert_eq!(vec!["", "a", "b2,c"].iter().map(|x|x.as_bytes().to_vec()).collect::<Vec<Vec<byte>>>(), bytes::SplitN(",a,b2,c".as_bytes(), ",", 3));
///    assert_eq!(
///        vec!["", "a", "b2", "c"].iter().map(|x|x.as_bytes().to_vec()).collect::<Vec<Vec<byte>>>(),
///        bytes::SplitN(",a,b2,c".as_bytes(), ",", 4)
///    );
///    assert_eq!(
///        vec!["", "a", "b2", "c"].iter().map(|x|x.as_bytes().to_vec()).collect::<Vec<Vec<byte>>>(),
///        bytes::SplitN(",a,b2,c".as_bytes(), ",", 5)
///    );
///    assert_eq!(
///        vec!["", "a", "b2", "c"].iter().map(|x|x.as_bytes().to_vec()).collect::<Vec<Vec<byte>>>(),
///        bytes::SplitN(",a,b2,c".as_bytes(), ",", 10)
///    );
///
/// ```
pub fn SplitN(s: impl AsRef<[byte]>, sep: impl AsRef<[byte]>, n: int) -> Vec<Vec<byte>> {
    if n == 0 {
        return vec![];
    } else {
        genSplit(s, sep, 0, n)
    }
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
/// use gostd_bytes as bytes;
///
/// assert_eq!("gopher".as_bytes().to_vec(),bytes::ToLower("Gopher"));
/// assert_eq!("gopher".as_bytes(),bytes::ToLower("Gopher"));
/// ```
pub fn ToLower(s: impl AsRef<[byte]>) -> Vec<byte> {
    s.as_ref().to_ascii_lowercase()
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
/// use gostd_bytes as bytes;
///
/// assert_eq!("GOPHER".as_bytes(),bytes::ToUpper("Gopher"));
/// assert_eq!("GOPHER".as_bytes().to_vec(),bytes::ToUpper("Gopher"));
/// ```
pub fn ToUpper(s: impl AsRef<[byte]>) -> Vec<byte> {
    s.as_ref().to_ascii_uppercase()
}

/// Trim returns a subslice of s by slicing off all leading and trailing UTF-8-encoded code points contained in cutset.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将s前后端所有cutset包含的utf-8码值都去掉的子切片。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// assert_eq!("Hello, 中国".as_bytes(),bytes::Trim("¡¡¡Hello, 中国!!!".as_bytes(), "!¡".as_bytes()));
///
/// ```

pub fn Trim(mut s: &[byte], cutset: impl AsRef<[byte]>) -> &[byte] {
    TrimRight(TrimLeft(s, cutset.as_ref()), cutset.as_ref())
}

/// TrimFunc returns a subslice of s by slicing off all leading and trailing UTF-8-encoded code points c that satisfy f(c).
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将s前后端所有满足f的unicode码值都去掉的子切片。
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
pub fn TrimFunc(s: &[byte], f: fn(rune) -> bool) -> &[byte] {
    TrimRightFunc(TrimLeftFunc(s, f), f)
}

/// TrimLeft returns a subslice of s by slicing off all leading UTF-8-encoded code points contained in cutset.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将s前端所有cutset包含的unicode码值都去掉的子切片
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// assert_eq!("Hello, Gophers!!!".as_bytes(),bytes::TrimLeft("¡¡¡Hello, Gophers!!!".as_bytes(), "!¡".as_bytes()))
/// ```
pub fn TrimLeft(s: &[byte], cutset: impl AsRef<[byte]>) -> &[byte] {
    let mut isStart = true;
    let mut i: usize = 0;
    for v in s.as_ref() {
        if isStart && cutset.as_ref().contains(v) {
            i += 1;
        } else {
            isStart = false
        }
    }
    &s[i..]
}

/// TrimLeftFunc treats s as UTF-8-encoded bytes and returns a subslice of s by slicing off all leading UTF-8-encoded code points c that satisfy f(c).
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将s前端所有满足f的unicode码值都去掉的子切片。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// let f = |x| x >= '1' as u32 && x <= '9' as u32;
/// assert_eq!("Hello, Rust654321".as_bytes(),bytes::TrimLeftFunc("123456Hello, Rust654321".as_bytes(), f));
///
///
/// ```
pub fn TrimLeftFunc(s: &[byte], f: fn(rune) -> bool) -> &[byte] {
    let mut isStart = true;
    let mut i: usize = 0;
    for &v in s.as_ref() {
        if isStart && f(v as rune) {
            i += 1;
        } else {
            isStart = false
        }
    }

    &s[i..]
}

/// TrimPrefix returns s without the provided leading prefix slices. If s doesn't start with prefix, s is returned unchanged.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回去除s可能的前缀prefix的子切片。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// assert_eq!("Hello, Rust!xxx".as_bytes(),bytes::TrimPrefix("xxxHello, Rust!xxx".as_bytes(),"xxx".as_bytes()));
///
/// ```
pub fn TrimPrefix(s: &[byte], prefix: impl AsRef<[byte]>) -> &[byte] {
    match s.strip_prefix(prefix.as_ref()) {
        Some(sub) => sub,
        None => s,
    }
}

/// TrimRight returns a subslice of s by slicing off all trailing UTF-8-encoded code points that are contained in cutset.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将s后端所有cutset包含的unicode码值都去掉的子切片。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
/// assert_eq!("¡¡¡Hello, Gophers".as_bytes(),bytes::TrimRight("¡¡¡Hello, Gophers!!!".as_bytes(), "!¡".as_bytes()));
///
/// ```
pub fn TrimRight(s: &[byte], cutset: impl AsRef<[byte]>) -> &[byte] {
    let mut isStart = true;
    let mut i: usize = 0;
    let n = len!(s);
    for v in s.as_ref().iter().rev() {
        if isStart && cutset.as_ref().contains(v) {
            i += 1;
        } else {
            isStart = false
        }
    }

    &s[..n - i]
}

/// TrimRightFunc returns a subslice of s by slicing off all trailing UTF-8-encoded code points c that satisfy f(c).
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将s后端所有满足f的unicode码值都去掉的子切片。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// let f = |x| x >= '1' as u32 && x <= '9' as u32;
/// assert_eq!("123456Hello, Rust".as_bytes(),bytes::TrimRightFunc("123456Hello, Rust654321".as_bytes(), f));
///
/// ```
pub fn TrimRightFunc(s: &[byte], f: fn(rune) -> bool) -> &[byte] {
    let mut isStart = true;
    let mut i: usize = 0;
    let n = len!(s);
    for &v in s.as_ref().iter().rev() {
        if isStart && f(v as rune) {
            i += 1;
        } else {
            isStart = false
        }
    }

    &s[..n - i]
}

/// TrimSpace returns a subslice of s by slicing off all leading and trailing white space, as defined by Unicode.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回将s前后端所有空白（is_whitespace()指定）都去掉的切片。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// assert_eq!("Hello, Rust!".as_bytes(),bytes::TrimSpace("  Hello, Rust!  ".as_bytes()));
/// assert_eq!("Hello, Rust!".as_bytes(),bytes::TrimSpace("\nHello, Rust! \t ".as_bytes()));
/// assert_eq!("Hello, Rust!".as_bytes(),bytes::TrimSpace("\n\t Hello, Rust! \t\r ".as_bytes()));
///
/// ```
pub fn TrimSpace(s: &[byte]) -> &[byte] {
    let n = len!(s);
    let mut start: usize = 0;
    let mut end: usize = n - 1;
    for i in 0..n {
        if s[i].is_ascii_whitespace() || s[i] == 0x85 || s[i] == 0xA0 {
            start = i;
        } else {
            break;
        }
    }
    while end > 0 {
        if s[end].is_ascii_whitespace() || s[end] == 0x85 || s[end] == 0xA0 {
            end -= 1;
        } else {
            break;
        }
    }
    &s[start + 1..end + 1]
}

/// TrimSuffix returns s without the provided trailing suffix string. If s doesn't end with suffix, s is returned unchanged.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 返回去除s可能的后缀suffix的子切片。
/// </details>
///
/// # Example
///
/// ```
/// use gostd_bytes as bytes;
///
/// assert_eq!("test".as_bytes(),bytes::TrimSuffix("test.rs".as_bytes(),".rs".as_bytes()))
///
/// ```
pub fn TrimSuffix(s: &[byte], suffix: impl AsRef<[byte]>) -> &[byte] {
    match s.strip_suffix(suffix.as_ref()) {
        Some(sub) => sub,
        None => s,
    }
}

// Buffer Begin

/// A Buffer is a variable-sized buffer of bytes with Read and Write methods. The zero value for Buffer is an empty buffer ready to use.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Buffer是一个实现了读写方法的可变大小的字节缓冲。本类型的零值是一个空的可用于读写的缓冲。
/// </details>
///
/// # Example
///
/// ```
///  use gostd_io::*;
///  use gostd_bytes::Buffer;
///
///  let mut buf = Buffer::new();
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
pub struct Buffer {
    addr: Box<Option<Buffer>>,
    buf: Vec<byte>,
}

impl Buffer {
    /// new initialization a Buffer
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 初始化生成器
    /// </details>
    pub fn new() -> Buffer {
        let mut b = Buffer::default();
        b.addr = Box::new(Some(b.clone()));
        b
    }

    /// with_bytes creates and initializes a new Buffer using buf as its initial contents. The new Buffer takes ownership of buf, and the caller should not use buf after this call. with_bytes is intended to prepare a Buffer to read existing data. It can also be used to set the initial size of the internal buffer for writing. To do that, buf should have the desired capacity but a length of zero.
    ///
    /// In most cases, new() is sufficient to initialize a Buffer.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 使用buf作为初始内容创建并初始化一个Buffer。本函数用于创建一个用于读取已存在数据的buffer；也用于指定用于写入的内部缓冲的大小，此时，buf应为一个具有指定容量但长度为0的切片。buf会被作为返回值的底层缓冲切片。
    /// </details>
    pub fn with_bytes(buf: Vec<byte>) -> Buffer {
        let mut b = Buffer::new();
        b.buf = buf;
        b
    }

    /// with_str creates and initializes a new Buffer using string s as its initial contents. It is intended to prepare a buffer to read an existing string.
    ///
    /// In most cases, new() is sufficient to initialize a Buffer.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 使用s作为初始内容创建并初始化一个Buffer。本函数用于创建一个用于读取已存在数据的buffer。
    /// </details>
    pub fn with_str(s: &str) -> Buffer {
        let mut b = Buffer::new();
        b.buf.extend_from_slice(s.as_bytes());
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
    pub fn Bytes(&self) -> Vec<byte> {
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

use io::ByteWriter;
impl ByteWriter for Buffer {
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

use io::StringWriter;
impl StringWriter for Buffer {
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

use io::Writer;
impl Writer for Buffer {
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

// Buffer End
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
    s: Vec<byte>,
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

use std::io::ErrorKind;
use std::io::{Error, Read};

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
        let n = int!(len!(self.s.as_slice()[uint!(self.i)..]));
        b.copy_from_slice(self.s.as_slice()[uint!(self.i)..].as_ref());
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
        let n = len!(self.s.as_slice()[uint!(off)..]);
        b.copy_from_slice(self.s.as_slice()[uint!(off)..].as_ref());
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
        let b = self.s.as_slice()[uint!(self.i)];
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
        if let Some(&c) = self.s.as_slice().get(uint!(self.i)) {
            if rune!(c) < utf8::RuneSelf {
                self.i += 1;
                return Ok((rune!(c), 1));
            }
            let size = c.to_ne_bytes().len() as int;
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
    fn WriteTo(&mut self, mut w: Box<dyn io::Writer>) -> Result<int64, Error> {
        self.prevRune = -1;
        if self.i >= int64!(len!(self.s)) {
            return Ok(0);
        }
        let s = self.s.get(self.i as usize..).unwrap();
        if let Ok(m) = w.Write(s.to_vec()) {
            if m > int!(len!(s)) {
                panic!("gostd::bytes::Reader::WriteTo: invalid Write count")
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

/// Replacer replaces a list of slices with replacements. It is safe for concurrent use by multiple goroutines.
///
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Replacer类型进行一系列字节切片的替换。
/// </details>
pub struct Replacer<'a> {
    oldnew: Vec<(&'a [byte], &'a [byte])>,
}

impl<'a> Replacer<'a> {
    /// new returns a new Replacer from a list of old, new bytes pairs. Replacements are performed in the order they appear in the target string, without overlapping matches. The old string comparisons are done in argument order.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 使用提供的多组old、new字节切片对创建并返回一个*Replacer。替换是依次进行的，匹配时不会重叠。
    /// </details>
    ///
    /// # Example
    ///
    /// ```
    /// use gostd_bytes as bytes;
    ///
    ///    let p = vec![("<".as_bytes(), "&lt;".as_bytes()), (">".as_bytes(), "&gt;".as_bytes())];
    ///    let r = bytes::Replacer::new(p);
    ///    let s = r.Replace("This is <b>HTML</b>!".as_bytes());
    ///    println!("{}", String::from_utf8(s).unwrap());
    ///
    /// ```
    /// # Output
    ///
    /// ```text
    /// This is &lt;b&gt;HTML&lt;/b&gt;!
    /// ```
    pub fn new(pairs: Vec<(&'a [byte], &'a [byte])>) -> Replacer<'a> {
        Replacer { oldnew: pairs }
    }
    /// Replace returns a copy of s with all replacements performed.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// Replace返回s的所有替换进行完后的拷贝.
    /// </details>
    pub fn Replace(self, s: &[byte]) -> Vec<byte> {
        let mut new_slice: Vec<byte> = s.to_vec();
        for pair in self.oldnew.clone() {
            new_slice = ReplaceAll(new_slice, pair.0, pair.1);
        }
        new_slice
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
