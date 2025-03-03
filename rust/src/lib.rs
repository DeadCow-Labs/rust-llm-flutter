mod model;
mod inference;
mod downloader;
mod tokenizer;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Once;
use std::sync::Mutex;
use lazy_static::lazy_static;
use model::{Model, load_model};
use inference::run_inference;
use downloader::download_if_needed;

// Global model instance
lazy_static! {
    pub(crate) static ref MODEL: Mutex<Option<Model>> = Mutex::new(None);
}
static INIT: Once = Once::new();

#[no_mangle]
pub extern "C" fn load_model_c(model_name: *const c_char) -> *mut c_char {
    let model_str = unsafe { 
        if model_name.is_null() {
            return CString::new("Model name is null").unwrap().into_raw();
        }
        match CStr::from_ptr(model_name).to_str() {
            Ok(s) => s,
            Err(_) => return CString::new("Invalid model name string").unwrap().into_raw(),
        }
    };
    
    println!("Loading model: {}", model_str);
    
    // Clear existing model first
    {
        let mut model_ref = match MODEL.lock() {
            Ok(guard) => guard,
            Err(_) => return CString::new("Failed to acquire model lock").unwrap().into_raw(),
        };
        *model_ref = None;  // Clear existing model
    }
    
    // Load new model
    match load_model(model_str) {
        Ok(model) => {
            let mut model_ref = MODEL.lock().unwrap();
            *model_ref = Some(model);
            println!("Model loaded successfully");
            CString::new("Model loaded successfully").unwrap().into_raw()
        },
        Err(e) => {
            println!("Error loading model: {:?}", e);
            CString::new(format!("Failed to load model: {:?}", e)).unwrap().into_raw()
        }
    }
}

#[no_mangle]
pub extern "C" fn run_inference_c(input: *const c_char) -> *mut c_char {
    let input_str = unsafe {
        if input.is_null() {
            return CString::new("Input is null").unwrap().into_raw();
        }
        match CStr::from_ptr(input).to_str() {
            Ok(s) => s,
            Err(_) => return CString::new("Invalid input string").unwrap().into_raw(),
        }
    };

    println!("Running inference with input: {}", input_str);

    match run_inference(input_str) {
        Ok(output) => CString::new(output).unwrap().into_raw(),
        Err(e) => CString::new(format!("Inference error: {:?}", e)).unwrap().into_raw(),
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
