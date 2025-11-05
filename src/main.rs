use chip8::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes},
};
#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, elwt: &ActiveEventLoop) {
        let scale = 15u32;
        let attrs: WindowAttributes = Window::default_attributes()
            .with_title("CHIP-8")
            .with_inner_size(PhysicalSize::new(
                DISPLAY_HEIGHT * scale,
                DISPLAY_WIDTH * scale,
            ));
        let window = elwt.create_window(attrs).unwrap();
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        elwt: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => elwt.exit(),
            WindowEvent::Resized(_) => {
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                // Do your rendering here (wgpu/softbuffer/OpenGL/etc.)
                if let Some(w) = &self.window {
                    w.request_redraw();
                } // continuous redraw (optional)
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?; // build the event loop
    event_loop.set_control_flow(ControlFlow::Wait); // or ControlFlow::Poll for games
    let mut app = App::default();
    let _ = event_loop.run_app(&mut app); // enter the loop
    Ok(())
}
