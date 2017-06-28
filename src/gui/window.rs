use conrod;
use conrod::glium;
use conrod::backend::glium::glium::glutin::WindowBuilder;
use conrod::backend::glium::glium::DisplayBuild;
use conrod::widget::Id;
use vst2::plugin::HostCallback;
use std::os::raw::c_void;
use winit;

use gui::GUIError;
use Graphics;
use PluginConfig;

pub struct Window {
    pub ui: conrod::Ui,
    pub display: glium::Display,
    pub image_map: conrod::image::Map<glium::texture::Texture2d>,
    pub ids: Vec<conrod::widget::Id>,
    pub renderer: conrod::backend::glium::Renderer,
}

pub fn ui_event(event: conrod::event::Input) {
    match event {
        // glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
        // glium::glutin::Event::Closed => { return; },
        // _ => { info!(" -- glium event {:?}", event)},
        _ => (),
    }
}

impl Window {
    pub fn new<P:Graphics>(handle: *mut c_void, plugin: &mut P) -> Result<Window, GUIError> {

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
                Err(why) => Err(GUIError::CreationError(format!(".build_glium() failed: {:?}", why))),
                Ok(display) => {
                    info!("Window spawned OK with conrod.");

                    let app = Window::setup_display(display, plugin);

                    match app {
                        Ok(a) => Ok(a),
                        Err(why) => Err(GUIError::CreationError(format!(".setup_display() failed: {:?}", why)))
                    }
                }
            }
    }

    pub fn setup_display<P:Graphics>(window: glium::Display, plugin: &mut P) -> Result<Self, GUIError> {
        let (width, height) = try!(window.get_window()
            .ok_or(GUIError::CreationError("could not .get_window()".to_string()))
            .and_then({|window|
                window.get_inner_size().ok_or(GUIError::CreationError("could not get_inner_size() on window.".to_string()))
            }));

        use std;

        let mut ui = conrod::UiBuilder::new([width as f64, height as f64]).build();

        let renderer = match conrod::backend::glium::Renderer::new(&window) {
            Ok(r) => r,
            Err(why) => { return Err(GUIError::CreationError(format!(".conrod::backend::glium::Renderer::new() failed: {:?}", why))) },
        };

        let image_map = conrod::image::Map::new();

        let ids = plugin.setup_ids(&mut ui.widget_id_generator());

        let cw = Window{
            ui: ui,
            display: window,
            image_map: image_map,
            renderer: renderer,
            ids: ids,
        };
        
        Ok(cw)
    }

    pub fn draw<P>(&mut self, config: &mut PluginConfig, plugin: &mut P) where P: Graphics {
        use std;

        let mut last_update = std::time::Instant::now();
        'main: loop {

            // We don't want to loop any faster than 60 FPS, so wait until it has been at least
            // 16ms since the last yield.
            let sixteen_ms = std::time::Duration::from_millis(16);
            let now = std::time::Instant::now();
            let duration_since_last_update = now.duration_since(last_update);
            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }

            let mut events: Vec<_> = self.display.poll_events().collect();
            // if events.is_empty() {
            //     events.extend(self.display.wait_events().next());
            // }

            // Send any relevant events to the conrod thread.
            for event in events {

                // Use the `winit` backend feature to convert the winit event to a conrod one.
                if let Some(event) = conrod::backend::winit::convert(event.clone(), &self.display) {
                    self.ui.handle_event(event.clone());
                    ui_event(event);
                }

                match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                    glium::glutin::Event::Closed => {
                        info!("closing event loop.");
                        break 'main
                    },
                    _ => {},
                }

                // if let Some(e) = conrod::backend::winit::convert(event.clone(), &self.display) {
                //     ui_event(e.clone());
                //     self.ui.handle_event(e);
                // }
            }
            
            // set_widgets(self.ui.set_widgets(), &mut self.ids, plugin);
            plugin.do_layout(self.ui.set_widgets(), config, &mut self.ids);

            let mut target = self.display.draw();

            // Render the `Ui` and then display it on the screen.
            if let Some(primitives) = self.ui.draw_if_changed() {
                self.renderer.fill(&self.display, primitives, &self.image_map);
                self.renderer.draw(&self.display, &mut target, &self.image_map).expect("renderer to draw");
            }

            target.finish().expect("target to finish()");
            break 'main;
        }
    }
}
