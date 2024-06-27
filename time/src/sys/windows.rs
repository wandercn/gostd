use cvt::cvt;
use gostd_builtin::*;
use std::mem;
use std::num::Wrapping;
use winapi::shared::minwindef::FILETIME;
use winapi::shared::ntdef::LARGE_INTEGER;
use winapi::um::profileapi::{QueryPerformanceCounter, QueryPerformanceFrequency};
use winapi::um::sysinfoapi::GetSystemTimePreciseAsFileTime;

const EPOCH_DIFFERENCE: u64 = 11644473600; // windos时间戳和Unix时间戳的差值

#[cfg(windows)]
pub fn monotonic_now() -> uint64 {
    let mut frequency: LARGE_INTEGER = unsafe { mem::zeroed() };
    let mut counter: LARGE_INTEGER = unsafe { mem::zeroed() };

    cvt(unsafe { QueryPerformanceFrequency(&mut frequency as *mut _) }).unwrap();
    cvt(unsafe { QueryPerformanceCounter(&mut counter as *mut _) }).unwrap();
    let frequency_u64 = uint64!(unsafe { *frequency.QuadPart() });
    let counter_u64 = uint64!(unsafe { *counter.QuadPart() });
    let nanoseconds = Wrapping(counter_u64) * Wrapping(1_000_000_000) / Wrapping(frequency_u64); // Wrapping 处理算数溢出问题

    uint64!(nanoseconds.0)
}

#[cfg(windows)]
pub fn real_time_now() -> (uint64, uint64) {
    let mut ft: FILETIME = unsafe { mem::zeroed() };

    unsafe {
        GetSystemTimePreciseAsFileTime(&mut ft);
    }
    let mut li: LARGE_INTEGER = unsafe { mem::zeroed() };
    li.LowPart = ft.dwLowDateTime;
    li.HighPart = ft.dwHighDateTime;
    let nanoseconds = unsafe { *li.QuadPart() } * 100;
    let seconds = nanoseconds / 1_000_000_000 - EPOCH_DIFFERENCE;
    let nanoseconds = nanoseconds % 1_000_000_000;

    (uint64!(seconds), uint64!(nanoseconds))
}
