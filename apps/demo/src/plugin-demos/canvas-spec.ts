import { EventData, Observable, Page } from '@nativescript/core';
import { runSpecTests, specResults, setSpecPageCanvas } from '@demo/shared';
import { launchArgs } from '../launch-args';

/**
 * Web-spec conformance for the drawing contexts. One command:
 *
 *   adb shell am start -n org.nativescript.plugindemo/com.tns.NativeScriptActivity \
 *     --es demo canvas-spec
 *   adb logcat -d | grep 'SPEC|'
 *
 * `--es suite <prefix>` narrows it to a group or a single suite. Each test
 * prints `SPEC|start|…` first, so after a native abort the last such line names
 * the culprit.
 *
 * Tests build their own offscreen canvases; the one on this page is only for
 * WebGPU, which cannot configure a surface that has no window.
 */
export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new SpecModel();
}

class SpecModel extends Observable {
	private started = false;

	constructor() {
		super();
		this.set('status', 'waiting for the surface…');
		this.set('report', '');
	}

	canvasReady(args) {
		if (this.started) {
			return;
		}
		this.started = true;
		setSpecPageCanvas(args.object);
		this.set('status', 'running…');

		setTimeout(() => {
			const group = (launchArgs.suite as any) ?? 'all';
			runSpecTests(group)
				.then((summary) => {
					this.set('status', `${summary.passed}/${summary.total} passed, ${summary.failed} failed — see logcat (SPEC|…)`);
					const failures = (specResults ?? []).filter((r) => !r.ok);
					this.set('report', failures.length === 0 ? 'all passing' : failures.map((r) => `${r.suite} › ${r.name}\n    ${r.message}`).join('\n\n'));
				})
				.catch((e) => {
					console.log('SPEC|error|' + (e?.message ?? e));
					console.log(e?.stack ?? '');
					this.set('status', 'failed: ' + (e?.message ?? e));
				});
		}, 500);
	}
}
