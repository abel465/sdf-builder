use wgpu::TextureView;

use crate::{
    context::GraphicsContext,
    controller::Controller,
    model::Vertex,
    shader::CompiledShaderModules,
    ui::{Ui, UiState},
    Options,
};
use wgpu::util::DeviceExt;

#[cfg(not(target_arch = "wasm32"))]
mod shaders {
    #[allow(non_upper_case_globals)]
    pub const main_fs: &str = "main_fs";
    #[allow(non_upper_case_globals)]
    pub const main_vs: &str = "main_vs";
}
#[cfg(target_arch = "wasm32")]
mod shaders {
    include!(concat!(env!("OUT_DIR"), "/entry_points.rs"));
}

pub struct RenderPass {
    pipeline_layout: wgpu::PipelineLayout,
    render_pipeline: wgpu::RenderPipeline,
    ui_renderer: egui_wgpu::Renderer,
    options: Options,
    vertex_buffer: Option<wgpu::Buffer>,
}

impl RenderPass {
    pub fn new(
        ctx: &GraphicsContext,
        compiled_shader_modules: CompiledShaderModules,
        options: Options,
        maybe_vertices: Option<&[Vertex]>,
    ) -> Self {
        let pipeline_layout = ctx
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[wgpu::PushConstantRange {
                    stages: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    range: 0..shared::push_constants::largest_size() as u32,
                }],
            });

        let render_pipeline = create_pipeline(
            &options,
            &ctx.device,
            &pipeline_layout,
            ctx.config.format,
            compiled_shader_modules,
            maybe_vertices.is_some(),
        );
        let vertex_buffer = create_vertex_buffer(ctx, maybe_vertices);

        let ui_renderer = egui_wgpu::Renderer::new(&ctx.device, ctx.config.format, None, 1);

        Self {
            pipeline_layout,
            render_pipeline,
            ui_renderer,
            options,
            vertex_buffer,
        }
    }

    pub fn render(
        &mut self,
        ctx: &GraphicsContext,
        window: &winit::window::Window,
        ui: &mut Ui,
        ui_state: &mut UiState,
        controller: &mut dyn Controller,
    ) -> Result<(), wgpu::SurfaceError> {
        let output = match ctx.surface.get_current_texture() {
            Ok(surface_texture) => surface_texture,
            Err(err) => {
                eprintln!("get_current_texture error: {err:?}");
                return match err {
                    wgpu::SurfaceError::Lost => {
                        ctx.surface.configure(&ctx.device, &ctx.config);
                        Ok(())
                    }
                    _ => Err(err),
                };
            }
        };
        let output_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.render_shader(ctx, &output_view, controller);
        self.render_ui(ctx, &output_view, window, ui, ui_state, controller);

        output.present();

        Ok(())
    }

    fn render_shader(
        &mut self,
        ctx: &GraphicsContext,
        output_view: &TextureView,
        controller: &mut dyn Controller,
    ) {
        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Shader Encoder"),
            });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Shader Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &output_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_push_constants(
                wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                0,
                controller.push_constants(),
            );
            let num_vertices = if let Some(vertex_buffer) = &self.vertex_buffer {
                rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
                vertex_buffer.size() as u32 / std::mem::size_of::<Vertex>() as u32
            } else {
                3
            };
            rpass.draw(0..num_vertices, 0..1);
        }

        ctx.queue.submit(Some(encoder.finish()));
    }

    fn render_ui(
        &mut self,
        ctx: &GraphicsContext,
        output_view: &TextureView,
        window: &winit::window::Window,
        ui: &mut Ui,
        ui_state: &mut UiState,
        controller: &mut dyn Controller,
    ) {
        let (clipped_primitives, textures_delta) = ui.prepare(window, ui_state, controller);

        let screen_descriptor = egui_wgpu::renderer::ScreenDescriptor {
            size_in_pixels: [ctx.config.width, ctx.config.height],
            pixels_per_point: window.scale_factor() as f32,
        };

        for (id, delta) in &textures_delta.set {
            self.ui_renderer
                .update_texture(&ctx.device, &ctx.queue, *id, &delta);
        }

        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("UI Encoder"),
            });

        self.ui_renderer.update_buffers(
            &ctx.device,
            &ctx.queue,
            &mut encoder,
            &clipped_primitives,
            &screen_descriptor,
        );

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("UI Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &output_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            for id in &textures_delta.free {
                self.ui_renderer.free_texture(id);
            }

            self.ui_renderer
                .render(&mut rpass, &clipped_primitives, &screen_descriptor);
        }

        ctx.queue.submit(Some(encoder.finish()));
    }

    pub fn new_module(
        &mut self,
        ctx: &GraphicsContext,
        new_module: CompiledShaderModules,
        maybe_vertices: Option<&[Vertex]>,
    ) {
        self.vertex_buffer = create_vertex_buffer(ctx, maybe_vertices);
        self.render_pipeline = create_pipeline(
            &self.options,
            &ctx.device,
            &self.pipeline_layout,
            ctx.config.format,
            new_module,
            maybe_vertices.is_some(),
        );
    }
}

fn create_vertex_buffer(
    ctx: &GraphicsContext,
    maybe_vertices: Option<&[Vertex]>,
) -> Option<wgpu::Buffer> {
    maybe_vertices.map(|vertices| {
        ctx.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            })
    })
}

fn create_pipeline(
    options: &Options,
    device: &wgpu::Device,
    pipeline_layout: &wgpu::PipelineLayout,
    surface_format: wgpu::TextureFormat,
    compiled_shader_modules: CompiledShaderModules,
    has_vertex_buffer: bool,
) -> wgpu::RenderPipeline {
    // FIXME(eddyb) automate this decision by default.
    let create_module = |module| {
        if options.force_spirv_passthru {
            unsafe { device.create_shader_module_spirv(&module) }
        } else {
            let wgpu::ShaderModuleDescriptorSpirV { label, source } = module;
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label,
                source: wgpu::ShaderSource::SpirV(source),
            })
        }
    };

    let vs_entry_point = shaders::main_vs;
    let fs_entry_point = shaders::main_fs;

    let vs_module_descr = compiled_shader_modules.spv_module_for_entry_point(vs_entry_point);
    let fs_module_descr = compiled_shader_modules.spv_module_for_entry_point(fs_entry_point);

    // HACK(eddyb) avoid calling `device.create_shader_module` twice unnecessarily.
    let vs_fs_same_module = std::ptr::eq(&vs_module_descr.source[..], &fs_module_descr.source[..]);

    let vs_module = &create_module(vs_module_descr);
    let fs_module;
    let fs_module = if vs_fs_same_module {
        vs_module
    } else {
        fs_module = create_module(fs_module_descr);
        &fs_module
    };

    let buffers_empty = &[];
    let buffers = &[crate::model::Vertex::desc()];

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(pipeline_layout),
        vertex: wgpu::VertexState {
            module: vs_module,
            entry_point: vs_entry_point,
            buffers: if has_vertex_buffer {
                buffers
            } else {
                buffers_empty
            },
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        fragment: Some(wgpu::FragmentState {
            module: fs_module,
            entry_point: fs_entry_point,
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
    })
}
