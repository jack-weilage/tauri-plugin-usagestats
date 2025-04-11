use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::Result;
use crate::UsageStatsExt;

#[command]
pub(crate) async fn check_permissions<R: Runtime>(app: AppHandle<R>) -> Result<PermissionStatus> {
    app.usagestats().check_permissions()
}

#[command]
pub(crate) async fn request_permissions<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    app.usagestats().request_permissions()
}

#[command]
pub(crate) async fn get_stats_for_range<R: Runtime>(
    app: AppHandle<R>,
    payload: GetStatsForRangeRequest,
) -> Result<GetStatsForRangeResponse> {
    app.usagestats().get_stats_for_range(payload)
}
