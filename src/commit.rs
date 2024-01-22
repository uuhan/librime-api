use crate::api;
use crate::prelude::{Rime, RimeSession};
use std::ffi::CStr;

pub struct RimeCommit {
    pub(crate) commit: api::RimeCommit,
    pub(crate) session: RimeSession,
}

impl RimeCommit {
    pub fn text(&self) -> String {
        if self.commit.text.is_null() {
            return String::new();
        }

        unsafe {
            CStr::from_ptr(self.commit.text)
                .to_string_lossy()
                .to_string()
        }
    }
}

impl Drop for RimeCommit {
    fn drop(&mut self) {
        Rime::FreeCommit(&mut self.commit);
    }
}
