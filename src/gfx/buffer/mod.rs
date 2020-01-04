mod vao;
mod indexprimitive;
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
pub use primitive::Primitive;
pub use indexprimitive::IndexPrimitive;
pub use buffertype::*;
pub use vao::Vao;

pub type VboBuffer<T, Acces: BufferAcces> = Buffer<T, ArrayBuffer, Acces>;
pub type EboBuffer<T: IndexPrimitive, Acces: BufferAcces> = Buffer<T, ElementArrayBuffer, Acces>;