extern crate sdl2;
extern crate gl;

use anyhow::Result;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

use std::rc::Rc;
use std::time::Duration;

mod gfx;
pub use gfx::shader::*;
pub use gfx::buffer::*;

#[repr(C)]
pub struct Vertex {
    x: f32,
    y: f32,
    r: f32,
    g: f32,
    b: f32,
}

impl VboData for Vertex {
    fn prototype() -> Vec<(Primitive, u32)> {
        vec![
            (Primitive::Float, 2),
            (Primitive::Float, 3)
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
        layout (location = 1) in vec3 Color;

        out VS_OUTPUT {
            vec3 Color;
        } OUT;

        void main()
        {
            gl_Position = vec4(Position, 1.0, 1.0);
            OUT.Color = Color;
        }\
    ")?;

    let frag_shader = FragmentShader::from_source("\
        #version 330 core

        in VS_OUTPUT {
            vec3 Color;
        } IN;
        
        out vec4 Color;
        
        void main()
        {
            Color = vec4(IN.Color, 1.0f);
        }\
    ")?;
    
    //CREATE PROGRAM
    let mut program = Program::from_shaders(vec![
        Rc::new(vert_shader),
        Rc::new(frag_shader)
    ])?;
    program.set_used();
    
    // CREATE VBO
    let mut vbo = Vbo::new();
    vbo.bind_data(
        &[
            Vertex{ x: -0.5, y: -0.5,    r: 1.0, g: 0.0, b: 0.0},

            Vertex{ x: -0.5, y: -0.5,    r: 1.0, g: 0.0, b: 0.0},
            Vertex{ x:  0.5, y: -0.5,    r: 0.0, g: 1.0, b: 0.0},
            Vertex{ x:  0.0, y:  0.5,    r: 0.0, g: 0.0, b: 1.0},

            Vertex{ x: -0.5, y: -0.5,    r: 1.0, g: 0.0, b: 0.0},
        ],
        VboDataType::Static
    );

    //CREATE VAO
     let mut vao = Vao::new();
     vao.bind_vbo(
         &mut vbo
     );

     let mut index = IndexVbo::new();
    index.bind_int(
        &[
            1, 2, 3
        ],
        VboDataType::Static
    );
    
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
        vao.draw_elements(
            gl::TRIANGLES,
            3,
            Primitive::UInt,
            0,
        );

        window.gl_swap_window();
        std::thread::sleep(Duration::from_millis(10))
    }
    Ok(())
}