use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tauri::plugin::PermissionState;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionStatus {
    pub state: PermissionState,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStatsForRangeRequest {
    pub start: u64,
    pub end: u64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStatsForRangeResponse {
    pub stats: HashMap<String, UsageStats>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageStats {
    pub first_time_stamp: u64,
    pub last_time_stamp: u64,
    pub last_time_used: u64,
    pub package_name: String,
    pub total_time_in_foreground: u64,
}
