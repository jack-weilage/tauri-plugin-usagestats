import { invoke, PermissionState } from "@tauri-apps/api/core";

export async function checkPermissions(): Promise<PermissionState> {
	return await invoke<{ state: PermissionState }>("plugin:usagestats|check_permissions").then(
		(r) => r.state,
	);
}

export async function requestPermissions(): Promise<void> {
	return await invoke("plugin:usagestats|request_permissions");
}

export async function getStatsForRange(
	start: number,
	end: number,
): Promise<Record<string, UsageStats>> {
	return await invoke<{ stats: Record<string, UsageStats> }>(
		"plugin:usagestats|get_stats_for_range",
		{
			payload: { start, end },
		},
	).then((r) => r.stats);
}

export interface UsageStats {
	firstTimeStamp: number;
	lastTimeStamp: number;
	lastTimeUsed: number;
	packageName: string;
	totalTimeInForeground: number;
}
