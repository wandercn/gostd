use crate::builtin::*;
use cvt::cvt;
use std::ptr;
#[cfg(unix)]
pub fn monotonic_now() -> uint64 {
    let mut t = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    cvt(unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut t) }).unwrap();

    uint64!(t.tv_nsec)
}

#[cfg(unix)]
pub fn real_time_now() -> (uint64, uint64) {
    let mut t = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    cvt(unsafe { libc::clock_gettime(libc::CLOCK_REALTIME, &mut t) }).unwrap();

    (uint64!(t.tv_sec), uint64!(t.tv_nsec))
}
