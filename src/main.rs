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
    gl_check!();

    let mut opts = TextureOptions::default();
    opts.internal_format = gl::RGBA as i32;
    opts.image_format = gl::RGBA as u32;
    let texture = Texture::new("./textures/fidget_spinner.png", Dimensions::Image, opts);
    let renderer = SpriteRenderer::new(&shader);
    gl_check!();

    let mut render_opts = RenderOptions {
        position: Vector2::new(200., 200.),
        size: Vector2::new(300., 400.),
        rotate: 45.,
        color: Vector3::new(0., 1., 0.),
    };


    events_loop.run_forever(|event| {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        renderer.draw(&texture, &render_opts);
        gl_check!();

        window.swap_buffers().unwrap();

        match event {
            glutin::Event::WindowEvent { event: glutin::WindowEvent::Closed, .. } => {
                events_loop.interrupt();
            }
            _ => {}
        }
    });
}
