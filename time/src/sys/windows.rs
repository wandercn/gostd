use cvt::cvt;
use gostd_builtin::*;
use std::mem;
use std::num::Wrapping;
use winapi::shared::minwindef::FILETIME;
use winapi::shared::ntdef::LARGE_INTEGER;
use winapi::um::profileapi::{QueryPerformanceCounter, QueryPerformanceFrequency};
use winapi::um::sysinfoapi::GetSystemTimePreciseAsFileTime;

const EPOCH_DIFFERENCE: u64 = 11644473600; // windos时间戳和Unix时间戳的差值以秒为单位

#[cfg(windows)]
pub fn monotonic_now() -> uint64 {
    let mut frequency: LARGE_INTEGER = unsafe { mem::zeroed() };
    let mut counter: LARGE_INTEGER = unsafe { mem::zeroed() };

    cvt(unsafe { QueryPerformanceFrequency(&mut frequency as *mut _) }).unwrap();
    cvt(unsafe { QueryPerformanceCounter(&mut counter as *mut _) }).unwrap();
    let frequency_u64 = uint64!(unsafe { *frequency.QuadPart() }); // 每秒钟周期数
    let counter_u64 = uint64!(unsafe { *counter.QuadPart() }); // 总共计算周期数
    let nanoseconds = counter_u64 / frequency_u64 * 1_000_000_000; //获得从系统启动以来的时间间隔信息以纳秒为单位

    uint64!(nanoseconds)
}

#[cfg(windows)]
pub fn real_time_now() -> (uint64, uint64) {
    let mut t: FILETIME = unsafe { mem::zeroed() };

    unsafe {
        GetSystemTimePreciseAsFileTime(&mut t);
    }
    let u1 = (uint64!(t.dwHighDateTime)) << 32;
    let u2 = (uint64!(t.dwLowDateTime));
    let nanoseconds = (u1 | u2) * 100;
    let seconds = nanoseconds / 1_000_000_000 - EPOCH_DIFFERENCE;
    let nanoseconds = nanoseconds % 1_000_000_000;

    (uint64!(seconds), uint64!(nanoseconds))
}
