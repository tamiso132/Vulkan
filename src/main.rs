#![feature(try_blocks, offset_of)]
use anyhow::Result;
use ash::{
    prelude::VkResult,
    vk::{self, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT, DebugUtilsMessengerCreateInfoEXT},
    Entry, Instance,
};
use std::ptr::{self};
use std::{
    ffi::{c_void, CStr, CString},
    os::raw::c_char,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use vulky::{
    buffer::{
        create_command_buffers, create_command_pool, create_frame_buffer, create_index_buffer, create_sync_objects,
        create_vertex_buffer, record_command_buffer, MAX_FRAMES_IN_FLIGHT,
    },
    constant::{validation, version},
    device::{create_logical_device, pick_physical_device},
    pipeline::{create_pipeline_layout, create_render_pass},
    platform, utility, SwapChainSupportDetails,
};

mod texture;
mod types;

/// The Vulkan SDK version that started requiring the portability subset extension for macOS.
pub const PORTABILITY_MACOS_VERSION: u32 = vk::make_api_version(0, 1, 3, 216);
fn main() {
    // Create an event loop and window using winit
    unsafe {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().with_title("Vulkan Window").build(&event_loop).unwrap();

        let mut app: VulkanApp = match VulkanApp::new(&window) {
            Ok(el) => el,
            Err(e) => panic!("{e}"),
        };
        let mut quit = false;
        let _resize = false;

        event_loop.run(move |event, _, control_flow| {
            // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
            // dispatched any events. This is ideal for games and similar applications.
            control_flow.set_poll();

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("The close button was pressed; stopping");
                    let _ = app.device.device_wait_idle();
                    app.destroy();
                    quit = true;
                    control_flow.set_exit();
                }
                Event::MainEventsCleared => {
                    // Application update code.
                    // Queue a RedrawRequested event.
                    //
                    // You only need to call this if you've determined that you need to redraw, in
                    // applications which do not always need to. Applications that redraw continuously
                    // can just render here instead.

                    if !quit {
                        match app.draw_frame() {
                            Ok(_x) => {}
                            Err(_e) => {
                                panic!("recreates");
                            }
                        }
                    } //app.draw_frame();

                    window.request_redraw();
                }
                Event::RedrawRequested(_) => {

                    // Redraw the application.
                    //
                    // It's preferable for applications that do not render continuously to render in
                    // this event rather than in MainEventsCleared, since rendering in here allows
                    // the program to gracefully handle redraws requested by the OS.
                }
                Event::WindowEvent { window_id: _, event } => match event {
                    WindowEvent::Resized(x) => {
                        if x.width == 0 && x.height == 0 {
                            app.minimized = true;
                        } else {
                            window.set_inner_size(x);
                            app.minimized = false;
                        }
                    }

                    _ => {}
                },
                _ => (),
            }
        });
    }
}

struct VulkanApp {
    /// Global state for the app
    /// includes application specific info, including layers and extensions
    instance: ash::Instance,
    /// Used for loading vulkan statically or during runtime
    entry: ash::Entry,

    /// debug extension
    debug_util_loader: ash::extensions::ext::DebugUtils,
    debug_messenger: vk::DebugUtilsMessengerEXT,

    /// it is the interface to communicate with the gpu,
    /// has all info about the capabilities of the gpu.
    physical_device: vk::PhysicalDevice,
    /// Serves as a handle to interact with Vulkan API
    /// like managing vulkan resources, like (command buffers, queue handles, swapchain, pipeline, etc)
    /// Also used to enable extensions
    device: ash::Device,

    //The interfacce with the surface
    surface_loader: ash::extensions::khr::Surface,
    /// Is the surface used when drawing, platform specific.
    surface: vk::SurfaceKHR,

    // Queues
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,
    transfer_queue: vk::Queue,

    //Swapchain
    swapchain: vk::SwapchainKHR,
    swapchain_loader: ash::extensions::khr::Swapchain,
    swapchain_format: vk::Format,
    swapchain_extent: vk::Extent2D,
    swapchain_images: Vec<vk::Image>,
    swapchain_image_views: Vec<vk::ImageView>,

    // Pipeline
    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,

    //CommandPool
    graphic_command_pool: vk::CommandPool,
    transfer_command_pool: vk::CommandPool,

    // buffers
    swapchain_framebuffers: Vec<vk::Framebuffer>,
    command_buffers: Vec<vk::CommandBuffer>,

    // semaphore
    image_availables: Vec<vk::Semaphore>,
    render_finisheds: Vec<vk::Semaphore>,
    in_flights: Vec<vk::Fence>,

    current_frame: usize,
    framebuffer_resized: bool,
    minimized: bool,

    vertex_buffer: vk::Buffer,
    vertex_memory: vk::DeviceMemory,

    index_buffer: vk::Buffer,
    index_memory: vk::DeviceMemory,
}
impl VulkanApp {
    unsafe fn new(window: &Window) -> Result<Self> {
        let entry = ash::Entry::load()?;
        let instance = create_instance(&entry)?;

        let (surface, surface_loader) = create_surface(&entry, &instance, window)?;
        let (debug_util_loader, debug_messenger) = setup_debug_utils(&entry, &instance)?;

        let physical_device = pick_physical_device(&instance, &surface_loader, &surface)?;
        let (device, queue_family) = create_logical_device(physical_device, &instance, surface, &surface_loader)?;
        let graphics_queue = device.get_device_queue(queue_family.graphics_family.unwrap(), 0);
        let present_queue = device.get_device_queue(queue_family.present_family.unwrap(), 0);
        let transfer_queue = device.get_device_queue(queue_family.transfer_family.unwrap(), 0);

        let (swapchain_loader, swapchain, swapchain_extent, swapchain_format, swapchain_images, swapchain_image_views) =
            SwapChainSupportDetails::create_swapchain(&instance, &device, &surface_loader, surface, physical_device)?;

        let render_pass = create_render_pass(swapchain_format, &device)?;
        let swapchain_framebuffers = create_frame_buffer(&device, &swapchain_image_views, render_pass, swapchain_extent)?;
        let (pipeline, pipeline_layout) = create_pipeline_layout(&device, swapchain_extent, render_pass)?;

        let graphic_command_pool = create_command_pool(&device, &queue_family.graphics_family)?;
        let transfer_command_pool = create_command_pool(&device, &queue_family.transfer_family)?;
        let (vertex_buffer, vertex_memory) =
            create_vertex_buffer(&device, physical_device, &instance, transfer_command_pool, transfer_queue)?;
        let (index_buffer, index_memory) =
            create_index_buffer(&device, &instance, physical_device, transfer_command_pool, transfer_queue)?;

        let command_buffers = create_command_buffers(&device, graphic_command_pool)?;
        let (in_flights, image_availables, render_finisheds) = create_sync_objects(&device)?;
        Ok(Self {
            instance,
            entry,
            physical_device,
            device,
            graphics_queue,
            present_queue,
            transfer_queue,
            transfer_command_pool,
            surface,
            surface_loader,
            swapchain,
            swapchain_loader,
            swapchain_extent,
            swapchain_format,
            swapchain_images,
            swapchain_image_views,
            swapchain_framebuffers,
            render_pass,
            pipeline_layout,
            pipeline,
            graphic_command_pool,
            command_buffers,
            debug_util_loader,
            debug_messenger,
            in_flights,
            image_availables,
            render_finisheds,
            current_frame: 0,
            framebuffer_resized: false,
            minimized: false,
            vertex_buffer,
            vertex_memory,
            index_buffer,
            index_memory,
        })
    }

    pub unsafe fn draw_frame(&mut self) -> VkResult<()> {
        // a render pass, is a sequence of rendering operations, organized as series of subpasses
        // each subpass describes, image, rendering commands
        let wait_fences = [self.in_flights[self.current_frame]];

        self.device
            .wait_for_fences(&wait_fences, true, std::u64::MAX)
            .expect("Failed to wait for Fence!");

        let (image_index, _is_sub_optimal) = unsafe {
            let result = self.swapchain_loader.acquire_next_image(
                self.swapchain,
                std::u64::MAX,
                self.image_availables[self.current_frame],
                vk::Fence::null(),
            );
            match result {
                Ok(image_index) => image_index,
                Err(vk_result) => match vk_result {
                    vk::Result::ERROR_OUT_OF_DATE_KHR => {
                        self.recreate_swapchain()?;
                        return Ok(());
                    }
                    _ => panic!("Failed to acquire Swap Chain Image!"),
                },
            }
        };

        self.device.reset_fences(&wait_fences)?;
        self.device
            .reset_command_buffer(self.command_buffers[self.current_frame], vk::CommandBufferResetFlags::empty())?;
        record_command_buffer(
            &self.device,
            self.command_buffers[self.current_frame],
            self.render_pass,
            &self.swapchain_framebuffers,
            image_index,
            self.swapchain_extent,
            self.pipeline,
            self.vertex_buffer,
            self.index_buffer,
        )?;

        let wait_semaphores = [self.image_availables[self.current_frame]];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let signal_semaphores = [self.render_finisheds[self.current_frame]];

        let submit_infos = [vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            p_next: ptr::null(),
            wait_semaphore_count: wait_semaphores.len() as u32,
            p_wait_semaphores: wait_semaphores.as_ptr(),
            p_wait_dst_stage_mask: wait_stages.as_ptr(),
            command_buffer_count: 1,
            p_command_buffers: &self.command_buffers[self.current_frame],
            signal_semaphore_count: signal_semaphores.len() as u32,
            p_signal_semaphores: signal_semaphores.as_ptr(),
        }];

        self.device
            .queue_submit(self.graphics_queue, &submit_infos, wait_fences[0])
            .expect("Failed to execute queue submit.");

        let swapchains = self.swapchain;

        let present_info = vk::PresentInfoKHR {
            s_type: vk::StructureType::PRESENT_INFO_KHR,
            p_next: ptr::null(),
            wait_semaphore_count: 1,
            p_wait_semaphores: signal_semaphores.as_ptr(),
            swapchain_count: 1,
            p_swapchains: &swapchains,
            p_image_indices: &image_index,
            p_results: ptr::null_mut(),
        };

        let result = unsafe { self.swapchain_loader.queue_present(self.present_queue, &present_info) };

        let is_resized = match result {
            Ok(_) => self.framebuffer_resized,
            Err(vk_result) => match vk_result {
                vk::Result::ERROR_OUT_OF_DATE_KHR | vk::Result::SUBOPTIMAL_KHR => true,
                _ => panic!("Failed to execute queue present."),
            },
        };
        if is_resized {
            self.framebuffer_resized = false;
            self.recreate_swapchain()?;
        }

        self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT as usize;
        Ok(())
    }

    unsafe fn destroy(&mut self) {
        if validation::ENABLED {
            self.debug_util_loader
                .destroy_debug_utils_messenger(self.debug_messenger, None);
        }
        for i in 0..MAX_FRAMES_IN_FLIGHT as usize {
            self.device.destroy_fence(self.in_flights[i], None);
            self.device.destroy_semaphore(self.image_availables[i], None);
            self.device.destroy_semaphore(self.render_finisheds[i], None);
        }
        self.device.destroy_command_pool(self.graphic_command_pool, None);
        self.device.destroy_command_pool(self.transfer_command_pool, None);

        self.clean_swapchain();

        self.device.destroy_buffer(self.vertex_buffer, None);
        self.device.destroy_buffer(self.index_buffer, None);
        self.device.free_memory(self.vertex_memory, None);
        self.device.free_memory(self.index_memory, None);

        self.device.destroy_pipeline(self.pipeline, None);
        self.device.destroy_pipeline_layout(self.pipeline_layout, None);
        self.device.destroy_render_pass(self.render_pass, None);

        self.surface_loader.destroy_surface(self.surface, None);
        self.device.destroy_device(None);
        self.instance.destroy_instance(None);
    }

    pub unsafe fn recreate_swapchain(&mut self) -> VkResult<()> {
        self.device.device_wait_idle()?;
        self.clean_swapchain();

        (
            self.swapchain_loader,
            self.swapchain,
            self.swapchain_extent,
            self.swapchain_format,
            self.swapchain_images,
            self.swapchain_image_views,
        ) = SwapChainSupportDetails::create_swapchain(
            &self.instance,
            &self.device,
            &self.surface_loader,
            self.surface,
            self.physical_device,
        )?;

        self.swapchain_framebuffers = create_frame_buffer(
            &self.device,
            &self.swapchain_image_views,
            self.render_pass,
            self.swapchain_extent,
        )?;

        Ok(())
    }

    unsafe fn clean_swapchain(&mut self) {
        while self.swapchain_framebuffers.len() > 0 {
            let e = self.swapchain_framebuffers.pop().unwrap();
            self.device.destroy_framebuffer(e, None);
        }

        for _ in 0..self.swapchain_image_views.len() {
            let image_view = self.swapchain_image_views.pop().unwrap();
            self.device.destroy_image_view(image_view, None);
        }

        self.swapchain_loader.destroy_swapchain(self.swapchain, None);
    }
}

unsafe fn create_instance(entry: &ash::Entry) -> Result<ash::Instance> {
    let app_name = CString::new("window_title").unwrap();
    let engine_name = CString::new("Vulkan Engine").unwrap();

    if validation::ENABLED && !check_validation_support(&entry)? {
        panic!("Validation layer is requested, but no available");
    }
    let debug_utils_create_info = debug_create_info()?;

    let app_info = vk::ApplicationInfo::builder()
        .engine_name(&engine_name)
        .application_name(&app_name)
        .api_version(version::API_VERSION)
        .engine_version(version::ENGINE_VERSION)
        .application_version(version::APPLICATION_VERSION)
        .build();

    let mut extension = vulky::platform::required_extension_names();

    let layer_names = [CStr::from_bytes_with_nul_unchecked(validation::LAYER_NAME_BYTES)];
    let layers_names_raw: Vec<*const c_char> = layer_names.iter().map(|raw_name| raw_name.as_ptr()).collect();

    //macos portability
    let flags = if cfg!(target_os = "macos") && PORTABILITY_MACOS_VERSION >= version::API_VERSION {
        extension.push(ash::vk::KhrGetPhysicalDeviceProperties2Fn::name().as_ptr());
        extension.push(ash::vk::KhrPortabilityEnumerationFn::name().as_ptr());
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
    } else {
        vk::InstanceCreateFlags::empty()
    };

    let instance_info = vk::InstanceCreateInfo {
        s_type: vk::StructureType::INSTANCE_CREATE_INFO,
        p_next: if validation::ENABLED {
            &debug_utils_create_info as *const vk::DebugUtilsMessengerCreateInfoEXT as *const c_void
        } else {
            ptr::null()
        },
        flags,
        p_application_info: &app_info,
        pp_enabled_layer_names: if validation::ENABLED {
            println!("validation enabled");
            layers_names_raw.as_ptr()
        } else {
            ptr::null()
        },
        enabled_layer_count: if validation::ENABLED { 1 } else { 0 },
        enabled_extension_count: extension.len() as u32,
        pp_enabled_extension_names: extension.as_ptr(),
    };

    let instance = entry.create_instance(&instance_info, None)?;
    Ok(instance)
}

unsafe fn create_surface(
    entry: &Entry,
    instance: &Instance,
    window: &Window,
) -> Result<(ash::vk::SurfaceKHR, ash::extensions::khr::Surface)> {
    let surface = platform::create_surface(entry, instance, window)?;
    let surface_loader = ash::extensions::khr::Surface::new(entry, instance);

    Ok((surface, surface_loader))
}

unsafe fn check_validation_support(entry: &Entry) -> Result<bool> {
    let layer_properties = entry.enumerate_instance_layer_properties()?;
    let mut is_layer_found = false;

    for layer_property in layer_properties.iter() {
        let s = utility::vk_to_string(&layer_property.layer_name);

        if s == validation::LAYER_NAME {
            is_layer_found = true;
        }
        println!("layer: {}", s);
    }
    if !is_layer_found {
        eprintln!("Required Layer is not found");
        return Ok(false);
    }

    Ok(is_layer_found)
}

fn setup_debug_utils(
    entry: &ash::Entry,
    instance: &ash::Instance,
) -> Result<(ash::extensions::ext::DebugUtils, vk::DebugUtilsMessengerEXT)> {
    let debug_utils_loader = ash::extensions::ext::DebugUtils::new(entry, instance);
    if !validation::ENABLED {
        Ok((debug_utils_loader, ash::vk::DebugUtilsMessengerEXT::null()))
    } else {
        let messenger_ci = debug_create_info()?;

        println!("debug enabled setup");

        let utils_messenger = unsafe {
            debug_utils_loader
                .create_debug_utils_messenger(&messenger_ci, None)
                .expect("Debug Utils Callback")
        };
        Ok((debug_utils_loader, utils_messenger))
    }
}

unsafe extern "system" fn debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> vk::Bool32 {
    let severity = match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => "[Verbose]",
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => "[Warning]",
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => "[Error]",
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => "[Info]",
        _ => "[Unknown]",
    };
    let types = match message_type {
        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "[General]",
        vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "[Performance]",
        vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "[Validation]",
        _ => "[Unknown]",
    };
    let message = CStr::from_ptr((*p_callback_data).p_message);
    println!("[Debug]{}{}{:?}", severity, types, message);

    vk::FALSE
}

fn debug_create_info() -> Result<DebugUtilsMessengerCreateInfoEXT> {
    Ok(vk::DebugUtilsMessengerCreateInfoEXT {
        s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
        p_next: ptr::null(),
        flags: vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
        message_severity: DebugUtilsMessageSeverityFlagsEXT::WARNING
            | DebugUtilsMessageSeverityFlagsEXT::ERROR
            | DebugUtilsMessageSeverityFlagsEXT::VERBOSE,

        message_type: DebugUtilsMessageTypeFlagsEXT::GENERAL
            | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
            | DebugUtilsMessageTypeFlagsEXT::VALIDATION,
        pfn_user_callback: Some(debug_callback),
        p_user_data: ptr::null_mut(),
    })
}
