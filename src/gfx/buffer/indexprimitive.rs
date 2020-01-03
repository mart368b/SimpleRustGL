use gl::types::*;

pub trait IndexPrimitive {
    fn value() -> GLenum;
}

macro_rules! index_primitive{
    ($($ty:ty => $p:ident),*) => {
        $(
            impl IndexPrimitive for $ty {
                fn value() -> GLenum {
                    gl::$p
                }
            }
        )+
    }
}

index_primitive!{
    i16 => SHORT,
    u16 => UNSIGNED_SHORT,
    i32 => INT,
    u32 => UNSIGNED_INT
}