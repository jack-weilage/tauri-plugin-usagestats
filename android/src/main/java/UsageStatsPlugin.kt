package dev.weilage.usagestats

import android.app.Activity
import android.content.Intent
import android.provider.Settings
import androidx.core.net.toUri
import app.tauri.PermissionState
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@InvokeArg
class GetStatsForRangeArgs {
    var start: Long? = null
    var end: Long? = null
}

@TauriPlugin
class UsageStatsPlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = UsageStats(activity.applicationContext)

    @Command
    override fun checkPermissions(invoke: Invoke) {
        val ret = JSObject()
        ret.put("state", implementation.checkPermissions())
        invoke.resolve(ret)
    }

    @Command
    override fun requestPermissions(invoke: Invoke) {
        val ctx = activity.applicationContext

        if (implementation.checkPermissions() == PermissionState.GRANTED) {
            return
        }

        // Attempt to launch the usage access settings pointing to the current package's name, falling
        // back to just opening the settings.
        try {
            val intent = Intent(Settings.ACTION_USAGE_ACCESS_SETTINGS)
            intent.flags = Intent.FLAG_ACTIVITY_NEW_TASK
            intent.data = ("package:" + ctx.packageName).toUri()

            ctx.startActivity(intent)
        } catch (e: Exception) {
            val intent = Intent(Settings.ACTION_USAGE_ACCESS_SETTINGS)
            intent.flags = Intent.FLAG_ACTIVITY_NEW_TASK

            ctx.startActivity(intent)
        }
    }

    @Command
    fun getStatsForRange(invoke: Invoke) {
        val args = invoke.parseArgs(GetStatsForRangeArgs::class.java)
        val stats = implementation.getStatsForRange(args.start!!, args.end!!)

        val statsObj = JSObject()
        for (packageStats in stats.values) {
            val entryObj = JSObject()

            entryObj.put("firstTimeStamp", packageStats.firstTimeStamp)
//            entryObj.put("lastTimeForegroundServiceUsed", packageStats.lastTimeForegroundServiceUsed)
            entryObj.put("lastTimeStamp", packageStats.lastTimeStamp)
            entryObj.put("lastTimeUsed", packageStats.lastTimeUsed)
//            entryObj.put("lastTimeVisible", packageStats.lastTimeVisible)
            entryObj.put("packageName", packageStats.packageName)
//            entryObj.put("totalTimeForegroundServiceUsed", packageStats.totalTimeForegroundServiceUsed)
            entryObj.put("totalTimeInForeground", packageStats.totalTimeInForeground)
//            entryObj.put("totalTimeVisible", packageStats.totalTimeVisible)

            statsObj.put(packageStats.packageName, entryObj)
        }

        val ret = JSObject()
        ret.put("stats", statsObj)
        invoke.resolve(ret)
    }
}
