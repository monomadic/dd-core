use conrod;
use winit;

use libc::c_void;

use conrod::glium::GliumCreationError;
use conrod::backend::glium::glium::glutin::{WindowBuilder, Window};
use conrod::backend::glium::glium::{DisplayBuild, Surface};

#[derive(Debug)]
pub enum WindowError {
    GliumError
}

pub struct WindowContext {
    pub display: conrod::glium::Display,
}

impl WindowContext {

    pub fn new(handle: *mut c_void) -> Result<WindowContext, WindowError> {
        info!("Building window with conrod.");

        let wb = winit::WindowBuilder::new().with_parent(handle);

        match WindowBuilder::from_winit_builder(wb)
            .build_glium() {
                Err(why) => Err(WindowError::GliumError),
                Ok(window) => {
                    info!("Window spawned OK with conrod.");

                    // info!("Glutin facade version: {:?}", window.get_version());

                    // let (width, height) = window.get_inner_size().unwrap();
                    // let mut ui = conrod::UiBuilder::new([width as f64, height as f64]).build();
                    // let mut renderer = conrod::backend::glium::Renderer::new(&window).unwrap();

                    Ok(WindowContext{ display: window })
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
        // info!("update() called.");
        // loop {
        //     self.events_loop.poll_events(|event| {
        //         info!("-- event {:?}", event);
        //     });
        // }
        
        // self.events_loop.run_forever(|event| {
        //     info!("-- event {:?}", event);
        //     match event {
        //         winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } => self.events_loop.interrupt(),
        //         _ => ()
        //     }
        // });
    }
}
