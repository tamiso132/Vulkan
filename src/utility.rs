use anyhow::Result;
use std::{
    ffi::CStr,
    fs::{self, File},
    io::{BufReader, Read},
    os::raw::c_char,
};

pub fn vk_to_string(raw_string_array: &[c_char]) -> String {
    let raw_string = unsafe {
        let pointer = raw_string_array.as_ptr();
        CStr::from_ptr(pointer)
    };

    raw_string.to_str().expect("Failed to convert vulkan raw string.").to_owned()
}

pub fn read_file(file_path: &str) -> Result<()> {
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let mut buffer = vec![];
    buf_reader.buffer().read_to_end(&mut buffer)?;
    Ok(())
}
