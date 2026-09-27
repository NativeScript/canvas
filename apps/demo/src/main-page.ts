import { EventData, Frame, Page } from '@nativescript/core';
import { MainViewModel } from './main-view-model';
import { launchArgs, launchArgsReady } from './launch-args';

export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new MainViewModel();

	launchArgsReady().then(() => {
		const demo = launchArgs.demo;
		if (demo) {
			// One-shot: clear first so Back returns to this list.
			launchArgs.demo = undefined;
			// Out of the first navigation.
			setTimeout(() => {
				Frame.topmost().navigate({ moduleName: `plugin-demos/${demo}` });
			}, 0);
		}
	});
}
