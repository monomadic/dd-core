use conrod;
use conrod::glium;

use vst2::plugin::HostCallback;

use app::ui::*;
use app::event::*;
use app::config::*;

#[derive(Debug)]
pub enum ConrodWindowError {
    GetWindowFail,
    GetInnerSizeFail,
    LoadRendererFail,
}

pub struct ConrodWindow {
    pub ui: conrod::Ui,
    pub display: glium::Display,
    pub image_map: conrod::image::Map<glium::texture::Texture2d>,
    pub ids: Ids,
    pub renderer: conrod::backend::glium::Renderer,
}

pub fn run_conrod() {
    info!("in thread!");
}

impl ConrodWindow {
    pub fn new(window: glium::Display) -> Result<Self, ConrodWindowError> {
        let (width, height) = try!(window.get_window()
            .ok_or(ConrodWindowError::GetWindowFail)
            .and_then({|window|
                window.get_inner_size().ok_or(ConrodWindowError::GetInnerSizeFail)
            }));

        use std;

        let mut ui = conrod::UiBuilder::new([width as f64, height as f64]).build();

        let renderer = match conrod::backend::glium::Renderer::new(&window) {
            Ok(r) => r,
            Err(e) => { return Err(ConrodWindowError::LoadRendererFail) },
        };

        let image_map = conrod::image::Map::new();
        let ids = Ids::new(ui.widget_id_generator());

        let cw = ConrodWindow{ui: ui, display: window, image_map: image_map, renderer: renderer, ids: ids};

        // std::thread::spawn(move || run_conrod(rust_logo, event_rx, render_tx, window_proxy));
        std::thread::spawn(move || run_conrod());

        info!("back from thread");

        // std::thread::spawn(move || draw(cw, default::Default()));

        // cw.set_timer(window.get_window().unwrap());
        
        Ok(cw)
    }

    //conrod::backend::glium::backend::glutin_backend::WinRef

    // pub fn set_timer(&mut self, window: glium::glutin_backend::WinRef) {

    // }

    pub fn draw(&mut self, app: &mut AppConfig) {
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
                    // event_tx.send(event).unwrap();
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
            
            set_widgets(self.ui.set_widgets(), &mut self.ids, app);

            let mut target = self.display.draw();

            // Render the `Ui` and then display it on the screen.
            if let Some(primitives) = self.ui.draw_if_changed() {
                self.renderer.fill(&self.display, primitives, &self.image_map);
                self.renderer.draw(&self.display, &mut target, &self.image_map).unwrap();
            }

            target.finish().unwrap();
            break 'main;
        }
    }
}
