use std::io;
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::Read;

use glium;
use glium::Surface;
use widgets::Widget;

mod program;

#[derive(Clone, Debug)]
pub struct Rect {
    x: i32,
    y: i32,
}

#[derive(Clone)]
pub enum RenderElement {
    Triangle(Rect)
}

pub struct Renderer {
    pub display: glium::Display,
    pub triangle_program: glium::Program,
    pub instructions: Vec<RenderElement>,
}

impl Renderer{
    pub fn new(display: glium::Display) -> Renderer {
        let triangle_program = program_from_shader(&display,
                                                   include_str!("shaders/polygon.vert"),
                                                   include_str!("shaders/polygon.frag"));
        Renderer {
            display: display,
            instructions: Vec::new(),
            triangle_program: triangle_program,
        }
    }

    pub fn set(&mut self) { self.instructions.push(RenderElement::Triangle(Rect{x:0, y:0})) }

    pub fn render(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.1, 0.1, 0.1, 1.0);

        for instruction in self.instructions.clone() {
            match instruction {
                RenderElement::Triangle(position) => {
                    #[derive(Copy, Clone)]
                    struct Vertex {
                        position: [f32; 2],
                    }

                    implement_vertex!(Vertex, position);

                    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

                    let vertex1 = Vertex { position: [-0.5, -0.5] };
                    let vertex2 = Vertex { position: [ 0.0,  0.5] };
                    let vertex3 = Vertex { position: [ 0.5, -0.25] };

                    let shape = vec![vertex1, vertex2, vertex3];

                    let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();

                    target.draw(
                        &vertex_buffer,
                        &indices,
                        &self.triangle_program,
                        &glium::uniforms::EmptyUniforms,
                        &Default::default()).unwrap();
                }
            }
        }
        target.finish().expect("target to unwrap");
    }
}

pub fn program_from_shader(display: &glium::Display, vertex_shader: &str, fragment_shader: &str) -> glium::Program {
    glium::Program::from_source(
        display,
        &vertex_shader,
        &fragment_shader,
        None).expect("program to complete.")
}
