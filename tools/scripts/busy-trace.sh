#!/usr/bin/env bash
# Main-thread time per frame for busy-screen scenarios, from a Perfetto trace (Android):
#   flush = each canvas's vsync callback, js = requestAnimationFrame, plus CPU ms/s per thread.
#   tools/scripts/busy-trace.sh 2d 'only:fill,2d:1'
# Needs trace_processor (https://get.perfetto.dev/trace_processor) at $TRACE_PROCESSOR or /tmp.
set -euo pipefail

PKG=org.nativescript.plugindemo
ACTIVITY=com.tns.NativeScriptActivity
TP=${TRACE_PROCESSOR:-/tmp/trace_processor}
OUT=${TMPDIR:-/tmp}/busy-trace
mkdir -p "$OUT"

CONFIG='buffers { size_kb: 65536 fill_policy: RING_BUFFER }
data_sources { config { name: "linux.ftrace" ftrace_config {
  ftrace_events: "sched/sched_switch"
  atrace_categories: "gfx" atrace_categories: "view"
  atrace_apps: "org.nativescript.plugindemo"
} } }
data_sources { config { name: "linux.process_stats" } }
duration_ms: 3000'

QUERY="
with d as (
  select case s.name
      when 'AChoreographer_frameCallback' then 'flush'
      when 'eglSwapBuffers' then 'swap'
      when 'AChoreographer_frameCallback64' then 'js'
      when 'traversal' then 'traversal' end as what, s.dur
  from slice s join thread_track tt on s.track_id = tt.id join thread t using(utid) join process p using(upid)
  where p.name = '$PKG' and t.tid = p.pid
    and s.name in ('AChoreographer_frameCallback', 'eglSwapBuffers', 'AChoreographer_frameCallback64', 'traversal')),
r as (select what, dur, row_number() over (partition by what order by dur) as rn, count(*) over (partition by what) as n from d)
select what, n, round(avg(dur) / 1e6, 2) as avg_ms,
  round(max(case when rn = cast(n * 0.95 as int) then dur end) / 1e6, 2) as p95_ms
from r group by what order by what;"

CPU_QUERY="
select case when t.tid = p.pid then 'main' else t.name end as who,
  round(sum(ts.dur) / 1e6 / 3, 1) as ms_per_s
from thread_state ts join thread t using(utid) join process p using(upid)
where p.name = '$PKG' and ts.state = 'Running'
  and (t.tid = p.pid or t.name in ('nsc-2d-render', 'RenderThread'))
group by who order by who;"

for scenario in "$@"; do
	adb shell am force-stop "$PKG"
	adb logcat -c
	adb shell am start -n "$PKG/$ACTIVITY" --es demo canvas-busy --es suite "$scenario" >/dev/null
	started=
	for _ in $(seq 1 300); do
		if adb logcat -d -s JS:* 2>/dev/null | grep -qF "BUSY|$scenario|measuring"; then
			started=1
			break
		fi
		sleep 0.2
	done
	if [ -z "$started" ]; then
		echo "$scenario | never started measuring"
		continue
	fi
	sleep 1
	echo "$CONFIG" | adb shell "perfetto --txt -c - -o /data/misc/perfetto-traces/busy.pftrace" >/dev/null 2>&1
	trace="$OUT/$(echo "$scenario" | tr ':,+' '-_p').pftrace"
	adb pull /data/misc/perfetto-traces/busy.pftrace "$trace" >/dev/null
	summary=$(echo "$QUERY" > "$OUT/q.sql" && "$TP" -q "$OUT/q.sql" "$trace" 2>/dev/null | tail -n +2 | tr -d '"' |
		awk -F, '{ printf "%s n=%s avg=%s p95=%s | ", $1, $2, $3, $4 }')
	cpu=$(echo "$CPU_QUERY" > "$OUT/c.sql" && "$TP" -q "$OUT/c.sql" "$trace" 2>/dev/null | tail -n +2 | tr -d '"' |
		awk -F, '{ printf "%s=%s ", $1, $2 }')
	echo "$scenario | $summary cpu ms/s: $cpu"
done
adb shell am force-stop "$PKG"
