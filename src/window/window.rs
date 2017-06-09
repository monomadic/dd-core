use conrod;
use conrod::glium;

use vst2::plugin::HostCallback;

use app::ui::*;
use app::event::*;

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

impl ConrodWindow {
    pub fn new(window: glium::Display) -> Result<Self, ConrodWindowError> {
        let (width, height) = try!(window.get_window()
            .ok_or(ConrodWindowError::GetWindowFail)
            .and_then({|window|
                window.get_inner_size().ok_or(ConrodWindowError::GetInnerSizeFail)
            }));

        let mut ui = conrod::UiBuilder::new([width as f64, height as f64]).build();

        let renderer = match conrod::backend::glium::Renderer::new(&window) {
            Ok(r) => r,
            Err(e) => { return Err(ConrodWindowError::LoadRendererFail) },
        };

        let image_map = conrod::image::Map::new();
        let ids = Ids::new(ui.widget_id_generator());

        let cw = ConrodWindow{ui: ui, display: window, image_map: image_map, renderer: renderer, ids: ids};

        // std::thread::spawn(move || draw(cw, default::Default()));
        
        Ok(cw)
    }

    pub fn draw(&mut self, host: &mut HostCallback) {

        loop {
            let events: Vec<_> = self.display.poll_events().collect();

            // Send any relevant events to the conrod thread.
            for event in events {

                ui_event(event);

                // // Use the `winit` backend feature to convert the winit event to a conrod one.
                // if let Some(event) = conrod::backend::winit::convert(event.clone(), &self.display) {
                //     // event_tx.send(event).unwrap();
                //     // info!(" -- event:: {:?}", event);
                // }

                // match event {
                //     // Break from the loop upon `Escape`.
                //     // glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                //     glium::glutin::Event::Closed => { return; },
                //     // _ => { info!(" -- another {:?}", event)},
                //     _ => (),
                // }
            }

            set_widgets(self.ui.set_widgets(), &mut self.ids, host);

            let mut target = self.display.draw();

            // Render the `Ui` and then display it on the screen.
            if let Some(primitives) = self.ui.draw_if_changed() {
                self.renderer.fill(&self.display, primitives, &self.image_map);
                self.renderer.draw(&self.display, &mut target, &self.image_map).unwrap();
            }

            target.finish().unwrap();
        }
    }
}
