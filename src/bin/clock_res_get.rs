extern crate wasi;

use wasi::*;

unsafe fn test_clock_res_get(clockid : Clockid) {
    let result = clock_res_get(clockid, 0);
    assert!(result.is_ok());
    assert!(result.unwrap() > 0);
}

fn main() {
    unsafe {
        test_clock_res_get(CLOCKID_REALres);
        test_clock_res_get(CLOCKID_MONOTONIC);
        test_clock_res_get(CLOCKID_PROCESS_CPUres_ID);
        test_clock_res_get(CLOCKID_THREAD_CPUres_ID);
    }
}
