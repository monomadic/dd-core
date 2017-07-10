use glium::backend::glutin_backend::GlutinFacade;
use glium::Frame;
use widgets::Widget;

pub struct Triangle {}

impl Widget for Triangle {
    fn new() -> Self {
        Triangle {}
    }

    fn draw(&self, display: &GlutinFacade, target: &mut Frame) {
    }
}
