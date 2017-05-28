extern crate cgmath;
extern crate gl;
extern crate glutin;
extern crate png;

mod shader;
mod sprite_renderer;
mod texture;

use cgmath::{Vector2, Vector3};
use shader::Shader;
use sprite_renderer::{RenderOptions, SpriteRenderer};
use texture::{Dimensions, Texture, TextureOptions};

fn main() {
    let window = glutin::Window::new().unwrap();
    unsafe { window.make_current() }.unwrap();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let shader = Shader::from_files("./shaders/sprite.vs.glsl", "./shaders/sprite.frag.glsl")
        .unwrap();
    let opts = TextureOptions::default();
    let texture = Texture::new("./textures/fidget_spinner.png", Dimensions::Image, opts);
    let renderer = SpriteRenderer::new(&shader);

    let mut render_opts = RenderOptions {
        position: Vector2::new(0., 0.),
        size: Vector2::new(100., 100.),
        rotate: 60.,
        color: Vector3::new(0., 0., 0.),
    };

    for event in window.wait_events() {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        renderer.draw(&texture, &render_opts);

        if let glutin::Event::Closed = event {
            break;
        }
    }
}
