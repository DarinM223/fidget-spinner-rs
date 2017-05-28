use gl;
use gl::types::*;
use png;
use std::borrow::Cow;
use std::fs::File;
use std::os::raw::c_void;

pub struct TextureOptions {
    pub wrap_s: GLint,
    pub wrap_t: GLint,
    pub internal_format: GLint,
    pub image_format: GLuint,
    pub filter_min: GLint,
    pub filter_mag: GLint,
}

impl TextureOptions {
    pub fn default() -> TextureOptions {
        TextureOptions {
            wrap_s: gl::REPEAT as GLint,
            wrap_t: gl::REPEAT as GLint,
            internal_format: gl::RGB as GLint,
            image_format: gl::RGB,
            filter_min: gl::LINEAR as GLint,
            filter_mag: gl::LINEAR as GLint,
        }
    }
}

pub enum Dimensions {
    Image,
    Custom(u32, u32),
}

pub struct Texture {
    texture: u32,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn new<'a, S>(path: S, dim: Dimensions, opts: TextureOptions) -> Texture
        where S: Into<Cow<'a, str>>
    {
        let path: &str = &path.into();

        let decoder = png::Decoder::new(File::open(path).unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();

        let (width, height) = match dim {
            Dimensions::Custom(width, height) => (width, height),
            Dimensions::Image => (info.width, info.height),
        };

        let mut texture = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           opts.internal_format,
                           width as i32,
                           height as i32,
                           0,
                           opts.image_format,
                           gl::UNSIGNED_BYTE,
                           buf.as_ptr() as *const c_void);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, opts.wrap_s);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, opts.wrap_t);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, opts.filter_min);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, opts.filter_mag);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Texture {
            texture: texture,
            width: width,
            height: height,
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.texture) };
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.texture) };
    }
}
