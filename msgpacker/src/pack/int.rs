use super::{Format, Packable};
use core::iter;
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

impl Packable for u8 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self <= 127 {
            buf.extend(iter::once(self & Format::POSITIVE_FIXINT));
            1
        } else {
            buf.extend(iter::once(Format::UINT8).chain(iter::once(*self)));
            2
        }
    }
}

impl Packable for Option<NonZeroU8> {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        match self {
            Some(t) => u8::from(*t).pack(buf),
            None => 0u32.pack(buf),
        }
    }
}

impl Packable for u16 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self <= 127 {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        } else if *self <= u8::MAX as u16 {
            buf.extend(iter::once(Format::UINT8).chain(iter::once(*self as u8)));
            2
        } else {
            buf.extend(iter::once(Format::UINT16).chain(self.to_be_bytes()));
            3
        }
    }
}

impl Packable for Option<NonZeroU16> {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        match self {
            Some(t) => u16::from(*t).pack(buf),
            None => 0u32.pack(buf),
        }
    }
}

impl Packable for u32 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self <= 127 {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        } else if *self <= u8::MAX as u32 {
            buf.extend(iter::once(Format::UINT8).chain(iter::once(*self as u8)));
            2
        } else if *self <= u16::MAX as u32 {
            buf.extend(iter::once(Format::UINT16).chain((*self as u16).to_be_bytes()));
            3
        } else {
            buf.extend(iter::once(Format::UINT32).chain(self.to_be_bytes()));
            5
        }
    }
}

impl Packable for Option<NonZeroU32> {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        match self {
            Some(t) => u32::from(*t).pack(buf),
            None => 0u32.pack(buf),
        }
    }
}

impl Packable for u64 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self <= 127 {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        } else if *self <= u8::MAX as u64 {
            buf.extend(iter::once(Format::UINT8).chain(iter::once(*self as u8)));
            2
        } else if *self <= u16::MAX as u64 {
            buf.extend(iter::once(Format::UINT16).chain((*self as u16).to_be_bytes()));
            3
        } else if *self <= u32::MAX as u64 {
            buf.extend(iter::once(Format::UINT32).chain((*self as u32).to_be_bytes()));
            5
        } else {
            buf.extend(iter::once(Format::UINT64).chain(self.to_be_bytes()));
            9
        }
    }
}

impl Packable for Option<NonZeroU64> {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        match self {
            Some(t) => u64::from(*t).pack(buf),
            None => 0u32.pack(buf),
        }
    }
}

impl Packable for u128 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self <= 127 {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        } else if *self <= u8::MAX as u128 {
            buf.extend(iter::once(Format::UINT8).chain(iter::once(*self as u8)));
            2
        } else if *self <= u16::MAX as u128 {
            buf.extend(iter::once(Format::UINT16).chain((*self as u16).to_be_bytes()));
            3
        } else if *self <= u32::MAX as u128 {
            buf.extend(iter::once(Format::UINT32).chain((*self as u32).to_be_bytes()));
            5
        } else if *self <= u64::MAX as u128 {
            buf.extend(iter::once(Format::UINT64).chain((*self as u64).to_be_bytes()));
            9
        } else {
            buf.extend(
                iter::once(Format::BIN8)
                    .chain(iter::once(16))
                    .chain(self.to_be_bytes()),
            );
            18
        }
    }
}

impl Packable for Option<NonZeroU128> {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        match self {
            Some(t) => u128::from(*t).pack(buf),
            None => 0u32.pack(buf),
        }
    }
}

impl Packable for usize {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self <= 127 {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        } else if *self <= u8::MAX as usize {
            buf.extend(iter::once(Format::UINT8).chain(iter::once(*self as u8)));
            2
        } else if *self <= u16::MAX as usize {
            buf.extend(iter::once(Format::UINT16).chain((*self as u16).to_be_bytes()));
            3
        } else if *self <= u32::MAX as usize {
            buf.extend(iter::once(Format::UINT32).chain((*self as u32).to_be_bytes()));
            5
        } else {
            buf.extend(iter::once(Format::UINT64).chain(self.to_be_bytes()));
            9
        }
    }
}

impl Packable for Option<NonZeroUsize> {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        match self {
            Some(t) => usize::from(*t).pack(buf),
            None => 0u32.pack(buf),
        }
    }
}

impl Packable for i8 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self <= -33 {
            buf.extend(iter::once(Format::INT8).chain(iter::once(*self as u8)));
            2
        } else if *self <= -1 {
            buf.extend(iter::once((*self | -32i8) as u8));
            1
        } else {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        }
    }
}

impl Packable for Option<NonZeroI8> {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        match self {
            Some(t) => i8::from(*t).pack(buf),
            None => 0u32.pack(buf),
        }
    }
}

impl Packable for i16 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self < i8::MIN as i16 {
            buf.extend(iter::once(Format::INT16).chain(self.to_be_bytes()));
            3
        } else if *self <= -33 {
            buf.extend(iter::once(Format::INT8).chain(iter::once((*self as i8) as u8)));
            2
        } else if *self <= -1 {
            buf.extend(iter::once((*self as i8 | -32i8) as u8));
            1
        } else if *self <= i8::MAX as i16 {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        } else {
            buf.extend(iter::once(Format::INT16).chain(self.to_be_bytes()));
            3
        }
    }
}

impl Packable for Option<NonZeroI16> {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        match self {
            Some(t) => i16::from(*t).pack(buf),
            None => 0u32.pack(buf),
        }
    }
}

impl Packable for i32 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self < i16::MIN as i32 {
            buf.extend(iter::once(Format::INT32).chain(self.to_be_bytes()));
            5
        } else if *self < i8::MIN as i32 {
            buf.extend(iter::once(Format::INT16).chain((*self as i16).to_be_bytes()));
            3
        } else if *self <= -33 {
            buf.extend(iter::once(Format::INT8).chain(iter::once((*self as i8) as u8)));
            2
        } else if *self <= -1 {
            buf.extend(iter::once((*self | -32i32) as u8));
            1
        } else if *self <= i8::MAX as i32 {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        } else if *self <= i16::MAX as i32 {
            buf.extend(iter::once(Format::INT16).chain((*self as i16).to_be_bytes()));
            3
        } else {
            buf.extend(iter::once(Format::INT32).chain(self.to_be_bytes()));
            5
        }
    }
}

impl Packable for Option<NonZeroI32> {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        match self {
            Some(t) => i32::from(*t).pack(buf),
            None => 0u32.pack(buf),
        }
    }
}

impl Packable for i64 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self < i32::MIN as i64 {
            buf.extend(iter::once(Format::INT64).chain(self.to_be_bytes()));
            9
        } else if *self < i16::MIN as i64 {
            buf.extend(iter::once(Format::INT32).chain((*self as i32).to_be_bytes()));
            5
        } else if *self < i8::MIN as i64 {
            buf.extend(iter::once(Format::INT16).chain((*self as i16).to_be_bytes()));
            3
        } else if *self <= -33 {
            buf.extend(iter::once(Format::INT8).chain(iter::once((*self as i8) as u8)));
            2
        } else if *self <= -1 {
            buf.extend(iter::once((*self | -32i64) as u8));
            1
        } else if *self <= i8::MAX as i64 {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        } else if *self <= i16::MAX as i64 {
            buf.extend(iter::once(Format::INT16).chain((*self as i16).to_be_bytes()));
            3
        } else if *self <= i32::MAX as i64 {
            buf.extend(iter::once(Format::INT32).chain((*self as i32).to_be_bytes()));
            5
        } else {
            buf.extend(iter::once(Format::INT64).chain(self.to_be_bytes()));
            9
        }
    }
}

impl Packable for Option<NonZeroI64> {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        match self {
            Some(t) => i64::from(*t).pack(buf),
            None => 0u32.pack(buf),
        }
    }
}

impl Packable for i128 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self < i64::MIN as i128 {
            buf.extend(
                iter::once(Format::BIN8)
                    .chain(iter::once(16))
                    .chain(self.to_be_bytes()),
            );
            18
        } else if *self < i32::MIN as i128 {
            buf.extend(iter::once(Format::INT64).chain((*self as i64).to_be_bytes()));
            9
        } else if *self < i16::MIN as i128 {
            buf.extend(iter::once(Format::INT32).chain((*self as i32).to_be_bytes()));
            5
        } else if *self < i8::MIN as i128 {
            buf.extend(iter::once(Format::INT16).chain((*self as i16).to_be_bytes()));
            3
        } else if *self <= -33 {
            buf.extend(iter::once(Format::INT8).chain(iter::once((*self as i8) as u8)));
            2
        } else if *self <= -1 {
            buf.extend(iter::once((*self | -32i128) as u8));
            1
        } else if *self <= i8::MAX as i128 {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        } else if *self <= i16::MAX as i128 {
            buf.extend(iter::once(Format::INT16).chain((*self as i16).to_be_bytes()));
            3
        } else if *self <= i32::MAX as i128 {
            buf.extend(iter::once(Format::INT32).chain((*self as i32).to_be_bytes()));
            5
        } else if *self <= i64::MAX as i128 {
            buf.extend(iter::once(Format::INT64).chain((*self as i64).to_be_bytes()));
            9
        } else {
            buf.extend(
                iter::once(Format::BIN8)
                    .chain(iter::once(16))
                    .chain(self.to_be_bytes()),
            );
            18
        }
    }
}

impl Packable for Option<NonZeroI128> {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        match self {
            Some(t) => i128::from(*t).pack(buf),
            None => 0u32.pack(buf),
        }
    }
}

impl Packable for isize {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self < i32::MIN as isize {
            buf.extend(iter::once(Format::INT64).chain(self.to_be_bytes()));
            9
        } else if *self < i16::MIN as isize {
            buf.extend(iter::once(Format::INT32).chain((*self as i32).to_be_bytes()));
            5
        } else if *self < i8::MIN as isize {
            buf.extend(iter::once(Format::INT16).chain((*self as i16).to_be_bytes()));
            3
        } else if *self <= -33 {
            buf.extend(iter::once(Format::INT8).chain(iter::once((*self as i8) as u8)));
            2
        } else if *self <= -1 {
            buf.extend(iter::once((*self | -32isize) as u8));
            1
        } else if *self <= i8::MAX as isize {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        } else if *self <= i16::MAX as isize {
            buf.extend(iter::once(Format::INT16).chain((*self as i16).to_be_bytes()));
            3
        } else if *self <= i32::MAX as isize {
            buf.extend(iter::once(Format::INT32).chain((*self as i32).to_be_bytes()));
            5
        } else {
            buf.extend(iter::once(Format::INT64).chain(self.to_be_bytes()));
            9
        }
    }
}

impl Packable for Option<NonZeroIsize> {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        match self {
            Some(t) => isize::from(*t).pack(buf),
            None => 0u32.pack(buf),
        }
    }
}
