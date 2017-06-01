use debugging;

use conrod;
use conrod::glium;
use conrod::glium::GliumCreationError;
use conrod::backend::glium::glium::glutin::{WindowBuilder, Window};
use conrod::backend::glium::glium::{DisplayBuild, Surface};

// pub mod ui;

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

        // Create UI.
        let mut ui = conrod::UiBuilder::new([width as f64, height as f64]).build();

        let renderer = conrod::backend::glium::Renderer::new(&window).unwrap();

        let image_map = conrod::image::Map::new();
        let ids = Ids::new(ui.widget_id_generator());
        
        Ok(App{ui: ui, display: window, image_map: image_map, renderer: renderer, ids: ids})
    }
}

widget_ids! {
    pub struct Ids {
        // Root IDs
        root,
        body,
    }
}
