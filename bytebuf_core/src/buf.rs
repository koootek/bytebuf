#[derive(Default, Debug)]
pub struct ByteBuf {
    endianness: Endianness,
    position: usize,
    data: Vec<u8>,
}

#[derive(Default, Debug)]
pub enum Endianness {
    #[default]
    Little,
    Big,
}

impl ByteBuf {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_endianness(endianness: Endianness) -> Self {
        Self {
            endianness,
            position: 0,
            data: vec![],
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            endianness: Endianness::default(),
            position: 0,
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn with_zeored(len: usize) -> Self {
        Self {
            endianness: Endianness::default(),
            position: 0,
            data: vec![0; len],
        }
    }

    pub fn set_endianness(&mut self, endianness: Endianness) {
        self.endianness = endianness;
    }

    pub fn inner(&self) -> &[u8] {
        &self.data
    }

    pub fn inner_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn read_exact<const LEN: usize>(&mut self) -> Option<[u8; LEN]> {
        if self.position + LEN > self.data.len() {
            return None;
        }

        let data = &self.data[self.position..self.position + LEN];
        self.position += LEN;
        Some(data.try_into().unwrap())
    }

    unsafe fn read_exact_unchecked<const LEN: usize>(&mut self) -> Option<[u8; LEN]> {
        let data = &self.data[self.position..self.position + LEN];
        self.position += LEN;
        Some(data.try_into().unwrap())
    }

    pub fn read_bytes(&mut self, len: usize) -> Option<&[u8]> {
        if self.position + len >= self.data.len() {
            return None;
        }

        let data = &self.data[self.position..self.position + len];
        self.position += len;
        Some(data)
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        if self.position >= self.data.len() {
            return None;
        }

        let byte = self.data[self.position];
        self.position += 1;
        Some(byte)
    }

    pub fn read_bool(&mut self) -> Option<bool> {
        Some(self.read_u8()? == 1)
    }

    pub fn read_value<T: crate::FromBytes>(&mut self) -> Option<T> {
        T::from_bytes(self)
    }

    pub fn write_bytes(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    pub fn write_u8(&mut self, value: u8) {
        self.data.push(value);
    }

    pub fn write_bool(&mut self, value: bool) {
        self.write_u8(if value { 1 } else { 0 });
    }

    pub fn write_value<T: crate::IntoBytes>(&mut self, value: T) {
        value.into_bytes(self);
    }

    pub fn clear(&mut self) {
        self.position = 0;
        self.data.clear();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn has_remaining(&self) -> bool {
        self.remaining() != 0
    }

    pub fn remaining(&self) -> usize {
        self.data.len() - self.position
    }
}

impl From<Vec<u8>> for ByteBuf {
    fn from(value: Vec<u8>) -> Self {
        Self {
            endianness: Endianness::default(),
            position: 0,
            data: value,
        }
    }
}

macro_rules! impl_primitive {
    ($ty:ty, $len:literal) => {
        paste::paste! {
        impl ByteBuf {
            pub fn [<read_ $ty>](&mut self) -> Option<$ty> {
                match self.endianness {
                    Endianness::Little => self.[<read_ $ty _le>](),
                    Endianness::Big => self.[<read_ $ty _be>](),
                }
            }

            pub fn [<read_ $ty _le>](&mut self) -> Option<$ty> {
                if self.position + $len > self.data.len() {
                    return None;
                }

                Some(<$ty>::from_le_bytes(unsafe { self.read_exact_unchecked::<$len>()? }))
            }

            pub fn [<read_ $ty _be>](&mut self) -> Option<$ty> {
                if self.position + $len > self.data.len() {
                    return None;
                }

                Some(<$ty>::from_be_bytes(unsafe { self.read_exact_unchecked::<$len>()? }))
            }

            pub fn [<write_ $ty>](&mut self, value: $ty) {
                match self.endianness {
                    Endianness::Little => self.[<write_ $ty _le>](value),
                    Endianness::Big => self.[<write_ $ty _be>](value),
                }
            }

            pub fn [<write_ $ty _le>](&mut self, value: $ty) {
                self.data.extend_from_slice(&<$ty>::to_le_bytes(value));
            }

            pub fn [<write_ $ty _be>](&mut self, value: $ty) {
                self.data.extend_from_slice(&<$ty>::to_be_bytes(value));
            }
        }

        impl crate::FromBytes for $ty {
            fn from_bytes(buf: &mut ByteBuf) -> Option<Self> {
                buf.[<read_ $ty>]()
            }
        }

        impl crate::IntoBytes for $ty {
            fn into_bytes(self, buf: &mut ByteBuf) {
                buf.[<write_ $ty>](self);
            }
        }
        }
    };
}

impl_primitive!(u16, 2);
impl_primitive!(u32, 4);
impl_primitive!(u64, 8);
impl_primitive!(u128, 16);
impl_primitive!(usize, 8);
impl_primitive!(i16, 2);
impl_primitive!(i32, 4);
impl_primitive!(i64, 8);
impl_primitive!(i128, 16);
impl_primitive!(isize, 8);
impl_primitive!(f32, 4);
impl_primitive!(f64, 8);
