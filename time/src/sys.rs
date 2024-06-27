use gostd_builtin::*;
pub fn monotonic_now() -> uint64 {
    let mon_now = inner::monotonic_now();
    dbg!("mon_now: {}", mon_now);
    panic!("");
    mon_now
}

pub fn real_time_now() -> (uint64, uint64) {
    let real_now = inner::real_time_now();
    dbg!("real_now: {:?}", real_now);
    real_now
}

#[cfg(all(all(unix), not(target_os = "macos")))]
#[path = "sys/unix.rs"]
mod inner;

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[path = "sys/darwin.rs"]
mod inner;

#[cfg(any(target_os = "windows"))]
#[path = "sys/windows.rs"]
mod inner;
