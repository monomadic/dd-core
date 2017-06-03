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
    pub view: id,
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

/// @property NSRect frame;
// pub unsafe fn host_window_frame(view: id) -> NSRect {
//     NSView::frame(view)
// }

use log_panics;

// pub unsafe fn add_child_view(view: id) -> id {
//     let child_nsview = NSView::alloc(nil);
//     let child_view = child_nsview.initWithFrame_(host_window_frame(view));

//     add_subview(view, child_view);
//     child_view
// }

// /// - (void)addSubview:(UIView *)view;
// pub unsafe fn add_subview(parent_id: id, child_id: id) {
//     msg_send![parent_id, addSubview:child_id];
// }

// /// - (void)addChildWindow:(NSWindow *)childWin 
// ///             ordered:(NSWindowOrderingMode)place;
// pub unsafe fn add_child_window(parent_id: id, child_id: id) {
//     msg_send![parent_id, addChildWindow:child_id ordered: NSWindowAbove];
// }

impl WindowContext {

    pub fn new(handle: *mut libc::c_void) -> Result<WindowContext, WindowError> {
        info!("Building window with conrod.");
        info!("id: {:?}", handle);
        log_panics::init();

        let child_view: id = unsafe { add_child_view(handle as id) };
        info!("added child_view. id: {:?}", child_view);

        let wb = winit::WindowBuilder::new()
            .with_min_dimensions(500, 300)
            .with_max_dimensions(500, 300)
            .with_dimensions(500, 300)
            .with_parent(child_view as *mut libc::c_void);

        match WindowBuilder::from_winit_builder(wb)
            .with_vsync()
            .with_multisampling(8)
            .with_dimensions(500, 300)
            .build_glium() {
                Err(why) => Err(WindowError::GliumError),
                Ok(display) => {
                    info!("Window spawned OK with conrod.");
                    // return Ok(WindowContext{ view: child_view as id });

                    let mut app = App::new(display);

                    match app {
                        Ok(a) => Ok(WindowContext{ app: a, view: handle as id }),
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
