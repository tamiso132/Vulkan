use lazy_static::lazy_static;
use std::ffi::{CStr, CString};
type Layer<'a> = &'a CStr;

lazy_static! {
    /// Can be used to to assist developers in isolating incorrect usage, and in verifying that applications correctly use the API
    static ref KHRONOS_VALIDATION_LAYER: CString = CString::new("VK_LAYER_KHRONOS_validation").unwrap();
    /// API Version 1.3.216 needed
    /// utility layer prints API calls, parameters, and values to the identified output stream.
    static ref LUNARG_API_DUMP: CString = CString::new("VK_LAYER_LUNARG_api_dump").unwrap();

}
