use std::mem;
use super::BufferTooShortError;
use packet::*;
use fletcher_16::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WrapperTypeValueError(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WrapperType {
    ACK     = 0,
    NACK    = 1,
    DATA    = 2,
}

impl WrapperType {
    const MAX_TYPE: u8 = WrapperType::DATA as u8;
    fn try_from(c: u8) -> Result<Self, WrapperTypeValueError> {
        if c <= Self::MAX_TYPE {
            Ok(unsafe { mem::transmute(c) })
        } else {
            Err(WrapperTypeValueError(c))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Wrapper<'a> {
    seqence_number: u16,
    ack_number: u16,
    type_: WrapperType,
    packet: Packet<'a>,
}

#[cfg(test)]
mod test {
    use super::*;
    use std;

    #[test]
    fn wrapper_type_is_sane() {
        for i in 0..WrapperType::MAX_TYPE {
            let c = WrapperType::try_from(i);
            assert!(c.is_ok());
            let c = c.unwrap();
            assert_eq!(c as u8, i);
        }
    }

    #[test]
    fn wrapper_type_ctor_fails() {
        let c = WrapperType::try_from(WrapperType::MAX_TYPE + 1);
        assert!(c.is_err());
        let c = WrapperType::try_from(std::u8::MAX);
        assert!(c.is_err());
    }
}