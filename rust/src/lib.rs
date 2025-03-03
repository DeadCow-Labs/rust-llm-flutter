mod model;
mod inference;
mod downloader;
mod tokenizer;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use model::load_model;
use inference::run_inference;
use downloader::download_model;

#[no_mangle]
pub extern "C" fn load_model_ffi(model_name: *const c_char) -> *mut c_char {
    let model_str = unsafe { CStr::from_ptr(model_name).to_str().unwrap() };
    
    match load_model(model_str) {
        Ok(_) => CString::new("Model loaded successfully").unwrap().into_raw(),
        Err(e) => CString::new(format!("Error: {:?}", e)).unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn run_inference_ffi(input_text: *const c_char) -> *mut c_char {
    let input = unsafe { CStr::from_ptr(input_text).to_str().unwrap() };
    
    match run_inference(input) {
        Ok(output) => CString::new(output).unwrap().into_raw(),
        Err(e) => CString::new(format!("Error: {:?}", e)).unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn download_model_ffi(model_name: *const c_char) -> *mut c_char {
    let model_str = unsafe { CStr::from_ptr(model_name).to_str().unwrap() };

    match download_model(model_str) {
        Ok(_) => CString::new("Download complete").unwrap().into_raw(),
        Err(e) => CString::new(format!("Download failed: {:?}", e)).unwrap().into_raw(),
    }
}
