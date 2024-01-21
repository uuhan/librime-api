#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;
use std::fmt::Display;
use std::path::PathBuf;
use std::sync::Arc;

pub mod api {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct Rime(Arc<RimeInner>);

struct RimeInner {
    inner: api::RimeTraits,
}

impl Display for Rime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, env!("CARGO_PKG_NAME"))
    }
}

unsafe impl Send for RimeInner {}
unsafe impl Sync for RimeInner {}

#[derive(Default, Debug)]
pub struct RimeBuilder {
    shared_data_dir: Option<PathBuf>,
    user_data_dir: Option<PathBuf>,
    log_dir: Option<PathBuf>,
    min_log_level: i32,
    distribution_name: Option<String>,
    distribution_code_name: Option<String>,
    distribution_version: Option<String>,
    app_name: Option<String>,
}

impl RimeBuilder {
    pub fn new() -> Self {
        RimeBuilder::default()
    }

    pub fn shared_data_dir(&mut self, dir: impl AsRef<str>) -> &mut Self {
        let dir = PathBuf::from(dir.as_ref());
        self.shared_data_dir = Some(dir);
        self
    }

    pub fn user_data_dir(&mut self, dir: impl AsRef<str>) -> &mut Self {
        let dir = PathBuf::from(dir.as_ref());
        self.user_data_dir = Some(dir);
        self
    }

    pub fn log_dir(&mut self, dir: impl AsRef<str>) -> &mut Self {
        let dir = PathBuf::from(dir.as_ref());
        self.log_dir = Some(dir);
        self
    }

    pub fn main_log_level(&mut self, level: i32) -> &mut Self {
        self.min_log_level = level;
        self
    }

    pub fn distribution_name(&mut self, value: impl AsRef<str>) -> &mut Self {
        self.distribution_name.replace(value.as_ref().to_owned());
        self
    }

    pub fn distribution_code_name(&mut self, value: impl AsRef<str>) -> &mut Self {
        self.distribution_code_name
            .replace(value.as_ref().to_owned());
        self
    }

    pub fn distribution_version(&mut self, value: impl AsRef<str>) -> &mut Self {
        self.distribution_version.replace(value.as_ref().to_owned());
        self
    }

    pub fn app_name(&mut self, value: impl AsRef<str>) -> &mut Self {
        self.app_name.replace(value.as_ref().to_owned());
        self
    }

    pub fn build(&mut self) -> Option<Rime> {
        unsafe {
            // initialize a rime instance
            let mut inner = api::rime_traits_init();

            // configurations
            inner.shared_data_dir = CString::new(self.shared_data_dir.take()?.to_str()?)
                .ok()?
                .into_raw();

            inner.user_data_dir = CString::new(self.user_data_dir.take()?.to_str()?)
                .ok()?
                .into_raw();

            if let Some(dir) = self.log_dir.take() {
                inner.user_data_dir = CString::new(dir.to_str()?).ok()?.into_raw();
                inner.min_log_level = self.min_log_level;
            }

            inner.distribution_name = CString::new(
                self.distribution_name
                    .take()
                    .or(Some(env!("CARGO_PKG_NAME").to_string()))?,
            )
            .ok()?
            .into_raw();

            inner.distribution_code_name =
                CString::new(self.distribution_code_name.take().or(Some(String::new()))?)
                    .ok()?
                    .into_raw();

            inner.distribution_version = CString::new(
                self.distribution_version
                    .take()
                    .or(Some(env!("CARGO_PKG_VERSION").to_string()))?,
            )
            .ok()?
            .into_raw();

            inner.app_name = CString::new(
                self.app_name
                    .take()
                    .or(Some(env!("CARGO_PKG_NAME").to_string()))?,
            )
            .ok()?
            .into_raw();

            api::RimeSetup(&mut inner);
            api::RimeInitialize(&mut inner);

            if api::RimeStartMaintenanceOnWorkspaceChange() != 0 {
                api::RimeJoinMaintenanceThread();
            }

            Some(Rime(Arc::new(RimeInner { inner })))
        }
    }
}

impl Rime {
    pub fn new() -> Self {
        unsafe {
            // initialize a rime instance
            let mut inner = api::rime_traits_init();

            Rime(Arc::new(RimeInner { inner }))
        }
    }
}

impl AsRef<api::RimeTraits> for Rime {
    fn as_ref(&self) -> &api::RimeTraits {
        unsafe { &self.0.inner }
    }
}

impl Drop for RimeInner {
    fn drop(&mut self) {
        unsafe {
            api::RimeCleanupAllSessions();
            api::RimeFinalize();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
