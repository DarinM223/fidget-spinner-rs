use cgmath::{Matrix4, Vector3};
use gl;
use gl::types::*;
use std::borrow::Cow;
use std::result;
use std::ffi::CString;
use std::fs::File;
use std::io::{self, Read};
use std::ptr;

#[derive(Debug)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

#[derive(Debug)]
pub enum ShaderErr {
    CompileErr(ShaderType, String),
    LinkErr(String),
    IoError(io::Error),
}

impl From<io::Error> for ShaderErr {
    fn from(err: io::Error) -> ShaderErr {
        ShaderErr::IoError(err)
    }
}

pub type Result<T> = result::Result<T, ShaderErr>;

pub struct Shader {
    program: u32,
}

impl Shader {
    pub fn new<'a, 'b, A, B>(vertex_src: A, fragment_src: B) -> Result<Shader>
    where
        A: Into<Cow<'a, str>>,
        B: Into<Cow<'b, str>>,
    {
        let vertex_src = vertex_src.into();
        let fragment_src = fragment_src.into();

        let vertex_shader = unsafe { compile_shader(ShaderType::Vertex, &vertex_src)? };
        let fragment_shader = unsafe { compile_shader(ShaderType::Fragment, &fragment_src)? };
        let program = unsafe { link_program(vertex_shader, fragment_shader)? };

        Ok(Shader { program: program })
    }

    pub fn from_files<'a, 'b, A, B>(vertex_path: A, fragment_path: B) -> Result<Shader>
    where
        A: Into<Cow<'a, str>>,
        B: Into<Cow<'a, str>>,
    {
        let vertex_path: &str = &vertex_path.into();
        let fragment_path: &str = &fragment_path.into();

        let mut vertex_file = File::open(vertex_path)?;
        let mut fragment_file = File::open(fragment_path)?;

        let (mut vertex_src, mut fragment_src) = (String::new(), String::new());
        vertex_file.read_to_string(&mut vertex_src)?;
        fragment_file.read_to_string(&mut fragment_src)?;

        Shader::new(vertex_src, fragment_src)
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.program) };
    }

    pub fn set_int(&self, name: &str, value: GLint) {
        let name = CString::new(name.as_bytes()).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, name.as_ptr());
            gl::Uniform1i(location, value);
        }
    }

    pub fn set_mat4(&self, name: &str, mat: Matrix4<GLfloat>) {
        let name = CString::new(name.as_bytes()).unwrap();
        let matrix: [[GLfloat; 4]; 4] = mat.into();

        unsafe {
            let location = gl::GetUniformLocation(self.program, name.as_ptr());
            gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr() as *const GLfloat);
        }
    }

    pub fn set_vec3(&self, name: &str, v: Vector3<GLfloat>) {
        let name = CString::new(name.as_bytes()).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program, name.as_ptr());
            gl::Uniform3f(location, v.x, v.y, v.z);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.program) };
    }
}

unsafe fn compile_shader(shader_type: ShaderType, source: &str) -> Result<u32> {
    let source_c = CString::new(source.as_bytes()).unwrap();
    let shader = match shader_type {
        ShaderType::Vertex => gl::CreateShader(gl::VERTEX_SHADER),
        ShaderType::Fragment => gl::CreateShader(gl::FRAGMENT_SHADER),
    };

    gl::ShaderSource(shader, 1, &source_c.as_ptr(), ptr::null());
    gl::CompileShader(shader);

    let mut success = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    if success != (gl::TRUE as GLint) {
        let mut buf = Vec::with_capacity(512 - 1);
        gl::GetShaderInfoLog(
            shader,
            512,
            ptr::null_mut(),
            buf.as_mut_ptr() as *mut GLchar,
        );
        let info_log = String::from_utf8(buf).unwrap();
        return Err(ShaderErr::CompileErr(shader_type, info_log));
    }

    Ok(shader)
}

unsafe fn link_program(vertex_shader: u32, fragment_shader: u32) -> Result<u32> {
    let program = gl::CreateProgram();
    gl::AttachShader(program, vertex_shader);
    gl::AttachShader(program, fragment_shader);
    gl::LinkProgram(program);

    let mut success = 0;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
    if success != (gl::TRUE as GLint) {
        let mut buf = Vec::with_capacity(512 - 1);
        gl::GetProgramInfoLog(
            program,
            512,
            ptr::null_mut(),
            buf.as_mut_ptr() as *mut GLchar,
        );
        let info_log = String::from_utf8(buf).unwrap();
        return Err(ShaderErr::LinkErr(info_log));
    }

    Ok(program)
}
