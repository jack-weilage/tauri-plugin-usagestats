package dev.weilage.usagestats

import android.app.AppOpsManager
import android.app.usage.UsageStats
import android.app.usage.UsageStatsManager
import android.content.Context
import app.tauri.PermissionState

class UsageStats(private val context: Context) {
    private val usageStatsManager = context.getSystemService(Context.USAGE_STATS_SERVICE) as UsageStatsManager

    fun checkPermissions(): PermissionState {
        val appOps = context.getSystemService(Context.APP_OPS_SERVICE) as AppOpsManager?
        val mode = appOps!!.checkOpNoThrow(AppOpsManager.OPSTR_GET_USAGE_STATS, context.applicationInfo.uid, context.packageName)

        return if (mode == AppOpsManager.MODE_ALLOWED) {
            PermissionState.GRANTED
        } else {
            PermissionState.PROMPT
        }
    }

    fun getStatsForRange(start: Long, end: Long): Map<String, UsageStats> {
        return usageStatsManager.queryAndAggregateUsageStats(start, end)
    }
}
