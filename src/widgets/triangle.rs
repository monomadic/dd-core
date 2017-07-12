use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::{ Frame, Surface };

use widgets::Widget;
use render::*;

//fn triangle(&input: InputState, rect:Rect, text:String) -> (Option<Event> , Vec<RenderElement>) {
//
//}

pub struct Triangle {
    program: glium::Program,
}

//impl Widget for Triangle {
//    fn new(program: glium::Program) -> Self {
//        let shader = ShaderData {
//            vertex_shader: include_str!("shaders/polygon.vert"),
//            fragment_shader: include_str!("shaders/polygon.frag"),
//        };
//
//        Triangle {
//            program: program_from_shader(shader, display)
//        }
//    }
//
//    fn draw(&self, target: &mut Frame) {
//        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
//
//        let vertex1 = Vertex { position: [-0.5, -0.5] };
//        let vertex2 = Vertex { position: [ 0.0,  0.5] };
//        let vertex3 = Vertex { position: [ 0.5, -0.25] };
//        let shape = vec![vertex1, vertex2, vertex3];
//
//        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
//
////        let vertex_shader_src = r#"
////        #version 140
////        in vec2 position;
////        void main() {
////            gl_Position = vec4(position, 0.0, 1.0);
////        }
////        "#;
////
////        let fragment_shader_src = r#"
////        #version 140
////        out vec4 color;
////        void main() {
////            color = vec4(1.0, 0.0, 0.0, 1.0);
////        }
////        "#;
//
////        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();
//
//        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
//                    &Default::default()).unwrap();
//
//    }
//}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);
