use std::mem;
use packet::{Checksum, Packet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WrapperType {
    ACK     = 0,
    NACK    = 1,
    DATA    = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WrapperTypeValueError(pub u8);

impl WrapperType {
    const MAX_TYPE: u8 = WrapperType::DATA as u8;
    pub fn try_from(c: u8) -> Result<Self, WrapperTypeValueError> {
        if c <= Self::MAX_TYPE {
            Ok(unsafe { mem::transmute(c) })
        } else {
            Err(WrapperTypeValueError(c))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SequenceNumber(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AckNumber(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Wrapper<'a> {
    pub sequence_number: SequenceNumber,
    pub ack_number: AckNumber,
    pub type_: WrapperType,
    pub packet: Packet<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChecksumMismatchError {
    pub provided: Checksum,
    pub computed: Checksum,
}

impl<'a> Wrapper<'a> {
    pub const HEADER_LEN: usize = 4 * mem::size_of::<u16>() + mem::size_of::<u8>();
    pub fn total_len(&self) -> usize {
        Self::HEADER_LEN + self.packet.total_len()
    }
}