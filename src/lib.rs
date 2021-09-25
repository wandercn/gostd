pub mod archive;
pub mod bufio;
/// 简单实现go的builtin
#[macro_use]
pub mod builtin;
extern crate lazy_static;
pub mod bytes;
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
pub mod io;
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
pub mod strings;
pub mod sync;
pub mod syscall;
pub mod testdata;
pub mod testing;
pub mod text;
pub mod time;
pub mod unicode;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
