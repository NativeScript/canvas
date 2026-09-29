import { EventData, GridLayout, Observable, Page } from '@nativescript/core';
import { runBusyBench } from '@demo/shared';
import { launchArgs } from '../launch-args';

// adb shell am start -n org.nativescript.plugindemo/com.tns.NativeScriptActivity --es demo canvas-busy --es suite 2d+views
export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new BusyModel();
}

class BusyModel extends Observable {
	private ran = false;

	constructor() {
		super();
		this.set('status', 'starting…');
	}

	hostLoaded(args: EventData) {
		if (this.ran) {
			return;
		}
		this.ran = true;
		const scenario = launchArgs.suite ?? '2d+views';
		this.set('status', `running ${scenario}…`);
		runBusyBench(args.object as GridLayout, scenario)
			.then(() => this.set('status', `${scenario} done — see logcat (BUSY|…)`))
			.catch((e) => {
				console.log(`BUSY|${scenario}|error|${e?.message ?? e}`);
				this.set('status', `failed: ${e?.message ?? e}`);
			});
	}
}
