mod vao;
mod primitive;
mod format;

mod buffer;
mod buffertype;
mod readbuffermap;
mod writebuffermap;
mod bufferdata;

pub use buffer::*;
pub use bufferdata::*;
pub use readbuffermap::*;
pub use writebuffermap::*;
pub use format::Format;
pub use primitive::Primitive;
pub use buffertype::*;
pub use vao::Vao;

pub type VboBuffer<T, Acces> = Buffer<T, ArrayBuffer, Acces>;
pub type EboBuffer<T, Acces> = Buffer<T, ElementArrayBuffer, Acces>;