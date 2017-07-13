use render::*;

use Rect;

pub struct Triangle {
    position: Rect,
}

impl Triangle {
    pub fn new(rect: Rect) -> Triangle {
        Triangle {
            position: rect,
        }
    }

    pub fn set(&self, renderer: &mut Renderer) {
        renderer.instructions.push(
            RenderElement::Triangle(self.position.clone())
        );
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);
