use std::{
    ffi::c_void,
    mem::{size_of, size_of_val},
    ptr,
};

use anyhow::Result;
use ash::{
    prelude::VkResult,
    vk::{self, BufferUsageFlags, ImageCreateFlags, MemoryMapFlags, MemoryPropertyFlags, StructureType},
};

use crate::{
    constant::{Index, Vertex, INDICES, VERTICES},
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

pub unsafe fn create_command_buffers(device: &ash::Device, command_pool: vk::CommandPool) -> Result<Vec<vk::CommandBuffer>> {
    let alloc_info = vk::CommandBufferAllocateInfo {
        s_type: StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
        p_next: ptr::null(),
        command_pool,
        level: vk::CommandBufferLevel::PRIMARY,
        command_buffer_count: MAX_FRAMES_IN_FLIGHT as u32,
    };
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
    vertex_buffer: vk::Buffer,
    index_buffer: vk::Buffer,
) -> VkResult<()> {
    let begin_info = vk::CommandBufferBeginInfo {
        s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
        p_next: ptr::null(),
        flags: vk::CommandBufferUsageFlags::SIMULTANEOUS_USE,
        p_inheritance_info: std::ptr::null(),
    };

    device.begin_command_buffer(command_buffer, &begin_info)?;

    let clear_values = [vk::ClearValue {
        // draw the frame black before drawing the scene
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

    // // Begin the render pass

    device.cmd_begin_render_pass(command_buffer, &render_pass_info, vk::SubpassContents::INLINE);

    device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);

    let mut viewport = vk::Viewport::default();
    viewport.x = 0.0; // bottom left corner
    viewport.y = 0.0;
    viewport.width = swapchain_extent.width as f32;
    viewport.height = swapchain_extent.height as f32;
    viewport.min_depth = 0.0;
    viewport.max_depth = 1.0;
    device.cmd_set_viewport(command_buffer, 0, &[viewport]);

    // clip area of pixels, in this instance, we use the whole window
    let scissor = [vk::Rect2D {
        // clip area of pixels, in this instance, we use the whole window
        offset: vk::Offset2D { x: 0, y: 0 },
        extent: swapchain_extent,
    }];
    device.cmd_set_scissor(command_buffer, 0, &scissor);

    let vertex_buffers = [vertex_buffer];
    let offsets = [0];
    device.cmd_bind_vertex_buffers(command_buffer, 0, &vertex_buffers, &offsets);
    device.cmd_bind_index_buffer(command_buffer, index_buffer, 0, vk::IndexType::UINT16);

    device.cmd_draw_indexed(command_buffer, INDICES.len() as u32, 1, 0, 0, 0);

    // End the render pass
    device.cmd_end_render_pass(command_buffer);

    device.end_command_buffer(command_buffer).expect("failed to record");

    Ok(())
}

pub unsafe fn create_command_pool(device: &ash::Device, queue_family: &Option<u32>) -> Result<vk::CommandPool> {
    let pool_info = vk::CommandPoolCreateInfo {
        s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
        queue_family_index: queue_family.unwrap(),
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

pub unsafe fn create_index_buffer(
    device: &ash::Device,
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    transfer_pool: vk::CommandPool,
    transfer_queue: vk::Queue,
) -> VkResult<(vk::Buffer, vk::DeviceMemory)> {
    let buffer_size = std::mem::size_of_val(&INDICES) as u64;

    let (staging_buffer, stage_memory) = create_buffer(
        device,
        instance,
        physical_device,
        buffer_size,
        BufferUsageFlags::TRANSFER_SRC,
        MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
    )?;

    let data = device.map_memory(stage_memory, 0, buffer_size, MemoryMapFlags::empty())? as *mut Index;
    data.copy_from_nonoverlapping(INDICES.as_ptr(), INDICES.len());
    device.unmap_memory(stage_memory);

    let (index_buffer, index_buffer_memory) = create_buffer(
        device,
        instance,
        physical_device,
        buffer_size,
        BufferUsageFlags::INDEX_BUFFER | BufferUsageFlags::TRANSFER_DST,
        MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    copy_buffer(
        device,
        staging_buffer,
        index_buffer,
        buffer_size,
        transfer_pool,
        transfer_queue,
    )?;

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(stage_memory, None);
    Ok((index_buffer, index_buffer_memory))
}

pub unsafe fn create_vertex_buffer(
    device: &ash::Device,
    physical_device: vk::PhysicalDevice,
    instance: &ash::Instance,
    transfer_pool: vk::CommandPool,
    transfer_queue: vk::Queue,
) -> VkResult<(vk::Buffer, vk::DeviceMemory)> {
    let properties_vertex = MemoryPropertyFlags::DEVICE_LOCAL;
    let properties_staging = MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT;
    let buffer_size = (size_of_val(&VERTICES)) as u64;

    let (stage_buffer, stage_buffer_memory) = create_buffer(
        device,
        instance,
        physical_device,
        buffer_size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        properties_staging,
    )?;

    let data = device.map_memory(stage_buffer_memory, 0, buffer_size, MemoryMapFlags::empty())? as *mut Vertex;
    data.copy_from_nonoverlapping(VERTICES.as_ptr(), VERTICES.len());
    device.unmap_memory(stage_buffer_memory);

    let (vertex_buffer, vertex_buffer_memory) = create_buffer(
        device,
        instance,
        physical_device,
        buffer_size,
        vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
        properties_vertex,
    )?;

    copy_buffer(
        device,
        stage_buffer,
        vertex_buffer,
        buffer_size,
        transfer_pool,
        transfer_queue,
    )?;

    device.destroy_buffer(stage_buffer, None);
    device.free_memory(stage_buffer_memory, None);

    Ok((vertex_buffer, vertex_buffer_memory))
}

unsafe fn copy_buffer(
    device: &ash::Device,
    src: vk::Buffer,
    dst: vk::Buffer,
    size: vk::DeviceSize,
    transfer_pool: vk::CommandPool,
    transfer_queue: vk::Queue,
) -> VkResult<()> {
    let command_buffer = begin_single_commands(device, transfer_pool)?;

    let copy_regions = [vk::BufferCopy {
        src_offset: 0,
        dst_offset: 0,
        size,
    }];

    device.cmd_copy_buffer(command_buffer, src, dst, &copy_regions);

    end_single_time_command(device, command_buffer, transfer_pool, transfer_queue)?;

    Ok(())
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

unsafe fn create_buffer(
    device: &ash::Device,
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    size: vk::DeviceSize,
    usage: vk::BufferUsageFlags,
    properties: vk::MemoryPropertyFlags,
) -> VkResult<(vk::Buffer, vk::DeviceMemory)> {
    let buffer_info = vk::BufferCreateInfo {
        s_type: StructureType::BUFFER_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::BufferCreateFlags::empty(),
        size,
        usage,
        sharing_mode: vk::SharingMode::EXCLUSIVE,
        queue_family_index_count: 0,
        p_queue_family_indices: ptr::null(),
    };
    let buffer = device.create_buffer(&buffer_info, None)?;

    let mem_requirement = device.get_buffer_memory_requirements(buffer);

    let alloc_info = vk::MemoryAllocateInfo {
        s_type: StructureType::MEMORY_ALLOCATE_INFO,
        p_next: ptr::null(),
        allocation_size: mem_requirement.size,
        memory_type_index: find_memory_type(mem_requirement.memory_type_bits, properties, physical_device, instance),
    };

    let device_memory = device.allocate_memory(&alloc_info, None)?;

    device.bind_buffer_memory(buffer, device_memory, 0)?;
    Ok((buffer, device_memory))
}

use stb_image::image::{self, LoadResult};

fn load_texture(path: &str) -> Result<(Vec<u8>, u32, u32), String> {
    let (width, height, data) = match image::load(path) {
        LoadResult::Error(_) => todo!(),
        LoadResult::ImageU8(x) => (x.width as u32, x.height as u32, x.data),
        LoadResult::ImageF32(x) => panic!("no "),
    };

    Ok((data, width, height))
}

// Create Vulkan image from loaded texture data
unsafe fn create_texture_image(
    device: &ash::Device,
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    texture_data: &[u8],
    width: u32,
    height: u32,
) -> VkResult<(vk::Image, vk::DeviceMemory)> {
    let image_size = (width * height * 4) as vk::DeviceSize;

    let (stage_buffer, stage_memory) = create_buffer(
        device,
        instance,
        physical_device,
        image_size,
        BufferUsageFlags::TRANSFER_SRC,
        MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
    )?;

    let data = device.map_memory(stage_memory, 0, image_size, MemoryMapFlags::empty())? as *mut u8;
    data.copy_from_nonoverlapping(texture_data.as_ptr(), VERTICES.len());
    device.unmap_memory(stage_memory);

    let (image, image_memory) = create_image(
        device,
        instance,
        physical_device,
        texture_data,
        width,
        height,
        vk::Format::R8G8B8A8_SRGB,
        vk::ImageTiling::OPTIMAL,
        vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
        MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    Ok((image, image_memory))
}

unsafe fn create_image(
    device: &ash::Device,
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    texture_data: &[u8],
    width: u32,
    height: u32,
    format: vk::Format,
    tiling: vk::ImageTiling,
    usage: vk::ImageUsageFlags,
    properties: MemoryPropertyFlags,
) -> VkResult<(vk::Image, vk::DeviceMemory)> {
    let image_size = (width * height * 4) as vk::DeviceSize;

    let extent = vk::Extent3D { width, height, depth: 1 };
    let image_info = vk::ImageCreateInfo {
        s_type: StructureType::IMAGE_CREATE_INFO,
        p_next: ptr::null(),
        flags: ImageCreateFlags::empty(),
        image_type: vk::ImageType::TYPE_2D,
        format,
        extent: extent,
        mip_levels: 1,
        array_layers: 1,
        samples: vk::SampleCountFlags::TYPE_1,
        tiling,
        usage,
        sharing_mode: vk::SharingMode::EXCLUSIVE,
        queue_family_index_count: 0,
        p_queue_family_indices: ptr::null(),
        initial_layout: vk::ImageLayout::UNDEFINED,
    };

    let image = device.create_image(&image_info, None)?;

    let mem_requirement = device.get_image_memory_requirements(image);

    let alloc_info = vk::MemoryAllocateInfo {
        s_type: StructureType::MEMORY_ALLOCATE_INFO,
        p_next: ptr::null(),
        allocation_size: mem_requirement.size,
        memory_type_index: find_memory_type(mem_requirement.memory_type_bits, properties, physical_device, instance),
    };

    let image_memory = device.allocate_memory(&alloc_info, None)?;
    device.bind_image_memory(image, image_memory, 0)?;
    Ok((image, image_memory))
}

unsafe fn begin_single_commands(device: &ash::Device, command_pool: vk::CommandPool) -> VkResult<(vk::CommandBuffer)> {
    let alloc_info = vk::CommandBufferAllocateInfo {
        s_type: StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
        p_next: ptr::null(),
        command_pool: command_pool,
        level: vk::CommandBufferLevel::PRIMARY,
        command_buffer_count: 1,
    };
    let command_buffer = device.allocate_command_buffers(&alloc_info)?;

    let begin_info = vk::CommandBufferBeginInfo {
        s_type: StructureType::COMMAND_BUFFER_BEGIN_INFO,
        p_next: ptr::null(),
        flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
        p_inheritance_info: ptr::null(),
    };

    device.begin_command_buffer(command_buffer[0], &begin_info)?;

    Ok((command_buffer[0]))
}

unsafe fn end_single_time_command(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    command_pool: vk::CommandPool,
    queue: vk::Queue,
) -> VkResult<()> {
    device.end_command_buffer(command_buffer)?;

    let submit_info = [vk::SubmitInfo {
        s_type: StructureType::SUBMIT_INFO,
        p_next: ptr::null(),
        wait_semaphore_count: 0,
        p_wait_semaphores: ptr::null(),
        p_wait_dst_stage_mask: ptr::null(),
        command_buffer_count: 1,
        p_command_buffers: &command_buffer,
        signal_semaphore_count: 0,
        p_signal_semaphores: ptr::null(),
    }];

    device.queue_submit(queue, &submit_info, vk::Fence::null())?;
    device.queue_wait_idle(queue)?;

    device.free_command_buffers(command_pool, &[command_buffer]);

    Ok(())
}

unsafe fn transition_image_layout(
    device: &ash::Device,
    command_pool: vk::CommandPool,
    image: vk::Image,
    format: vk::Format,
    old_layout: vk::ImageLayout,
    new_layout: vk::ImageLayout,
) -> VkResult<()> {
    let command_buffer = begin_single_commands(device, command_pool)?;

    let barrier = vk::ImageMemoryBarrier {
        s_type: StructureType::IMAGE_MEMORY_BARRIER,
        p_next: ptr::null(),
        src_access_mask: vk::AccessFlags::empty(),
        dst_access_mask: vk::AccessFlags::empty(),
        old_layout,
        new_layout,
        src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
        dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
        image,
        subresource_range: vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        },
    };

    // device.cmd_pipeline_barrier(
    //     command_buffer,
    //     vk::PipelineStageFlags::empty(),
    //     vk::PipelineStageFlags::empty(),
    //     vk::DependencyFlags::empty(),
    //     ptr::null(),
    //     ptr::null(),
    //     &[barrier],
    // );

    Ok(())
}
