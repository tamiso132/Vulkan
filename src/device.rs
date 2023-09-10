use anyhow::{Error, Result};
use ash::vk;

use crate::{constant::validation, utility};
unsafe fn is_device_suitable(
    physical_device: vk::PhysicalDevice,
    instance: &ash::Instance,
) -> Option<u32> {
    let mut device_properties = vk::PhysicalDeviceProperties2::default();
    let mut device_features = vk::PhysicalDeviceFeatures2::default();

    instance.get_physical_device_properties2(physical_device, &mut device_properties);
    instance.get_physical_device_features2(physical_device, &mut device_features);

    let device_type = match device_properties.properties.device_type {
        vk::PhysicalDeviceType::CPU => "Cpu",
        vk::PhysicalDeviceType::INTEGRATED_GPU => "Integrated GPU",
        vk::PhysicalDeviceType::DISCRETE_GPU => "Discrete GPU",
        vk::PhysicalDeviceType::VIRTUAL_GPU => "Virtual GPU",
        vk::PhysicalDeviceType::OTHER => "Unknown",
        _ => panic!(),
    };

    let device_name = utility::vk_to_string(&device_properties.properties.device_name);
    let driver_version = get_version_api(device_properties.properties.driver_version);

    println!(
        "\tDevice Name: {}, id: {}, type: {}, driver version: {}.{}.{}.",
        device_name,
        device_properties.properties.device_id,
        device_type,
        driver_version.1,
        driver_version.2,
        driver_version.3,
    );

    let (variant, major, minior, patch) = get_version_api(device_properties.properties.api_version);

    println!("\tVersion:, {}.{}.{}.{}", variant, major, minior, patch); // supported vulkan

    return QueueFamilyIndices::find_queue_family(physical_device, instance);

    //println!("{:?}", device_properties.properties.device_type);
    // if device_properties.properties.device_type == ash::vk::PhysicalDeviceType::DISCRETE_GPU {}
    // // TODO, minimum requirements
    // None
}

pub unsafe fn pick_phyiscal_device(
    entity: &ash::Entry,
    instance: &ash::Instance,
) -> Result<(vk::PhysicalDevice, u32)> {
    let devices = instance.enumerate_physical_devices()?;
    for device in devices {
        let dev_ret = is_device_suitable(device, instance);

        match dev_ret {
            Some(graphic_index) => return Ok((device, graphic_index)),
            None => continue,
        }
    }
    Err(Error::msg("No Vulkan Supported GPU"))
}

pub unsafe fn create_logical_device(
    physical_device: vk::PhysicalDevice,
    instance: &ash::Instance,
    graphic_index: u32,
) -> Result<ash::Device> {
    let queue_priorities = [1.0];

    // Create the queue info with the correct queue priorities
    let queue_info = vk::DeviceQueueCreateInfo {
        s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: vk::DeviceQueueCreateFlags::empty(),
        queue_family_index: graphic_index,
        queue_count: 1,
        p_queue_priorities: queue_priorities.as_ptr(),
    };

    let feature_info = vk::PhysicalDeviceFeatures::default();

    let mut device_info = vk::DeviceCreateInfo::default();
    device_info.s_type = vk::StructureType::DEVICE_CREATE_INFO;
    device_info.p_queue_create_infos = &queue_info;
    device_info.queue_create_info_count = 1;
    device_info.p_enabled_features = &feature_info;
    device_info.enabled_extension_count = 0;

    let device = instance.create_device(physical_device, &device_info, None)?;
    Ok(device)
}

struct QueueFamilyIndices {}
impl QueueFamilyIndices {
    // GRAPHICS | COMPUTE | TRANSFER | SPARSE_BINDING
    // TRANSFER | SPARSE_BINDING
    // COMPUTE | TRANSFER | SPARSE_BINDING
    // TRANSFER | SPARSE_BINDING | VIDEO_DECODE_KHR

    unsafe fn find_queue_family(
        device: vk::PhysicalDevice,
        instance: &ash::Instance,
    ) -> Option<u32> {
        let queue_count = instance.get_physical_device_queue_family_properties2_len(device);

        let mut queue_families = vec![ash::vk::QueueFamilyProperties2::default(); queue_count];

        instance.get_physical_device_queue_family_properties2(device, &mut queue_families);

        for (index, queue) in queue_families.iter().enumerate() {
            if queue.queue_family_properties.queue_flags & vk::QueueFlags::GRAPHICS
                != vk::QueueFlags::empty()
            {
                return Some(index as u32);
            }
        }
        return None;
    }
}

pub fn get_version_api(api: u32) -> (u32, u32, u32, u32) {
    let variant = api >> 29;
    let major = api >> 22;
    let minor = (api >> 12) & (0x3FF as u32);
    let patch = api & (0xFF as u32);

    (variant, major, minor, patch)
}
