#!/usr/bin/env bash
# Busy-screen benchmark (tools/demo/canvas/busy.ts), one launch per run. On Android the second
# half of each line is HWUI's frame stats for the same window, i.e. how the views fared.
#   tools/scripts/busy-bench.sh [runs] [scenario ...]
#   PLATFORM=ios SIM=<udid> tools/scripts/busy-bench.sh 3 2d
set -euo pipefail

PKG=org.nativescript.plugindemo
ACTIVITY=com.tns.NativeScriptActivity
PLATFORM=${PLATFORM:-android}
SIM=${SIM:-booted}
IOS_LOG=${TMPDIR:-/tmp}/busy-bench-ios.log
RUNS=${1:-3}
shift || true
if [ $# -gt 0 ]; then
	SCENARIOS=("$@")
else
	SCENARIOS=(views 2d 2d+views 2dx2+views webgl webgl+views 2d+webgl+views)
fi

busy_lines() {
	if [ "$PLATFORM" = ios ]; then
		grep -ao "BUSY|.*" "$IOS_LOG" 2>/dev/null || true
	else
		adb logcat -d -s JS:* 2>/dev/null | grep -o "BUSY|.*" || true
	fi
}

launch() {
	if [ "$PLATFORM" = ios ]; then
		xcrun simctl terminate "$SIM" "$PKG" >/dev/null 2>&1 || true
		: >"$IOS_LOG"
		xcrun simctl launch "$SIM" "$PKG" --demo=canvas-busy --suite="$1" >/dev/null
	else
		adb shell am force-stop "$PKG"
		adb logcat -c
		adb shell am start -n "$PKG/$ACTIVITY" --es demo canvas-busy --es suite "$1" >/dev/null
	fi
}

if [ "$PLATFORM" = ios ]; then
	xcrun simctl spawn "$SIM" log stream --level debug --style compact --predicate 'process == "demo"' >"$IOS_LOG" 2>/dev/null &
	LOG_PID=$!
	trap 'kill $LOG_PID 2>/dev/null || true' EXIT
	sleep 1
fi

wait_for() {
	local pattern=$1 limit=$2 waited=0 line
	while [ "$waited" -lt $((limit * 5)) ]; do
		line=$(busy_lines | grep -F -- "$pattern" | tail -1 || true)
		if [ -n "$line" ]; then
			echo "$line"
			return 0
		fi
		sleep 0.2
		waited=$((waited + 1))
	done
	return 1
}

gfx() {
	adb shell dumpsys gfxinfo "$PKG" | awk -F': *' '
		/^Total frames rendered/ { total = $2 }
		/^Janky frames:/ && !legacy { split($2, j, " "); janky = j[1]; pct = j[2]; legacy = 1 }
		/^90th percentile/ && !p90 { p90 = $2 }
		/^99th percentile/ && !p99 { p99 = $2 }
		/^Number Missed Vsync/ { missed = $2 }
		/^Number Slow UI thread/ { slowui = $2 }
		END { printf "ui_frames %s janky %s %s p90 %s p99 %s missed_vsync %s slow_ui %s", total, janky, pct, p90, p99, missed, slowui }'
}

for scenario in "${SCENARIOS[@]}"; do
	for run in $(seq 1 "$RUNS"); do
		launch "$scenario"
		if ! wait_for "BUSY|$scenario|measuring" 60 >/dev/null; then
			echo "$scenario #$run | never started measuring"
			busy_lines | tail -3
			continue
		fi
		if [ "$PLATFORM" = android ]; then
			adb shell dumpsys gfxinfo "$PKG" reset >/dev/null
		fi
		if ! wait_for "BUSY|$scenario|done" 30 >/dev/null; then
			echo "$scenario #$run | never finished"
			continue
		fi
		result=$(wait_for "BUSY|$scenario|result" 1 | sed 's/^BUSY|[^|]*|result|//' | tr '|' ' ')
		if [ "$PLATFORM" = android ]; then
			echo "$scenario #$run | $result | $(gfx)"
		else
			echo "$scenario #$run | $result"
		fi
	done
done
if [ "$PLATFORM" = ios ]; then
	xcrun simctl terminate "$SIM" "$PKG" >/dev/null 2>&1 || true
else
	adb shell am force-stop "$PKG"
fi
