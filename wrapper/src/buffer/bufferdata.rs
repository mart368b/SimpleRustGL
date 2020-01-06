use super::Primitive;
use gl::types::*;

pub trait BufferData {
    fn prototype() -> Vec<(Primitive, GLuint)>;
}

macro_rules! bufferdata_impl {
    ($ty:ty, $name:ident;) => {
        impl BufferData for $ty {
            fn prototype() -> Vec<(Primitive, GLuint)> {
                vec![(Primitive::$name, 1)]
            }
        }
    };
    ($ty:ty, $name:ident; $($t:tt)+) => {
        bufferdata_impl!{$ty, $name;}
        bufferdata_impl!{$($t)+}
    };
    (2, $ty:ty, $name:ident; $($t:tt)*) => {
        impl BufferData for [$ty; 2] {
            fn prototype() -> Vec<(Primitive, GLuint)> {
                vec![(Primitive::$name, 2)]
            }
        }
        bufferdata_impl!{$ty, $name; $($t)*}
    };
    (3, $ty:ty, $name:ident; $($t:tt)*) => {
        impl BufferData for [$ty; 3] {
            fn prototype() -> Vec<(Primitive, GLuint)> {
                vec![(Primitive::$name, 3)]
            }
        }
        bufferdata_impl!{2, $ty, $name; $($t)*}
    };
    (4, $ty:ty, $name:ident; $($t:tt)*) => {
        impl BufferData for [$ty; 4] {
            fn prototype() -> Vec<(Primitive, GLuint)> {
                vec![(Primitive::$name, 4)]
            }
        }
        bufferdata_impl!{3, $ty, $name; $($t)*}
    };
}

bufferdata_impl!{
    4, i16, Short;
    4, u16, UShort;
    4, i32, Int;
    4, u32, UInt;
    4, f32, Float;
    4, f64, Double;
}
