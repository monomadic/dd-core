use libc::c_void;

use winit;

use conrod;
use conrod::glium;
use conrod::glium::GliumCreationError;
use conrod::backend::glium::glium::glutin::{WindowBuilder, Window};
use conrod::backend::glium::glium::{DisplayBuild, Surface};

use find_folder;
use image;
use std;

pub use app::App;

#[derive(Debug)]
pub enum WindowError {
    GliumError
}

pub struct WindowContext {
    pub app: App,
}

impl WindowContext {

    pub fn new(handle: *mut c_void) -> Result<WindowContext, WindowError> {
        info!("Building window with conrod.");

        let wb = winit::WindowBuilder::new().with_parent(handle);

        match WindowBuilder::from_winit_builder(wb)
            .with_vsync()
            .with_multisampling(8)
            .with_dimensions(500, 300)
            .build_glium() {
                Err(why) => Err(WindowError::GliumError),
                Ok(display) => {
                    info!("Window spawned OK with conrod.");

                    let mut app = App::new(display);

                    match app {
                        Ok(a) => Ok(WindowContext{ app: a }),
                        Err(why) => { error!("{:?}", why); panic!("{:?}", why) }
                    }
                }
            }
    }

    pub fn open(&mut self) {
        info!("showing window");
        // self.window.show()
    }

    pub fn close(&mut self) {
        info!("hiding window");
        // self.window.hide()
    }

    pub fn update(&mut self) {
        self.app.draw();
    }
}
