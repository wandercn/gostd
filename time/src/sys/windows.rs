use cvt::cvt;
use gostd_builtin::*;
use std::mem;
use std::num::Wrapping;
use winapi::shared::minwindef::FILETIME;
use winapi::shared::ntdef::LARGE_INTEGER;
use winapi::um::profileapi::{QueryPerformanceCounter, QueryPerformanceFrequency};
use winapi::um::sysinfoapi::GetSystemTimePreciseAsFileTime;
#[cfg(windows)]
pub fn monotonic_now() -> uint64 {
    let mut frequency: LARGE_INTEGER = unsafe { mem::zeroed() };
    let mut counter: LARGE_INTEGER = unsafe { mem::zeroed() };

    cvt(unsafe { QueryPerformanceFrequency(&mut frequency as *mut _) }).unwrap();
    cvt(unsafe { QueryPerformanceCounter(&mut counter as *mut _) }).unwrap();
    let frequency_u64 = uint64!(unsafe { *frequency.QuadPart() });
    let counter_u64 = uint64!(unsafe { *counter.QuadPart() });
    println!("frequency_u64: {}", frequency_u64);
    println!("counter_u64: {}", counter_u64);
    let nanoseconds = Wrapping(counter_u64) * Wrapping(1_000_000_000) / Wrapping(frequency_u64);

    uint64!(nanoseconds.0)
}

#[cfg(windows)]
pub fn real_time_now() -> (uint64, uint64) {
    let mut t: FILETIME = unsafe { mem::zeroed() };

    unsafe {
        GetSystemTimePreciseAsFileTime(&mut t);
    }
    println!("hight: {}", t.dwHighDateTime);
    println!("low: {}", t.dwLowDateTime);
    let u1 = (uint64!(t.dwHighDateTime)) << 32;
    let u2 = (uint64!(t.dwLowDateTime));
    let nanoseconds = (u1 | u2) * 100;
    let seconds = nanoseconds / 1_000_000_000;
    let nanoseconds = nanoseconds % 1_000_000_000;

    (uint64!(seconds), uint64!(nanoseconds))
}
