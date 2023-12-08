use std::mem::{offset_of, size_of};

use ash::vk;
use glm::Vector3;

extern crate nalgebra as glm;

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

#[repr(C)]
pub struct Index(u16);

#[repr(C)]
pub struct Vertex {
    pos: glm::Vector2<f32>,
    color: glm::Vector3<f32>,
}

impl Vertex {
    pub const fn get_binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<Vertex>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        }
    }

    pub const fn get_input_attribute_description() -> [vk::VertexInputAttributeDescription; 2] {
        [
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32G32_SFLOAT,
                offset: offset_of!(Vertex, pos) as u32,
            },
            vk::VertexInputAttributeDescription {
                binding: 0,
                location: 1,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: offset_of!(Vertex, color) as u32 as u32,
            },
        ]
    }
}

pub const VERTICES: [Vertex; 4] = [
    Vertex {
        pos: glm::Vector2::<f32>::new(-0.5, -0.5),
        color: glm::Vector3::new(0.0, 0.0, 1.0),
    },
    Vertex {
        pos: glm::Vector2::<f32>::new(0.5, -0.5),
        color: glm::Vector3::new(1.0, 0.0, 0.0),
    },
    Vertex {
        pos: glm::Vector2::<f32>::new(0.5, 0.5),
        color: glm::Vector3::new(0.0, 1.0, 0.0),
    },
    Vertex {
        pos: glm::Vector2::<f32>::new(-0.5, 0.5),
        color: glm::Vector3::new(0.0, 0.0, 1.0),
    },
];

pub static INDICES: [Index; 6] = [Index(0), Index(1), Index(2), Index(2), Index(3), Index(0)];
