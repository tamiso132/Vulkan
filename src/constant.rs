pub mod version {
    use ash::vk;

    pub const APPLICATION_VERSION: u32 = vk::make_api_version(0, 1, 3, 0);
    pub const ENGINE_VERSION: u32 = vk::make_api_version(0, 1, 3, 0);
    pub const API_VERSION: u32 = vk::make_api_version(0, 1, 3, 0);
}
pub mod validation {
    pub const ENABLED: bool = cfg!(debug_assertions);
    pub const LAYER_NAME: &'static str = "VK_LAYER_KHRONOS_validation";
    pub const LAYER_NAME_BYTES: &[u8; 28] = b"VK_LAYER_KHRONOS_validation\0";
}

pub mod support {
    use std::ffi::CStr;

    pub const EXTENSION_SUPPORT_ARRAY_BYTES: &[&[u8]] = &[ash::extensions::khr::Swapchain::name().to_bytes()];
    pub const EXTENSION_SUPPORT_ARRAY_NAME: &[&'static CStr] = &[ash::extensions::khr::Swapchain::name()];
}

pub mod Window_Info {
    pub const HEIGHT: u32 = 900;
    pub const WIDTH: u32 = 900;
}

lazy_static::lazy_static! {
    pub static ref PATH_TO_PROJECT: String = {
        // Retrieve the project path at runtime and store it as a static variable.
        format!("{}/", std::env::current_dir().unwrap().to_string_lossy().to_string())
    };
}
