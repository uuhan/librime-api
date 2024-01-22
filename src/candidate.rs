use std::ffi::CStr;
use std::marker::PhantomData;

use crate::api;
use crate::prelude::{Rime, RimeSession};
use crate::util;

#[derive(Debug)]
pub struct RimeCandidate {
    pub(crate) candidate: api::RimeCandidate,
    pub(crate) session: RimeSession,
}

impl RimeCandidate {
    pub fn text(&self) -> String {
        util::safe_text(self.candidate.text)
    }

    pub fn comment(&self) -> String {
        util::safe_text(self.candidate.text)
    }
}

pub struct RimeCandidateList {
    pub(crate) list: api::RimeCandidateListIterator,
    pub(crate) session: RimeSession,
}

impl RimeCandidateList {
    pub fn index(&self) -> i32 {
        self.list.index as _
    }

    pub fn next(&mut self) {
        Rime::CandidateListNext(&mut self.list);
    }

    pub fn end(&mut self) {
        Rime::CandidateListEnd(&mut self.list);
    }

    pub fn get(&self) -> RimeCandidate {
        let candidate = self.list.candidate;
        RimeCandidate {
            candidate,
            session: self.session.clone(),
        }
    }
}
