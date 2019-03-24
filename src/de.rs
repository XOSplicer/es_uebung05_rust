use std::mem;
use packet;
use wrapper;
use nom::{be_u16, be_u8};

named!(packet_command<&[u8], packet::Command>,
    map_res!(be_u16, packet::Command::try_from)
);

named!(packet_sequence_number<&[u8], packet::SequenceNumber>,
    do_parse!( i: be_u16 >> (packet::SequenceNumber(i)))
);

named!(packet_handle<&[u8], packet::Handle>,
    do_parse!( i: be_u16 >> (packet::Handle(i)))
);

named!(packet_checksum<&[u8], packet::Checksum>,
    do_parse!( i: be_u16 >> (packet::Checksum(i)))
);

named!(pub parse_packet<&[u8], packet::Packet>,
    do_parse!(
        data_len: be_u16 >>
        sequence_number: packet_sequence_number >>
        command: packet_command >>
        handle: packet_handle >>
        data: take!(data_len) >>
        (packet::Packet {
            sequence_number,
            command,
            handle,
            data
        })
    )
);

named!(wrapper_wrapper_type<&[u8], wrapper::WrapperType>,
    map_res!(be_u8, wrapper::WrapperType::try_from)
);

named!(wrapper_sequence_number<&[u8], wrapper::SequenceNumber>,
    do_parse!( i: be_u16 >> (wrapper::SequenceNumber(i)))
);

named!(wrapper_ack_number<&[u8], wrapper::AckNumber>,
    do_parse!( i: be_u16 >> (wrapper::AckNumber(i)))
);

named!(pub parse_wrapper<&[u8], wrapper::Wrapper>,
    length_value!(
        map!(be_u16, |l| l - mem::size_of::<u16>() as u16),
        map_res!(
            do_parse!(
                sequence_number: wrapper_sequence_number >>
                ack_number: wrapper_ack_number >>
                checksum: packet_checksum >>
                type_: wrapper_wrapper_type >>
                packet: parse_packet >>
                (wrapper::Wrapper {
                    sequence_number,
                    ack_number,
                    type_,
                    packet
                }, checksum)
            ),
            check_checksum
        )
    )
);


fn check_checksum(arg: (wrapper::Wrapper, packet::Checksum))
    -> Result<wrapper::Wrapper, wrapper::ChecksumMismatchError>
{
    let provided: packet::Checksum = arg.1;
    let w: wrapper::Wrapper = arg.0;
    let computed: packet::Checksum = w.packet.checksum();
    if provided == computed {
        Ok(w)
    } else {
        Err(wrapper::ChecksumMismatchError {
            provided,
            computed,
        })
    }
}


