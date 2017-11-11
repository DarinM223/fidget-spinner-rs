use cgmath::{Deg, Matrix4, Vector2, Vector3};
use cgmath::prelude::*;
use gl;
use gl::types::*;
use shader::Shader;
use std::mem;
use std::ptr;
use texture::Texture;

pub const VERTICES: [GLfloat; 24] = [
    0.,
    1.,
    0.,
    1.,
    1.,
    0.,
    1.,
    0.,
    0.,
    0.,
    0.,
    0.,
    0.,
    1.,
    0.,
    1.,
    1.,
    1.,
    1.,
    1.,
    1.,
    0.,
    1.,
    0.,
];

pub struct RenderOptions {
    pub position: Vector2<GLfloat>,
    pub size: Vector2<GLfloat>,
    pub rotate: GLfloat,
    pub color: Vector3<GLfloat>,
}

pub struct SpriteRenderer<'a> {
    vao: u32,
    shader: &'a Shader,
}

impl<'a> SpriteRenderer<'a> {
    pub fn new(shader: &'a Shader) -> SpriteRenderer<'a> {
        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&VERTICES[0]),
                gl::STATIC_DRAW,
            );

            gl::BindVertexArray(vao);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                4,
                gl::FLOAT,
                gl::FALSE,
                (4 * mem::size_of::<GLfloat>()) as i32,
                ptr::null(),
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        SpriteRenderer {
            vao: vao,
            shader: shader,
        }
    }

    pub fn draw(&self, texture: &Texture, opts: &RenderOptions) {
        self.shader.use_program();
        let mut m: Matrix4<GLfloat> = Matrix4::identity();
        m = m * Matrix4::from_translation(Vector3::new(opts.position.x, opts.position.y, 0.0));
        m = m * Matrix4::from_translation(Vector3::new(0.5 * opts.size.x, 0.5 * opts.size.y, 0.0));
        m = m * Matrix4::from_angle_z(Deg(opts.rotate));
        m = m
            * Matrix4::from_translation(Vector3::new(-0.5 * opts.size.x, -0.5 * opts.size.y, 0.0));
        m = m * Matrix4::from_nonuniform_scale(opts.size.x, opts.size.y, 1.0);

        self.shader.set_mat4("model", m);
        self.shader.set_vec3("spriteColor", opts.color);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            texture.bind();

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }
    }
}

impl<'a> Drop for SpriteRenderer<'a> {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.vao) };
    }
}
