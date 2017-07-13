use std::io;
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::Read;

use glium;
use glium::Surface;
use glium::uniforms::*;
use widgets::Widget;
use cgmath;

mod program;

pub type Matrix4 = cgmath::Matrix4<f64>;


#[derive(Clone, Debug)]
pub struct Rect {
    pub origin: Point,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
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

//pub fn ui_projection(width: f64, height: f64) -> Mat4 {
//    // left, right, bottom, top, near, far
////    cgmath::ortho(0.0, width, 0.0, height, -100.0, 100.0) // having trouble with this z stuff
//    let ortho_matrix: cgmath::Matrix4<f64> = cgmath::ortho(0.0, 800.0, 0.0, 500.0, -1.0, 1.0);

//}

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

    pub fn set(&mut self) { self.instructions.push(RenderElement::Triangle(Rect{ origin: Point{ x:0, y:0 }, width:100, height:100 }))}

    pub fn get_inner_size_points(&mut self) -> (u32, u32) {
//        use glium::backend::glutin_backend::WinRef;
        self.display.get_window()
            .expect("window to exist")
            .get_inner_size_points()
            .expect("window to exist")
    }

    pub fn render(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.1, 0.1, 0.1, 1.0);

        for instruction in self.instructions.clone() {
            match instruction {
                RenderElement::Triangle(position) => {
                    let (view_width, view_height) = self.get_inner_size_points();

                    #[derive(Copy, Clone)]
                    struct Vertex {
                        position: [f32; 2],
                    }

                    implement_vertex!(Vertex, position);

                    use num_traits::float::Float;


                    let flat_projection: [[f32; 4]; 4] = cgmath::ortho(0.0, 1.0, 0.0, 1.0, -1.0, 1.0).into();

                    // left right bottom top near far
                    let ortho_projection: [[f32; 4]; 4] = cgmath::ortho(0.0, (view_width as f32), 0.0, (view_height as f32), -1.0, 1.0).into();

                    let uniforms = uniform! {
                        flat_projection: flat_projection,
                        ortho_projection: ortho_projection,
                        matrix: [
                            [1.0, 0.0, 0.0, 1.0],
                            [0.0, 1.0, 0.0, 1.0],
                            [0.0, 0.0, 1.0, 1.0],
                            [0.0, 0.0, 0.0, 1.0f32],
                        ]
                    };

                    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

                    let shape = vec![
                        Vertex { position: [ 0.0,  0.0 ] },
                        Vertex { position: [ 0.5,  1.0 ] },
                        Vertex { position: [ 1.0,  0.0 ] },
                    ];

                    let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();

                    target.draw(
                        &vertex_buffer,
                        &indices,
                        &self.triangle_program,
                        &uniforms,
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
