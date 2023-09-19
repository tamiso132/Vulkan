use anyhow::Result;
use std::{
    ffi::CStr,
    fs::{self, File},
    io::{BufReader, Read},
    os::raw::c_char,
};

use crate::constant::PATH_TO_PROJECT;

pub fn vk_to_string(raw_string_array: &[c_char]) -> String {
    let raw_string = unsafe {
        let pointer = raw_string_array.as_ptr();
        CStr::from_ptr(pointer)
    };

    raw_string.to_str().expect("Failed to convert vulkan raw string.").to_owned()
}

pub fn read_file(file_path: &str) -> Result<Vec<u8>> {
    let path = format!("{}{}", PATH_TO_PROJECT.to_string(), file_path);

    let mut file = File::open(file_path)?;
    let file_length = file.metadata()?.len();
    let mut buffer = Vec::with_capacity(file_length as usize);
    file.read_to_end(&mut buffer)?;

    println!("bytes: {}", buffer.len());
    println!("Constant: {}", path);
    Ok(buffer)
}
