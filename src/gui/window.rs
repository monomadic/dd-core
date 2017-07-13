use winit;
//use glium;
use glium::{ DisplayBuild, Surface, Frame };
use glium::backend::glutin_backend::GlutinFacade;

use glutin;
use std::os::raw::c_void;

use gui::GUIError;
use Graphics;
use PluginConfig;
use widgets;
use widgets::*;
use Renderer;

pub struct Window {
//    display: GlutinFacade,
    renderer: Renderer,
}

impl Window {
    pub fn new<P:Graphics>(handle: *mut c_void, _: &mut P) -> Result<Window, GUIError> {
        let wb = winit::WindowBuilder::new()
            .with_visibility(true)
            .with_transparency(false)
            .with_parent(handle);

        match glutin::WindowBuilder::from_winit_builder(wb)
            .with_decorations(false)
            // .with_vsync()
            // .with_multisampling(8)
            // .with_visibility(true)
            // .with_transparency(false)
            // .with_gl_robustness(Robustness::RobustLoseContextOnReset)
            .build_glium() {
            Err(why) => Err(GUIError::CreationError(format!(".build_glium() failed: {:?}", why))),
            Ok(display) => {
                info!("Window spawned OK with conrod.");

//                let app = Window::setup_display(display, plugin);

                Ok(Window{ renderer: Renderer::new(display) })

//                match app {
//                    Ok(a) => Ok(a),
//                    Err(why) => Err(GUIError::CreationError(format!(".setup_display() failed: {:?}", why)))
//                }
            }
        }
    }

    pub fn draw<P:Graphics>(&mut self, config: &mut PluginConfig, plugin: &mut P) {
        use widgets::*;
        use Point;
        use Rect;

        Triangle::new(Rect{ origin: Point{ x:0, y:0 }, width:100, height:100 })
            .set(&mut self.renderer);

        self.renderer.set();
        self.renderer.render();
    }
}
