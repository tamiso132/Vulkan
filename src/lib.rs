use anyhow::{Error, Result};
use ash::vk;

pub mod constant;
pub mod device;
pub mod platform;
pub mod utility;

pub struct QueueFamilyIndices {
    pub graphics_family: Option<u32>,
    pub present_family: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn is_completed(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }
    // GRAPHICS | COMPUTE | TRANSFER | SPARSE_BINDING
    // TRANSFER | SPARSE_BINDING
    // COMPUTE | TRANSFER | SPARSE_BINDING
    // TRANSFER | SPARSE_BINDING | VIDEO_DECODE_KHR

    pub unsafe fn find_queue_family(
        physical_device: vk::PhysicalDevice,
        instance: &ash::Instance,
        surface_loader: &ash::extensions::khr::Surface,
        surface: &vk::SurfaceKHR,
    ) -> Result<QueueFamilyIndices> {
        let queue_count = instance.get_physical_device_queue_family_properties2_len(physical_device);

        let mut queue_family_ret = QueueFamilyIndices {
            graphics_family: None,
            present_family: None,
        };

        let mut queue_families = vec![ash::vk::QueueFamilyProperties2::default(); queue_count];

        instance.get_physical_device_queue_family_properties2(physical_device, &mut queue_families);

        for (index, queue) in queue_families.iter().enumerate() {
            if queue.queue_family_properties.queue_flags & vk::QueueFlags::GRAPHICS != vk::QueueFlags::empty() {
                queue_family_ret.graphics_family = Some(index as u32);
            }
            let present_support =
                surface_loader.get_physical_device_surface_support(physical_device, index as u32, surface.to_owned())?;

            if queue.queue_family_properties.queue_count > 0 && present_support {
                queue_family_ret.present_family = Some(index as u32);
            }

            if queue_family_ret.is_completed() {
                return Ok(queue_family_ret);
            }
        }
        return Err(Error::msg("no compatible queue family found"));
    }
}
