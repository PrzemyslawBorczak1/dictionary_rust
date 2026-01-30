use core::ptr::null_mut;
use libc::{free, malloc};

#[repr(C)]
pub struct MyString {
    pub ptr: *mut u8,
    pub len: usize,
}

impl MyString {
    pub fn from_str(s: &str) -> Self {
        unsafe {
            let len = s.len();
            let mem = malloc(len) as *mut u8;
            if mem.is_null() {
                return Self {
                    ptr: null_mut(),
                    len: 0,
                };
            }
            core::ptr::copy_nonoverlapping(s.as_ptr(), mem, len);
            Self { ptr: mem, len }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            let slice = core::slice::from_raw_parts(self.ptr, self.len);
            core::str::from_utf8_unchecked(slice)
        }
    }
}

impl Drop for MyString {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                free(self.ptr as *mut _);
            }
        }
    }
}
