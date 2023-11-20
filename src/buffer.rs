use std::ptr;

use anyhow::Result;
use ash::{prelude::VkResult, vk};

use crate::QueueFamilyIndices;

pub const MAX_FRAMES_IN_FLIGHT: u8 = 2;

pub unsafe fn create_frame_buffer(
    device: &ash::Device,
    swapchain_image_views: &Vec<vk::ImageView>,
    render_pass: vk::RenderPass,
    swapchain_extent: vk::Extent2D,
) -> VkResult<Vec<vk::Framebuffer>> {
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

pub unsafe fn create_command_buffers(device: &ash::Device, command_pool: vk::CommandPool) -> Result<Vec<vk::CommandBuffer>> {
    let mut alloc_info = vk::CommandBufferAllocateInfo::default();

    alloc_info.s_type = vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO;
    alloc_info.command_pool = command_pool;
    alloc_info.level = vk::CommandBufferLevel::PRIMARY;
    alloc_info.command_buffer_count = MAX_FRAMES_IN_FLIGHT as u32;

    let command_buffer = device.allocate_command_buffers(&alloc_info)?;
    Ok(command_buffer)
}

pub unsafe fn record_command_buffer(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    render_pass: vk::RenderPass,
    swap_chain_framebuffer: &Vec<vk::Framebuffer>,
    image_index: u32,
    swapchain_extent: vk::Extent2D,
    pipeline: vk::Pipeline,
) -> VkResult<()> {
    let mut begin_info = vk::CommandBufferBeginInfo::default();

    begin_info.s_type = vk::StructureType::COMMAND_BUFFER_BEGIN_INFO;
    begin_info.flags = vk::CommandBufferUsageFlags::empty();
    begin_info.p_inheritance_info = std::ptr::null();
    device.begin_command_buffer(command_buffer, &begin_info)?;

    let mut clear_color = vk::ClearValue::default();
    clear_color.color.float32 = [0.0, 0.0, 0.0, 1.0];

    let mut render_pass_info = vk::RenderPassBeginInfo::default();
    render_pass_info.s_type = vk::StructureType::RENDER_PASS_BEGIN_INFO;
    render_pass_info.p_next = ptr::null();
    render_pass_info.render_pass = render_pass;
    render_pass_info.framebuffer = swap_chain_framebuffer[image_index as usize];
    render_pass_info.render_area.offset = vk::Offset2D::builder().x(0).y(0).build();
    render_pass_info.render_area.extent = swapchain_extent;
    render_pass_info.clear_value_count = 1;
    render_pass_info.p_clear_values = &clear_color;

    println!("{:?}", render_pass_info);
    // Begin the render pass
    device.cmd_begin_render_pass(command_buffer, &render_pass_info, vk::SubpassContents::INLINE);

    println!("buta56");
    device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);

    println!("buta");

    let mut viewport = vk::Viewport::default();
    viewport.x = 0.0;
    viewport.y = 0.0;
    viewport.width = swapchain_extent.width as f32;
    viewport.height = swapchain_extent.height as f32;
    viewport.min_depth = 0.0;
    viewport.max_depth = 1.0;
    device.cmd_set_viewport(command_buffer, 0, &[viewport]);

    let mut scissor = vk::Rect2D::default();
    scissor.offset = vk::Offset2D { x: 0, y: 0 };
    scissor.extent = swapchain_extent;
    device.cmd_set_scissor(command_buffer, 0, &[scissor]);

    device.cmd_draw(command_buffer, 3, 1, 0, 0);

    // End the render pass
    device.cmd_end_render_pass(command_buffer);

    device.end_command_buffer(command_buffer)?;

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

pub unsafe fn create_sync_objects(device: &ash::Device) -> Result<(Vec<vk::Fence>, Vec<vk::Semaphore>, Vec<vk::Semaphore>)> {
    let semphore_info = vk::SemaphoreCreateInfo::default();
    let mut fence_info = vk::FenceCreateInfo::default();
    fence_info.flags = vk::FenceCreateFlags::SIGNALED; // created at true

    let mut fences = vec![];
    let mut semphores = vec![];
    let mut semphores2 = vec![];
    for _ in 0..MAX_FRAMES_IN_FLIGHT {
        let fence = device.create_fence(&fence_info, None)?;
        let semphore = device.create_semaphore(&semphore_info, None)?;
        let semphore2 = device.create_semaphore(&semphore_info, None)?;

        fences.push(fence);
        semphores.push(semphore);
        semphores2.push(semphore2);
    }
    Ok((fences, semphores, semphores2))
}
