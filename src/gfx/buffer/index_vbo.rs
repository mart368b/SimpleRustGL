use gl::types::*;
use super::VboDataType;

pub struct IndexVbo {
    id: GLuint,
}

impl IndexVbo {
    pub fn new() -> IndexVbo {
        let mut id = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        IndexVbo {
            id,
        }
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    pub fn bind_int(&mut self, data: &[u32], ty: VboDataType) {
        self.bind();
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                data as *const [u32] as *const GLvoid,
                ty.value(),
            );
        }
    }
}


impl Drop for IndexVbo {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}