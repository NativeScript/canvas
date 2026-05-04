import { Observable, EventData, Page, Device } from '@nativescript/core';
import { DemoSharedCanvasThree } from '@demo/shared';
import {} from '@nativescript/canvas-three';

export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new DemoModel();
}

export function pageLoaded(args) {
	if (__IOS__) {
		if (parseFloat(Device.osVersion) >= 26.0) {
			args.object.viewController.navigationController.interactiveContentPopGestureRecognizer.enabled = false;
		}
	}
}

export class DemoModel extends DemoSharedCanvasThree {}
