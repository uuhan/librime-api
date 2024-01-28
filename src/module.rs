use crate::{api, prelude::*, util::safe_text};
use std::{ffi::CString, fmt::Debug};

pub struct RimeModule {
    pub(crate) module: api::RimeModule,
}

impl Debug for RimeModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = safe_text(self.module.module_name);
        write!(f, "RimeModule({})", name)
    }
}

impl Default for RimeModule {
    fn default() -> Self {
        unsafe {
            let module = rime_struct_init!(api::RimeModule);
            RimeModule { module }
        }
    }
}

impl RimeModule {
    pub fn find(module_name: impl AsRef<str>) -> *const api::RimeModule {
        unsafe {
            let name = CString::new(module_name.as_ref()).unwrap().into_raw();
            api::RimeFindModule(name)
        }
    }

    pub fn new(module_name: impl AsRef<str>) -> Self {
        unsafe {
            let name = CString::new(module_name.as_ref()).unwrap().into_raw();
            let mut module = Self::default();

            module.module.module_name = name;
            module.module.initialize = Some(initialize);
            module.module.finalize = Some(finalize);

            module
        }
    }

    pub fn register(&mut self) -> bool {
        unsafe { api::RimeRegisterModule(&mut self.module) != 0 }
    }
}

impl Drop for RimeModule {
    fn drop(&mut self) {
        println!("RimeModule drop");
    }
}

unsafe extern "C" fn initialize() {
    println!("module initialized");
}

unsafe extern "C" fn finalize() {
    println!("module finalized");
}

unsafe extern "C" fn get_api() -> *mut api::RimeCustomApi {
    println!("module get_api");

    let mut api = rime_struct_init!(api::RimeCustomApi);

    return &mut api;
}
