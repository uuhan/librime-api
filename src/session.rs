use crate::api;
use crate::prelude::{Rime, RimeCandidateList, RimeCommit, RimeContext, RimeSessionId};
use std::marker::PhantomData;

pub struct RimeSession<'a> {
    pub(crate) id: RimeSessionId,
    pub(crate) rime: PhantomData<&'a Rime>,
}

impl<'a> RimeSession<'a> {
    pub fn process_key(&self, keycode: i32) -> bool {
        Rime::ProcessKey(self.id, keycode, 0)
    }

    pub fn process_key_mask(&self, keycode: i32, mask: i32) -> bool {
        Rime::ProcessKey(self.id, keycode, mask)
    }

    pub fn process_char(&self, keycode: char) -> bool {
        Rime::ProcessKey(self.id, keycode as _, 0)
    }

    pub fn process_string(&self, input: impl AsRef<str>) {
        for key in input.as_ref().chars() {
            self.process_char(key);
        }
    }

    pub fn set_property(&self, prop: impl AsRef<str>, value: impl AsRef<str>) {
        Rime::SetProperty(self.id, prop, value)
    }

    pub fn get_property(&self, prop: impl AsRef<str>, size: usize) -> Option<String> {
        Rime::GetProperty(self.id, prop, size)
    }

    pub fn context(&self) -> RimeContext<'a> {
        let mut context = rime_struct_init!(api::RimeContext);

        unsafe {
            api::RimeGetContext(self.id, &mut context);

            RimeContext {
                context,
                session: PhantomData,
            }
        }
    }

    pub fn commit(&self) -> RimeCommit<'a> {
        let mut commit = rime_struct_init!(api::RimeCommit);

        unsafe {
            api::RimeGetCommit(self.id, &mut commit);
            RimeCommit {
                commit,
                session: PhantomData,
            }
        }
    }

    pub fn candidates(&self) -> RimeCandidateList<'a> {
        let mut list = std::mem::MaybeUninit::uninit();
        unsafe {
            api::RimeCandidateListBegin(self.id, list.as_mut_ptr());

            let list = list.assume_init();

            RimeCandidateList {
                list,
                session: PhantomData,
            }
        }
    }

    pub fn clear_composition(&self) {
        Rime::ClearComposition(self.id);
    }
}

impl<'a> Drop for RimeSession<'a> {
    fn drop(&mut self) {
        Rime::DestroySession(self.id);
    }
}
