use gl::types::*;

pub enum Uniform<'a> {
    Int(GLint),
    Int2(GLint, GLint),
    Int3(GLint, GLint, GLint),
    Int4(GLint, GLint, GLint, GLint),
    IntVec(&'a [GLint]),
    IntVec2(&'a [[GLint; 2]]),
    IntVec3(&'a [[GLint; 3]]),
    IntVec4(&'a [[GLint; 4]]),
    UInt(GLuint),
    UInt2(GLuint, GLuint),
    UInt3(GLuint, GLuint, GLuint),
    UInt4(GLuint, GLuint, GLuint, GLuint),
    UIntVec(&'a [GLuint]),
    UIntVec2(&'a [[GLuint; 2]]),
    UIntVec3(&'a [[GLuint; 3]]),
    UIntVec4(&'a [[GLuint; 4]]),
    Float(GLfloat),
    Float2(GLfloat, GLfloat),
    Float3(GLfloat, GLfloat, GLfloat),
    Float4(GLfloat, GLfloat, GLfloat, GLfloat),
    FloatVec(&'a [GLfloat]),
    FloatVec2(&'a [[GLfloat; 2]]),
    FloatVec3(&'a [[GLfloat; 3]]),
    FloatVec4(&'a [[GLfloat; 4]]),
    Matrix2(&'a [[[GLfloat; 2]; 2]]),
    Matrix3(&'a [[[GLfloat; 3]; 3]]),
    Matrix4(&'a [[[GLfloat; 4]; 4]]),
    Matrix2x3(&'a [[[GLfloat; 2]; 3]]),
    Matrix3x2(&'a [[[GLfloat; 3]; 2]]),
    Matrix2x4(&'a [[[GLfloat; 2]; 4]]),
    Matrix4x2(&'a [[[GLfloat; 4]; 2]]),
    Matrix3x4(&'a [[[GLfloat; 3]; 4]]),
    Matrix4x3(&'a [[[GLfloat; 4]; 3]]),
}

macro_rules! uniform_from {
    ($name:ident, &$ty:ty) => {
        impl<'a> From<&'a $ty> for Uniform<'a> 
        {
            fn from(v: &'a $ty) -> Uniform<'a> {
                Uniform::$name(v)
            }
        }
    };
    ($name1:ident, $name2:ident, $name3:ident, $name4:ident, $ty:ty) => {
        impl<'a> From<&'a $ty> for Uniform<'a> {
            fn from(v: &'a $ty) -> Uniform<'a> {
                Uniform::$name1(*v)
            }
        }
        impl<'a> From<&'a [$ty; 2]> for Uniform<'a> {
            fn from(v: &'a [$ty; 2]) -> Uniform<'a> {
                Uniform::$name2(v[0], v[1])
            }
        }
        impl<'a> From<&'a [$ty; 3]> for Uniform<'a> {
            fn from(v: &'a [$ty; 3]) -> Uniform<'a> {
                Uniform::$name3(v[0], v[1], v[2])
            }
        }
        impl<'a> From<&'a [$ty; 4]> for Uniform<'a> {
            fn from(v: &'a [$ty; 4]) -> Uniform<'a> {
                Uniform::$name4(v[0], v[1], v[2], v[3])
            }
        }
    };
}

uniform_from!(Int, Int2, Int3, Int4, i32);
uniform_from!(UInt, UInt2, UInt3, UInt4, u32);
uniform_from!(Float, Float2, Float3, Float4, f32);

uniform_from!(IntVec, &[GLint]);
uniform_from!(IntVec2, &[[GLint; 2]]);
uniform_from!(IntVec3, &[[GLint; 3]]);
uniform_from!(IntVec4, &[[GLint; 4]]);

uniform_from!(UIntVec, &[GLuint]);
uniform_from!(UIntVec2, &[[GLuint; 2]]);
uniform_from!(UIntVec3, &[[GLuint; 3]]);
uniform_from!(UIntVec4, &[[GLuint; 4]]);

uniform_from!(FloatVec, &[GLfloat]);
uniform_from!(FloatVec2, &[[GLfloat; 2]]);
uniform_from!(FloatVec3, &[[GLfloat; 3]]);
uniform_from!(FloatVec4, &[[GLfloat; 4]]);

uniform_from!(Matrix2, &[[[GLfloat; 2]; 2]]);
uniform_from!(Matrix3, &[[[GLfloat; 3]; 3]]);
uniform_from!(Matrix4, &[[[GLfloat; 4]; 4]]);

uniform_from!(Matrix2x3, &[[[GLfloat; 2]; 3]]);
uniform_from!(Matrix3x2, &[[[GLfloat; 3]; 2]]);
uniform_from!(Matrix2x4, &[[[GLfloat; 2]; 4]]);
uniform_from!(Matrix4x2, &[[[GLfloat; 4]; 2]]);
uniform_from!(Matrix3x4, &[[[GLfloat; 3]; 4]]);
uniform_from!(Matrix4x3, &[[[GLfloat; 4]; 3]]);