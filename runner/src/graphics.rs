use crate::{maybe_watch, state, window::Window, CompiledShaderModules, Options};

use winit::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

async fn run(options: Options, window: Window, compiled_shader_modules: CompiledShaderModules) {
    let mut app = state::State::new(&window, compiled_shader_modules, options).await;

    window.event_loop.run(move |event, _, control_flow| {
        let window = &window.window;

        *control_flow = ControlFlow::Wait;
        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // TODO: only redraw if needed
                window.request_redraw();

                match app.update_and_render() {
                    Ok(()) => (),
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        *control_flow = ControlFlow::Exit;
                        ()
                    }
                    _ => (),
                }
            }
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::Resized(size) => app.resize(size),
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    app.mouse_input(state, button);
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    app.mouse_scroll(delta);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    app.mouse_move(position);
                }
                _ => {}
            },
            Event::UserEvent(new_module) => {
                app.new_module(new_module);
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

    let window = Window::new();

    // Build the shader before we pop open a window, since it might take a while.
    let initial_shader = maybe_watch(
        options,
        #[cfg(not(target_arch = "wasm32"))]
        {
            let proxy = window.event_loop.create_proxy();
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
                window,
                initial_shader,
            ));
        }
    }
}
