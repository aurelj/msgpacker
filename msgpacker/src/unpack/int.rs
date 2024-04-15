use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

use super::{
    helpers::{take_byte, take_byte_iter, take_num, take_num_iter},
    Error, Format, Unpackable,
};

impl Unpackable for u8 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format)),
            Format::UINT8 => take_byte(&mut buf).map(|v| (2, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format)),
            Format::UINT8 => take_byte_iter(bytes).map(|v| (2, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for Option<NonZeroU8> {
    type Error = Error;

    fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        u8::unpack(buf).map(|(s, v)| (s, NonZeroU8::new(v)))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        u8::unpack_iter(bytes).map(|(s, v)| (s, NonZeroU8::new(v)))
    }
}

impl Unpackable for u16 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as u16)),
            Format::UINT8 => take_byte(&mut buf).map(|v| (2, v as u16)),
            Format::UINT16 => take_num(&mut buf, u16::from_be_bytes).map(|v| (3, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as u16)),
            Format::UINT8 => take_byte_iter(bytes).map(|v| (2, v as u16)),
            Format::UINT16 => take_num_iter(bytes, u16::from_be_bytes).map(|v| (3, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for Option<NonZeroU16> {
    type Error = Error;

    fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        u16::unpack(buf).map(|(s, v)| (s, NonZeroU16::new(v)))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        u16::unpack_iter(bytes).map(|(s, v)| (s, NonZeroU16::new(v)))
    }
}

impl Unpackable for u32 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as u32)),
            Format::UINT8 => take_byte(&mut buf).map(|v| (2, v as u32)),
            Format::UINT16 => take_num(&mut buf, u16::from_be_bytes).map(|v| (3, v as u32)),
            Format::UINT32 => take_num(&mut buf, u32::from_be_bytes).map(|v| (5, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as u32)),
            Format::UINT8 => take_byte_iter(bytes).map(|v| (2, v as u32)),
            Format::UINT16 => take_num_iter(bytes, u16::from_be_bytes).map(|v| (3, v as u32)),
            Format::UINT32 => take_num_iter(bytes, u32::from_be_bytes).map(|v| (5, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for Option<NonZeroU32> {
    type Error = Error;

    fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        u32::unpack(buf).map(|(s, v)| (s, NonZeroU32::new(v)))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        u32::unpack_iter(bytes).map(|(s, v)| (s, NonZeroU32::new(v)))
    }
}

impl Unpackable for u64 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as u64)),
            Format::UINT8 => take_byte(&mut buf).map(|v| (2, v as u64)),
            Format::UINT16 => take_num(&mut buf, u16::from_be_bytes).map(|v| (3, v as u64)),
            Format::UINT32 => take_num(&mut buf, u32::from_be_bytes).map(|v| (5, v as u64)),
            Format::UINT64 => take_num(&mut buf, u64::from_be_bytes).map(|v| (9, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as u64)),
            Format::UINT8 => take_byte_iter(bytes).map(|v| (2, v as u64)),
            Format::UINT16 => take_num_iter(bytes, u16::from_be_bytes).map(|v| (3, v as u64)),
            Format::UINT32 => take_num_iter(bytes, u32::from_be_bytes).map(|v| (5, v as u64)),
            Format::UINT64 => take_num_iter(bytes, u64::from_be_bytes).map(|v| (9, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for Option<NonZeroU64> {
    type Error = Error;

    fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        u64::unpack(buf).map(|(s, v)| (s, NonZeroU64::new(v)))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        u64::unpack_iter(bytes).map(|(s, v)| (s, NonZeroU64::new(v)))
    }
}

impl Unpackable for u128 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as u128)),
            Format::UINT8 => take_byte(&mut buf).map(|v| (2, v as u128)),
            Format::UINT16 => take_num(&mut buf, u16::from_be_bytes).map(|v| (3, v as u128)),
            Format::UINT32 => take_num(&mut buf, u32::from_be_bytes).map(|v| (5, v as u128)),
            Format::UINT64 => take_num(&mut buf, u64::from_be_bytes).map(|v| (9, v as u128)),
            Format::BIN8 => {
                if take_byte(&mut buf)? != 16 {
                    return Err(Error::UnexpectedBinLength);
                }
                take_num(&mut buf, u128::from_be_bytes).map(|v| (18, v))
            }
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as u128)),
            Format::UINT8 => take_byte_iter(bytes).map(|v| (2, v as u128)),
            Format::UINT16 => take_num_iter(bytes, u16::from_be_bytes).map(|v| (3, v as u128)),
            Format::UINT32 => take_num_iter(bytes, u32::from_be_bytes).map(|v| (5, v as u128)),
            Format::UINT64 => take_num_iter(bytes, u64::from_be_bytes).map(|v| (9, v as u128)),
            Format::BIN8 => {
                if take_byte_iter(bytes.by_ref())? != 16 {
                    return Err(Error::UnexpectedBinLength);
                }
                take_num_iter(bytes, u128::from_be_bytes).map(|v| (18, v))
            }
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for Option<NonZeroU128> {
    type Error = Error;

    fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        u128::unpack(buf).map(|(s, v)| (s, NonZeroU128::new(v)))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        u128::unpack_iter(bytes).map(|(s, v)| (s, NonZeroU128::new(v)))
    }
}

impl Unpackable for usize {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as usize)),
            Format::UINT8 => take_byte(&mut buf).map(|v| (2, v as usize)),
            Format::UINT16 => take_num(&mut buf, u16::from_be_bytes).map(|v| (3, v as usize)),
            Format::UINT32 => take_num(&mut buf, u32::from_be_bytes).map(|v| (5, v as usize)),
            Format::UINT64 => take_num(&mut buf, u64::from_be_bytes).map(|v| (9, v as usize)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as usize)),
            Format::UINT8 => take_byte_iter(bytes).map(|v| (2, v as usize)),
            Format::UINT16 => take_num_iter(bytes, u16::from_be_bytes).map(|v| (3, v as usize)),
            Format::UINT32 => take_num_iter(bytes, u32::from_be_bytes).map(|v| (5, v as usize)),
            Format::UINT64 => take_num_iter(bytes, usize::from_be_bytes).map(|v| (9, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for Option<NonZeroUsize> {
    type Error = Error;

    fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        usize::unpack(buf).map(|(s, v)| (s, NonZeroUsize::new(v)))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        usize::unpack_iter(bytes).map(|(s, v)| (s, NonZeroUsize::new(v)))
    }
}

impl Unpackable for i8 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as i8)),
            0xe0.. => Ok((1, format as i8)),
            Format::INT8 => take_byte(&mut buf).map(|v| (2, v as i8)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as i8)),
            0xe0.. => Ok((1, format as i8)),
            Format::INT8 => take_byte_iter(bytes).map(|v| (2, v as i8)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for Option<NonZeroI8> {
    type Error = Error;

    fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        i8::unpack(buf).map(|(s, v)| (s, NonZeroI8::new(v)))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        i8::unpack_iter(bytes).map(|(s, v)| (s, NonZeroI8::new(v)))
    }
}

impl Unpackable for i16 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, (format as i8) as i16)),
            0xe0.. => Ok((1, (format as i8) as i16)),
            Format::INT8 => take_byte(&mut buf).map(|v| (2, v as i8 as i16)),
            Format::INT16 => take_num(&mut buf, i16::from_be_bytes).map(|v| (3, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, (format as i8) as i16)),
            0xe0.. => Ok((1, (format as i8) as i16)),
            Format::INT8 => take_byte_iter(bytes).map(|v| (2, v as i8 as i16)),
            Format::INT16 => take_num_iter(bytes, i16::from_be_bytes).map(|v| (3, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for Option<NonZeroI16> {
    type Error = Error;

    fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        i16::unpack(buf).map(|(s, v)| (s, NonZeroI16::new(v)))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        i16::unpack_iter(bytes).map(|(s, v)| (s, NonZeroI16::new(v)))
    }
}

impl Unpackable for i32 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, (format as i8) as i32)),
            0xe0.. => Ok((1, (format as i8) as i32)),
            Format::INT8 => take_byte(&mut buf).map(|v| (2, v as i8 as i32)),
            Format::INT16 => take_num(&mut buf, i16::from_be_bytes).map(|v| (3, v as i32)),
            Format::INT32 => take_num(&mut buf, i32::from_be_bytes).map(|v| (5, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, (format as i8) as i32)),
            0xe0.. => Ok((1, (format as i8) as i32)),
            Format::INT8 => take_byte_iter(bytes).map(|v| (2, v as i8 as i32)),
            Format::INT16 => take_num_iter(bytes, i16::from_be_bytes).map(|v| (3, v as i32)),
            Format::INT32 => take_num_iter(bytes, i32::from_be_bytes).map(|v| (5, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for Option<NonZeroI32> {
    type Error = Error;

    fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        i32::unpack(buf).map(|(s, v)| (s, NonZeroI32::new(v)))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        i32::unpack_iter(bytes).map(|(s, v)| (s, NonZeroI32::new(v)))
    }
}

impl Unpackable for i64 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, (format as i8) as i64)),
            0xe0.. => Ok((1, (format as i8) as i64)),
            Format::INT8 => take_byte(&mut buf).map(|v| (2, v as i8 as i64)),
            Format::INT16 => take_num(&mut buf, i16::from_be_bytes).map(|v| (3, v as i64)),
            Format::INT32 => take_num(&mut buf, i32::from_be_bytes).map(|v| (5, v as i64)),
            Format::INT64 => take_num(&mut buf, i64::from_be_bytes).map(|v| (9, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, (format as i8) as i64)),
            0xe0.. => Ok((1, (format as i8) as i64)),
            Format::INT8 => take_byte_iter(bytes).map(|v| (2, v as i8 as i64)),
            Format::INT16 => take_num_iter(bytes, i16::from_be_bytes).map(|v| (3, v as i64)),
            Format::INT32 => take_num_iter(bytes, i32::from_be_bytes).map(|v| (5, v as i64)),
            Format::INT64 => take_num_iter(bytes, i64::from_be_bytes).map(|v| (9, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for Option<NonZeroI64> {
    type Error = Error;

    fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        i64::unpack(buf).map(|(s, v)| (s, NonZeroI64::new(v)))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        i64::unpack_iter(bytes).map(|(s, v)| (s, NonZeroI64::new(v)))
    }
}

impl Unpackable for i128 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, (format as i8) as i128)),
            0xe0.. => Ok((1, (format as i8) as i128)),
            Format::INT8 => take_byte(&mut buf).map(|v| (2, v as i8 as i128)),
            Format::INT16 => take_num(&mut buf, i16::from_be_bytes).map(|v| (3, v as i128)),
            Format::INT32 => take_num(&mut buf, i32::from_be_bytes).map(|v| (5, v as i128)),
            Format::INT64 => take_num(&mut buf, i64::from_be_bytes).map(|v| (9, v as i128)),
            Format::BIN8 => {
                if take_byte(&mut buf)? != 16 {
                    return Err(Error::UnexpectedBinLength);
                }
                take_num(&mut buf, i128::from_be_bytes).map(|v| (18, v))
            }
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, (format as i8) as i128)),
            0xe0.. => Ok((1, (format as i8) as i128)),
            Format::INT8 => take_byte_iter(bytes).map(|v| (2, v as i8 as i128)),
            Format::INT16 => take_num_iter(bytes, i16::from_be_bytes).map(|v| (3, v as i128)),
            Format::INT32 => take_num_iter(bytes, i32::from_be_bytes).map(|v| (5, v as i128)),
            Format::INT64 => take_num_iter(bytes, i64::from_be_bytes).map(|v| (9, v as i128)),
            Format::BIN8 => {
                if take_byte_iter(bytes.by_ref())? != 16 {
                    return Err(Error::UnexpectedBinLength);
                }
                take_num_iter(bytes, i128::from_be_bytes).map(|v| (18, v))
            }
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for Option<NonZeroI128> {
    type Error = Error;

    fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        i128::unpack(buf).map(|(s, v)| (s, NonZeroI128::new(v)))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        i128::unpack_iter(bytes).map(|(s, v)| (s, NonZeroI128::new(v)))
    }
}

impl Unpackable for isize {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, (format as i8) as isize)),
            0xe0.. => Ok((1, (format as i8) as isize)),
            Format::INT8 => take_byte(&mut buf).map(|v| (2, v as i8 as isize)),
            Format::INT16 => take_num(&mut buf, i16::from_be_bytes).map(|v| (3, v as isize)),
            Format::INT32 => take_num(&mut buf, i32::from_be_bytes).map(|v| (5, v as isize)),
            Format::INT64 => take_num(&mut buf, i64::from_be_bytes).map(|v| (9, v as isize)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, (format as i8) as isize)),
            0xe0.. => Ok((1, (format as i8) as isize)),
            Format::INT8 => take_byte_iter(bytes).map(|v| (2, v as i8 as isize)),
            Format::INT16 => take_num_iter(bytes, i16::from_be_bytes).map(|v| (3, v as isize)),
            Format::INT32 => take_num_iter(bytes, i32::from_be_bytes).map(|v| (5, v as isize)),
            Format::INT64 => take_num_iter(bytes, i64::from_be_bytes).map(|v| (9, v as isize)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for Option<NonZeroIsize> {
    type Error = Error;

    fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        isize::unpack(buf).map(|(s, v)| (s, NonZeroIsize::new(v)))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        isize::unpack_iter(bytes).map(|(s, v)| (s, NonZeroIsize::new(v)))
    }
}
