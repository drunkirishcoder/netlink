use super::{QdiscAddRequest, QdiscDelRequest};
use crate::Handle;

pub struct QdiscHandle(Handle);

impl QdiscHandle {
    pub fn new(handle: Handle) -> Self {
        QdiscHandle(handle)
    }

    pub fn add(&self) -> QdiscAddRequest {
        QdiscAddRequest::new(self.0.clone())
    }

    pub fn del(&self) -> QdiscDelRequest {
        QdiscDelRequest::new(self.0.clone())
    }
}
