use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "dev.weilage.usagestats";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_usagestats);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<UsageStats<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "UsageStatsPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_usagestats)?;
    Ok(UsageStats(handle))
}

/// Access to the usagestats APIs.
pub struct UsageStats<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> UsageStats<R> {
    pub fn check_permissions(&self) -> crate::Result<PermissionStatus> {
        self.0
            .run_mobile_plugin("checkPermissions", ())
            .map_err(Into::into)
    }

    pub fn request_permissions(&self) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("requestPermissions", ())
            .map_err(Into::into)
    }

    pub fn get_stats_for_range(
        &self,
        payload: GetStatsForRangeRequest,
    ) -> crate::Result<GetStatsForRangeResponse> {
        self.0
            .run_mobile_plugin("getStatsForRange", payload)
            .map_err(Into::into)
    }
}
