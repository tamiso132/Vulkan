use std::ptr;

use ash::vk::{self, StructureType};

use crate::{constant::Vertex, utility};
use anyhow::Result;

pub unsafe fn create_pipeline_layout(
    device: &ash::Device,
    swapchain_extent: vk::Extent2D,
    render_pass: vk::RenderPass,
) -> Result<(vk::Pipeline, vk::PipelineLayout)> {
    let frag_bytes = utility::read_file("shaders/spv/frag.spv")?;
    let vert_bytes = utility::read_file("shaders/spv/vert.spv")?;

    let vert_shader = create_shader_module(device, vert_bytes)?;
    let frag_shader = create_shader_module(device, frag_bytes)?;

    let entry_point_name = std::ffi::CString::new("main").expect("CString::new failed");

    let mut shader_stages = [
        vk::PipelineShaderStageCreateInfo {
            s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineShaderStageCreateFlags::empty(),
            stage: vk::ShaderStageFlags::VERTEX,
            module: vert_shader,
            p_name: entry_point_name.as_ptr(),
            p_specialization_info: ptr::null(),
        },
        vk::PipelineShaderStageCreateInfo {
            s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::PipelineShaderStageCreateFlags::empty(),
            stage: vk::ShaderStageFlags::FRAGMENT,
            module: frag_shader,
            p_name: entry_point_name.as_ptr(),
            p_specialization_info: ptr::null(),
        },
    ];

    let binding_description = Vertex::get_binding_description();
    let attribute_description = Vertex::get_input_attribute_description();

    let vertex_input = vk::PipelineVertexInputStateCreateInfo {
        s_type: vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::PipelineVertexInputStateCreateFlags::empty(),
        vertex_binding_description_count: 1,
        p_vertex_binding_descriptions: &binding_description,
        vertex_attribute_description_count: attribute_description.len() as u32,
        p_vertex_attribute_descriptions: attribute_description.as_ptr(),
    };

    let vertex_assembly_info = vk::PipelineInputAssemblyStateCreateInfo {
        s_type: vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::PipelineInputAssemblyStateCreateFlags::empty(),
        topology: vk::PrimitiveTopology::TRIANGLE_LIST,
        primitive_restart_enable: vk::FALSE,
    };

    let mut view_port = vk::Viewport::default();
    view_port.x = 0.0;
    view_port.y = 0.0;
    view_port.width = swapchain_extent.width as f32;
    view_port.height = swapchain_extent.height as f32;
    view_port.min_depth = 0.0;
    view_port.max_depth = 1.0;

    let mut scissor = vk::Rect2D::default();
    scissor.offset = vk::Offset2D { x: 0, y: 0 };
    scissor.extent = swapchain_extent;

    let mut view_state = vk::PipelineViewportStateCreateInfo {
        s_type: vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::PipelineViewportStateCreateFlags::empty(),
        viewport_count: 1,
        p_viewports: &view_port,
        scissor_count: 1,
        p_scissors: &scissor,
    };

    let states = vec![vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    let mut dynamic_state = vk::PipelineDynamicStateCreateInfo::default();

    dynamic_state.s_type = vk::StructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO;
    dynamic_state.dynamic_state_count = states.len() as u32;
    dynamic_state.p_dynamic_states = states.as_ptr();

    let mut input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default();
    input_assembly.s_type = StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO;
    input_assembly.topology = vk::PrimitiveTopology::TRIANGLE_LIST;
    input_assembly.primitive_restart_enable = vk::FALSE;

    let mut rasterizer = vk::PipelineRasterizationStateCreateInfo::default();
    rasterizer.s_type = vk::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO;
    rasterizer.depth_clamp_enable = vk::FALSE;
    rasterizer.rasterizer_discard_enable = vk::FALSE;
    // fills the primitive triangle
    rasterizer.polygon_mode = vk::PolygonMode::FILL;
    rasterizer.line_width = 1.0;

    // face is forward or whatever
    rasterizer.cull_mode = vk::CullModeFlags::BACK;
    rasterizer.front_face = vk::FrontFace::CLOCKWISE;

    // can be used for shadow mapping
    rasterizer.depth_bias_enable = vk::FALSE;
    rasterizer.depth_bias_constant_factor = 0.0;
    rasterizer.depth_bias_clamp = 0.0;
    rasterizer.depth_bias_slope_factor = 0.0;

    let mut multi_sampling = vk::PipelineMultisampleStateCreateInfo::default();
    multi_sampling.s_type = vk::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO;
    multi_sampling.sample_shading_enable = vk::FALSE;
    multi_sampling.rasterization_samples = vk::SampleCountFlags::TYPE_1;
    multi_sampling.min_sample_shading = 1.0;
    multi_sampling.p_sample_mask = std::ptr::null();
    multi_sampling.alpha_to_coverage_enable = vk::FALSE;
    multi_sampling.alpha_to_one_enable = vk::FALSE;

    let mut color_blend_attachment = vk::PipelineColorBlendAttachmentState::default();
    color_blend_attachment.color_write_mask =
        vk::ColorComponentFlags::R | vk::ColorComponentFlags::G | vk::ColorComponentFlags::B | vk::ColorComponentFlags::A;
    color_blend_attachment.blend_enable = vk::FALSE;
    color_blend_attachment.src_color_blend_factor = vk::BlendFactor::ONE;
    color_blend_attachment.dst_color_blend_factor = vk::BlendFactor::ZERO;
    color_blend_attachment.src_alpha_blend_factor = vk::BlendFactor::ONE;
    color_blend_attachment.dst_alpha_blend_factor = vk::BlendFactor::ZERO;

    color_blend_attachment.color_blend_op = vk::BlendOp::ADD;
    color_blend_attachment.alpha_blend_op = vk::BlendOp::ADD;

    let mut color_blending = vk::PipelineColorBlendStateCreateInfo::default();
    color_blending.s_type = vk::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO;
    color_blending.logic_op_enable = vk::FALSE;
    color_blending.logic_op = vk::LogicOp::COPY;
    color_blending.attachment_count = 1;
    color_blending.p_attachments = &color_blend_attachment;
    color_blending.blend_constants[0] = 0.0;
    color_blending.blend_constants[1] = 0.0;
    color_blending.blend_constants[2] = 0.0;
    color_blending.blend_constants[3] = 0.0;

    let mut pipeline_layout_info = vk::PipelineLayoutCreateInfo::default();
    pipeline_layout_info.s_type = vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO;
    pipeline_layout_info.set_layout_count = 0;
    pipeline_layout_info.p_set_layouts = std::ptr::null();
    pipeline_layout_info.push_constant_range_count = 0;
    pipeline_layout_info.p_push_constant_ranges = std::ptr::null();

    let layout = device.create_pipeline_layout(&pipeline_layout_info, None)?;

    let mut info = vk::GraphicsPipelineCreateInfo::default();
    info.s_type = vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO;

    info.stage_count = 2;
    info.p_stages = shader_stages.as_ptr();
    info.p_vertex_input_state = &vertex_input;
    info.p_viewport_state = &view_state;
    info.p_rasterization_state = &rasterizer;
    info.p_multisample_state = &multi_sampling;
    info.p_depth_stencil_state = std::ptr::null();
    info.p_color_blend_state = &color_blending;
    info.p_dynamic_state = &dynamic_state;
    info.layout = layout;
    info.render_pass = render_pass;
    info.subpass = 0;
    info.base_pipeline_handle = vk::Pipeline::null();
    info.base_pipeline_index = -1;
    info.p_input_assembly_state = &input_assembly;

    let pipeline = device
        .create_graphics_pipelines(vk::PipelineCache::null(), &[info], None)
        .expect("Error creating the graphic pipeline");

    device.destroy_shader_module(vert_shader, None);
    device.destroy_shader_module(frag_shader, None);

    Ok((pipeline[0], layout))
}

unsafe fn create_shader_module(device: &ash::Device, bytes: Vec<u8>) -> Result<vk::ShaderModule> {
    let mut create_info = vk::ShaderModuleCreateInfo::default();

    create_info.s_type = vk::StructureType::SHADER_MODULE_CREATE_INFO;
    create_info.p_next = std::ptr::null();
    create_info.flags = vk::ShaderModuleCreateFlags::empty();
    create_info.code_size = bytes.len();
    create_info.p_code = bytes.as_ptr() as *const u32; // MAY HAVE TO DO SOMETHING HERE

    let shader_module = device.create_shader_module(&create_info, None)?;
    Ok(shader_module)
}

pub unsafe fn create_render_pass(swapchain_format: vk::Format, device: &ash::Device) -> Result<vk::RenderPass> {
    let color_attachment = vk::AttachmentDescription {
        format: swapchain_format,
        flags: vk::AttachmentDescriptionFlags::empty(),
        samples: vk::SampleCountFlags::TYPE_1,
        load_op: vk::AttachmentLoadOp::CLEAR,
        store_op: vk::AttachmentStoreOp::STORE,
        stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
        stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
        initial_layout: vk::ImageLayout::UNDEFINED,
        final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
    };

    let color_attachment_ref = vk::AttachmentReference {
        attachment: 0,
        layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
    };

    let subpasses = [vk::SubpassDescription {
        color_attachment_count: 1,
        p_color_attachments: &color_attachment_ref,
        p_depth_stencil_attachment: ptr::null(),
        flags: vk::SubpassDescriptionFlags::empty(),
        pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
        input_attachment_count: 0,
        p_input_attachments: ptr::null(),
        p_resolve_attachments: ptr::null(),
        preserve_attachment_count: 0,
        p_preserve_attachments: ptr::null(),
    }];

    let render_pass_attachments = [color_attachment];

    let subpass_dependencies = [vk::SubpassDependency {
        src_subpass: vk::SUBPASS_EXTERNAL,
        dst_subpass: 0,
        src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        src_access_mask: vk::AccessFlags::empty(),
        dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
        dependency_flags: vk::DependencyFlags::empty(),
    }];

    let renderpass_create_info = vk::RenderPassCreateInfo {
        s_type: vk::StructureType::RENDER_PASS_CREATE_INFO,
        flags: vk::RenderPassCreateFlags::empty(),
        p_next: ptr::null(),
        attachment_count: render_pass_attachments.len() as u32,
        p_attachments: render_pass_attachments.as_ptr(),
        subpass_count: subpasses.len() as u32,
        p_subpasses: subpasses.as_ptr(),
        dependency_count: subpass_dependencies.len() as u32,
        p_dependencies: subpass_dependencies.as_ptr(),
    };

    unsafe {
        Ok(device
            .create_render_pass(&renderpass_create_info, None)
            .expect("Failed to create render pass!"))
    }
}
