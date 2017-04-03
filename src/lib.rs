use std::os::raw::c_char;
use std::ptr::copy_nonoverlapping;

fn my_fun(in1: *const c_char, len1: usize, in2: *const c_char, len2: usize, out: *mut c_char, out_len: usize) -> usize {
    unsafe {
        copy_nonoverlapping("abc".as_ptr(), out as *mut u8, 3);
    }
    3
}

#[test]
fn test_app() {
    use std::ptr;
    use std::str::from_utf8;
    let something = "{}";
    let mut out = [0u8; 10000];
    let written = my_fun(something.as_ptr() as *const c_char, something.len(), ptr::null(), 0, &mut out[0] as *mut u8 as *mut c_char, 10000);
    let result = unsafe { Vec::from_raw_parts(&mut out[0] as *mut u8, written as usize, written as usize) };
    assert_eq!(from_utf8(result.as_ref()).unwrap(), "123");
}
