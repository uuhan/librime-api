use std::{
    borrow::Cow,
    ffi::{c_char, CStr},
};

pub fn safe_text(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }

    unsafe { CStr::from_ptr(ptr).to_string_lossy().to_string() }
}

pub fn safe_text_cow<'a>(ptr: *const c_char) -> Cow<'a, str> {
    if ptr.is_null() {
        return Cow::default();
    }

    unsafe { CStr::from_ptr(ptr).to_string_lossy() }
}
