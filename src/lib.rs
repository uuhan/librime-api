#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{c_char, c_void, CStr, CString};
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;

pub mod api {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use api::RimeSessionId;

#[doc = include_str!("../README.md")]
#[derive(Debug)]
pub struct Rime(RimeInner);

type NotificationHandler = Box<dyn for<'a, 'b> Fn(RimeSessionId, &str, &str) + Send + 'static>;
struct RimeInner {
    /// The inner RimeTraits in librime world
    inner: *mut api::RimeTraits,
    /// The notification handler
    handler: Option<*mut NotificationHandler>,
}

impl Debug for RimeInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, env!("CARGO_PKG_NAME"))
    }
}

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
            let inner = api::rime_traits_init();

            // configurations
            (*inner).shared_data_dir = CString::new(self.shared_data_dir.take()?.to_str()?)
                .ok()?
                .into_raw();

            (*inner).user_data_dir = CString::new(self.user_data_dir.take()?.to_str()?)
                .ok()?
                .into_raw();

            if let Some(dir) = self.log_dir.take() {
                (*inner).log_dir = CString::new(dir.to_str()?).ok()?.into_raw();
                (*inner).min_log_level = self.min_log_level;
            }

            (*inner).distribution_name = CString::new(
                self.distribution_name
                    .take()
                    .or(Some(env!("CARGO_PKG_NAME").to_string()))?,
            )
            .ok()?
            .into_raw();

            (*inner).distribution_code_name =
                CString::new(self.distribution_code_name.take().or(Some(String::new()))?)
                    .ok()?
                    .into_raw();

            (*inner).distribution_version = CString::new(
                self.distribution_version
                    .take()
                    .or(Some(env!("CARGO_PKG_VERSION").to_string()))?,
            )
            .ok()?
            .into_raw();

            (*inner).app_name = CString::new(
                self.app_name
                    .take()
                    .or(Some(env!("CARGO_PKG_NAME").to_string()))?,
            )
            .ok()?
            .into_raw();

            api::RimeSetup(inner);
            api::RimeInitialize(inner);

            if api::RimeStartMaintenanceOnWorkspaceChange() != 0 {
                api::RimeJoinMaintenanceThread();
            }

            Some(Rime(RimeInner {
                inner,
                handler: None,
            }))
        }
    }
}

impl Rime {
    pub fn new() -> Self {
        unsafe {
            // initialize a rime instance
            let mut inner = api::rime_traits_init();

            Rime(RimeInner {
                inner,
                handler: None,
            })
        }
    }

    /// Call this function before accessing any other API.
    pub fn setup(&mut self) {
        unsafe { api::RimeSetup(self.0.inner) }
    }

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

    /// Entry
    pub fn initialize(&mut self) {
        unsafe { api::RimeInitialize(self.0.inner) }
    }

    /// Exit
    pub fn finalize() {
        unsafe { api::RimeFinalize() }
    }

    /// Deployment
    pub fn deployer_initialize(&mut self) {
        unsafe { api::RimeDeployerInitialize(self.0.inner) }
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
    pub fn set_notification_handler<Handler>(&mut self, handler: Handler)
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

            // This handler should be collected when Rime is dropped.
            std::mem::forget(handler);
        }

        let context_object: Box<Box<dyn Fn(RimeSessionId, &str, &str) + Send>> =
            Box::new(Box::new(handler));
        let context_object = Box::into_raw(context_object);

        self.0.handler.replace(context_object);

        unsafe {
            api::RimeSetNotificationHandler(Some(rime_notification_handler), context_object as _);
        }
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

    pub fn CreateSession() -> RimeSessionId {
        unsafe { api::RimeCreateSession() }
    }

    pub fn FindSession(id: RimeSessionId) -> bool {
        unsafe { api::RimeFindSession(id) != 0 }
    }

    pub fn DestroySession(id: RimeSessionId) -> bool {
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
                    .to_str()
                    .ok()?
                    .to_owned();
                return Some(value);
            }

            None
        }
    }

    pub fn GetSchemaList(schema_list: *mut api::RimeSchemaList) -> bool {
        unsafe { api::RimeGetSchemaList(schema_list) != 0 }
    }
}

impl AsRef<api::RimeTraits> for Rime {
    fn as_ref(&self) -> &api::RimeTraits {
        unsafe { &*self.0.inner }
    }
}

impl Drop for RimeInner {
    fn drop(&mut self) {
        unsafe {
            api::RimeCleanupAllSessions();
            api::RimeFinalize();

            if let Some(handler) = self.handler {
                let _ = Box::from_raw(handler);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
