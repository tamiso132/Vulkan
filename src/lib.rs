use anyhow::Result;
use ash::{prelude::VkResult, vk};

pub mod buffer;
pub mod constant;
pub mod device;
pub mod pipeline;
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
    ) -> VkResult<QueueFamilyIndices> {
        let queue_count = instance.get_physical_device_queue_family_properties(physical_device).len();

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
        return Err(vk::Result::ERROR_FORMAT_NOT_SUPPORTED);
    }
}

// Basic surface capabilities (min/max number of images in swap chain, min/max width and height of images)
// Surface formats (pixel format, color space)
// Available presentation modes

pub struct SwapChainSupportDetails {
    capabilities: vk::SurfaceCapabilitiesKHR,
    formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
}
// VK_PRESENT_MODE_IMMEDIATE_KHR: Images submitted by your application are transferred to the screen right away, which may result in tearing.
// VK_PRESENT_MODE_FIFO_KHR: The swap chain is a queue where the display takes an image from the front of the queue when the display is refreshed and the program inserts rendered images at the back of the queue. If the queue is full then the program has to wait. This is most similar to vertical sync as found in modern games. The moment that the display is refreshed is known as "vertical blank".
// VK_PRESENT_MODE_FIFO_RELAXED_KHR: This mode only differs from the previous one if the application is late and the queue was empty at the last vertical blank. Instead of waiting for the next vertical blank, the image is transferred right away when it finally arrives. This may result in visible tearing.
// VK_PRESENT_MODE_MAILBOX_KHR: This is another variation of the second mode. Instead of blocking the application when the queue is full, the images that are already queued are simply replaced with the newer ones. This mode can be used to render frames as fast as possible while still avoiding tearing, resulting in fewer latency issues than standard vertical sync. This is commonly known as "triple buffering", although the existence of three buffers alone does not necessarily mean that the framerate is unlocked.

impl SwapChainSupportDetails {
    unsafe fn query_swapchain_support(
        surface_loader: &ash::extensions::khr::Surface,
        surface: vk::SurfaceKHR,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<SwapChainSupportDetails> {
        let capabilities = surface_loader.get_physical_device_surface_capabilities(physical_device, surface)?;
        let formats = surface_loader.get_physical_device_surface_formats(physical_device, surface)?;
        let present_modes = surface_loader.get_physical_device_surface_present_modes(physical_device, surface)?;

        Ok(SwapChainSupportDetails {
            capabilities,
            formats,
            present_modes,
        })
    }
    unsafe fn choose_format(available_formats: Vec<vk::SurfaceFormatKHR>) -> vk::SurfaceFormatKHR {
        let mut index = 0;
        for (i, format_available) in available_formats.iter().enumerate() {
            if format_available.format == vk::Format::B8G8R8A8_SRGB {
                index = i;
                break;
            }
        }
        return available_formats[index];
    }

    unsafe fn choose_present_mode(present_modes: Vec<vk::PresentModeKHR>) -> vk::PresentModeKHR {
        let mut present_ret = vk::PresentModeKHR::FIFO;
        for present_mode in present_modes {
            if present_mode == vk::PresentModeKHR::MAILBOX {
                present_ret = present_mode;
                break;
            }
        }
        return present_ret;
    }

    unsafe fn choose_extent(capabilities: vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {
        vk::Extent2D {
            width: num::clamp(
                constant::Window_Info::WIDTH,
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
            ),
            height: num::clamp(
                constant::Window_Info::HEIGHT,
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
            ),
        }
    }

    unsafe fn create_image_views(
        swapchain_images: &Vec<vk::Image>,
        swapchain_format: vk::Format,
        device: &ash::Device,
    ) -> Result<Vec<vk::ImageView>, vk::Result> {
        let mut image_views = vec![];
        for image in swapchain_images {
            let mut image_view_info = vk::ImageViewCreateInfo::default();
            image_view_info.s_type = vk::StructureType::IMAGE_VIEW_CREATE_INFO;
            image_view_info.image = image.clone();
            image_view_info.view_type = vk::ImageViewType::TYPE_2D;
            image_view_info.format = swapchain_format;

            image_view_info.components.r = vk::ComponentSwizzle::IDENTITY;
            image_view_info.components.g = vk::ComponentSwizzle::IDENTITY;
            image_view_info.components.b = vk::ComponentSwizzle::IDENTITY;
            image_view_info.components.a = vk::ComponentSwizzle::IDENTITY;

            image_view_info.subresource_range.aspect_mask = vk::ImageAspectFlags::COLOR;
            image_view_info.subresource_range.base_mip_level = 0;
            image_view_info.subresource_range.level_count = 1;
            image_view_info.subresource_range.base_array_layer = 0;
            image_view_info.subresource_range.layer_count = 1;

            let image_view = device.create_image_view(&image_view_info, None)?;
            image_views.push(image_view);
        }
        Ok(image_views)
    }

    pub unsafe fn create_swapchain(
        instance: &ash::Instance,
        device: &ash::Device,
        surface_loader: &ash::extensions::khr::Surface,
        surface: vk::SurfaceKHR,
        physical_device: vk::PhysicalDevice,
    ) -> Result<
        (
            ash::extensions::khr::Swapchain,
            vk::SwapchainKHR,
            vk::Extent2D,
            vk::Format,
            Vec<vk::Image>,
            Vec<vk::ImageView>,
        ),
        vk::Result,
    > {
        let swap_chain_support = SwapChainSupportDetails::query_swapchain_support(surface_loader, surface, physical_device)?;

        let extent = SwapChainSupportDetails::choose_extent(swap_chain_support.capabilities);
        let surface_format = SwapChainSupportDetails::choose_format(swap_chain_support.formats);
        let present_mode = SwapChainSupportDetails::choose_present_mode(swap_chain_support.present_modes);
        let mut image_count = swap_chain_support.capabilities.min_image_count + 1;

        if swap_chain_support.capabilities.max_image_count > 0
            && image_count > swap_chain_support.capabilities.max_image_count
        {
            image_count = swap_chain_support.capabilities.max_image_count;
        }
        let mut swapchain_info = vk::SwapchainCreateInfoKHR::default();
        let family_queue = QueueFamilyIndices::find_queue_family(physical_device, instance, surface_loader, &surface)?;

        swapchain_info.s_type = vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR;
        swapchain_info.surface = surface;
        swapchain_info.min_image_count = image_count;
        swapchain_info.image_format = surface_format.format;
        swapchain_info.image_color_space = surface_format.color_space;
        swapchain_info.image_extent = extent;
        swapchain_info.image_array_layers = 1;
        swapchain_info.image_usage = vk::ImageUsageFlags::COLOR_ATTACHMENT;
        swapchain_info.pre_transform = swap_chain_support.capabilities.current_transform;
        swapchain_info.composite_alpha = vk::CompositeAlphaFlagsKHR::OPAQUE;
        swapchain_info.present_mode = present_mode;
        swapchain_info.clipped = vk::TRUE;
        swapchain_info.old_swapchain = vk::SwapchainKHR::null();
        // VK_SHARING_MODE_EXCLUSIVE: An image is owned by one queue family at a time and ownership must be explicitly transferred before using it in another queue family. This option offers the best performance.
        // VK_SHARING_MODE_CONCURRENT: Images can be used across multiple queue families without explicit ownership transfers.

        if family_queue.graphics_family.unwrap() != family_queue.present_family.unwrap() {
            swapchain_info.image_sharing_mode = vk::SharingMode::CONCURRENT;
            swapchain_info.queue_family_index_count = 2;
            swapchain_info.p_queue_family_indices =
                [family_queue.graphics_family.unwrap(), family_queue.present_family.unwrap()].as_ptr();
        } else {
            swapchain_info.image_sharing_mode = vk::SharingMode::EXCLUSIVE;
            swapchain_info.queue_family_index_count = 0;
            swapchain_info.p_queue_family_indices = std::ptr::null();
        }
        let swapchain_loader = ash::extensions::khr::Swapchain::new(instance, device);
        let swapchain = swapchain_loader.create_swapchain(&swapchain_info, None)?;
        let swapchain_images = swapchain_loader.get_swapchain_images(swapchain)?;
        let swapchain_image_views =
            SwapChainSupportDetails::create_image_views(&swapchain_images, surface_format.format, device)?;
        Ok((
            swapchain_loader,
            swapchain,
            extent,
            surface_format.format,
            swapchain_images,
            swapchain_image_views,
        ))
    }
}
