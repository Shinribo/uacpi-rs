use alloc::string::String;
use core::ffi::{c_char, c_int, c_void};
use printf_compat::{format, output};

#[no_mangle]
pub unsafe extern "C" fn __snprintf_chk(
    s: *mut c_char,
    max_len: usize,
    _flags: c_int,
    len: usize,
    fmt: *const c_char,
    mut va: ...
) -> c_int {
    if max_len < len {
        panic!("Buffer overflow in __snprintf_chk");
    }

    let mut out = String::new();
    let done = format(fmt, va.as_va_list(), output::fmt_write(&mut out));

    core::ptr::copy_nonoverlapping(out.as_ptr(), s.cast(), max_len);

    done
}

#[no_mangle]
pub unsafe extern "C" fn snprintf(
    s: *mut c_char,
    len: usize,
    fmt: *const c_char,
    mut va: ...
) -> c_int {
    let mut out = String::new();
    let done = format(fmt, va.as_va_list(), output::fmt_write(&mut out));

    core::ptr::copy_nonoverlapping(out.as_ptr(), s.cast(), len);

    done
}

#[no_mangle]
pub unsafe extern "C" fn __memcpy_chk(
    dst: *mut c_void,
    src: *const c_void,
    len: usize,
    dest_len: usize,
) -> *mut c_void {
    if dest_len < len {
        panic!("Buffer overflow in __memcpy_chk");
    }

    core::ptr::copy_nonoverlapping(src, dst, len);

    dst
}

#[no_mangle]
pub unsafe extern "C" fn strcmp(s1: *const c_char, s2: *const c_char) -> usize {
    for i in 0.. {
        let s1_i = s1.add(i);
        let s2_i = s2.add(i);

        let val = *s1_i - *s2_i;
        if val != 0 || *s1_i == 0 {
            return val as usize;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> usize {
    for i in 0..n {
        let s1_i = s1.add(i);
        let s2_i = s2.add(i);

        let val = *s1_i - *s2_i;
        if val != 0 || *s1_i == 0 {
            return val as usize;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn strnlen(mut s: *const c_char, max_len: usize) -> usize {
    let mut result = 0;
    while *s != 0 && result < max_len {
        s = s.add(1);
        result += 1;
    }

    result
}
