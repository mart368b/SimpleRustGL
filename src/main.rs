use anyhow::Result;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use std::time::Duration;

pub mod gfx;

mod application;
use application::{Graphics, World};

type Vector2 = nalgebra::Vector2<f32>;
type Vector3 = nalgebra::Vector3<f32>;

const TILE_X_COUNT: usize = 50;
const TILE_Y_COUNT: usize = 50;

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
    let mut gfx = Graphics::new()?;
    let mut world = World::new(
        1., 1., TILE_X_COUNT, TILE_Y_COUNT
    )?;
    
    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }
    
    let mut event_pump = sdl.event_pump().unwrap();

    let mut margin = 0.5;

    let mut amount = 0.1;

    let mut mouse = Vector2::new(0., 0.);
    let mut mouse_down = false;
    let mut direction = true;
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    margin += 0.02;
                    gfx.program.set_uniform("margin", margin);
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    margin -= 0.02;
                    gfx.program.set_uniform("margin", margin);
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    amount = clamp(0., 1., amount + 0.05);
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    amount = clamp(0., 1., amount - 0.05);
                },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    let mut vbo = world.avbo.write();
                    let map = &mut *vbo;
                    for vertex in &mut *map {
                        *vertex = 0.;
                    }
                },
                Event::MouseMotion{
                    x, y, ..
                } => {
                    if mouse_down {
                        let win_size = &window.size();
                        let win_size_vec = Vector2::new(win_size.0 as f32, win_size.1 as f32);
                        let world_size = Vector2::new(TILE_X_COUNT as f32, TILE_Y_COUNT as f32);
                        let ratio = win_size_vec.component_div(&world_size);
                        let pos = mouse.component_div(&ratio);

                        let change = if direction {
                            amount
                        }else {
                            -amount
                        };

                        world.add(pos, change);
                    }
                    mouse = Vector2::new(x as f32, y as f32);
                },
                Event::MouseButtonDown{
                    mouse_btn, ..
                } => {
                    match mouse_btn {
                        MouseButton::Left => {
                            direction = true;
                            mouse_down = true;
                        },
                        MouseButton::Right => {
                            direction = false;
                            mouse_down = true;
                        },
                        _ => {}
                    }
                },
                Event::MouseButtonUp{
                    mouse_btn, ..
                } => {
                    match mouse_btn {
                        MouseButton::Left => {
                            mouse_down = false;
                        },
                        MouseButton::Right => {
                            mouse_down = false;
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // DRAW VAO
        world.draw();

        window.gl_swap_window();
        std::thread::sleep(Duration::from_millis(30))
    }
    Ok(())
}

pub fn clamp<T: PartialOrd>(v0: T, v1: T, v: T) -> T {
    if v > v1 {
        v1
    }else if v < v0 {
        v0
    }else {
        v
    }
}