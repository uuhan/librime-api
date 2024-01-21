use std::ffi::CStr;
use std::marker::PhantomData;

use crate::api;
use crate::prelude::{Rime, RimeSession};
use crate::util;

#[derive(Debug)]
pub struct RimeCandidate<'a> {
    pub(crate) candidate: api::RimeCandidate,
    pub(crate) session: PhantomData<RimeSession<'a>>,
}

impl<'a> RimeCandidate<'a> {
    pub fn text(&self) -> String {
        util::safe_text(self.candidate.text)
    }

    pub fn comment(&self) -> String {
        util::safe_text(self.candidate.text)
    }
}

pub struct RimeCandidateList<'a> {
    pub(crate) list: api::RimeCandidateListIterator,
    pub(crate) session: PhantomData<RimeSession<'a>>,
}

impl<'a> RimeCandidateList<'a> {
    pub fn index(&self) -> i32 {
        self.list.index as _
    }

    pub fn next(&mut self) {
        Rime::CandidateListNext(&mut self.list);
    }

    pub fn end(&mut self) {
        Rime::CandidateListEnd(&mut self.list);
    }

    pub fn get(&self) -> RimeCandidate<'a> {
        let candidate = self.list.candidate;
        RimeCandidate {
            candidate,
            session: PhantomData,
        }
    }
}
