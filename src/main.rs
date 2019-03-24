#[macro_use]
extern crate nom;
extern crate failure;

mod packet;
mod wrapper;
mod fletcher_16;
mod ser;
mod de;
mod cast;

use packet::*;
use wrapper::*;
use ser::Serialize;
use std::io::Cursor;

// Full network packet layout
// u16 are network endianess (big endian)
// ------------------------------------------------------------------
// u16 - total length of message including all headers and payload
// u16 - wrapper seq number
// u16 - wrapper ack number
// u16 - wrapper checksum of packet and payload
// u8  - wrapper type
// u16 - packet payload length
// u16 - packet sequence number
// u16 - packet command
// u16 - packet handle
// Nu8 - payload data
// -----------------------------------------------------------------

pub fn main() {
    let payload = &[1u8, 2, 3, 4, 5];
    let packet = Packet {
        sequence_number: packet::SequenceNumber(42),
        command: Command::EchoReply,
        handle: Handle(43),
        data: payload
    };
    let wrapper = Wrapper {
        sequence_number: wrapper::SequenceNumber(44),
        ack_number: AckNumber(45),
        type_: WrapperType::DATA,
        packet: packet
    };
    let buf: &mut [u8] = &mut [0u8; 64];
    let mut writer = Cursor::new(buf);
    wrapper.write_net_bytes(&mut writer).unwrap();
    let bytes = &writer.get_ref()[0..writer.position() as usize];
    println!("{:?}", &bytes);
    let p = de::parse_wrapper(&bytes);
    println!("{:#?}", &p);
}