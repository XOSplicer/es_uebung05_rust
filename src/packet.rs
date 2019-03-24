use std::mem;
use fletcher_16::Fletcher16Hasher;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandValueError(pub u16);

impl Command {
    const MAX_COMMAND: u16 = Command::Unsupported as u16;
    pub fn try_from(c: u16) -> Result<Self, CommandValueError> {
        if c <= Self::MAX_COMMAND {
            Ok(unsafe { mem::transmute(c) })
        } else {
            Err(CommandValueError(c))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SequenceNumber(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Handle(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Checksum(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Packet<'a> {
    pub sequence_number: SequenceNumber,
    pub command: Command,
    pub handle: Handle,
    pub data: &'a [u8],
}


impl<'a> Packet<'a> {
    pub const HEADER_LEN: usize = 4 * mem::size_of::<u16>();
    pub fn total_len(&self) -> usize {
        Self::HEADER_LEN + self.data.len()
    }

    pub fn checksum(&self) -> Checksum {
        let mut hasher = Fletcher16Hasher::new();
        hasher.write_u16_platform(self.data.len() as u16);
        hasher.write_u16_platform(self.sequence_number.0);
        hasher.write_u16_platform(self.command as u16);
        hasher.write_u16_platform(self.handle.0);
        Checksum(hasher.finish())
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use std;

//     #[test]
//     fn command_is_sane() {
//         for i in 0..Command::MAX_COMMAND {
//             let c = Command::try_from(i);
//             assert!(c.is_ok());
//             let c = c.unwrap();
//             assert_eq!(c as u16, i);
//         }
//     }

//     #[test]
//     fn command_ctor_fails() {
//         let c = Command::try_from(Command::MAX_COMMAND + 1);
//         assert!(c.is_err());
//         let c = Command::try_from(std::u16::MAX);
//         assert!(c.is_err());
//     }

//     #[test]
//     fn packet_to_net_bytes_fits() {
//         let p = Packet {
//             sequence_number: 1,
//             command: Command::Unsupported,
//             handle: 1,
//             data: &[]
//         };
//         let mut b = [0; 8];
//         let x = p.to_net_bytes_buf(&mut b);
//         assert!(x.is_ok());
//     }

//     #[test]
//     fn packet_to_net_bytes_too_short() {
//         let p = Packet {
//             sequence_number: 1,
//             command: Command::Unsupported,
//             handle: 1,
//             data: &[]
//         };
//         let mut b = [0; 2];
//         let x = p.to_net_bytes_buf(&mut b);
//         assert!(x.is_err());
//     }

//     #[test]
//     fn packet_to_net_bytes_correct() {
//         let p = Packet {
//             sequence_number: 1,
//             command: Command::Unsupported,
//             handle: 2,
//             data: &[40, 41, 42, 43]
//         };
//         let mut b = [0; 16];
//         let x = p.to_net_bytes_buf(&mut b);
//         assert!(x.is_ok());
//         let x = x.unwrap();
//         assert_eq!(x, &[0, 4, 0, 1, 0, 6, 0, 2, 40, 41, 42, 43]);
//     }

//     #[test]
//     fn wrapper_type_is_sane() {
//         for i in 0..WrapperType::MAX_TYPE {
//             let c = WrapperType::try_from(i);
//             assert!(c.is_ok());
//             let c = c.unwrap();
//             assert_eq!(c as u8, i);
//         }
//     }

//     #[test]
//     fn wrapper_type_ctor_fails() {
//         let c = WrapperType::try_from(WrapperType::MAX_TYPE + 1);
//         assert!(c.is_err());
//         let c = WrapperType::try_from(std::u8::MAX);
//         assert!(c.is_err());
//     }

// }
