use vst2::editor::{Editor, KnobMode, KeyCode};

use libc;
use std::os::raw::c_void;

use window::init::create_window;
use vst::plugin::VSTPlugin;

impl Editor for VSTPlugin {
    fn size(&self) -> (i32, i32) {
        (500, 300)
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn open(&mut self, window: *mut libc::c_void) {
        info!("VST plugin called open()");
        // info!("id: {}", window as i32);

        match create_window(window as *mut c_void) {
            Ok(wc) => {
                info!("Window created ok in editor.");
                self.window = Some(wc);
            },
            Err(why) => { error!("{:?}", why) }
        }
    }

    fn close(&mut self) {
        info!("VST plugin called close()");
        // self.window = None;
    }

    fn is_open(&mut self) -> bool {
        self.window.is_some()
    }

    // / Set the knob mode for this editor (if supported by host).
    // /
    // / Return true if the knob mode was set.
    // fn set_knob_mode(&mut self, mode: KnobMode) -> bool { info!("VST plugin called KnobMode()"); false }

    // / Recieve key up event. Return true if the key was used.
    // fn key_up(&mut self, keycode: KeyCode) -> bool { info!("VST plugin called key_up()"); false }

    // / Receive key down event. Return true if the key was used.
    // fn key_down(&mut self, keycode: KeyCode) -> bool { info!("VST plugin called key_down()"); false }
    // fn idle(&mut self) { info!("VST plugin called idle()"); }

    fn idle(&mut self) {
		if let Some(ref mut window) = self.window {
			window.draw(&mut self.host);
		}
    }
}
