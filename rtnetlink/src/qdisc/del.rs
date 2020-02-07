use futures::stream::StreamExt;

use crate::{
    packet::{
        nlas::tc::Nla, NetlinkMessage, NetlinkPayload, RtnlMessage, TcMessage, NLM_F_ACK,
        NLM_F_CREATE, NLM_F_EXCL, NLM_F_REQUEST, TC_H_CLSACT,
    },
    Error, ErrorKind, Handle,
};

/// A request to delete a new qdisc. This is equivalent to the `tc qdisc del` commands.
pub struct QdiscDelRequest {
    handle: Handle,
    message: TcMessage,
}

impl QdiscDelRequest {
    pub(crate) fn new(handle: Handle) -> Self {
        QdiscDelRequest {
            handle,
            message: TcMessage::default(),
        }
    }

    /// Execute the request.
    pub async fn execute(self) -> Result<(), Error> {
        let QdiscDelRequest {
            mut handle,
            message,
        } = self;
        let mut req = NetlinkMessage::from(RtnlMessage::DelQueueDiscipline(message));
        req.header.flags = NLM_F_REQUEST | NLM_F_ACK | NLM_F_EXCL | NLM_F_CREATE;

        let mut response = handle.request(req)?;
        while let Some(message) = response.next().await {
            if let NetlinkPayload::Error(err) = message.payload {
                return Err(ErrorKind::NetlinkError(err).into());
            }
        }
        Ok(())
    }

    pub fn clsact(mut self, index: i32) -> Self {
        self.message.header.index = index;
        self.message.header.parent = TC_H_CLSACT as u32;
        self.message.header.handle = 0xffff0000;
        self.message.nlas.push(Nla::Kind("clsact".to_owned()));
        self
    }
}
