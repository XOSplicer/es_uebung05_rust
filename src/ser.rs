use std::io;
use std::io::Write;
use packet;
use wrapper;
use cast::cast_slice_to_bytes;

pub trait Serialize {
    fn write_net_bytes<W: Write>(&self, writer: &mut W) -> io::Result<()>;
}

fn write_net_bytes_u16<W: Write>(value: u16, writer: &mut W) -> io::Result<()> {
        let v = value.to_be();
        let buf: &[u16] = &[v];
        let buf: &[u8] = unsafe {
            cast_slice_to_bytes(buf)
        };
        writer.write_all(buf)
}

fn write_net_bytes_u8<W: Write>(value: u8, writer: &mut W) -> io::Result<()> {
        let buf: &[u8] = &[value];
        writer.write_all(buf)
}

impl<'a> Serialize for packet::Packet<'a> {
    fn write_net_bytes<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        write_net_bytes_u16(self.data.len() as u16, writer)?;
        write_net_bytes_u16(self.sequence_number.0, writer)?;
        write_net_bytes_u16(self.command as u16, writer)?;
        write_net_bytes_u16(self.handle.0, writer)?;
        writer.write(self.data)?;
        Ok(())
    }
}

impl<'a> Serialize for wrapper::Wrapper<'a> {
    fn write_net_bytes<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        write_net_bytes_u16(self.total_len() as u16, writer)?;
        write_net_bytes_u16(self.sequence_number.0 as u16, writer)?;
        write_net_bytes_u16(self.ack_number.0 as u16, writer)?;
        write_net_bytes_u16(self.packet.checksum().0 , writer)?;
        write_net_bytes_u8(self.type_ as u8, writer)?;
        self.packet.write_net_bytes(writer)?;
        Ok(())
    }
}



