use gostd_builtin::*;
use std::mem;
use winapi::shared::minwindef::FILETIME;
use winapi::um::profileapi::{QueryPerformanceCounter, QueryPerformanceFrequency};
use winapi::um::sysinfoapi::GetSystemTimePreciseAsFileTime;

#[cfg(windows)]
pub fn monotonic_now() -> uint64 {
    let mut frequency: int64 = 0;
    let mut counter: int64 = 0;

    unsafe {
        QueryPerformanceFrequency(&mut frequency);
        QueryPerformanceCounter(&mut counter);
    }

    let frequency = uint64!(frequency);
    let counter = uint64!(counter);
    let nanoseconds = counter * 1_000_000_000 / frequency;

    uint64!(nanoseconds)
}

#[cfg(windows)]
pub fn real_time_now() -> (uint64, uint64) {
    let mut t: FILETIME = unsafe { mem::zeroed() };

    unsafe {
        GetSystemTimePreciseAsFileTime(&mut t);
    }

    let nanoseconds = ((uint64!(t.dwHighDateTime)) << 32 | (uint64!(t.dwLowDateTime))) * 100;
    let seconds = nanoseconds / 1_000_000_000;
    let nanoseconds = nanoseconds % 1_000_000_000;

    (uint64!(seconds), uint64!(nanoseconds))
}
