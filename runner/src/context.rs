use crate::app::App;

pub struct GraphicsContext {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

impl GraphicsContext {
    pub async fn new(app: &App) -> GraphicsContext {
        let backends = wgpu::util::backend_bits_from_env()
            .unwrap_or(wgpu::Backends::VULKAN | wgpu::Backends::METAL);
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends,
            dx12_shader_compiler: wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default(),
        });

        let initial_surface = unsafe { instance.create_surface(&app.window) }
            .expect("Failed to create surface from window");

        let adapter = wgpu::util::initialize_adapter_from_env_or_default(
            &instance,
            // Request an adapter which can render to our surface
            Some(&initial_surface),
        )
        .await
        .expect("Failed to find an appropriate adapter");

        let mut features = wgpu::Features::PUSH_CONSTANTS;
        if app.options.force_spirv_passthru {
            features |= wgpu::Features::SPIRV_SHADER_PASSTHROUGH;
        }
        let limits = wgpu::Limits {
            max_push_constant_size: 128,
            ..Default::default()
        };

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features,
                    limits,
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let auto_configure_surface =
            |adapter: &_, device: &_, surface: wgpu::Surface, size: winit::dpi::PhysicalSize<_>| {
                let mut surface_config = surface
                    .get_default_config(adapter, size.width, size.height)
                    .unwrap_or_else(|| {
                        panic!(
                            "Missing formats/present modes in surface capabilities: {:#?}",
                            surface.get_capabilities(adapter)
                        )
                    });

                // TODO: make configurable
                surface_config.present_mode = wgpu::PresentMode::AutoVsync;

                surface.configure(device, &surface_config);

                (surface, surface_config)
            };
        let (surface, config) =
            auto_configure_surface(&adapter, &device, initial_surface, app.window.inner_size());

        GraphicsContext {
            surface,
            device,
            queue,
            config,
        }
    }
}
