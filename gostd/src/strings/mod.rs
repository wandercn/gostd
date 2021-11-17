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
use crate::builtin::*;
use crate::io;
use crate::unicode::utf8;
use gostd_derive::Fmt;
/// Compare returns an integer comparing two strings lexicographically. The result will be 0 if a==b, -1 if a < b, and +1 if a > b.
/// Compare is included only for symmetry with package bytes. It is usually clearer and always faster to use the built-in string comparison operators ==, <, >, and so on.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// Compare返回一个按字典顺序比较两个字符串的整数。如果a==b，结果为0；如果a<b，结果为-1；如果a>b，结果为+1
/// 比较仅用于与包字节对称。使用内置的字符串比较运算符==、<、>，等等通常更清晰，而且总是更快。
/// </details>
///
/// # Example
///
/// ```
/// use gostd::strings;
///
///    assert_eq!(-1, strings::Compare("a", "b"));
///    assert_eq!(0, strings::Compare("a", "a"));
///    assert_eq!(1, strings::Compare("b", "a"));
///
/// ```
pub fn Compare(a: &str, b: &str) -> int {
    if a == b {
        return 0;
    }
    if a < b {
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
/// use gostd::strings;
///
/// assert_eq!(true,strings::Contains("seafood", "foo"));
/// assert_eq!(false,strings::Contains("seafood", "bar"));
/// assert_eq!(true,strings::Contains("seafood", ""));
/// assert_eq!(true,strings::Contains("", ""));
/// ```
pub fn Contains(s: &str, substr: &str) -> bool {
    s.contains(substr)
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
/// use gostd::strings;
///
///    assert_eq!(false, strings::ContainsAny("team", "i"));
///    assert_eq!(true, strings::ContainsAny("failure", "u & i"));
///    assert_eq!(false, strings::ContainsAny("foo", ""));
///    assert_eq!(false, strings::ContainsAny("", ""));
///    assert_eq!(true, strings::ContainsAny("你好,中国", "hello,好"));
///
/// ```
pub fn ContainsAny(s: &str, chars: &str) -> bool {
    for c in chars.chars() {
        if s.contains(c) {
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
/// use gostd::strings;
///
/// // '中' as rune = 20013 or 0x4e2d
/// assert_eq!(true, strings::ContainsRune("hello中国!", 20013));
/// assert_eq!(true, strings::ContainsRune("hello中国!", 0x4e2d));
/// assert_eq!(false, strings::ContainsRune("hello世界!", 0x4e2d));
/// ```
pub fn ContainsRune(s: &str, r: rune) -> bool {
    s.contains(char::from_u32(r).unwrap())
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
/// use gostd::strings;
///     assert_eq!(3, strings::Count("cheese", "e"));
///     assert_eq!(5, strings::Count("five", ""));
///     assert_eq!(4, strings::Count("台湾人香港人澳门人都是中国人", "人"));
/// ```
pub fn Count(mut s: &str, substr: &str) -> int {
    if len!(substr) == 0 {
        return int!(s.chars().count() + 1);
    }

    if len!(substr) == 1 {
        let mut c: int = 0;
        let s1 = substr.bytes().nth(0).unwrap();
        for v in s.bytes() {
            if v == s1 {
                c += 1
            }
        }
        return c;
    }

    let mut n: int = 0;
    loop {
        let i = Index(s, substr);
        if i == -1 {
            return n;
        }
        n += 1;
        s = &s[uint!(i) + len!(substr)..];
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
/// use gostd::strings;
///
///    assert_eq!(true, strings::EqualFold("Hello, 世界", "heLLo, 世界"));
///    assert_eq!(false, strings::EqualFold("hello,world", "hello, 世界"));
///    assert_eq!(true, strings::EqualFold("RUST-LANG", "rust-lang"));
///    assert_eq!(true, strings::EqualFold("Go", "go"));
/// ```
pub fn EqualFold(s: &str, t: &str) -> bool {
    s.to_lowercase() == t.to_lowercase()
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
/// use gostd::strings;
///  assert_eq!(vec!["foo","bar","baz"],strings::Fields("  foo bar  baz   "));
///  assert_eq!(
///     vec!["aaa", "bbb", "cccc", "ddd"],
///     strings::Fields("  \taaa bbb\t  cccc\r ddd  \r"));
/// ```
pub fn Fields(s: &str) -> Vec<String> {
    let mut list = vec![];
    let mut wasSapce = false;
    let mut s1 = "".to_string();
    for (idx, v) in s.trim().chars().enumerate() {
        if idx == s.trim().len() - 1 {
            s1.push(v);
            wasSapce = true;
        }
        if v.is_ascii_whitespace() || v.is_whitespace() {
            wasSapce = true;
        } else {
            if wasSapce {
                wasSapce = false;
                list.push(s1.clone());
                s1.clear();
                s1.push(v);
            } else {
                s1.push(v);
            }
        }
    }
    list
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
///
/// ```
pub fn FieldsFunc(s: &str, f: fn(rune) -> bool) -> Vec<&str> {
    todo!()
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
///
/// ```
pub fn HasPrefix(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
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
///
/// ```
pub fn HasSuffix(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
}

/// Index returns the index of the first instance of substr in s, or -1 if substr is not present in s.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn Index(s: &str, substr: &str) -> int {
    if substr == "" {
        return -1;
    }
    if let Some(i) = s.find(substr) {
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
/// use gostd::strings;
///
/// assert_eq!(strings::IndexAny("chicken", "aeiouy"),2);
/// assert_eq!(strings::IndexAny("crwth", "aeiouy"),-1);
///
/// ```
pub fn IndexAny(s: &str, chars: &str) -> int {
    if chars == "" || s == "" {
        return -1;
    }
    for (idx, r) in s.chars().enumerate() {
        if IndexRune(chars, r as u32) != -1 {
            return int!(idx);
        }
    }

    -1
}

/// IndexByte returns the index of the first instance of c in s, or -1 if c is not present in s.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn IndexByte(s: &str, c: byte) -> int {
    if let Some(i) = s.bytes().find(|&x| x == c) {
        return int!(i);
    } else {
        return -1;
    }
}

/// IndexFunc returns the index into s of the first Unicode code point satisfying f(c), or -1 if none do.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
use std::ops::FnMut;
pub fn IndexFunc(s: &str, f: fn(rune) -> bool) -> int {
    for (i, r) in s.chars().enumerate() {
        if f(r as u32) == true {
            return int!(i);
        }
    }
    -1
}

/// IndexRune returns the index of the first instance of the Unicode code point r, or -1 if rune is not present in s. If r is utf8.RuneError, it returns the first instance of any invalid UTF-8 byte sequence.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn IndexRune(s: &str, r: rune) -> int {
    if let Some(i) = s.find(|c: char| c == char::from_u32(r).unwrap()) {
        return int!(i);
    }
    -1
}

/// Join concatenates the elements of its first argument to create a single string. The separator string sep is placed between elements in the resulting string.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn Join<'a>(elems: Vec<&'a str>, sep: &'a str) -> String {
    match len!(elems) {
        0 => return "".to_owned(),
        1 => return elems[0].to_string(),
        _ => (),
    }

    let mut n = len!(sep) * (len!(elems) - 1);
    for i in 0..len!(elems) {
        n += len!(elems[i]);
    }

    let mut b = Builder::new();

    b.Grow(int!(n));
    b.WriteString(elems[0]);
    for s in elems.get(1..).unwrap() {
        b.WriteString(sep);
        b.WriteString(s);
    }
    b.String()
}

/// LastIndex returns the index of the last instance of substr in s, or -1 if substr is not present in s.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn LastIndex(s: &str, substr: &str) -> int {
    if substr == "" {
        return -1;
    }
    if let Some(i) = s.rfind(substr) {
        return int!(i);
    }
    -1
}

/// LastIndexAny returns the index of the last instance of any Unicode code point from chars in s, or -1 if no Unicode code point from chars is present in s.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn LastIndexAny(s: &str, chars: &str) -> int {
    if let Some(i) = s.rfind(chars) {
        return int!(i);
    }
    -1
}

/// LastIndexByte returns the index of the last instance of c in s, or -1 if c is not present in s.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn LastIndexByte(s: &str, c: byte) -> int {
    if let Some(i) = s.bytes().rfind(|&x| x == c) {
        return int!(i);
    }
    -1
}

/// LastIndexFunc returns the index into s of the last Unicode code point satisfying f(c), or -1 if none do.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn LastIndexFunc(s: &str, f: fn(rune) -> bool) -> int {
    for (i, r) in s.chars().rev().enumerate() {
        if f(r as u32) == true {
            return int!(i);
        }
    }
    -1
}

/// Map returns a copy of the string s with all its characters modified according to the mapping function. If mapping returns a negative value, the character is dropped from the string with no replacement.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn Map(mapping: fn(rune) -> rune, s: &str) -> &str {
    todo!()
}

/// Repeat returns a new string consisting of count copies of the string s.
///
/// It panics if count is negative or if the result of (len!(s) * count) overflows.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn Repeat(s: &str, count: uint) -> String {
    if count == 0 {
        return "".to_owned();
    }

    if len!(s) * count / count != len!(s) {
        panic!("strings: Repeat count causes overflow")
    }
    let mut n = len!(s) * count;
    let mut b = Builder::new();
    b.Grow(int!(n));
    b.WriteString(s);
    while (b.Len() <= int!(n)) {
        if b.Len() <= int!(n) / 2 {
            b.WriteString(b.String().as_str());
        } else {
            b.WriteString(b.String().as_str());
            break;
        }
    }
    b.String()
}

/// Replace returns a copy of the string s with the first n non-overlapping instances of old replaced by new. If old is empty, it matches at the beginning of the string and after each UTF-8 sequence, yielding up to k+1 replacements for a k-rune string. If n < 0, there is no limit on the number of replacements.
/// It panics if count is negative or if the result of (len!(s) * count) overflows.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn Replace<'a>(s: &'a str, old: &str, new: &str, n: int) -> &'a str {
    todo!()
}

/// ReplaceAll returns a copy of the string s with all non-overlapping instances of old replaced by new. If old is empty, it matches at the beginning of the string and after each UTF-8 sequence, yielding up to k+1 replacements for a k-rune string.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn ReplaceAll<'a>(s: &'a str, old: &str, new: &str) -> &'a str {
    todo!()
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
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn Split<'a>(s: &'a str, sep: &'a str) -> Vec<&'a str> {
    todo!()
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
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn SplitAfter<'a>(s: &'a str, sep: &str) -> Vec<&'a str> {
    todo!()
}

/// SplitAfterN slices s into substrings after each instance of sep and returns a slice of those substrings.
///
/// The count determines the number of substrings to return:
/// ```text
/// n > 0: at most n substrings; the last substring will be the unsplit remainder.
/// n == 0: the result is nil (zero substrings)
/// n < 0: all substrings
/// ```
/// Edge cases for s and sep (for example, empty strings) are handled as described in the documentation for SplitAfter.
pub fn SplitAfterN<'a>(s: &'a str, sep: &str, n: int) -> Vec<&'a str> {
    todo!()
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
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn SplitN<'a>(s: &'a str, sep: &str, n: int) -> Vec<&'a str> {
    todo!()
}

/// Title returns a copy of the string s with all Unicode letters that begin words mapped to their Unicode title case.
///
/// BUG(rsc): The rule Title uses for word boundaries does not handle Unicode punctuation properly.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn Title(s: &str) -> &str {
    todo!()
}

/// ToLower returns s with all Unicode letters mapped to their lower case.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn ToLower(s: &str) -> &str {
    todo!()
}

/// ToLowerSpecial returns a copy of the string s with all Unicode letters mapped to their lower case using the case mapping specified by c.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
/* pub fn ToLowerSpecial(c: unicode.SpecialCase, s:&str)->&str{
todo!()
}  */

/// ToTitle returns a copy of the string s with all Unicode letters mapped to their Unicode title case.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn ToTitle(s: &str) -> &str {
    todo!()
}

/// ToTitleSpecial returns a copy of the string s with all Unicode letters mapped to their Unicode title case, giving priority to the special casing rules.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
/* pub fn ToTitleSpecial(c: unicode.SpecialCase, s: &str)->&str{ */
/* todo!() */
/* } */

/// ToUpper returns s with all Unicode letters mapped to their upper case.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn ToUpper(s: &str) -> &str {
    todo!()
}

/// ToValidUTF8 returns a copy of the string s with each run of invalid UTF-8 byte sequences replaced by the replacement string, which may be empty.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn ToValidUTF8<'a>(s: &'a str, replacement: &str) -> &'a str {
    todo!()
}

/// Trim returns a slice of the string s with all leading and trailing Unicode code points contained in cutset removed.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn Trim<'a>(s: &'a str, cutset: &str) -> &'a str {
    todo!()
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
///
/// ```
pub fn TrimFunc(s: &str, f: fn(rune) -> bool) -> &str {
    todo!()
}

/// TrimLeft returns a slice of the string s with all leading Unicode code points contained in cutset removed.
///
/// To remove a prefix, use TrimPrefix instead.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn TrimLeft<'a>(s: &'a str, cutset: &str) -> &'a str {
    todo!()
}

/// TrimLeftFunc returns a slice of the string s with all leading Unicode code points c satisfying f(c) removed.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn TrimLeftFunc(s: &str, f: fn(rune) -> bool) -> &str {
    todo!()
}

/// TrimPrefix returns s without the provided leading prefix string. If s doesn't start with prefix, s is returned unchanged.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn TrimPrefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    todo!()
}

/// TrimRight returns a slice of the string s, with all trailing Unicode code points contained in cutset removed.
///
/// To remove a suffix, use TrimSuffix instead.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn TrimRight<'a>(s: &'a str, cutset: &str) -> &'a str {
    todo!()
}

/// TrimRightFunc returns a slice of the string s with all trailing Unicode code points c satisfying f(c) removed.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn TrimRightFunc(s: &str, f: fn(rune) -> bool) -> &str {
    todo!()
}

/// TrimSpace returns a slice of the string s, with all leading and trailing white space removed, as defined by Unicode.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn TrimSpace(s: &str) -> &str {
    todo!()
}

/// TrimSuffix returns s without the provided trailing suffix string. If s doesn't end with suffix, s is returned unchanged.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
///
/// ```
pub fn TrimSuffix<'a>(s: &'a str, suffix: &str) -> &'a str {
    todo!()
}

// Builder Begin

/// A Builder is used to efficiently build a string using Write methods. It minimizes memory copying. The zero value is ready to use. Do not copy a non-zero Builder.
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
///
/// </details>
///
/// # Example
///
/// ```
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
    ///
    /// </details>
    pub fn new() -> Builder {
        let mut b = Builder::default();
        b.addr = Box::new(Some(b.clone()));
        b
    }

    /// Cap returns the capacity of the builder's underlying byte slice. It is the total space allocated for the string being built and includes any bytes already written.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    ///
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
    ///
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
    ///
    /// </details>
    pub fn Len(&self) -> int {
        int!(len!(self.buf))
    }

    /// Reset resets the Builder to be empty.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    ///
    /// </details>
    pub fn Reset(&mut self) {
        self.addr = Box::new(None);
        self.buf = Vec::new();
    }

    /// String returns the accumulated string.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    ///
    /// </details>
    pub fn String<'a>(&self) -> String {
        String::from_utf8(self.buf.clone()).unwrap()
    }

    /// Write appends the contents of p to b's buffer. Write always returns len!(p), nil.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    ///
    /// </details>
    pub fn Write<'a>(&mut self, p: Vec<byte>) -> int {
        self.buf.extend_from_slice(p.as_slice());
        int!(len!(p))
    }

    /// WriteByte appends the byte c to b's buffer. The returned error is always nil.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    ///
    /// </details>
    pub fn WriteByte(&mut self, c: byte) {
        self.buf.push(c)
    }

    /// WriteRune appends the UTF-8 encoding of Unicode code point r to b's buffer. It returns the length of r and a nil error.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    ///
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

    /// WriteString appends the contents of s to b's buffer. It returns the length of s and a nil error.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    ///
    /// </details>
    pub fn WriteString(&mut self, s: &str) -> Result<int, &str> {
        self.buf.append(s.as_bytes().to_vec().as_mut());
        return Ok(len!(self.buf) as int);
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
pub struct Reader {
    s: string,
    i: int64,      // current reading index
    prevRune: int, // index of previous rune; or < 0
}

impl Reader {
    /// NewReader returns a new Reader reading from s. It is similar to bytes.NewBufferString but more efficient and read-only.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    ///
    /// </details>
    pub fn new() -> Reader {
        todo!()
    }

    /// Len returns the number of bytes of the unread portion of the string.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    ///
    /// </details>
    pub fn Len(&self) -> int {
        todo!()
    }

    ///  Reset resets the Reader to be reading from s.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    ///
    /// </details>
    pub fn Reset(&self, s: &str) {
        todo!()
    }

    /// Size returns the original length of the underlying string. Size is the number of bytes available for reading via ReadAt. The returned value is always the same and is not affected by calls to any other method.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    ///
    /// </details>
    pub fn Size(&self) -> int64 {
        todo!()
    }
}

impl io::Reader for Reader {
    /// Read implements the io.Reader interface.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    ///
    /// </details>
    fn Read(&self, b: Vec<byte>) -> Result<int, &str> {
        todo!()
    }
}

impl io::ReaderAt for Reader {
    fn ReadAt(&self, b: Vec<byte>, off: int64) -> Result<int, &str> {
        todo!()
    }
}

impl io::ByteReader for Reader {
    fn ReadByte(&self) -> Result<byte, &str> {
        todo!()
    }
}

impl io::RuneReader for Reader {
    fn ReadRune(&self) -> Result<(rune, int), &str> {
        todo!()
    }
}

impl io::Seeker for Reader {
    fn Seek(&self, offset: int64, whence: int) -> Result<int64, &str> {
        todo!()
    }
}

impl io::ByteScanner for Reader {
    fn UnreadByte(&self) -> Result<int, &str> {
        todo!()
    }
}

impl io::WriterTo for Reader {
    fn WriteTo(&self, w: Box<dyn io::Writer>) -> Result<int64, &str> {
        todo!()
    }
}

trait replacer {
    fn Replace(&self, s: &str) -> &str
    where
        Self: Sized;
    fn WriteString(&self, w: Box<dyn io::Writer>, s: string) -> Result<int, &str>
    where
        Self: Sized;
}

use std::sync::Once;
/// NewReplacer returns a new Replacer from a list of old, new string pairs. Replacements are performed in the order they appear in the target string, without overlapping matches. The old string comparisons are done in argument order.

/// NewReplacer panics if given an odd number of arguments.
struct Replacer<'a> {
    once: Once, // guards buildOnce method
    r: Box<dyn replacer>,
    oldnew: Vec<&'a str>,
}

impl<'a> Replacer<'a> {
    /// NewReplacer returns a new Replacer from a list of old, new string pairs. Replacements are performed in the order they appear in the target string, without overlapping matches. The old string comparisons are done in argument order.
    ///
    /// NewReplacer panics if given an odd number of arguments.
    pub fn NewReplacer(oldnew: &str) -> &Replacer {
        todo!()
    }

    /// Replace returns a copy of s with all replacements performed.
    pub fn Replace(&self, s: &str) -> &str {
        todo!()
    }

    /// WriteString writes s to w with all replacements performed.
    pub fn WriteString(&self, w: Box<dyn io::Writer>, s: &str) -> Result<int, &str> {
        todo!()
    }
}
