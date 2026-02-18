mod buf;

pub use buf::{ByteBuf, Endianness};

pub trait FromBytes: Sized {
    fn from_bytes(buf: &mut ByteBuf) -> Option<Self>;
}

pub trait IntoBytes {
    fn into_bytes(self, buf: &mut ByteBuf);
}

pub fn from_bytes<T: FromBytes>(buf: &mut ByteBuf) -> Option<T> {
    T::from_bytes(buf)
}

pub fn into_bytes<T: IntoBytes>(value: T, buf: &mut ByteBuf) {
    T::into_bytes(value, buf);
}
