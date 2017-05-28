extern crate cgmath;
extern crate gl;
extern crate glutin;
extern crate png;

#[macro_use]
mod debug;
mod shader;
mod sprite_renderer;
mod texture;

use cgmath::{Vector2, Vector3};
use debug::gl_check;
use gl::types::*;
use shader::Shader;
use sprite_renderer::{RenderOptions, SpriteRenderer};
use texture::{Dimensions, Texture, TextureOptions};

pub const SPINNER_WIDTH: f32 = 500.;
pub const SPINNER_HEIGHT: f32 = 500.;

fn main() {
    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(800, 600)
        .build(&events_loop)
        .unwrap();
    unsafe { window.make_current() }.unwrap();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (width, height) = window.get_inner_size_pixels().unwrap();
    let projection = cgmath::ortho(0., width as GLfloat, height as GLfloat, 0., -1., 1.);
    let shader = Shader::from_files("./shaders/sprite.vs.glsl", "./shaders/sprite.frag.glsl")
        .unwrap();
    shader.use_program();
    shader.set_int("image", 0);
    shader.set_mat4("projection", projection);

    let mut opts = TextureOptions::default();
    opts.internal_format = gl::RGBA as i32;
    opts.image_format = gl::RGBA as u32;
    let texture = Texture::new("./textures/fidget_spinner.png", Dimensions::Image, opts);
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

    while running {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        if spinning {
            render_opts.rotate += 3.;
            if render_opts.rotate >= 360. {
                render_opts.rotate -= 360.;
            }
        }

        renderer.draw(&texture, &render_opts);
        window.swap_buffers().unwrap();

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::WindowEvent::MouseInput(glutin::ElementState::Pressed, ..) => {
                            if !pressed {
                                pressed = true;
                                spinning = !spinning;
                            }
                        }
                        glutin::WindowEvent::MouseInput(glutin::ElementState::Released, ..) => {
                            pressed = false;
                        }
                        glutin::WindowEvent::Closed => running = false,
                        _ => {}
                    }
                }
            }
        });
    }
}
