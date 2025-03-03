mod model;
mod inference;
mod downloader;
mod tokenizer;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use model::load_model;
use downloader::download_if_needed;

#[no_mangle]
pub extern "C" fn load_model_ffi(model_name: *const c_char) -> *mut c_char {
    let model_str = unsafe { CStr::from_ptr(model_name).to_str().unwrap() };
    
    match load_model(model_str) {
        Ok(_) => CString::new("Model loaded successfully").unwrap().into_raw(),
        Err(e) => CString::new(format!("Error: {:?}", e)).unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn run_inference_c(input: *const c_char) -> *mut c_char {
    let input_str = unsafe {
        if input.is_null() {
            return CString::new("Error: Null input")
                .unwrap_or_default()
                .into_raw();
        }
        match CStr::from_ptr(input).to_str() {
            Ok(s) => s,
            Err(_) => return CString::new("Error: Invalid UTF-8")
                .unwrap_or_default()
                .into_raw(),
        }
    };
    
    match inference::run_inference(input_str) {
        Ok(output) => CString::new(output)
            .unwrap_or_default()
            .into_raw(),
        Err(e) => CString::new(format!("Error: {}", e))
            .unwrap_or_default()
            .into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn download_model_ffi(model_name: *const c_char) -> *mut c_char {
    let model_str = unsafe { CStr::from_ptr(model_name).to_str().unwrap() };
    let save_path = std::path::Path::new("models").join(model_str);

    match download_if_needed(model_str, "model.safetensors", &save_path) {
        Ok(_) => CString::new("Download complete").unwrap().into_raw(),
        Err(e) => CString::new(format!("Download failed: {:?}", e)).unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    unsafe {
        if !ptr.is_null() {
            let _ = CString::from_raw(ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn tokenize_text_c(text: *const c_char, length: *mut usize) -> *mut u32 {
    let text_str = unsafe {
        if text.is_null() || length.is_null() {
            return std::ptr::null_mut();
        }
        match CStr::from_ptr(text).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        }
    };

    match tokenizer::tokenize(text_str) {
        Ok(tokens) => {
            unsafe { *length = tokens.len() };
            let mut vec = tokens.into_boxed_slice();
            let ptr = vec.as_mut_ptr();
            std::mem::forget(vec);
            ptr
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn free_array(ptr: *mut u32, length: usize) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(std::slice::from_raw_parts_mut(ptr, length));
        }
    }
}
