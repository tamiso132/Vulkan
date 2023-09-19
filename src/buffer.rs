use anyhow::Result;
use ash::vk;

use crate::QueueFamilyIndices;

pub unsafe fn create_frame_buffer(
    device: &ash::Device,
    swapchain_image_views: &Vec<vk::ImageView>,
    render_pass: vk::RenderPass,
    swapchain_extent: vk::Extent2D,
) -> Result<Vec<vk::Framebuffer>> {
    let mut frame_buffer = vec![];

    for index in 0..swapchain_image_views.len() {
        let mut info = vk::FramebufferCreateInfo::default();

        info.s_type = vk::StructureType::FRAMEBUFFER_CREATE_INFO;
        info.render_pass = render_pass;
        info.attachment_count = 1;
        info.p_attachments = [swapchain_image_views[index]].as_ptr();
        info.width = swapchain_extent.width;
        info.height = swapchain_extent.height;
        info.layers = 1;

        let frame = device.create_framebuffer(&info, None)?;
        frame_buffer.push(frame);
    }
    Ok(frame_buffer)
}

pub unsafe fn create_command_buffer(device: &ash::Device, command_pool: vk::CommandPool) -> Result<vk::CommandBuffer> {
    let mut alloc_info = vk::CommandBufferAllocateInfo::default();

    alloc_info.s_type = vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO;
    alloc_info.command_pool = command_pool;
    alloc_info.level = vk::CommandBufferLevel::PRIMARY;
    alloc_info.command_buffer_count = 1;

    let command_buffer = device.allocate_command_buffers(&alloc_info)?;
    Ok(command_buffer[0])
}

pub unsafe fn record_command_buffer(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    render_pass: vk::RenderPass,
    swap_chain_framebuffer: Vec<vk::Framebuffer>,
    image_index: u32,
) -> Result<()> {
    let mut begin_info = vk::CommandBufferBeginInfo::default();

    begin_info.s_type = vk::StructureType::COMMAND_BUFFER_BEGIN_INFO;
    begin_info.flags = vk::CommandBufferUsageFlags::empty();
    begin_info.p_inheritance_info = std::ptr::null();

    let mut render_pass_info = vk::RenderPassBeginInfo::default();
    render_pass_info.s_type = vk::StructureType::RENDER_PASS_BEGIN_INFO;
    render_pass_info.render_pass = render_pass;
    render_pass_info.framebuffer = swap_chain_framebuffer[image_index];

    device.begin_command_buffer(command_buffer, &begin_info)?;
    Ok(())
}

pub unsafe fn create_command_pool(
    device: &ash::Device,
    physical_device: vk::PhysicalDevice,
    instance: &ash::Instance,
    surface_loader: &ash::extensions::khr::Surface,
    surface: vk::SurfaceKHR,
) -> Result<vk::CommandPool> {
    let queue_family = QueueFamilyIndices::find_queue_family(physical_device, instance, surface_loader, &surface)?;

    let mut pool_info = vk::CommandPoolCreateInfo::default();
    pool_info.s_type = vk::StructureType::COMMAND_POOL_CREATE_INFO;
    pool_info.flags = vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER;
    pool_info.queue_family_index = queue_family.graphics_family.unwrap();

    let command_pool = device.create_command_pool(&pool_info, None)?;
    Ok(command_pool)
}
