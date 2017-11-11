extern crate cgmath;
extern crate fidget_spinner;
extern crate gl;
extern crate time;
extern crate glutin;

use cgmath::{Vector2, Vector3};
use gl::types::*;
use glutin::GlContext;
use fidget_spinner::shader::Shader;
use fidget_spinner::sprite_renderer::{RenderOptions, SpriteRenderer};
use std::env;
use fidget_spinner::texture::{Dimensions, Texture, TextureOptions};

pub const SPINNER_WIDTH: f32 = 500.;
pub const SPINNER_HEIGHT: f32 = 500.;
pub const SPINNER_VELOCITY: f32 = 0.000001;

pub enum FidgetType {
    Black,
    Green,
    Yellow,
}

impl FidgetType {
    pub fn path(&self) -> &'static str {
        match *self {
            FidgetType::Black => "./textures/fidget-spinner-black.png",
            FidgetType::Green => "./textures/fidget-spinner-green.png",
            FidgetType::Yellow => "./textures/fidget-spinner-yellow.png",
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Need to specify the color of the fidget spinner (black, yellow, or green)");
        return;
    }

    let spinner_type = match args[1].as_str() {
        "black" => FidgetType::Black,
        "green" => FidgetType::Green,
        "yellow" => FidgetType::Yellow,
        _ => {
            println!("Invalid color type (not black, yellow, or green)");
            return;
        }
    };

    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new();
    let window = glutin::WindowBuilder::new().with_dimensions(800, 600);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
    let _ = unsafe { gl_window.make_current() };

    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    let (width, height) = gl_window.get_inner_size_pixels().unwrap();
    let projection = cgmath::ortho(0., width as GLfloat, height as GLfloat, 0., -1., 1.);
    let shader = Shader::from_files("./shaders/sprite.vs.glsl", "./shaders/sprite.frag.glsl")
        .unwrap();
    shader.use_program();
    shader.set_int("image", 0);
    shader.set_mat4("projection", projection);

    let mut opts = TextureOptions::default();
    opts.internal_format = gl::RGBA as i32;
    opts.image_format = gl::RGBA as u32;
    let texture = Texture::new(spinner_type.path(), Dimensions::Image, opts);
    let renderer = SpriteRenderer::new(&shader);

    let mut render_opts = RenderOptions {
        position: Vector2::new(width as f32 / 2. - SPINNER_WIDTH / 2.,
                               height as f32 / 2. - SPINNER_HEIGHT / 2.),
        size: Vector2::new(SPINNER_WIDTH, SPINNER_HEIGHT),
        rotate: 0.,
        color: Vector3::new(0., 1., 0.),
    };

    let mut running = true;
    let mut spinning = false;
    let mut pressed = false;
    let mut dt;
    let mut last_time = 0;

    while running {
        let curr_time = time::precise_time_ns();
        dt = curr_time - last_time;
        last_time = curr_time;

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        if spinning {
            let velocity = SPINNER_VELOCITY * (dt as f32);
            render_opts.rotate += velocity;
            if render_opts.rotate >= 360. {
                render_opts.rotate -= 360.;
            }
        }

        renderer.draw(&texture, &render_opts);
        gl_window.swap_buffers().unwrap();

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::WindowEvent::MouseInput { state: glutin::ElementState::Pressed, .. } => {
                            if !pressed {
                                pressed = true;
                                spinning = !spinning;
                            }
                        }
                        glutin::WindowEvent::MouseInput { state: glutin::ElementState::Released, .. } => {
                            pressed = false;
                        }
                        glutin::WindowEvent::Closed => running = false,
                        _ => {}
                    }
                }
                _ => {}
            }
        });
    }
}
