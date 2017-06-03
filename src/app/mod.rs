use conrod;
use conrod::glium;
use conrod::glium::GliumCreationError;
use conrod::backend::glium::glium::glutin::{WindowBuilder, Window};
use conrod::backend::glium::glium::{DisplayBuild, Surface};

use conrod::{color, Colorable, Labelable, Positionable, Sizeable, Widget, Borderable};
use conrod::widget::*;

extern crate rand;
use self::rand::Rng;

pub struct App {
    pub ui: conrod::Ui,
    pub display: glium::Display,
    pub image_map: conrod::image::Map<glium::texture::Texture2d>,
    pub ids: Ids,
    pub renderer: conrod::backend::glium::Renderer,
}

#[derive(Debug)]
pub enum AppError {
    GetWindowFail,
    GetInnerSizeFail,
    LoadRendererFail,
}

impl App {
    pub fn new(window: glium::Display) -> Result<Self, AppError> {
        let (width, height) = try!(window.get_window()
            .ok_or(AppError::GetWindowFail)
            .and_then({|window|
                window.get_inner_size().ok_or(AppError::GetInnerSizeFail)
            }));

        let mut ui = conrod::UiBuilder::new([width as f64, height as f64]).build();

        let renderer = conrod::backend::glium::Renderer::new(&window).unwrap();

        let image_map = conrod::image::Map::new();
        let ids = Ids::new(ui.widget_id_generator());

        let body = Canvas::new()
            .color(color::CHARCOAL)
            .border(0.0);
        
        Ok(App{ui: ui, display: window, image_map: image_map, renderer: renderer, ids: ids})
    }

    pub fn draw(&mut self) {
        let mut target = self.display.draw();
        let mut rng = rand::thread_rng();

        let r = rng.gen::<f32>();
        let g = rng.gen::<f32>();
        let b = rng.gen::<f32>();

        target.clear_color(r, g, b, 1.0);
        // self.renderer.draw(&self.display, &mut target, &self.image_map).unwrap();
        target.finish().unwrap();

        // Collect all pending events.
        let mut events = Vec::new();
        events.extend(self.display.poll_events());

        for event in events {
            info!("-- event {:?}", event);
        }
    }
}

widget_ids! {
    pub struct Ids {
        // Root IDs
        root,
        body,
    }
}
