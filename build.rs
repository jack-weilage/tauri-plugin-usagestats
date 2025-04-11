const COMMANDS: &[&str] = &[
    "check_permissions",
    "request_permissions",
    "get_stats_for_range",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
