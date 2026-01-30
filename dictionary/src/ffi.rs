use core::ptr::null_mut;
use libc::{free, malloc};
use std::ptr;

use crate::{Dictionary, MyString};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_new() -> *mut Dictionary {
    unsafe {
        let mem = malloc(core::mem::size_of::<Dictionary>()) as *mut Dictionary;
        if mem.is_null() {
            return null_mut();
        }

        ptr::write(mem, Dictionary::new());
        mem
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_insert(dict: *mut Dictionary, key: u64, val: *const u8, len: usize) {
    unsafe {
        if dict.is_null() {
            return;
        }
        if val.is_null() && len != 0 {
            return;
        }

        let slice = core::slice::from_raw_parts(val, len);
        let s = core::str::from_utf8_unchecked(slice);
        (*dict).insert(key, MyString::from_str(s));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_contains(dict: *mut Dictionary, key: u64) -> bool {
    unsafe {
        if dict.is_null() {
            return false;
        }
        (*dict).contains(key)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_get(
    dict: *mut Dictionary,
    key: u64,
    out_len: *mut usize,
) -> *const u8 {
    unsafe {
        if !out_len.is_null() {
            *out_len = 0;
        }
        if dict.is_null() || out_len.is_null() {
            return null_mut();
        }

        match (*dict).get(key) {
            Some(s) => {
                *out_len = s.len();
                s.ptr as *const u8
            }
            None => null_mut(),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_remove(dict: *mut Dictionary, key: u64) {
    unsafe {
        if dict.is_null() {
            return;
        }
        (*dict).remove(key);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_free(dict: *mut Dictionary) {
    unsafe {
        if dict.is_null() {
            return;
        }

        ptr::drop_in_place(dict);

        free(dict as *mut _);
    }
}
