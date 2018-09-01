use std::mem;
use std::slice;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct CommandValueError(u16);

#[derive(Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Command {
      Invalid           = 0,
      GeneralErrorReply = 1,
      Echo              = 2,
      EchoReply         = 3,
      Shutdown          = 4,
      ShutdownReply     = 5,
      Unsupported       = 6
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

#[derive(Debug, PartialEq, Eq)]
pub enum ConstructPacketError {
    WrongDataLength,
    CommandValue(CommandValueError),
}

impl From<CommandValueError> for ConstructPacketError {
    fn from(error: CommandValueError) -> Self {
        ConstructPacketError::CommandValue(error)
    }
}

#[repr(C)]
#[repr(packed)]
pub struct Packet {
    data_length: u16,
    sequence_number: u16,
    command: u16,
    handle: u16,
    data: [u8],
}

impl Packet {

    pub const HEADER_SIZE: usize = 4 * mem::size_of::<u16>();

    pub fn try_from_slice<'a>(buf: &'a [u8])
        -> Result<&'a Self, ConstructPacketError>
    {
        unsafe {
            let t =  mem::transmute::<&[u8], &Self>(buf);
            Command::try_from(t.command)?;

            if buf.len() >= Self::HEADER_SIZE + t.data_length() {
                Ok(t)
            } else {
                Err(ConstructPacketError::WrongDataLength)
            }
        }
    }

    pub fn try_from_slice_mut<'a>(buf: &'a mut [u8])
        -> Result<&'a mut Self, ConstructPacketError>
    {
        unsafe {
            let t = mem::transmute::<&mut [u8], &mut Self>(buf);
            Command::try_from(t.command)?;

            if buf.len() >= Self::HEADER_SIZE + t.data_length() {
                Ok(t)
            } else {
                Err(ConstructPacketError::WrongDataLength)
            }
        }
    }

    pub fn try_copy_into_slice<'a>(
        buf: &'a mut [u8],
        sequence_number: u16,
        command: Command,
        handle: u16,
        data: &[u8] )
        ->  Result<&'a Self, ConstructPacketError>
    {
        Ok(Self::try_copy_into_slice_mut(
            buf, sequence_number, command, handle, data
        )?)
    }

    pub fn try_copy_into_slice_mut<'a>(
        buf: &'a mut [u8],
        sequence_number: u16,
        command: Command,
        handle: u16,
        data: &[u8] )
        ->  Result<&'a mut Self, ConstructPacketError>
    {
        if  buf.len() < Packet::HEADER_SIZE + data.len() {
            return Err(ConstructPacketError::WrongDataLength);
        }
        // safe since bounds are checked
        let t = unsafe { mem::transmute::<&mut [u8], &mut Self>(buf) };
        t.data_length = data.len() as u16;
        t.sequence_number = sequence_number;
        t.command = command as u16;
        t.handle = handle;
        t.data_as_slice_mut().copy_from_slice(data);
        Ok(t)
    }

    pub fn data_length(&self) -> usize {
        self.data_length as usize
    }

    pub fn sequence_number(&self) -> u16 {
        self.sequence_number
    }

    pub fn command(&self) -> Command {
        Command::try_from(self.command).unwrap()
    }

    pub fn handle(&self) -> u16 {
        self.handle
    }

    pub fn data_as_slice<'a>(&'a self) -> &'a [u8] {
        unsafe {
            slice::from_raw_parts(self.data.as_ptr(), self.data_length())
        }
    }

    pub fn data_as_slice_mut<'a>(&'a mut self) -> &'a mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self.data.as_mut_ptr(), self.data_length())
        }
    }

    fn as_slice<'a>(&'a self) -> &'a [u8] {
        unsafe {
            let len = Self::HEADER_SIZE + self.data_length();
            slice::from_raw_parts(mem::transmute(&self), len)
        }
    }
}

impl fmt::Debug for Packet {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("TLV")
            .field("data_length", &self.data_length())
            .field("sequence_number", &self.sequence_number())
            .field("command", &self.command())
            .field("handle", &self.handle())
            .field("data", &self.data_as_slice())
            .finish()
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for Packet {}

pub struct NetPacket<'a>(&'a mut Packet);

impl<'a> NetPacket<'a> {
    pub fn new(p: &'a mut Packet) -> Self {
        p.data_length = p.data_length.to_be();
        p.sequence_number = p.sequence_number.to_be();
        p.command = p.command.to_be();
        p.handle = p.handle.to_be();
        NetPacket(p)
    }

    pub fn as_slice<'b>(&'b self) -> &'b [u8] {
        self.0.as_slice()
    }
}

impl<'a> Drop for NetPacket<'a> {
    fn drop(&mut self) {
        self.0.data_length = u16::from_be(self.0.data_length);
        self.0.sequence_number = u16::from_be(self.0.sequence_number);
        self.0.command = u16::from_be(self.0.command);
        self.0.handle = u16::from_be(self.0.handle);
    }
}



#[cfg(test)]
mod test {
    use std;
    use super::*;

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
    fn packet_from_slice_is_ok() {
        let buf = &mut [0_u8; 16];
        let len: usize = 4;
        buf[0] = len as u8;
        let p = Packet::try_from_slice(buf);
        assert!(p.is_ok());
        let p: &Packet = p.unwrap();
        assert_eq!(p.data_length(), len);
        assert_eq!(p.data_as_slice().len(), len);
    }

    #[test]
    fn packet_from_slice_is_err() {
        let buf = &mut [0_u8; 16];
        let len: usize = 16;
        buf[0] = len as u8;
        let p = Packet::try_from_slice(buf);
        assert!(p.is_err());
    }

    #[test]
    fn packet_from_slice_is_eq_mut() {
        let buf1 = &mut [0_u8; 16];
        let buf2 = &mut [0_u8; 16];
        let len: usize = 4;
        buf1[0] = len as u8;
        buf2.copy_from_slice(buf1);
        let p1: &Packet = Packet::try_from_slice(buf1).unwrap();
        let p2: &mut Packet = Packet::try_from_slice_mut(buf2).unwrap();
        assert_eq!(p1, p2)
    }

}
