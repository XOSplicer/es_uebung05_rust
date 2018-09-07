use std::fmt;
use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq,)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq,)]
pub struct BufferTooShortError {
    expected: usize,
    actual: usize
}

#[derive(Debug, PartialEq, Eq)]
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
    fn to_bytes_fits() {
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
    fn to_bytes_too_short() {
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

}
