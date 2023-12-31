use netlink_packet_core::{
    NLM_F_REQUEST, NLM_F_DUMP,
    NetlinkHeader,
    NetlinkMessage,
    NetlinkPayload,
};
use netlink_packet_route::{RtnlMessage, TcMessage};
use netlink_sys::{protocols::NETLINK_ROUTE, Socket, SocketAddr};

use crate::errors::TcError;

pub fn netlink() -> Result<Vec<TcMessage>, TcError> {
    let socket = Socket::new(NETLINK_ROUTE)
        .map_err(|err| TcError::Socket(Box::new(err)))?;
    socket
        .connect(&SocketAddr::new(0, 0))
        .map_err(|err| TcError::Socket(Box::new(err)))?;

    let mut nl_hdr = NetlinkHeader::default();
    nl_hdr.flags = NLM_F_REQUEST | NLM_F_DUMP;

    let mut packet = NetlinkMessage::new(
        nl_hdr,
        NetlinkPayload::from(RtnlMessage::GetQueueDiscipline(TcMessage::default())),
    );
    packet.finalize();

    let mut buf = vec![0; packet.header.length as usize];
    packet.serialize(&mut buf[..]);

    if let Err(e) =  socket.send(&buf[..], 0) {
        return Err(TcError::Send(e.to_string()).into());
    }

    let mut receive_buffer = vec![0; 4096];
    let mut offset = 0;

    let mut tc_messages = Vec::new();

    while let Ok(size) = socket.recv(&mut &mut receive_buffer[..], 0) {
        loop {
            let bytes = &receive_buffer[offset..];
            let rx_packet =
                <NetlinkMessage<RtnlMessage>>::deserialize(bytes).unwrap();
            // // println!("<<< {rx_packet:?}");

            let payload = rx_packet.payload;
            match payload {
                NetlinkPayload::InnerMessage(message) => {
                    match message {
                        RtnlMessage::NewQueueDiscipline(message) => {
                            tc_messages.push(message.clone());
                        },
                        _ => {},
                    }

                },
                NetlinkPayload::Error(error) => return Err(TcError::Netlink(error.to_string()).into()),
                NetlinkPayload::Done(_) => return Ok(tc_messages),
                _ => {},
            }

            offset += rx_packet.header.length as usize;
            if offset == size || rx_packet.header.length == 0 {
                offset = 0;
                break;
            }
        }
    }

    Ok(tc_messages)
}
