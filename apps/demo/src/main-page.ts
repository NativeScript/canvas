import { EventData, Frame, Page } from '@nativescript/core';
import { MainViewModel } from './main-view-model';
import { launchArgs } from './launch-args';

export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new MainViewModel();

	const demo = launchArgs.demo;
	if (demo) {
		// One-shot: clear first so Back returns to this list.
		launchArgs.demo = undefined;
		setTimeout(() => {
			Frame.topmost().navigate({ moduleName: `plugin-demos/${demo}` });
		}, 0);
	}
}
