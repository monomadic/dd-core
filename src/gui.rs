use vst2::editor::{Editor, KnobMode, KeyCode};

use libc;
use conrod_context::{WindowContext};

pub struct Interface {
    is_open: bool,
    window: Option<WindowContext>,
}

impl Interface {
    pub fn new() -> Interface {
        Interface{ is_open: false, window: None }
    }
}

impl Editor for Interface {
    fn size(&self) -> (i32, i32) {
        (500, 300)
    }

    fn position(&self) -> (i32, i32) {
        (500, 300)
    }

    fn open(&mut self, window: *mut libc::c_void) {
        info!("VST plugin called open()");
        match self.window {
            Some(ref mut w) => { error!("VST called open but window already exists"); },
            None => {
                match WindowContext::new(window) {
                    Ok(wc) => { info!("ok!"); self.window = Some(wc) },
                    Err(why) => { error!("{:?}", why) }
                }
            }
        }
    }

    fn close(&mut self) {
        info!("VST plugin called close()");
        self.is_open = false;
        // self.window = None;
    }

    fn is_open(&mut self) -> bool {
        self.is_open
    }

    /// Set the knob mode for this editor (if supported by host).
    ///
    /// Return true if the knob mode was set.
    fn set_knob_mode(&mut self, mode: KnobMode) -> bool { info!("VST plugin called KnobMode()"); false }

    /// Recieve key up event. Return true if the key was used.
    fn key_up(&mut self, keycode: KeyCode) -> bool { info!("VST plugin called key_up()"); false }

    /// Receive key down event. Return true if the key was used.
    fn key_down(&mut self, keycode: KeyCode) -> bool { info!("VST plugin called key_down()"); false }
    // fn idle(&mut self) { info!("VST plugin called idle()"); }

    fn idle(&mut self) {
        match self.window {
            Some(ref mut w) => { w.update(); },
            None => (),
        }
    }
}
