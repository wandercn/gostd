use crate::builtin::*;
pub fn monotonic_now() -> uint64 {
    inner::monotonic_now()
}

pub fn real_time_now() -> (uint64, uint64) {
    inner::real_time_now()
}

#[cfg(all(all(unix), not(target_os = "macos")))]
#[path = "sys/unix.rs"]
mod inner;

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[path = "sys/darwin.rs"]
mod inner;
