use std::marker::PhantomData;

use crate::api;
use crate::prelude::*;

pub struct RimeStatus<'a> {
    pub(crate) status: api::RimeStatus,
    pub(crate) session: PhantomData<RimeSession<'a>>,
}

impl<'a> Drop for RimeStatus<'a> {
    fn drop(&mut self) {
        Rime::FreeStatus(&mut self.status);
    }
}
