extern crate sdl2;
extern crate gl;

use anyhow::Result;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

use std::rc::Rc;
use std::time::Duration;

mod gfx;
pub use gfx::graphics::*;
pub use gfx::buffer::*;

#[derive(Debug)]
#[repr(C)]
pub struct Vertex {
    x: f32,
    y: f32,
    amount: f32
}

impl VboData for Vertex {
    fn prototype() -> Vec<(Primitive, u32)> {
        vec![
            (Primitive::Float, 2),
            (Primitive::Float, 1)
        ]
    }
}

fn main() -> Result<()> {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // CREATE SHADER
    let vert_shader = VertexShader::from_source("\
    #version 330 core
    
    layout (location = 0) in vec2 Position;
    layout (location = 1) in float Amount;

    out VertexOut {
        float Amount;
    } vertex_out;

    void main()
    {
        gl_Position = vec4(Position.xy, 1.0, 1.0);
        vertex_out.Amount = Amount;
    }\
    ")?;

    let geom_shader = GeometryShader::from_source("\
    #version 330 core
    layout (lines_adjacency) in;
    layout (triangle_strip, max_vertices = 6) out;

    in VertexOut {
        float Amount;
    } vertex_in[]; 

    out VertexData {
        vec3 position;
        flat vec4 Amount;
        flat bool right;
    } vertex_out;

    void main() {
        vertex_out.Amount = vec4(
            vertex_in[0].Amount,
            vertex_in[1].Amount,
            vertex_in[2].Amount,
            vertex_in[3].Amount
        );
        
        gl_Position = gl_in[0].gl_Position;
        vertex_out.position = vec3(1, 0, 0);
        EmitVertex();
        gl_Position = gl_in[1].gl_Position;
        vertex_out.position = vec3(0, 0, 1);
        EmitVertex();
        gl_Position = gl_in[2].gl_Position;
        vertex_out.position = vec3(0, 1, 0);
        vertex_out.right = true;
        EmitVertex();
        
        gl_Position = gl_in[0].gl_Position;
        vertex_out.position = vec3(1, 0, 0);
        EmitVertex();
        gl_Position = gl_in[2].gl_Position;
        vertex_out.position = vec3(0, 1, 0);
        EmitVertex();
        gl_Position = gl_in[3].gl_Position;
        vertex_out.position = vec3(0, 0, 1);
        vertex_out.right = false;
        EmitVertex();

        EndPrimitive();
    }  \
    ")?;
    
    let frag_shader = FragmentShader::from_source("\
    #version 330 core

    const vec2 p0 = vec2(0, 0);
    const vec2 p1 = vec2(1, 0);
    const vec2 p2 = vec2(1, 1);
    const vec2 p3 = vec2(0, 1);
    
    in VertexData {
        vec3 position;
        flat vec4 Amount;
        flat bool right;
    } vertex;

    float sampleQuad(vec2 pos) {
        return mix (
            mix(vertex.Amount[0], vertex.Amount[1], pos[0]),
            mix(vertex.Amount[2], vertex.Amount[3], pos[0]),
            pos[1]
        );
    }

    out vec4 Color;
    
    void main()
    {
        vec2 pos;
        if (vertex.right) {
            pos = p0 * vertex.position.x + p2 * vertex.position.y + p1 * vertex.position.z;
        }else {
            pos = p0 * vertex.position.x + p2 * vertex.position.y + p3 * vertex.position.z;
        }
        float sample = sampleQuad(pos);
        Color = vec4(sample, pos[0], pos[1], 1);
    }\
    ")?;
        
    //CREATE PROGRAM
    let mut program = Program::from_shaders(vec![
        Rc::new(vert_shader),
        Rc::new(geom_shader),
        Rc::new(frag_shader),
    ])?;
    program.set_used();

    // CREATE VBO
    let mut vbo: Vbo<Vertex, DynamicBuffer> = Vbo::new(
        &[
            Vertex{ x: -0.9, y: -0.9, amount: 0.0 },
            Vertex{ x: -0.9, y:  0.9, amount: 1.0 },
            Vertex{ x:  0.9, y:  0.9, amount: 0.0 },
            Vertex{ x:  0.9, y: -0.9, amount: 1.0 },
        ]
    );

    //CREATE VAO
    let mut vao = Vao::new(Format::LinesAdj);
    vao.bind_vbo(
        0,
        &mut vbo
    );

     // CREATE EBO
    let mut ebo: Ebo<u32, StaticBuffer> = Ebo::new(
        &[
            0, 1, 2, 3
        ]
    );

    ebo.bind();
    
    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }
    
    let mut event_pump = sdl.event_pump().unwrap();
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // DRAW VAO
        vao.draw_elements(4, Primitive::UInt, 0);

        window.gl_swap_window();
        std::thread::sleep(Duration::from_millis(30))
    }
    Ok(())
}