use std::{
    ffi::c_void,
    mem::{size_of, size_of_val},
    ptr,
};

use anyhow::Result;
use ash::{
    prelude::VkResult,
    vk::{self, MemoryMapFlags, MemoryPropertyFlags, StructureType},
};

use crate::{
    constant::{Vertex, VERTICES},
    QueueFamilyIndices,
};

pub const MAX_FRAMES_IN_FLIGHT: u8 = 2;

pub unsafe fn create_frame_buffer(
    device: &ash::Device,
    swapchain_image_views: &Vec<vk::ImageView>,
    render_pass: vk::RenderPass,
    swapchain_extent: vk::Extent2D,
) -> VkResult<Vec<vk::Framebuffer>> {
    let mut frame_buffer = vec![];
    println!("frame_buffer length = {}", swapchain_image_views.len());
    for index in 0..swapchain_image_views.len() {
        let attachments = [swapchain_image_views[index]];
        let mut info = vk::FramebufferCreateInfo {
            s_type: vk::StructureType::FRAMEBUFFER_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::FramebufferCreateFlags::empty(),
            render_pass,
            attachment_count: 1,
            p_attachments: attachments.as_ptr(),
            width: swapchain_extent.width,
            height: swapchain_extent.height,
            layers: 1,
        };

        info.width = swapchain_extent.width;
        info.height = swapchain_extent.height;
        info.layers = 1;

        let frame = device.create_framebuffer(&info, None)?;
        frame_buffer.push(frame);
    }
    Ok(frame_buffer)
}

pub unsafe fn create_command_buffers(
    device: &ash::Device,
    command_pool: vk::CommandPool,
    graphics_pipeline: vk::Pipeline,
    framebuffers: &Vec<vk::Framebuffer>,
    render_pass: vk::RenderPass,
    surface_extent: vk::Extent2D,
    vertex_buffer: vk::Buffer,
    swapchain_extent: vk::Extent2D,
) -> Result<Vec<vk::CommandBuffer>> {
    let alloc_info = vk::CommandBufferAllocateInfo {
        s_type: StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
        p_next: ptr::null(),
        command_pool,
        level: vk::CommandBufferLevel::PRIMARY,
        command_buffer_count: MAX_FRAMES_IN_FLIGHT as u32,
    };
    let command_buffer = device.allocate_command_buffers(&alloc_info)?;

    // for (i, &command_buffer) in command_buffer.iter().enumerate() {
    //     let command_begin_info = vk::CommandBufferBeginInfo {
    //         s_type: StructureType::COMMAND_BUFFER_BEGIN_INFO,
    //         p_next: ptr::null(),
    //         flags: vk::CommandBufferUsageFlags::SIMULTANEOUS_USE,
    //         p_inheritance_info: ptr::null(),
    //     };

    //     println!("it fucks up here?");
    //     device.begin_command_buffer(command_buffer, &command_begin_info)?;

    //     let clear_values = [vk::ClearValue {
    //         color: vk::ClearColorValue {
    //             float32: [0.0, 0.0, 0.0, 1.0],
    //         },
    //     }];

    //     let view_port = [vk::Viewport {
    //         x: 0.0,
    //         y: 0.0,
    //         width: swapchain_extent.width as f32,
    //         height: swapchain_extent.height as f32,
    //         min_depth: 0.0,
    //         max_depth: 1.0,
    //     }];
    //     device.cmd_set_viewport(command_buffer, 0, &view_port);

    //     let scissor = [vk::Rect2D {
    //         offset: vk::Offset2D { x: 0, y: 0 },
    //         extent: swapchain_extent,
    //     }];

    //     device.cmd_set_scissor(command_buffer, 0, &scissor);

    //     // VkViewport viewport{};
    //     // viewport.x = 0.0f;
    //     // viewport.y = 0.0f;
    //     // viewport.width = (float) swapChainExtent.width;
    //     // viewport.height = (float) swapChainExtent.height;
    //     // viewport.minDepth = 0.0f;
    //     // viewport.maxDepth = 1.0f;
    //     // vkCmdSetViewport(commandBuffer, 0, 1, &viewport);

    //     // VkRect2D scissor{};
    //     // scissor.offset = {0, 0};
    //     // scissor.extent = swapChainExtent;
    //     // vkCmdSetScissor(commandBuffer, 0, 1, &scissor);

    //     let render_pass_begin_info = vk::RenderPassBeginInfo {
    //         s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
    //         p_next: ptr::null(),
    //         framebuffer: framebuffers[i],
    //         render_pass,
    //         clear_value_count: clear_values.len() as u32,
    //         p_clear_values: clear_values.as_ptr(),
    //         render_area: vk::Rect2D {
    //             offset: vk::Offset2D { x: 0, y: 0 },
    //             extent: surface_extent,
    //         },
    //     };

    //     unsafe {
    //         device.cmd_begin_render_pass(command_buffer, &render_pass_begin_info, vk::SubpassContents::INLINE);
    //         device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, graphics_pipeline);

    //         let vertex_buffers = [vertex_buffer];
    //         let offsets = [0_u64];

    //         device.cmd_bind_vertex_buffers(command_buffer, 0, &vertex_buffers, &offsets);

    //         device.cmd_draw(command_buffer, VERTICES.len() as u32, 1, 0, 0);

    //         device.cmd_end_render_pass(command_buffer);

    //         device
    //             .end_command_buffer(command_buffer)
    //             .expect("Failed to record Command Buffer at Ending!");
    //     }
    // }
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
    vertex_buffer: vk::Buffer,
) -> VkResult<()> {
    let begin_info = vk::CommandBufferBeginInfo {
        s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
        p_next: ptr::null(),
        flags: vk::CommandBufferUsageFlags::SIMULTANEOUS_USE,
        p_inheritance_info: std::ptr::null(),
    };

    device.begin_command_buffer(command_buffer, &begin_info)?;

    let clear_values = [vk::ClearValue {
        color: vk::ClearColorValue {
            float32: [0.0, 0.0, 0.0, 1.0],
        },
    }];

    let render_pass_info = vk::RenderPassBeginInfo {
        s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
        p_next: ptr::null(),
        render_pass,
        framebuffer: swap_chain_framebuffer[image_index as usize],
        render_area: vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: swapchain_extent,
        },
        clear_value_count: 1,
        p_clear_values: clear_values.as_ptr(),
    };

    let view_port = [vk::Viewport {
        x: 0.0,
        y: 0.0,
        width: swapchain_extent.width as f32,
        height: swapchain_extent.height as f32,
        min_depth: 0.0,
        max_depth: 1.0,
    }];
    device.cmd_set_viewport(command_buffer, 0, &view_port);

    let scissor = [vk::Rect2D {
        offset: vk::Offset2D { x: 0, y: 0 },
        extent: swapchain_extent,
    }];

    device.cmd_set_scissor(command_buffer, 0, &scissor);
    // Begin the render pass

    device.cmd_begin_render_pass(command_buffer, &render_pass_info, vk::SubpassContents::INLINE);

    device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);

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

    let vertex_buffers = [vertex_buffer];
    let offsets = [0];
    device.cmd_bind_vertex_buffers(command_buffer, 0, &vertex_buffers, &offsets);
    device.cmd_draw(command_buffer, VERTICES.len() as u32, 1, 0, 0);

    // End the render pass
    device.cmd_end_render_pass(command_buffer);

    device.end_command_buffer(command_buffer).expect("failed to record");

    Ok(())
}

pub unsafe fn create_command_pool(device: &ash::Device, queue_family: &QueueFamilyIndices) -> Result<vk::CommandPool> {
    let pool_info = vk::CommandPoolCreateInfo {
        s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
        queue_family_index: queue_family.graphics_family.unwrap(),
    };

    Ok(device.create_command_pool(&pool_info, None)?)
}

pub unsafe fn create_sync_objects(
    device: &ash::Device,
) -> VkResult<(Vec<vk::Fence>, Vec<vk::Semaphore>, Vec<vk::Semaphore>)> {
    let semaphore_create_info = vk::SemaphoreCreateInfo {
        s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::SemaphoreCreateFlags::empty(),
    };

    let fence_create_info = vk::FenceCreateInfo {
        s_type: vk::StructureType::FENCE_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::FenceCreateFlags::SIGNALED,
    };

    let mut image_available_semaphores = vec![];
    let mut render_finished_semaphores = vec![];
    let mut inflight_fences = vec![];

    for _ in 0..MAX_FRAMES_IN_FLIGHT {
        let image_available_semaphore = device
            .create_semaphore(&semaphore_create_info, None)
            .expect("Failed to create Semaphore Object!");
        let render_finished_semaphore = device
            .create_semaphore(&semaphore_create_info, None)
            .expect("Failed to create Semaphore Object!");
        let inflight_fence = device
            .create_fence(&fence_create_info, None)
            .expect("Failed to create Fence Object!");

        image_available_semaphores.push(image_available_semaphore);
        render_finished_semaphores.push(render_finished_semaphore);
        inflight_fences.push(inflight_fence);
    }
    Ok((inflight_fences, image_available_semaphores, render_finished_semaphores))
}

pub unsafe fn create_vertex_buffer(
    device: &ash::Device,
    physical_device: vk::PhysicalDevice,
    instance: &ash::Instance,
) -> VkResult<(vk::Buffer, vk::DeviceMemory)> {
    let buffer_info = vk::BufferCreateInfo {
        s_type: StructureType::BUFFER_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::BufferCreateFlags::empty(),
        size: (size_of_val(&VERTICES)) as u64,
        usage: vk::BufferUsageFlags::VERTEX_BUFFER,
        sharing_mode: vk::SharingMode::EXCLUSIVE,
        queue_family_index_count: 0,
        p_queue_family_indices: ptr::null(),
    };

    let buffer = device.create_buffer(&buffer_info, None)?;
    let mem_requirement = device.get_buffer_memory_requirements(buffer);

    let properties = MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT;

    let alloc_info = vk::MemoryAllocateInfo {
        s_type: StructureType::MEMORY_ALLOCATE_INFO,
        p_next: ptr::null(),
        allocation_size: mem_requirement.size,
        memory_type_index: find_memory_type(mem_requirement.memory_type_bits, properties, physical_device, instance),
    };

    let vertex_buffer_memory = device.allocate_memory(&alloc_info, None)?;

    device.bind_buffer_memory(buffer, vertex_buffer_memory, 0)?;

    let data = device.map_memory(vertex_buffer_memory, 0, buffer_info.size, MemoryMapFlags::empty())? as *mut Vertex;
    data.copy_from_nonoverlapping(VERTICES.as_ptr(), VERTICES.len());
    device.unmap_memory(vertex_buffer_memory);

    Ok((buffer, vertex_buffer_memory))
}

unsafe fn find_memory_type(
    type_filter: u32,
    properties: vk::MemoryPropertyFlags,
    physical_device: vk::PhysicalDevice,
    instance: &ash::Instance,
) -> u32 {
    let mut memproperties = vk::PhysicalDeviceMemoryProperties2::default();
    instance.get_physical_device_memory_properties2(physical_device, &mut memproperties);

    for i in 0..memproperties.memory_properties.memory_type_count {
        let is_memory_type_bits = type_filter & (1 << i) > 0;
        let is_property = memproperties.memory_properties.memory_types[i as usize]
            .property_flags
            .contains(properties);

        if is_memory_type_bits && is_property {
            return i;
        }
    }
    panic!("failed to fidnd suitable memory type!");
}
