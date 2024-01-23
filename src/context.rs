use crate::prelude::{Rime, RimeSession};
use crate::{api, util};
use std::ffi::CStr;
use std::marker::PhantomData;

pub struct RimeContext {
    pub(crate) context: api::RimeContext,
    pub(crate) session: RimeSession,
}

impl RimeContext {
    pub fn preedit(&self) -> String {
        util::safe_text(self.context.composition.preedit)
    }

    pub fn num_candidates(&self) -> i32 {
        self.context.menu.num_candidates
    }

    pub fn all_candidates(&self) -> Vec<(String, String)> {
        let mut cans = vec![];

        for i in 0..self.num_candidates() {
            let can = unsafe { *self.context.menu.candidates.offset(i as isize) };
            cans.push((util::safe_text(can.text), util::safe_text(can.comment)));
        }

        cans
    }
}

impl Drop for RimeContext {
    fn drop(&mut self) {
        unsafe {
            Rime::FreeContext(&mut self.context);
        }
    }
}
