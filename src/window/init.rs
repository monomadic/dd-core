use winit;

use conrod::backend::glium::glium::glutin::WindowBuilder;
use conrod::backend::glium::glium::DisplayBuild;

use std::os::raw::c_void;

use window::window::ConrodWindow;

#[derive(Debug)]
pub enum WindowError {
    GliumError
}

pub fn create_window(handle: *mut c_void) -> Result<ConrodWindow, WindowError> {
    // info!("Building window with conrod.");

    // log_panics::init();

    let wb = winit::WindowBuilder::new()
        .with_visibility(true)
        .with_transparency(false)
        .with_dimensions(500, 300)
        .with_parent(handle);

    match WindowBuilder::from_winit_builder(wb)
        .with_decorations(false)
        // .with_vsync()
        // .with_multisampling(8)
        // .with_dimensions(500, 300)
        // .with_visibility(true)
        // .with_transparency(false)
        // .with_gl_robustness(Robustness::RobustLoseContextOnReset)
        .build_glium() {
            Err(why) => { error!("Problem with build_glium(): {:?}", why); Err(WindowError::GliumError) },
            Ok(display) => {
                info!("Window spawned OK with conrod.");

                let app = ConrodWindow::new(display);

                match app {
                    Ok(a) => Ok(a),
                    Err(why) => { error!("{:?}", why); panic!("{:?}", why) }
                }
            }
        }
}