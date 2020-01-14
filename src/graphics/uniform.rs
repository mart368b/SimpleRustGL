use gl::types::*;

pub trait Uniform<T> {
    unsafe fn set(self, loc: GLint);
}

macro_rules! Uniform {
    ($ty:ty, $name0:ident, $name1:ident, $name2:ident, $name3:ident, $name4:ident, $name5:ident, $name6:ident, $name7:ident) => {
        impl<'a> Uniform<$ty> for $ty {
            unsafe fn set(self, loc: GLint) {
                gl::$name0(loc, self);
            }
        }
        impl<'a> Uniform<&'a [$ty; 2]> for &'a [$ty; 2] {
            unsafe fn set(self, loc: GLint) {
                gl::$name1(loc, self[0], self[1]);
            }
        }
        impl<'a> Uniform<&'a [$ty; 3]> for &'a [$ty; 3] {
            unsafe fn set(self, loc: GLint) {
                gl::$name2(loc, self[0], self[1], self[2]);
            }
        }
        impl<'a> Uniform<&'a [$ty; 4]> for &'a [$ty; 4] {
            unsafe fn set(self, loc: GLint) {
                gl::$name3(loc, self[0], self[1], self[2], self[3]);
            }
        }

        impl<'a> Uniform<&'a [$ty]> for &'a [$ty] {
            unsafe fn set(self, loc: GLint) {
                gl::$name4(loc, self.len() as GLsizei, self.as_ptr() as *const $ty)
            }
        }
        impl<'a> Uniform<&'a [[$ty; 2]]> for &'a [[$ty; 2]] {
            unsafe fn set(self, loc: GLint) {
                gl::$name5(loc, self.len() as GLsizei, self.as_ptr() as *const $ty)
            }
        }
        impl<'a> Uniform<&'a [[$ty; 3]]> for &'a [[$ty; 3]] {
            unsafe fn set(self, loc: GLint) {
                gl::$name6(loc, self.len() as GLsizei, self.as_ptr() as *const $ty)
            }
        }
        impl<'a> Uniform<&'a [[$ty; 4]]> for &'a [[$ty; 4]] {
            unsafe fn set(self, loc: GLint) {
                gl::$name7(loc, self.len() as GLsizei, self.as_ptr() as *const $ty)
            }
        }
    };
    (matrix, $name:ident, $ty0:ty, $inverse:ident) => {
        impl<'a> Uniform<&'a [$ty0]> for &'a [$ty0] {
            unsafe fn set(self, loc: GLint) {
                gl::$name(loc, self.len() as GLsizei, $inverse as GLboolean, self.as_ptr() as *const GLfloat);
            }
        }
    };
    ($($t0:tt),+: $($($t1:tt),+):+) => {
        Uniform! { $($t0),+ }
        Uniform! { $($($t1),+):+ }
    }
}

Uniform! {
    i32, Uniform1i, Uniform2i, Uniform3i, Uniform4i, Uniform1iv, Uniform2iv, Uniform3iv, Uniform4iv:
    u32, Uniform1ui, Uniform2ui, Uniform3ui, Uniform4ui, Uniform1uiv, Uniform2uiv, Uniform3uiv, Uniform4uiv:
    f32, Uniform1f, Uniform2f, Uniform3f, Uniform4f, Uniform1fv, Uniform2fv, Uniform3fv, Uniform4fv:
    matrix, UniformMatrix2fv, [[GLfloat; 2]; 2], false:
    matrix, UniformMatrix3fv, [[GLfloat; 3]; 3], false:
    matrix, UniformMatrix4fv, [[GLfloat; 4]; 4], false:
    matrix, UniformMatrix2x3fv, [[GLfloat; 2]; 3], true:
    matrix, UniformMatrix2x3fv, [[GLfloat; 3]; 2], false:
    matrix, UniformMatrix2x4fv, [[GLfloat; 2]; 4], true:
    matrix, UniformMatrix2x4fv, [[GLfloat; 4]; 2], false:
    matrix, UniformMatrix3x4fv, [[GLfloat; 3]; 4], true:
    matrix, UniformMatrix3x4fv, [[GLfloat; 4]; 3], false
}