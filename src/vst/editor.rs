use vst2::editor::Editor;
use vst::VSTPlugin;
use std::os::raw::c_void;

use gui::Window;

// use PluginConfig;
use BasePlugin;
use Graphics;

impl<P:BasePlugin + Graphics> Editor for VSTPlugin<P> {
    fn size(&self) -> (i32, i32) {
        self.config.size
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn open(&mut self, window: *mut c_void) {
        info!("VST plugin called open()");

        match Window::new(window as *mut c_void, &mut self.plugin) {
            Ok(w) => {
                info!("Window created ok in editor.");
                self.window = Some(w);
            },
            Err(why) => { error!("{:?}", why) }
        }
    }

    fn close(&mut self) {
        info!("VST plugin called close()");
        self.window = None;
    }

    fn is_open(&mut self) -> bool { self.window.is_some() }

    // fn set_knob_mode(&mut self, mode: KnobMode) -> bool { info!("VST plugin called KnobMode()"); false }
    // fn key_up(&mut self, keycode: KeyCode) -> bool { info!("VST plugin called key_up()"); false }
    // fn key_down(&mut self, keycode: KeyCode) -> bool { info!("VST plugin called key_down()"); false }

    fn idle(&mut self) {
		if let Some(ref mut window) = self.window {
            use std;
            let last_update = std::time::Instant::now();

            // We don't want to loop any faster than 60 FPS, so wait until it has been at least
            // 16ms since the last yield.
            let sixteen_ms = std::time::Duration::from_millis(116);
            let now = std::time::Instant::now();
            let duration_since_last_update = now.duration_since(last_update);
            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }

            window.draw(&mut self.config, &mut self.plugin);
		}
    }
}
