use neli::{
    socket::NlSocketHandle,
    consts::{socket::NlFamily, nl::{NlmFFlags, NlmF, Nlmsg},
    genl::Index},
    nl::{NlPayload, Nlmsghdr},
    types::{GenlBuffer, Buffer},
    genl::Genlmsghdr, err::NlError
};

use crate::common::{ETHTOOL_GENL_VERSION, ETHTOOL_GENL_NAME, ETHTOOL_MSG_STATS_GET};

pub fn connect() {
    let mut sock = NlSocketHandle::connect(
        NlFamily::Generic,
        None,
        &[],
    ).expect("failed to connect!");
    // println!("Connected successfully!");

    // generic message
    let attrs: GenlBuffer<Index, Buffer> = GenlBuffer::new();
    let genlhdr = Genlmsghdr::new(ETHTOOL_GENL_NAME, ETHTOOL_GENL_VERSION, attrs);

    let nlhdr = {
        let len = None;
        let nl_type = ETHTOOL_MSG_STATS_GET;
        let flags = NlmFFlags::new(&[NlmF::Request, NlmF::Dump]);
        let seq = None;
        let pid = None;
        let payload = NlPayload::Payload(genlhdr);

        Nlmsghdr::new(len, nl_type, flags, seq, pid, payload)
    };

    sock.send(nlhdr).expect("failed to send msg!");

    match  sock.recv::<Nlmsg, Genlmsghdr<u8, Index>>() {
        Ok(response) => handle_response(response),
        Err(err) => handle_error(err),
    };
}

fn handle_response(response: Option<Nlmsghdr<Nlmsg, Genlmsghdr<u8, Index>>>) {
    if let Some(msg) = response {
        if let Some(payload) = msg.nl_payload.get_payload() {
            println!("Yay, received msg: {}", payload.cmd);
            return;
        }
    }
    println!("Received an empty msg");
}

fn handle_error(err: NlError<Nlmsg, Genlmsghdr<u8, Index>>) {
    println!("Received error: {}", err);
}
