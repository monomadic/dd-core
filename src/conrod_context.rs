use libc::c_void;

use winit;

use conrod;
use conrod::glium;
use conrod::glium::GliumCreationError;
use conrod::backend::glium::glium::glutin::{WindowBuilder, Window};
use conrod::backend::glium::glium::{DisplayBuild, Surface};

use find_folder;
use image;
use std;

pub use app::App;

#[derive(Debug)]
pub enum WindowError {
    GliumError
}

pub struct WindowContext {
    pub app: App,
    // pub display: conrod::glium::Display,
    // pub ui: conrod::Ui,
    // pub renderer: conrod::backend::glium::Renderer,
    // pub image_map: conrod::image::Map<conrod::glium::texture::Texture2d>,
}

impl WindowContext {

    pub fn new(handle: *mut c_void) -> Result<WindowContext, WindowError> {
        info!("Building window with conrod.");

        let wb = winit::WindowBuilder::new().with_parent(handle);

        match WindowBuilder::from_winit_builder(wb)
            .with_vsync()
            .with_multisampling(8)
            .with_dimensions(500, 300)
            .build_glium() {
                Err(why) => Err(WindowError::GliumError),
                Ok(display) => {
                    info!("Window spawned OK with conrod.");

                    let mut app = App::new(display);

                    // let renderer = conrod::backend::glium::Renderer::new(&window).unwrap();

                    // let (width, height) = window.get_window()
                    //     .unwrap()
                    //     .get_inner_size()
                    //     .unwrap();

                    // // Create UI.
                    // let ui = conrod::UiBuilder::new([width as f64, height as f64]).build();

                    // widget::Canvas::new().set(ids.canvas, ui);

                    // widget::Rectangle::fill([80.0, 80.0]).right(SHAPE_GAP).set(ids.rectangle_fill, ui);

                    
                    // // Load the Rust logo from our assets folder to use as an example image.
                    // fn load_rust_logo(display: &conrod::glium::Display) -> conrod::glium::texture::Texture2d {
                    //     info!("loading image");
                    //     let cwd = ::std::env::current_dir().unwrap();
                    //     info!("in: {:?}", cwd);
                    //     let a = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets");

                    //     match a {
                    //         Ok(a) => info!("Found!"),
                    //         Err(why) => info!("Error finding folder: {:?}", why),
                    //     }

                    //     let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
                    //     let path = assets.join("images/rust.png");
                    //     info!("path {:?}", path);
                    //     let rgba_image = image::open(&std::path::Path::new(&path)).unwrap().to_rgba();
                    //     let image_dimensions = rgba_image.dimensions();
                    //     let raw_image = conrod::glium::texture::RawImage2d::from_raw_rgba_reversed(rgba_image.into_raw(), image_dimensions);
                    //     let texture = conrod::glium::texture::Texture2d::new(display, raw_image).unwrap();
                    //     texture
                    // }

                    // let mut image_map = conrod::image::Map::new();
                    // let rust_logo = image_map.insert(load_rust_logo(&window));
                    

                    // Ok(WindowContext{ display: window, ui: ui, image_map: image_map, renderer: renderer })
                    match app {
                        Ok(a) => Ok(WindowContext{ app: a }),
                        Err(why) => { error!("{:?}", why); panic!("{:?}", why) }
                    }
                }
            }
    }

    pub fn open(&mut self) {
        info!("showing window");
        // self.window.show()
    }

    pub fn close(&mut self) {
        info!("hiding window");
        // self.window.hide()
    }

    pub fn update(&mut self) {
        self.app.draw();
        // Draw the `Ui`.
        // if let Some(primitives) = self.ui.draw_if_changed() {
        //     self.renderer.fill(&self.display, primitives, &self.image_map);
        //     let mut target = self.display.draw();
        //     target.clear_color(0.0, 0.0, 0.0, 1.0);
        //     self.renderer.draw(&self.display, &mut target, &self.image_map).unwrap();
        //     target.finish().unwrap();
        // }

        // info!("update();");

        // self.app.events.update();
        // loop {
        //     self.events_loop.poll_events(|event| {
        //         info!("-- event {:?}", event);
        //     });
        // }
        
        // self.events_loop.run_forever(|event| {
        //     info!("-- event {:?}", event);
        //     match event {
        //         winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } => self.events_loop.interrupt(),
        //         _ => ()
        //     }
        // });
    }
}
