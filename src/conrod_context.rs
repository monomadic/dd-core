use conrod;
use winit;

use libc::c_void;

use conrod::backend::glium::glium::glutin::{CreationError, WindowBuilder, Window};

pub struct WindowContext {
    pub window: Window,
}

impl WindowContext {

    pub fn new(handle: *mut c_void) -> Result<WindowContext, CreationError> {
        info!("Building window with conrod.");

        let wb = winit::WindowBuilder::new().with_parent(handle);

        match WindowBuilder::from_winit_builder(wb)
            .build() {
                Err(why) => Err(why),
                Ok(window) => {
                    info!("Window spawned OK with conrod.");

                    let (width, height) = window.get_inner_size().unwrap();
                    let mut ui = conrod::UiBuilder::new([width as f64, height as f64]).build();
                    let mut renderer = conrod::backend::glium::Renderer::new(&window).unwrap();

                    Ok(WindowContext{ window: window })
                }
            }
    }

    pub fn open(&mut self) {
        info!("showing window");
        self.window.show()
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
