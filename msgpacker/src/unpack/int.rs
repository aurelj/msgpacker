use core::num::{NonZero, ZeroablePrimitive};

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

impl<X> Unpackable for Option<NonZero<X>>
where
    X: Unpackable + ZeroablePrimitive,
{
    type Error = <X as Unpackable>::Error;

    fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        X::unpack(buf).map(|(s, v)| (s, NonZero::new(v)))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        X::unpack_iter(bytes).map(|(s, v)| (s, NonZero::new(v)))
    }
}
