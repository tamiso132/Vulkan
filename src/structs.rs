use std::mem::size_of;

use anyhow::{anyhow, Result};
use vulkanalia::{
    vk::{self, InstanceV1_0, KhrSurfaceExtension},
    Instance,
};

use crate::{device::SuitabilityError, AppData};
#[derive(Copy, Clone, Debug)]
pub struct QueueFamilyIndices {
    pub(crate) graphics: u32,
    pub(crate) present: u32,
}

impl QueueFamilyIndices {
    pub(crate) unsafe fn get(instance: &Instance, data: &AppData, physical_device: vk::PhysicalDevice) -> Result<Self> {
        let properties = instance.get_physical_device_queue_family_properties(physical_device);

        let graphics = properties.iter().position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS)).map(|i| i as u32);

        let mut present = None;
        for (index, properties) in properties.iter().enumerate() {
            if instance.get_physical_device_surface_support_khr(physical_device, index as u32, data.surface)? {
                present = Some(index as u32);
                break;
            }
        }

        if let (Some(graphics), Some(present)) = (graphics, present) {
            Ok(Self { graphics, present })
        } else {
            Err(anyhow!(SuitabilityError("Missing required queue families.")))
        }
    }
}
