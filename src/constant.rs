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
