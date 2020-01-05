mod vao;
mod primitive;
mod format;

mod buffer;
mod buffertype;
mod buffermap;
mod bufferdata;

pub use buffer::*;
pub use bufferdata::*;
pub use buffermap::*;
pub use format::Format;
pub use primitive::{Primitive, IndexPrimitive};
pub use buffertype::*;
pub use vao::Vao;

pub type VboBuffer<T, Acces> = Buffer<T, ArrayBuffer, Acces>;
pub type EboBuffer<T: IndexPrimitive, Acces> = Buffer<T, ElementArrayBuffer, Acces>;