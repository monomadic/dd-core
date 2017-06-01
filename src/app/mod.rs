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

impl App {
    pub fn new(window: glium::Display) -> Self {
        let (width, height) = window.get_window()
            .uw("Failed to get window.")
            .get_inner_size()
            .uw("Failed to get window size.");

        // Create UI.
        let mut ui = conrod::UiBuilder::new([width as f64, height as f64]).build();

        let renderer = conrod::backend::glium::Renderer::new(&window)
            .uw("Failed to load conrod glium renderer.");

        let image_map = conrod::image::Map::new();
        let ids = Ids::new(ui.widget_id_generator());
        
        App{ui: ui, display: window, image_map: image_map, renderer: renderer}
    }
}

widget_ids! {
    pub struct Ids {
        // Root IDs
        root,
        body,
    }
}
