use std::collections::HashMap;

use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PermissionState, PluginApi},
    AppHandle, Runtime,
};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<UsageStats<R>> {
    Ok(UsageStats(app.clone()))
}

/// Access to the usagestats APIs.
pub struct UsageStats<R: Runtime>(AppHandle<R>);

impl<R: Runtime> UsageStats<R> {
    pub fn check_permissions(&self) -> crate::Result<PermissionStatus> {
        Ok(PermissionStatus {
            state: PermissionState::Denied,
        })
    }
    pub fn request_permissions(&self) -> crate::Result<()> {
        Ok(())
    }
    pub fn get_stats_for_range(
        &self,
        payload: GetStatsForRangeRequest,
    ) -> crate::Result<GetStatsForRangeResponse> {
        Ok(GetStatsForRangeResponse {
            stats: HashMap::new(),
        })
    }
}
