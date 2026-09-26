import { OrientationChangedEventData, Screen } from '@nativescript/core';
import { Application } from '@nativescript/core';
/*
 Window Resize Stub
*/

declare const __WINDOWS__: boolean;

/**
 * A window metric read from the screen when asked, as on the web (it follows the monitor and the
 * window). A page may still assign it (libraries pin devicePixelRatio); the assigned value sticks.
 */
function liveMetric(name: string, read: () => number) {
	let assigned: number | undefined;
	const descriptor: PropertyDescriptor = {
		configurable: true,
		enumerable: true,
		get: () => assigned ?? read(),
		set: (value: number) => {
			assigned = value;
		},
	};
	Object.defineProperty(global, name, descriptor);
	if ((global as any).window !== global) {
		Object.defineProperty((global as any).window, name, descriptor);
	}
}

// On Windows the scale and size come from the app window, which does not exist yet when this
// module loads (the screen reports 1x, 1920x1080 until then).
liveMetric('devicePixelRatio', () => Screen.mainScreen.scale || 1);

// const screenWidth = Screen.mainScreen.widthPixels;
// const screenHeight = Screen.mainScreen.heightPixels;
const screenWidth = Screen.mainScreen.widthDIPs;
const screenHeight = Screen.mainScreen.heightDIPs;
if (typeof __WINDOWS__ !== 'undefined' && __WINDOWS__) {
	for (const name of ['innerWidth', 'clientWidth']) {
		liveMetric(name, () => Screen.mainScreen.widthDIPs);
	}
	for (const name of ['innerHeight', 'clientHeight']) {
		liveMetric(name, () => Screen.mainScreen.heightDIPs);
	}
} else {
	(global as any).window.innerWidth = (global as any).innerWidth = screenWidth;
	(global as any).window.clientWidth = (global as any).clientWidth = screenWidth;
	(global as any).window.innerHeight = (global as any).innerHeight = screenHeight;
	(global as any).window.clientHeight = (global as any).clientHeight = screenHeight;
}
(global as any).window.screen = (global as any).screen = (global as any).screen || {};
(global as any).window.screen.orientation = (global as any).screen.orientation = (global as any).screen.orientation || (global as any).clientWidth < (global as any).clientHeight ? 0 : 90;
if (!(global as any).__TNS_BROWSER_POLYFILL_RESIZE) {
	(global as any).__TNS_BROWSER_POLYFILL_RESIZE = true;
	Application.on(Application.orientationChangedEvent, (args: OrientationChangedEventData) => {
		// const width = Screen.mainScreen.widthPixels;
		// const height = Screen.mainScreen.heightPixels;

		const portrait = args.newValue === 'portrait';
		const width = portrait ? screenWidth : screenHeight;
		const height = portrait ? screenHeight : screenWidth;

		(global as any).window.innerWidth = (global as any).innerWidth = width;
		(global as any).window.clientWidth = (global as any).clientWidth = width;
		(global as any).window.innerHeight = (global as any).innerHeight = height;
		(global as any).window.clientHeight = (global as any).clientHeight = height;
		(global as any).window.orientation = (global as any).orientation = args.newValue === 'portrait' ? 0 : 90;
		(global as any).window.screen.orientation = (global as any).screen.orientation = (global as any).orientation;
		if ((global as any).emitter && (global as any).emitter.emit) {
			(global as any).emitter.emit('resize');
		}
	});
}
