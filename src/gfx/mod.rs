pub mod shader;
pub mod buffer;
pub(crate) mod error;

pub fn get_value<T, F>(mut v: T, f: F) -> T
where
    F: FnOnce(&mut T) -> ()
{
    f(&mut v);
    v
}
