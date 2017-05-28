use gl;
use gl::types::*;

pub fn gl_check(file: &str, line: u32) {
    let mut error_code = 0;
    loop {
        error_code = unsafe { gl::GetError() };
        if error_code == gl::NO_ERROR {
            break;
        }

        let unknown_err = format!("UNKNOWN_ERROR {}", error_code);
        let err = match error_code {
            gl::INVALID_ENUM => "INVALID_ENUM",
            gl::INVALID_VALUE => "INVALID_VALUE",
            gl::INVALID_OPERATION => "INVALID_OPERATION",
            gl::STACK_OVERFLOW => "STACK_OVERFLOW",
            gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
            gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
            _ => unknown_err.as_str(),
        };

        println!("{} | {} ({})", err, file, line);
    }
}

#[macro_export]
macro_rules! gl_check {
    () => (gl_check(file!(), line!()));
}
