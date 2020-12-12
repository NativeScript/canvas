import { Screen } from '@nativescript/core';
import { Application } from '@nativescript/core';
/*
 Window Resize Stub
*/

const scale = Screen.mainScreen.scale;
const width = Screen.mainScreen.widthPixels;
const height = Screen.mainScreen.heightPixels;
(global as any).window.devicePixelRatio = (global as any).devicePixelRatio = 1;
(global as any).window.innerWidth = (global as any).innerWidth = width;
(global as any).window.clientWidth = (global as any).clientWidth = width;
(global as any).window.innerHeight = (global as any).innerHeight = height;
(global as any).window.clientHeight = (global as any).clientHeight = height;
(global as any).window.screen = (global as any).screen = (global as any).screen || {};
(global as any).window.screen.orientation = (global as any).screen.orientation = (global as any).screen.orientation || (global as any).clientWidth < (global as any).clientHeight ? 0 : 90;
if (!(global as any).__TNS_BROWSER_POLYFILL_RESIZE) {
	(global as any).__TNS_BROWSER_POLYFILL_RESIZE = true;
	Application.on(Application.orientationChangedEvent, (args) => {
		const width = Screen.mainScreen.widthPixels;
		const height = Screen.mainScreen.heightDIPs;
		(global as any).window.devicePixelRatio = (global as any).devicePixelRatio = 1;
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