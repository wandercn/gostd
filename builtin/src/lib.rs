//! Package builtin binds the basic type in go through the type alias, and implements the basic type cast macro function.
//! <details class="rustdoc-toggle top-doc">
//! <summary class="docblock">zh-cn</summary>
//! builtin 包通过类型别名绑定Go中的基础类型，并实现了基础类型强制转换宏函数。
//! </details>
//!
// prelude
// 导出宏函数 byte!(),int8!()等.避免用gostd::byte!()，可以直接byte!()调用。
#[macro_use]
mod macros;
pub use crate::macros::*;
