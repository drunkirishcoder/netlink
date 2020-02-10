use super::FilterAddRequest;
use crate::Handle;

pub struct FilterHandle(Handle);

impl FilterHandle {
    pub fn new(handle: Handle) -> Self {
        FilterHandle(handle)
    }

    pub fn add(&self) -> FilterAddRequest {
        FilterAddRequest::new(self.0.clone())
    }
}
