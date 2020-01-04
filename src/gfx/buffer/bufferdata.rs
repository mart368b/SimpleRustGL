use super::Primitive;
use gl::types::*;

pub trait BufferData {
    fn prototype() -> Vec<(Primitive, GLuint)>;
}

macro_rules! bufferdata_impl {
    ($($ty:ty, $name:ident);*) => {
        $(
            impl BufferData for $ty {
                fn prototype() -> Vec<(Primitive, GLuint)> {
                    vec![(Primitive::$name, 1)]
                }
            }
        )*
    }
}

bufferdata_impl!{
    i8, Byte;
    u8, UByte;
    i16, Short;
    u16, UShort;
    i32, Int;
    u32, UInt;
    f32, Float;
    f64, Double
}
