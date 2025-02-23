use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    event::{Event, WindowEvent},
};

fn main() {
    // 1) Create an event loop: this handles OS events (like window close, resize, etc.).
    let event_loop = EventLoop::new();

    // 2) Build the window itself.
    let window = WindowBuilder::new()
        .with_title("Hello, winit window!")
        .build(&event_loop)
        .expect("Failed to create window");

    // 3) Run your event loop. This never returns unless you explicitly exit.
    event_loop.run(move |event, _, control_flow| {
        // Wait for the next event
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                // If the close button is pressed, we exit.
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }

                _ => {}
            },
            _ => {}
        }
    });
}
