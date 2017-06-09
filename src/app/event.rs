use conrod::glium;
use winit;

pub fn ui_event(event: winit::Event) {
    match event {
        // Break from the loop upon `Escape`.
        // glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
        glium::glutin::Event::Closed => { return; },
        // _ => { info!(" -- another {:?}", event)},
        _ => (),
    }
}
