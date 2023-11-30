use crate::shader::CompiledShaderModules;

use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{
        ElementState, Event, KeyboardInput, MouseButton, MouseScrollDelta, VirtualKeyCode,
        WindowEvent,
    },
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::{self, WindowBuilder},
};

pub enum WindowEvents<'a> {
    Resized {
        size: PhysicalSize<u32>,
    },
    MouseInput {
        state: ElementState,
        button: MouseButton,
    },
    MouseWheel {
        delta: MouseScrollDelta,
    },
    MouseMoved {
        position: PhysicalPosition<f64>,
    },
    Draw {
        control_flow: &'a mut ControlFlow,
    },
    UserEvent(CompiledShaderModules),
}

pub struct Window {
    pub event_loop: EventLoop<CompiledShaderModules>,
    pub window: window::Window,
}

impl Window {
    pub fn new() -> Self {
        let mut event_loop_builder = EventLoopBuilder::with_user_event();
        let event_loop = event_loop_builder.build();
        let window = WindowBuilder::new()
            .with_title("Rust GPU Shaders")
            .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
            .build(&event_loop)
            .unwrap();

        Self { event_loop, window }
    }

    pub fn run(self, mut callback: impl 'static + FnMut(WindowEvents) -> ()) {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                    // TODO: only redraw if needed?
                    self.window.request_redraw();

                    callback(WindowEvents::Draw { control_flow });
                }
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                Event::WindowEvent { event, window_id } if window_id == self.window.id() => {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => callback(WindowEvents::Resized {
                            size: physical_size,
                        }),
                        WindowEvent::MouseInput { state, button, .. } => {
                            callback(WindowEvents::MouseInput { state, button })
                        }
                        WindowEvent::MouseWheel { delta, .. } => {
                            callback(WindowEvents::MouseWheel { delta })
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            callback(WindowEvents::MouseMoved { position })
                        }
                        _ => {}
                    }
                }
                Event::UserEvent(new_module) => {
                    callback(WindowEvents::UserEvent(new_module));
                    self.window.request_redraw();
                }
                _ => {}
            }
        });
    }
}
