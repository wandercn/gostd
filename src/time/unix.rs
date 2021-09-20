use cvt::cvt;
use std::ptr;
pub fn monotonic_now() -> u64 {
    extern "C" {
        fn mach_absolute_time() -> u64;
    }
    unsafe { mach_absolute_time() }
}

pub fn real_time_now() -> (u64, u64) {
    let mut t = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    cvt(unsafe { libc::clock_gettime(libc::CLOCK_REALTIME, &mut t) }).unwrap();

    (t.tv_sec as u64, t.tv_nsec as u64)
}
