use std::mem;
use fletcher_16::fletcher_16;

#[derive(Debug, Clone, Copy, PartialEq, Eq,)]
pub struct BufferTooShortError {
    expected: usize,
    actual: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandValueError(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Command {
    Invalid = 0,
    GeneralErrorReply = 1,
    Echo = 2,
    EchoReply = 3,
    Shutdown = 4,
    ShutdownReply = 5,
    Unsupported = 6,
}

impl Command {
    const MAX_COMMAND: u16 = Command::Unsupported as u16;

    fn try_from(c: u16) -> Result<Self, CommandValueError> {
        if c <= Self::MAX_COMMAND {
            Ok(unsafe { mem::transmute(c) })
        } else {
            Err(CommandValueError(c))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Packet<'a> {
    pub sequence_number: u16,
    pub command: Command,
    pub handle: u16,
    pub data: &'a [u8],
}

impl<'a> Packet<'a> {
    const HEADER_LEN: usize = 4 * mem::size_of::<u16>();

    pub fn to_net_bytes_buf<'b>(&self, buf: &'b mut [u8]) -> Result<&'b mut [u8], BufferTooShortError> {
        let total_len = Self::HEADER_LEN + self.data.len();
        if buf.len() < total_len {
            return Err(BufferTooShortError {
                expected: total_len,
                actual: buf.len(),
            });
        }
        let buf = &mut buf[..total_len];
        // copy header over
        {
            let header_buf = unsafe {
                mem::transmute::<_, &mut [u16]>(&mut buf[..Self::HEADER_LEN])
            };
            header_buf[0] = (self.data.len() as u16).to_be();
            header_buf[1] = self.sequence_number.to_be();
            header_buf[2] = (self.command as u16).to_be();
            header_buf[3] = self.handle.to_be();
        }
        // copy over data
        buf[Self::HEADER_LEN..].copy_from_slice(self.data);
        Ok(buf)
    }
}

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

impl<'a> Wrapper<'a> {
    const HEADER_LEN: usize = 4 * mem::size_of::<u16>() + mem::size_of::<u8>();
    const U16_HEADER_LEN: usize = Self::HEADER_LEN - mem::size_of::<u8>();

    pub fn to_net_bytes_buf<'b>(&self, buf: &'b mut [u8]) -> Result<&'b mut [u8], BufferTooShortError> {
        let total_len = Self::HEADER_LEN + Packet::HEADER_LEN + self.packet.data.len();
        if buf.len() < total_len {
            return Err(BufferTooShortError {
                expected: total_len,
                actual: buf.len(),
            });
        }
        let buf = &mut buf[..total_len];
        // copy header over
        {
            let u16_header_buf = unsafe {
                mem::transmute::<_, &mut [u16]>(&mut buf[..Self::U16_HEADER_LEN])
            };
            u16_header_buf[0] = (total_len as u16).to_be();
            u16_header_buf[1] = self.seqence_number.to_be();
            u16_header_buf[2] = self.ack_number.to_be();
            u16_header_buf[3] = self.checksum().to_be();
        }
        // copy that single header byte over
        buf[Self::U16_HEADER_LEN] = self.type_ as u8;
        // copy payload over
        self.packet.to_net_bytes_buf(&mut buf[Self::HEADER_LEN..]).unwrap(); // len checked before
        Ok(buf)
    }

    fn checksum(&self) -> u16 {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std;

    #[test]
    fn command_is_sane() {
        for i in 0..Command::MAX_COMMAND {
            let c = Command::try_from(i);
            assert!(c.is_ok());
            let c = c.unwrap();
            assert_eq!(c as u16, i);
        }
    }

    #[test]
    fn command_ctor_fails() {
        let c = Command::try_from(Command::MAX_COMMAND + 1);
        assert!(c.is_err());
        let c = Command::try_from(std::u16::MAX);
        assert!(c.is_err());
    }

    #[test]
    fn packet_to_net_bytes_fits() {
        let p = Packet {
            sequence_number: 1,
            command: Command::Unsupported,
            handle: 1,
            data: &[]
        };
        let mut b = [0; 8];
        let x = p.to_net_bytes_buf(&mut b);
        assert!(x.is_ok());
    }

    #[test]
    fn packet_to_net_bytes_too_short() {
        let p = Packet {
            sequence_number: 1,
            command: Command::Unsupported,
            handle: 1,
            data: &[]
        };
        let mut b = [0; 2];
        let x = p.to_net_bytes_buf(&mut b);
        assert!(x.is_err());
    }

    #[test]
    fn packet_to_net_bytes_correct() {
        let p = Packet {
            sequence_number: 1,
            command: Command::Unsupported,
            handle: 2,
            data: &[40, 41, 42, 43]
        };
        let mut b = [0; 16];
        let x = p.to_net_bytes_buf(&mut b);
        assert!(x.is_ok());
        let x = x.unwrap();
        assert_eq!(x, &[0, 4, 0, 1, 0, 6, 0, 2, 40, 41, 42, 43]);
    }

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
