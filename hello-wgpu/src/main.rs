mod surface;

use std::sync::Arc;
use surface::State;
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowId},
};

struct App {
    runtime: tokio::runtime::Runtime,
    window: Option<Arc<Window>>,
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());
        let state = self.runtime.block_on(State::new(window.clone()));
        self.state = Some(state);
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(window) = self.window.as_ref() {
            if window_id != window.id() {
                return;
            }
        }
        if let Some(state) = self.state.as_mut() {
            if !state.input(&event) {
                match event {
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: Key::Named(NamedKey::Escape),
                                ..
                            },
                        ..
                    }
                    | WindowEvent::CloseRequested => event_loop.exit(),
                    WindowEvent::Resized(physical_size) => {
                        if let Some(state) = self.state.as_mut() {
                            state.resize(physical_size);
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        if let Some(state) = self.state.as_mut() {
                            state.update();
                            match state.render() {
                                Ok(_) => {}
                                // 当展示平面的上下文丢失，就需重新配置
                                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                                // 所有其他错误（过期、超时等）应在下一帧解决
                                Err(e) => eprintln!("{:?}", e),
                            }
                            // 除非我们手动请求，RedrawRequested 将只会触发一次。
                            state.request_redraw();
                        }
                        
                    }
                    _ => {}
                }
            }
        }
        
    }
}

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App {
        runtime,
        window: None,
        state: None,
    };
    let _ = event_loop.run_app(&mut app);
}