#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{c_char, c_void, CStr, CString};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::Arc;

#[macro_use]
mod mac;
pub mod util;

mod api {
    use std::sync::OnceLock;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    // It's just a builder
    impl RimeTraits {
        pub unsafe fn setup(mut self) {
            RimeSetup(&mut self);
            RimeInitialize(&mut self);
            RimeDeployerInitialize(&mut self);

            if RimeStartMaintenanceOnWorkspaceChange() != 0 {
                RimeJoinMaintenanceThread();
            }
        }
    }

    impl Default for RimeTraits {
        fn default() -> Self {
            rime_struct_init!(RimeTraits)
        }
    }
}

pub mod candidate;
pub mod commit;
pub mod context;
pub mod error;
pub mod module;
pub mod session;
pub mod status;

pub mod prelude {
    pub use super::{Rime, RimeBuilder};
    #[cfg(feature = "logging")]
    pub use super::{RimeLogKind, RimeMinLogLevel};

    pub use super::api::RimeSessionId;
    pub use super::candidate::{RimeCandidate, RimeCandidateList};
    pub use super::commit::RimeCommit;
    pub use super::context::RimeContext;
    pub use super::session::RimeSession;
    pub use super::status::RimeStatus;
}

use context::RimeContext;
use session::RimeSession;

pub use api::RimeSessionId;

#[cfg(feature = "logging")]
/// Minimal level of logged messages.
///  Value is passed to Glog library using FLAGS_minloglevel variable.
///  0 = INFO (default), 1 = WARNING, 2 = ERROR, 3 = FATAL
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub enum RimeMinLogLevel {
    #[default]
    Info = 0,
    Warn = 1,
    Error = 2,
    Fatal = 3,
}

#[cfg(feature = "logging")]
#[derive(Debug, Default, Clone)]
pub enum RimeLogKind {
    #[default]
    StdErr,
    TempDir,
    Dir(PathBuf),
}

#[derive(Default, Debug)]
pub struct RimeBuilder {
    shared_data_dir: Option<PathBuf>,
    user_data_dir: Option<PathBuf>,
    #[cfg(feature = "logging")]
    /// Directory of log files.
    /// Value is passed to Glog library using FLAGS_log_dir variable.
    /// NULL means temporary directory, and "" means only writing to stderr.
    log_kind: RimeLogKind,
    #[cfg(feature = "logging")]
    min_log_level: RimeMinLogLevel,
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

    #[cfg(feature = "logging")]
    pub fn log_kind(&mut self, kind: RimeLogKind) -> &mut Self {
        self.log_kind = kind;
        self
    }

    #[cfg(feature = "logging")]
    pub fn min_log_level(&mut self, level: RimeMinLogLevel) -> &mut Self {
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

    pub fn build(&mut self) -> Option<()> {
        let mut traits = api::RimeTraits::default();

        if let Some(dir) = self.shared_data_dir.take() {
            traits.shared_data_dir = CString::new(dir.to_str()?).ok()?.into_raw();
        }

        if let Some(dir) = self.user_data_dir.take() {
            traits.user_data_dir = CString::new(dir.to_str()?).ok()?.into_raw();
        }

        #[cfg(feature = "logging")]
        {
            traits.min_log_level = self.min_log_level as _;
            match &self.log_kind {
                // only writing to stderr
                RimeLogKind::StdErr => {
                    traits.log_dir = CString::new("").ok()?.into_raw();
                }
                RimeLogKind::TempDir => {
                    // traits.log_dir, NULL means temporary directory
                }
                RimeLogKind::Dir(path) => {
                    if path.exists() && path.metadata().ok()?.is_dir() {
                        traits.log_dir = CString::new(path.to_str()?).ok()?.into_raw();
                    } else {
                        // writing to stderr if log_dir does not exists
                        traits.log_dir = CString::new("").ok()?.into_raw();
                    }
                }
            }
        }

        traits.distribution_name = CString::new(
            self.distribution_name
                .take()
                .or(Some(env!("CARGO_PKG_NAME").to_string()))?,
        )
        .ok()?
        .into_raw();

        traits.distribution_code_name = CString::new(
            self.distribution_code_name
                .take()
                .or(Some(env!("CARGO_PKG_NAME").to_string()))?,
        )
        .ok()?
        .into_raw();

        traits.distribution_version = CString::new(
            self.distribution_version
                .take()
                .or(Some(env!("CARGO_PKG_VERSION").to_string()))?,
        )
        .ok()?
        .into_raw();

        traits.app_name = CString::new(
            self.app_name
                .take()
                .or(Some(env!("CARGO_PKG_NAME").to_string()))?,
        )
        .ok()?
        .into_raw();

        unsafe { traits.setup() };

        Some(())
    }
}

#[doc = include_str!("../README.md")]
pub mod Rime {
    use super::*;
    /// Pass a C-string constant in the format "rime.x"
    /// where 'x' is the name of your application.
    /// Add prefix "rime." to ensure old log files are automatically cleaned.
    #[deprecated(note = "Use RimeSetup() instead.")]
    pub fn setup_logging(app_name: impl AsRef<str>) {
        unsafe {
            let app_name = CString::new(app_name.as_ref()).unwrap().into_raw();
            api::RimeSetupLogging(app_name);
        }
    }

    /// Receive notifications
    ///
    /// - on loading schema:
    ///   + message_type="schema", message_value="luna_pinyin/Luna Pinyin"
    /// - on changing mode:
    ///   + message_type="option", message_value="ascii_mode"
    ///   + message_type="option", message_value="!ascii_mode"
    /// - on deployment:
    ///   + session_id = 0, message_type="deploy", message_value="start"
    ///   + session_id = 0, message_type="deploy", message_value="success"
    ///   + session_id = 0, message_type="deploy", message_value="failure"
    ///
    ///   handler will be called with context_object as the first parameter
    ///   every time an event occurs in librime, until RimeFinalize() is called.
    ///   when handler is NULL, notification is disabled.
    ///
    /// This handler setup should live as long as RimeTraits
    pub fn SetNotificationHandler<Handler>(handler: Handler)
    where
        // It's safe to cast to &str
        Handler: Fn(RimeSessionId, &str, &str) + Send + 'static,
    {
        unsafe extern "C" fn rime_notification_handler(
            context_object: *mut c_void,
            session_id: RimeSessionId,
            message_type: *const c_char,
            message_value: *const c_char,
        ) {
            let handler: Box<Box<dyn Fn(RimeSessionId, &str, &str) + Send>> =
                Box::from_raw(context_object as _);

            let type_ = CStr::from_ptr(message_type).to_str().unwrap();
            let value = CStr::from_ptr(message_value).to_str().unwrap();

            handler(session_id, type_, value);

            // Just Leak it. There is no way to easily collect the handler in librime.
            std::mem::forget(handler);
        }

        let context_object: Box<Box<dyn Fn(RimeSessionId, &str, &str) + Send>> =
            Box::new(Box::new(handler));
        let context_object = Box::into_raw(context_object);

        unsafe {
            api::RimeSetNotificationHandler(Some(rime_notification_handler), context_object as _);
        }
    }

    pub fn CreateSession() -> RimeSession {
        RimeSession::new(unsafe { api::RimeCreateSession() })
    }

    pub fn DeployWorkspace() -> bool {
        unsafe { api::RimeDeployWorkspace() != 0 }
    }

    pub fn DeploySchema(schema: impl AsRef<str>) -> bool {
        unsafe {
            let schema = CString::new(schema.as_ref()).unwrap().into_raw();
            api::RimeDeploySchema(schema) != 0
        }
    }

    pub fn DeployConfigFile(file: impl AsRef<str>, version: impl AsRef<str>) -> bool {
        unsafe {
            let file = CString::new(file.as_ref()).unwrap().into_raw();
            let version = CString::new(version.as_ref()).unwrap().into_raw();
            api::RimeDeployConfigFile(file, version) != 0
        }
    }

    pub fn SyncUserData() -> bool {
        unsafe { api::RimeSyncUserData() != 0 }
    }

    fn FindSession(id: RimeSessionId) -> bool {
        unsafe { api::RimeFindSession(id) != 0 }
    }

    pub(crate) fn DestroySession(id: RimeSessionId) -> bool {
        unsafe { api::RimeDestroySession(id) != 0 }
    }

    pub fn CleanupStaleSessions() {
        unsafe { api::RimeCleanupStaleSessions() }
    }

    pub fn CleanupAllSessions() {
        unsafe { api::RimeCleanupAllSessions() }
    }

    pub fn ProcessKey(id: RimeSessionId, keycode: i32, mask: i32) -> bool {
        unsafe { api::RimeProcessKey(id, keycode, mask) != 0 }
    }

    pub fn CommitComposition(id: RimeSessionId) -> bool {
        unsafe { api::RimeCommitComposition(id) != 0 }
    }

    pub fn ClearComposition(id: RimeSessionId) {
        unsafe { api::RimeClearComposition(id) }
    }

    pub fn GetCommit(id: RimeSessionId, commit: *mut api::RimeCommit) -> bool {
        unsafe { api::RimeGetCommit(id, commit) != 0 }
    }

    pub fn FreeCommit(commit: *mut api::RimeCommit) -> bool {
        unsafe { api::RimeFreeCommit(commit) != 0 }
    }

    pub fn GetContext(id: RimeSessionId, context: *mut api::RimeContext) -> bool {
        unsafe { api::RimeGetContext(id, context) != 0 }
    }

    pub fn FreeContext(context: *mut api::RimeContext) -> bool {
        unsafe { api::RimeFreeContext(context) != 0 }
    }

    pub fn GetStatus(id: RimeSessionId, status: *mut api::RimeStatus) -> bool {
        unsafe { api::RimeGetStatus(id, status) != 0 }
    }

    pub fn FreeStatus(status: *mut api::RimeStatus) -> bool {
        unsafe { api::RimeFreeStatus(status) != 0 }
    }

    pub fn CandidateListBegin(
        id: RimeSessionId,
        iterator: *mut api::RimeCandidateListIterator,
    ) -> bool {
        unsafe { api::RimeCandidateListBegin(id, iterator) != 0 }
    }

    pub fn CandidateListNext(iterator: *mut api::RimeCandidateListIterator) -> bool {
        unsafe { api::RimeCandidateListNext(iterator) != 0 }
    }

    pub fn CandidateListEnd(iterator: *mut api::RimeCandidateListIterator) {
        unsafe { api::RimeCandidateListEnd(iterator) }
    }

    pub fn CandidateListFromIndex(
        id: RimeSessionId,
        iterator: *mut api::RimeCandidateListIterator,
        index: u32,
    ) -> bool {
        unsafe { api::RimeCandidateListFromIndex(id, iterator, index as _) != 0 }
    }

    pub fn SelectCandidate(id: RimeSessionId, index: usize) -> bool {
        unsafe { api::RimeSelectCandidate(id, index as _) != 0 }
    }

    pub fn SelectCandidateOnCurrentPage(id: RimeSessionId, index: usize) -> bool {
        unsafe { api::RimeSelectCandidateOnCurrentPage(id, index as _) != 0 }
    }

    pub fn DeleteCandidate(id: RimeSessionId, index: usize) -> bool {
        unsafe { api::RimeDeleteCandidate(id, index as _) != 0 }
    }

    pub fn DeleteCandidateOnCurrentPage(id: RimeSessionId, index: usize) -> bool {
        unsafe { api::RimeDeleteCandidateOnCurrentPage(id, index as _) != 0 }
    }

    pub fn SetOption(id: RimeSessionId, option: impl AsRef<str>, value: bool) {
        unsafe {
            let option = CString::new(option.as_ref()).unwrap().into_raw();
            api::RimeSetOption(id, option, value as _)
        }
    }

    pub fn GetOption(id: RimeSessionId, option: impl AsRef<str>) -> bool {
        unsafe {
            let option = CString::new(option.as_ref()).unwrap().into_raw();
            api::RimeGetOption(id, option) != 0
        }
    }

    pub fn SetProperty(id: RimeSessionId, prop: impl AsRef<str>, value: impl AsRef<str>) {
        unsafe {
            let prop = CString::new(prop.as_ref()).unwrap().into_raw();
            let value = CString::new(value.as_ref()).unwrap().into_raw();
            api::RimeSetProperty(id, prop, value)
        }
    }

    pub fn GetProperty(id: RimeSessionId, prop: impl AsRef<str>, size: usize) -> Option<String> {
        unsafe {
            let prop = CString::new(prop.as_ref()).unwrap().into_raw();
            let mut value = vec![0; size];

            if api::RimeGetProperty(id, prop, value.as_mut_ptr() as _, size) != 0 {
                value.push(0);
                let value = CStr::from_bytes_until_nul(&value)
                    .ok()?
                    .to_string_lossy()
                    .to_string();

                return Some(value);
            }

            None
        }
    }

    pub fn GetSchemaList(schema_list: *mut api::RimeSchemaList) -> bool {
        unsafe { api::RimeGetSchemaList(schema_list) != 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
