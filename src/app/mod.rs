use winit;

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

#[derive(Debug)]
pub enum WindowError {
    GliumError
}

use std::os::raw::c_void;

pub fn create_app(handle: *mut c_void) -> Result<App, WindowError> {
    // info!("Building window with conrod.");

    // log_panics::init();

    let wb = winit::WindowBuilder::new()
        .with_visibility(true)
        .with_transparency(false)
        .with_dimensions(500, 300)
        .with_parent(handle);

    match WindowBuilder::from_winit_builder(wb)
        // .with_vsync()
        // .with_multisampling(8)
        // .with_dimensions(500, 300)
        // .with_visibility(true)
        // .with_transparency(false)
        // .with_gl_robustness(Robustness::RobustLoseContextOnReset)
        .build_glium() {
            Err(why) => { error!("Problem with build_glium(): {:?}", why); Err(WindowError::GliumError) },
            Ok(display) => {
                info!("Window spawned OK with conrod.");

                let mut app = App::new(display);

                match app {
                    Ok(a) => Ok(a),
                    Err(why) => { error!("{:?}", why); panic!("{:?}", why) }
                }
            }
        }
}

impl App {
    pub fn new(window: glium::Display) -> Result<Self, AppError> {
        let (width, height) = try!(window.get_window()
            .ok_or(AppError::GetWindowFail)
            .and_then({|window|
                window.get_inner_size().ok_or(AppError::GetInnerSizeFail)
            }));
        
        info!("size : {}x{}", width, height);

        info!("framebuffer: {:?}", window.get_framebuffer_dimensions());

        let mut ui = conrod::UiBuilder::new([width as f64, height as f64]).build();

        let renderer = match conrod::backend::glium::Renderer::new(&window) {
            Ok(r) => r,
            Err(e) => { error!("Error creating Renderer: {:?}", e); return Err(AppError::LoadRendererFail) },
        };

        let image_map = conrod::image::Map::new();
        let ids = Ids::new(ui.widget_id_generator());
        
        Ok(App{ui: ui, display: window, image_map: image_map, renderer: renderer, ids: ids})
    }

    pub fn draw(&mut self) {
        let mut target = self.display.draw();

        target.clear_color(0.0, 0.0, 0.0, 1.0); // set display to black

        self.renderer.draw(&self.display, &mut target, &self.image_map).unwrap();

        set_widgets(self.ui.set_widgets(), &mut self.ids);

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = self.ui.draw_if_changed() {
            self.renderer.fill(&self.display, primitives, &self.image_map);
            self.renderer.draw(&self.display, &mut target, &self.image_map).unwrap();
        }

        target.finish().unwrap();


        // Collect all pending events.
        // let mut events = Vec::new();
        // events.extend(self.display.poll_events());

        // for event in events {
        //     info!("-- event {:?}", event);
        //     match event {
        //         "Closed" => { info!(" closed."); },
        //         _ => ()
        //     }
        // }
    }
}

fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids) {
    use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

    let body = Canvas::new()
        .color(color::BLUE)
        .border(0.5)
        .w_h(110.0, 150.0)
        .set(ids.body, ui);

    // let floating = widget::Canvas::new().floating(true).w_h(110.0, 150.0).label_color(color::RED);

    let button = widget::Button::new().color(color::RED).w_h(30.0, 30.0);
    for _click in button.floating(true).set(ids.button, ui) {
        info!("Bing!");
    }
}

widget_ids! {
    pub struct Ids {
        // Root IDs
        // root,
        body,
        button,
        // floating,
    }
}
