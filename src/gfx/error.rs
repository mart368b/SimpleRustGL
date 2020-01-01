use anyhow::{Error, anyhow};
use gl::types::*;
use std::ffi::CString;

fn get_error<Len, Log>(get_len: Len, get_log: Log) -> Error 
    where
        Len: FnOnce(&mut GLint),
        Log: FnOnce(GLint, *mut gl::types::GLchar)
{
    let mut len: GLint = 0;
    get_len(&mut len);

    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len as usize));
    // convert buffer to CString
    let error: CString = unsafe { CString::from_vec_unchecked(buffer) };

    get_log(len, error.as_ptr() as *mut gl::types::GLchar);
    match error.into_string() {
        Ok(msg) => anyhow!(msg),
        Err(e) => Error::from(e)
    }
}

pub fn get_program_error(id: GLuint) -> Error {
    get_error(
        |len| unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, len);
        },
        |len, log| unsafe {
            gl::GetProgramInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                log
            );
        }
    )
}

pub fn get_shader_error(id: GLuint) -> Error {
    get_error(
        |len| unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, len);
        },
        |len, log| unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                log
            );
        }
    )
}