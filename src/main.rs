use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    platform::wayland::WindowExtWayland,
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("bello").build(&event_loop).unwrap();
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();
        let surface = window.wayland_surface().unwrap();
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                panic!("abc");
            },
            _ => (),
        }
    });
}
