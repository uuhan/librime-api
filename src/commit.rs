use std::ffi::CStr;
use std::marker::PhantomData;

use crate::api;
use crate::prelude::{Rime, RimeSession};

pub struct RimeCommit<'a> {
    pub(crate) commit: api::RimeCommit,
    pub(crate) session: PhantomData<RimeSession<'a>>,
}

impl<'a> RimeCommit<'a> {
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

impl<'a> Drop for RimeCommit<'a> {
    fn drop(&mut self) {
        Rime::FreeCommit(&mut self.commit);
    }
}
