#![doc(html_playground_url = "https://play.rust-lang.org/")]
/// 保持兼容性: use gostd::builtin 等同于 use gostd_builtin
pub use gostd_builtin as builtin;
/// 保存兼容性: use gostd::time 等同于 use gostd_time;
pub use gostd_time as time;
pub mod archive;
pub mod bufio;
pub use gostd_bytes as bytes;
pub mod cmd;
pub mod compress;
pub mod container;
pub mod context;
pub mod crypto;
pub mod database;
pub mod debug;
pub mod embed;
pub mod encoding;
pub mod errors;
pub mod expvar;
pub mod flag;
pub mod fmt;
pub mod go;
pub mod gounsafe;
pub mod hash;
pub mod html;
pub mod image;
pub mod index;
pub mod internal;
pub use gostd_io as io;
pub mod log;
pub mod math;
pub mod mime;
pub mod net;
pub mod os;
pub mod path;
pub mod plugin;
pub mod reflect;
pub mod regexp;
pub mod runtime;
pub mod sort;
pub mod strconv;
pub use gostd_strings as strings;
pub mod sync;
pub mod syscall;
pub mod testdata;
pub mod testing;
pub mod text;
pub use gostd_unicode as unicode;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
