use crate::api;
use crate::prelude::{RimeCandidateList, RimeCommit, RimeContext, RimeSessionId};
use crate::Rime;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RimeSession(Arc<RimeSessionInner>);

#[derive(Debug)]
struct RimeSessionInner {
    pub(crate) id: RimeSessionId,
}

impl RimeSession {
    pub fn new(id: RimeSessionId) -> Self {
        RimeSession(Arc::new(RimeSessionInner { id }))
    }

    pub fn process_key(&self, keycode: i32) -> bool {
        Rime::ProcessKey(self.0.id, keycode, 0)
    }

    pub fn process_key_mask(&self, keycode: i32, mask: i32) -> bool {
        Rime::ProcessKey(self.0.id, keycode, mask)
    }

    pub fn process_char(&self, keycode: char) -> bool {
        Rime::ProcessKey(self.0.id, keycode as _, 0)
    }

    pub fn process_string(&self, input: impl AsRef<str>) {
        for key in input.as_ref().chars() {
            self.process_char(key);
        }
    }

    pub fn set_property(&self, prop: impl AsRef<str>, value: impl AsRef<str>) {
        Rime::SetProperty(self.0.id, prop, value)
    }

    pub fn get_property(&self, prop: impl AsRef<str>, size: usize) -> Option<String> {
        Rime::GetProperty(self.0.id, prop, size)
    }

    pub fn context(&self) -> RimeContext {
        let mut context = rime_struct_init!(api::RimeContext);

        unsafe {
            api::RimeGetContext(self.0.id, &mut context);

            RimeContext {
                context,
                session: self.clone(),
            }
        }
    }

    pub fn commit(&self) -> RimeCommit {
        let mut commit = rime_struct_init!(api::RimeCommit);

        unsafe {
            api::RimeGetCommit(self.0.id, &mut commit);
            RimeCommit {
                commit,
                session: self.clone(),
            }
        }
    }

    pub fn candidates(&self) -> RimeCandidateList {
        let mut list = std::mem::MaybeUninit::uninit();
        unsafe {
            api::RimeCandidateListBegin(self.0.id, list.as_mut_ptr());

            let list = list.assume_init();

            RimeCandidateList {
                list,
                session: self.clone(),
            }
        }
    }

    pub fn clear_composition(&self) {
        Rime::ClearComposition(self.0.id);
    }
}

impl Drop for RimeSessionInner {
    fn drop(&mut self) {
        Rime::DestroySession(self.id);
    }
}
