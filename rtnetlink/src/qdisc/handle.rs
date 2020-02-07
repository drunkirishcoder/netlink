use super::QdiscAddRequest;
use crate::Handle;

pub struct QdiscHandle(Handle);

impl QdiscHandle {
    pub fn new(handle: Handle) -> Self {
        QdiscHandle(handle)
    }

    pub fn add(&self) -> QdiscAddRequest {
        QdiscAddRequest::new(self.0.clone())
    }
}
