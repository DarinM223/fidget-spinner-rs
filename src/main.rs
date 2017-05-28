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
    let window = glutin::Window::new().unwrap();
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

    let opts = TextureOptions::default();
    let texture = Texture::new("./textures/fidget_spinner.png", Dimensions::Image, opts);
    let renderer = SpriteRenderer::new(&shader);
    gl_check!();

    let mut render_opts = RenderOptions {
        position: Vector2::new(200., 200.),
        size: Vector2::new(300., 400.),
        rotate: 45.,
        color: Vector3::new(0., 1., 0.),
    };

    for event in window.wait_events() {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        renderer.draw(&texture, &render_opts);
        gl_check!();

        if let glutin::Event::Closed = event {
            break;
        }
    }
}
