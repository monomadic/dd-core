use vst2::editor::Editor;

// impl Editor for PluginWindow {
//     fn size(&self) -> (i32, i32) { (500, 300) }
//     fn position(&self) -> (i32, i32) { (0, 0) }
//     fn is_open(&mut self) -> bool { self.window.is_some() }
//     fn close(&mut self) { self.window = None; }

//     fn open(&mut self, window: *mut c_void) {
//         // match Window::new(window as *mut c_void, &mut self.plugin) {
//         //     Ok(w) => {
//         //         self.window = Some(w);
//         //     },
//         //     Err(why) => { panic!("{:?}", why) }
//         // }
//     }

//     fn idle(&mut self) {
//         // if let Some(ref mut window) = self.window {
//         //     window.draw(&mut self.config, &mut self.plugin);
//         // }
//     }
// }
