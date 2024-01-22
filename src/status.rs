use std::marker::PhantomData;

use crate::api;
use crate::prelude::*;

pub struct RimeStatus {
    pub(crate) status: api::RimeStatus,
    pub(crate) session: RimeSession,
}

impl Drop for RimeStatus {
    fn drop(&mut self) {
        Rime::FreeStatus(&mut self.status);
    }
}
