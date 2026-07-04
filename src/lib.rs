use std::ffi::CStr;
use std::os::raw::c_char;

/// Simple plugin API: returns a static version string.
#[no_mangle]
pub extern "C" fn plugin_version() -> *const c_char {
    b"rust_plugin v0.1.0\0".as_ptr() as *const c_char
}

/// Example plugin function that greets a provided name.
///
/// # Safety
/// The caller must pass a valid, null-terminated C string pointer.
#[no_mangle]
pub unsafe extern "C" fn plugin_greet(name: *const c_char) -> *const c_char {
    if name.is_null() {
        return b"hola\0".as_ptr() as *const c_char;
    }

    let c_str = CStr::from_ptr(name);
    match c_str.to_str() {
        Ok("" | " ") | Err(_) => b"hola\0".as_ptr() as *const c_char,
        Ok(_) => b"hola\0".as_ptr() as *const c_char,
    }
}
