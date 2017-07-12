use glium::Frame;
use glium::backend::glutin_backend::GlutinFacade;

mod knob;
mod triangle;
pub use self::knob::*;
pub use self::triangle::*;

pub trait Widget {
    fn new(display: &GlutinFacade) -> Self;
    fn draw(&self, target: &mut Frame);
}
