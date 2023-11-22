use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

use anyhow::Error;
use anyhow::Result;
use ash::prelude::VkResult;
use ash::{vk, Instance};

use crate::constant;
use crate::SwapChainSupportDetails;

use crate::{constant::support, utility, QueueFamilyIndices};

unsafe fn is_device_suitable(
    physical_device: vk::PhysicalDevice,
    instance: &ash::Instance,
    surface_loader: &ash::extensions::khr::Surface,
    surface: &vk::SurfaceKHR,
) -> VkResult<QueueFamilyIndices> {
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

    let _ = device_extension_support(instance, physical_device)?;
    let _ = SwapChainSupportDetails::query_swapchain_support(surface_loader, surface.clone(), physical_device)?;

    return QueueFamilyIndices::find_queue_family(physical_device, instance, surface_loader, surface);

    //println!("{:?}", device_properties.properties.device_type);
    // if device_properties.properties.device_type == ash::vk::PhysicalDeviceType::DISCRETE_GPU {}
    // // TODO, minimum requirements
    // None
}

unsafe fn device_extension_support(instance: &Instance, physical_device: vk::PhysicalDevice) -> VkResult<bool> {
    let extensions = instance.enumerate_device_extension_properties(physical_device)?;

    //write out all extensions
    // let mut s = String::new();
    // for extension in &extensions {
    //     s.push_str(&utility::vk_to_string(&extension.extension_name));
    //     s.push_str(", ");
    // }
    // println!("All extensions: {}", s);

    let mut vec_names: Vec<&'static CStr> = support::EXTENSION_SUPPORT_ARRAY_NAME.to_vec();
    for extension in extensions {
        let s = utility::vk_to_string(&extension.extension_name);
        for (index, extension_required) in vec_names.clone().iter().enumerate() {
            if s == extension_required.to_str().unwrap() {
                vec_names.remove(index);
                break;
            }
        }
    }
    if vec_names.len() > 0 {
        return Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT);
    }

    Ok(true)
}

pub unsafe fn pick_physical_device(
    instance: &ash::Instance,
    surface_loader: &ash::extensions::khr::Surface,
    surface: &vk::SurfaceKHR,
) -> Result<(vk::PhysicalDevice)> {
    let devices = instance.enumerate_physical_devices()?;
    for device in devices {
        let dev_ret = is_device_suitable(device, instance, surface_loader, surface);

        match dev_ret {
            Ok(x) => return Ok(device),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    Err(Error::msg("No Vulkan Supported GPU"))
}

pub unsafe fn create_logical_device(
    physical_device: vk::PhysicalDevice,
    instance: &ash::Instance,
    surface: vk::SurfaceKHR,
    surface_loader: &ash::extensions::khr::Surface,
) -> Result<(ash::Device, QueueFamilyIndices)> {
    let queue_priorities = [1.0];
    let indices = QueueFamilyIndices::find_queue_family(physical_device, instance, &surface_loader, &surface)?;
    // Create the queue info with the correct queue priorities
    let mut queues_infos = vec![];

    let mut unique_queue = HashSet::new();

    unique_queue.insert(indices.graphics_family);
    unique_queue.insert(indices.present_family);
    unique_queue.insert(indices.transfer_family);

    for queue_index in unique_queue.iter() {
        let queue_info = vk::DeviceQueueCreateInfo {
            s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::DeviceQueueCreateFlags::empty(),
            queue_family_index: queue_index.unwrap(),
            queue_count: 1,
            p_queue_priorities: queue_priorities.as_ptr(),
        };
        queues_infos.push(queue_info);
    }

    let feature_info = vk::PhysicalDeviceFeatures::default();

    let mut extension_names = vec![];
    for extension_required in constant::support::EXTENSION_SUPPORT_ARRAY_BYTES {
        extension_names.push(CStr::from_bytes_with_nul_unchecked(*extension_required));
    }
    let extension_names_raw: Vec<*const c_char> = extension_names.iter().map(|raw_name| raw_name.as_ptr()).collect();

    let mut device_info = vk::DeviceCreateInfo {
        s_type: vk::StructureType::DEVICE_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::DeviceCreateFlags::empty(),
        queue_create_info_count: queues_infos.len() as u32,
        p_queue_create_infos: queues_infos.as_ptr(),
        enabled_layer_count: 0,
        pp_enabled_layer_names: ptr::null(),
        enabled_extension_count: extension_names_raw.len() as u32,
        pp_enabled_extension_names: extension_names_raw.as_ptr(),
        p_enabled_features: &feature_info,
    };

    let device = instance.create_device(physical_device, &device_info, None)?;
    Ok((device, indices))
}

pub fn get_version_api(api: u32) -> (u32, u32, u32, u32) {
    let variant = api >> 29;
    let major = api >> 22;
    let minor = (api >> 12) & (0x3FF as u32);
    let patch = api & (0xFF as u32);

    (variant, major, minor, patch)
}
