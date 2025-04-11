use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::UsageStats as UsageStatsPlugin;
#[cfg(mobile)]
use mobile::UsageStats as UsageStatsPlugin;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the usagestats APIs.
pub trait UsageStatsExt<R: Runtime> {
    fn usagestats(&self) -> &UsageStatsPlugin<R>;
}

impl<R: Runtime, T: Manager<R>> crate::UsageStatsExt<R> for T {
    fn usagestats(&self) -> &UsageStatsPlugin<R> {
        self.state::<UsageStatsPlugin<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("usagestats")
        .invoke_handler(tauri::generate_handler![
            commands::check_permissions,
            commands::request_permissions,
            commands::get_stats_for_range
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let usagestats = mobile::init(app, api)?;
            #[cfg(desktop)]
            let usagestats = desktop::init(app, api)?;
            app.manage(usagestats);
            Ok(())
        })
        .build()
}
