use cvt::cvt;
use gostd_builtin::*;
use std::ptr;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub fn monotonic_now() -> uint64 {
    extern "C" {
        fn mach_absolute_time() -> uint64;
    }
    unsafe { mach_absolute_time() }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub fn real_time_now() -> (uint64, uint64) {
    let mut t = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    cvt(unsafe { libc::clock_gettime(libc::CLOCK_REALTIME, &mut t) }).unwrap();

    (uint64!(t.tv_sec.abs()), uint64!(t.tv_nsec.abs()))
}
