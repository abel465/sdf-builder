use crate::{
    context, maybe_watch, render_pass::create_pipeline, CompiledShaderModules, Options,
    RustGPUShader,
};

use shared::ShaderConstants;
use winit::{
    event::{
        ElementState, Event, KeyboardInput, MouseButton, MouseScrollDelta, VirtualKeyCode,
        WindowEvent,
    },
    event_loop::ControlFlow,
};

fn mouse_button_index(button: MouseButton) -> usize {
    match button {
        MouseButton::Left => 0,
        MouseButton::Middle => 1,
        MouseButton::Right => 2,
        MouseButton::Other(i) => 3 + (i as usize),
    }
}

async fn run(
    options: Options,
    app: crate::app::App,
    compiled_shader_modules: CompiledShaderModules,
) {
    let mut context = context::GraphicsContext::new(&app).await;

    let pipeline_layout = context
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[wgpu::PushConstantRange {
                stages: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                range: 0..std::mem::size_of::<ShaderConstants>() as u32,
            }],
        });

    let mut render_pipeline = create_pipeline(
        &options,
        &context.device,
        &pipeline_layout,
        context.config.format,
        compiled_shader_modules,
    );

    let start = std::time::Instant::now();

    let (mut cursor_x, mut cursor_y) = (0.0, 0.0);
    let (mut drag_start_x, mut drag_start_y) = (0.0, 0.0);
    let (mut drag_end_x, mut drag_end_y) = (0.0, 0.0);
    let mut mouse_button_pressed = 0;
    let mut mouse_button_press_since_last_frame = 0;
    let mut mouse_button_press_time = [f32::NEG_INFINITY; 3];
    let mut zoom = 1.0;
    let (mut translate_x, mut translate_y) = (0.0, 0.0);

    app.event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        // let _ = (&instance, &adapter, &pipeline_layout);
        let render_pipeline = &mut render_pipeline;
        let window = &app.window;

        *control_flow = ControlFlow::Wait;
        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // TODO: only redraw if needed
                window.request_redraw();

                let output = match context.surface.get_current_texture() {
                    Ok(surface_texture) => surface_texture,
                    Err(err) => {
                        eprintln!("get_current_texture error: {err:?}");
                        match err {
                            wgpu::SurfaceError::Lost => {
                                context.surface.configure(&context.device, &context.config);
                            }
                            wgpu::SurfaceError::OutOfMemory => {
                                *control_flow = ControlFlow::Exit;
                            }
                            _ => (),
                        }
                        return;
                    }
                };
                let output_view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = context
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
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

                    let time = start.elapsed().as_secs_f32();
                    for (i, press_time) in mouse_button_press_time.iter_mut().enumerate() {
                        if (mouse_button_press_since_last_frame & (1 << i)) != 0 {
                            *press_time = time;
                        }
                    }
                    mouse_button_press_since_last_frame = 0;

                    let push_constants = ShaderConstants {
                        width: window.inner_size().width,
                        height: window.inner_size().height,
                        time,
                        cursor_x,
                        cursor_y,
                        drag_start_x,
                        drag_start_y,
                        drag_end_x,
                        drag_end_y,
                        zoom,
                        translate_x,
                        translate_y,
                        mouse_button_pressed,
                        mouse_button_press_time,
                    };

                    rpass.set_pipeline(render_pipeline);
                    rpass.set_push_constants(
                        wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        0,
                        bytemuck::bytes_of(&push_constants),
                    );
                    rpass.draw(0..3, 0..1);
                }

                context.queue.submit(Some(encoder.finish()));
                output.present();
            }
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::Resized(size) => {
                        if size.width != 0 && size.height != 0 {
                            // Recreate the swap chain with the new size
                            context.config.width = size.width;
                            context.config.height = size.height;
                            context.surface.configure(&context.device, &context.config);
                        }
                    }
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::MouseInput { state, button, .. } => {
                        let mask = 1 << mouse_button_index(button);
                        match state {
                            ElementState::Pressed => {
                                mouse_button_pressed |= mask;
                                mouse_button_press_since_last_frame |= mask;
                            }
                            ElementState::Released => {
                                if button == MouseButton::Left {
                                    translate_x += drag_start_x - drag_end_x;
                                    translate_y += drag_start_y - drag_end_y;
                                }
                                mouse_button_pressed &= !mask
                            }
                        }
                        if button == MouseButton::Left {
                            drag_start_x = cursor_x;
                            drag_start_y = cursor_y;
                            drag_end_x = cursor_x;
                            drag_end_y = cursor_y;
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        let scalar = match delta {
                            MouseScrollDelta::LineDelta(_, y) => {
                                if y < 0.0 {
                                    1.0 - 0.1 * y
                                } else {
                                    1.0 / (1.0 + 0.1 * y)
                                }
                            }
                            MouseScrollDelta::PixelDelta(p) => {
                                if p.y < 0.0 {
                                    1.0 + 0.1 * (1.0 - p.y as f32).ln()
                                } else {
                                    1.0 / (1.0 + 0.1 * (1.0 + p.y as f32).ln())
                                }
                            }
                        };
                        zoom *= scalar;
                        if options.shader == RustGPUShader::Mandelbrot
                            || options.shader == RustGPUShader::SierpinskiTriangle
                        {
                            translate_x *= 1.0 / scalar;
                            translate_y *= 1.0 / scalar;
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        cursor_x = position.x as f32;
                        cursor_y = position.y as f32;
                        if (mouse_button_pressed & (1 << mouse_button_index(MouseButton::Left)))
                            != 0
                        {
                            drag_end_x = cursor_x;
                            drag_end_y = cursor_y;
                        }
                    }
                    _ => {}
                }
            }
            Event::UserEvent(new_module) => {
                *render_pipeline = create_pipeline(
                    &options,
                    &context.device,
                    &pipeline_layout,
                    context.config.format,
                    new_module,
                );
                window.request_redraw();
                *control_flow = ControlFlow::Poll;
            }
            _ => {}
        }
    });
}

#[allow(clippy::match_wild_err_arm)]
pub fn start(options: &Options) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init().expect("could not initialize logger");
        } else {
            env_logger::init();
        }
    }

    let app = crate::app::App::new();

    // Build the shader before we pop open a window, since it might take a while.
    let initial_shader = maybe_watch(
        options,
        #[cfg(not(target_arch = "wasm32"))]
        {
            let proxy = app.event_loop.create_proxy();
            Some(Box::new(move |res| match proxy.send_event(res) {
                Ok(it) => it,
                // ShaderModuleDescriptor is not `Debug`, so can't use unwrap/expect
                Err(_err) => panic!("Event loop dead"),
            }))
        },
    );

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            use winit::platform::web::WindowExtWebSys;
            // On wasm, append the canvas to the document body
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.body())
                .and_then(|body| {
                    body.append_child(&web_sys::Element::from(window.canvas()))
                        .ok()
                })
                .expect("couldn't append canvas to document body");
            wasm_bindgen_futures::spawn_local(run(
                options.clone(),
                window,
                initial_shader,
            ));
        } else {
            futures::executor::block_on(run(
                options.clone(),
                app,
                initial_shader,
            ));
        }
    }
}
