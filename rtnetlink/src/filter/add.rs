use futures::stream::StreamExt;
use std::os::unix::io::RawFd;

use crate::{
    packet::{
        nlas::tc::{Bpf, Nla},
        traits::Emitable,
        NetlinkMessage, NetlinkPayload, RtnlMessage, TcMessage, ETH_P_ALL, NLM_F_ACK, NLM_F_CREATE,
        NLM_F_EXCL, NLM_F_REQUEST, TCA_BPF_FLAG_ACT_DIRECT,
    },
    Error, ErrorKind, Handle,
};

/// A request to create a new filter. This is equivalent to the `tc filter add` commands.
pub struct FilterAddRequest {
    handle: Handle,
    message: TcMessage,
    protocol: u16,
    priority: u8,
}

impl FilterAddRequest {
    pub(crate) fn new(handle: Handle) -> Self {
        FilterAddRequest {
            handle,
            message: TcMessage::default(),
            protocol: ETH_P_ALL,
            priority: 0,
        }
    }

    /// Execute the request.
    pub async fn execute(self) -> Result<(), Error> {
        let FilterAddRequest {
            mut handle,
            mut message,
            protocol,
            priority,
        } = self;
        message.header.info = (priority as u32) << 16 | u16::to_be(protocol) as u32;
        let mut req = NetlinkMessage::from(RtnlMessage::NewTrafficFilter(message));
        req.header.flags = NLM_F_REQUEST | NLM_F_ACK | NLM_F_EXCL | NLM_F_CREATE;

        let mut response = handle.request(req)?;
        while let Some(message) = response.next().await {
            if let NetlinkPayload::Error(err) = message.payload {
                return Err(ErrorKind::NetlinkError(err).into());
            }
        }
        Ok(())
    }

    pub fn bpf(mut self, index: i32, fd: RawFd, name: &str, parent: u32) -> Self {
        self.message.header.index = index;
        self.message.header.parent = parent;
        self.message.header.handle = 1;
        self.message.nlas.push(Nla::Kind("bpf".to_owned()));

        let mut options = vec![];
        options.push(Bpf::Classid(1));
        options.push(Bpf::Fd(fd as u32));
        options.push(Bpf::Name(name.to_owned()));
        options.push(Bpf::Flags(TCA_BPF_FLAG_ACT_DIRECT));

        let len = options.as_slice().buffer_len();
        let mut buffer = vec![0; len];
        options.as_slice().emit(&mut buffer);
        self.message.nlas.push(Nla::Options(buffer));

        self
    }
}
