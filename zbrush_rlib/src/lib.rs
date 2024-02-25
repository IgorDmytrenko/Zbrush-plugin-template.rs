/* 
    RUST LIBRARY FOR ZBRUSH
    Author: Ihor Dmytrenko
    License: MIT
    Version: 0.1.0
    Description: This is a template Rust library that 
                 can be called from ZBrush using the 
                 FileExecute command.
*/

use std::ptr;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_float, c_int};

#[no_mangle]
pub extern "C" fn get_version(
        _optional_text_input: *const c_char, 
        _optional_number_input: c_double, // float number
        _input_memory_block: *mut c_char, 
        _output_memory_block: *mut c_char ) -> c_float {

    5i32 as c_float
}

#[no_mangle]
pub extern "C" fn get_string(
    _optional_text_input: *const c_char, 
    _optional_number_input: f64, // float number
    _input_memory_block: *mut c_char, 
    _output_memory_block: *mut c_char ) {

    let z_string = CString::new("I your string from external rust dylib").unwrap();
    let bytes = z_string.as_bytes_with_nul();

    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), _input_memory_block as *mut u8, bytes.len());
    }
}

#[no_mangle]
pub extern "C" fn modify_string(
    _optional_text_input: *const c_char, 
    _optional_number_input: c_double, // float number
    _input_memory_block: *mut c_char, 
    _output_memory_block: *mut c_char ) -> c_int {
    
    // Get data from _input_memory_block   
    let input_block = unsafe { CStr::from_ptr(_optional_text_input).to_str().unwrap() };
    // Get data from _optional_text_input
    let message_body = format!("I've recived you string and returned it back:\n{}", input_block);
    let c_string = CString::new(message_body.as_bytes()).unwrap();
 
    // Write data to the output memory block
    unsafe {
        ptr::copy_nonoverlapping(
            c_string.as_ptr(), 
            _input_memory_block, 
            c_string.as_bytes().len()
        );
    }
    0 // Success
}
