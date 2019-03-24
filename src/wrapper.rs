use std::mem;
use packet::Packet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WrapperType {
    ACK     = 0,
    NACK    = 1,
    DATA    = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SequenceNumber(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AckNumber(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Wrapper<'a, 'b: 'a> {
    pub sequence_number: SequenceNumber,
    pub ack_number: AckNumber,
    pub type_: WrapperType,
    pub packet: &'a Packet<'b>,
}

impl<'a, 'b: 'a> Wrapper<'a, 'b> {
    const HEADER_LEN: usize = 4 * mem::size_of::<u16>() + mem::size_of::<u8>();
    pub fn total_len(&self) -> usize {
        Self::HEADER_LEN + self.packet.total_len()
    }
}