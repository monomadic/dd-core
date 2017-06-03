use libc::c_void;

use winit;

use conrod;
use conrod::glium;
use conrod::glium::GliumCreationError;
use conrod::backend::glium::glium::glutin::{WindowBuilder, Window, Robustness};
use conrod::backend::glium::glium::{DisplayBuild, Surface};

use find_folder;
use image;
use std;

use cocoa_helpers::*;

pub use app::App;

#[cfg(feature = "exception")]
extern crate objc_exception;

#[derive(Debug)]
pub enum WindowError {
    GliumError
}

pub struct WindowContext {
    pub app: App,
}

use libc;

use cocoa::base::{selector, nil, NO, YES, class};
#[macro_use] use objc::runtime;
#[allow(non_camel_case_types)]
pub type id = *mut runtime::Object;
use cocoa::foundation::{NSRect};
use cocoa::appkit::{NSWindow, NSView,
                    NSTitledWindowMask, NSBackingStoreBuffered,
                    NSRunningApplication, NSApplicationActivateIgnoringOtherApps,
                    NSBorderlessWindowMask, NSWindowAbove};

use log_panics;

impl WindowContext {

    pub fn new(handle: *mut libc::c_void) -> Result<WindowContext, WindowError> {
        info!("Building window with conrod.");
        info!("id: {:?}", handle);
        log_panics::init();

        // unsafe {
        //     // set properties on the nswindow
        //     let ns_window: id = get_nswindow_for_nsview(handle as id);
        //     retain(ns_window);
        //     set_window_properties(ns_window);
        // }

        let child_view: id = unsafe { add_child_view(handle as id) };
        info!("added child_view. id: {:?}", child_view);

        let wb = winit::WindowBuilder::new()
            // .with_min_dimensions(500, 300)
            // .with_max_dimensions(500, 300)
            .with_visibility(true)
            .with_transparency(false)
            .with_dimensions(500, 300)
            .with_parent(child_view as *mut libc::c_void);

        match WindowBuilder::from_winit_builder(wb)
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
                    // return Ok(WindowContext{ view: child_view as id });

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
        // self.app.event_loop();
    }
}
