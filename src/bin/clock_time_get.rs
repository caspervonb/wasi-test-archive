extern crate wasi;

use wasi::*;

unsafe fn test_clock_time_get(clockid : Clockid) {
    let result = clock_time_get(clockid, 0);
    assert!(result.is_ok());
    assert!(result.unwrap() > 0);

    let result = clock_time_get(clockid, 1);
    assert!(result.is_ok());
    assert!(result.unwrap() > 0);
}

fn main() {
    unsafe {
        test_clock_time_get(CLOCKID_REALTIME);
        test_clock_time_get(CLOCKID_MONOTONIC);
        test_clock_time_get(CLOCKID_PROCESS_CPUTIME_ID);
        test_clock_time_get(CLOCKID_THREAD_CPUTIME_ID);
    }
}
