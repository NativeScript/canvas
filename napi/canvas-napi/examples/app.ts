import type { NativeWindow } from '@nativescript/foundation/views/window/native-window.js';
import type { Window } from '@nativescript/foundation/views/window/window.js';

objc.import('AppKit');

@NativeClass
class AppDelegate extends NSObject implements NSApplicationDelegate {
	window?: NativeWindow;
	running = true;
	isActive = true;
	static windowTitle: string;
	static ObjCProtocols = [NSApplicationDelegate];
	static ObjCExposedMethods = {
		showMainWindow: { returns: interop.types.void, params: [interop.types.id] },
		themeChanged: { returns: interop.types.void, params: [interop.types.id] },
	};

	applicationDidFinishLaunching(_notification: NSNotification) {
		NSApp.activateIgnoringOtherApps(false);
		NSApp.stop(this);
		// Allow users to customize the app's Touch Bar items
		NSApplication.sharedApplication.isAutomaticCustomizeTouchBarMenuItemEnabled = true;
		NSDistributedNotificationCenter.defaultCenter.addObserverSelectorNameObject(this, 'themeChanged', 'AppleInterfaceThemeChangedNotification', null);
		RunLoop();
	}

	applicationShouldHandleReopenHasVisibleWindows(sender: NSApplication, hasVisibleWindows: boolean): boolean {
		if (!hasVisibleWindows) {
			(sender.windows.firstObject as NSWindow).makeKeyAndOrderFront(sender);
		}
		return true;
	}

	applicationWillTerminate(_notification: NSNotification): void {
		this.running = false;
		process.exit(0);
	}

	showMainWindow(_id: this) {
		NativeScriptApplication.showMainWindow();
	}

	themeChanged(_id: this) {
		console.log('themeChanged', NSApp.effectiveAppearance.name === 'NSAppearanceNameDarkAqua' ? 'dark' : 'light');
	}
}

function RunLoop() {
	let delay = 2;
	let lastEventTime = 0;

	const loop = () => {
		const event = NSApp.nextEventMatchingMaskUntilDateInModeDequeue(NSEventMask.Any, null, 'kCFRunLoopDefaultMode', true);

		const timeSinceLastEvent = Date.now() - lastEventTime;
		if (event != null) {
			NSApp.sendEvent(event);
			delay = timeSinceLastEvent < 32 ? 2 : 8;
			lastEventTime = Date.now();
		} else {
			delay = timeSinceLastEvent > 6000 ? 128 : timeSinceLastEvent > 4000 ? 64 : timeSinceLastEvent > 2000 ? 16 : 8;
		}

		if (NativeScriptApplication.delegate.running) {
			const time = NativeScriptApplication.ensure120FPS ? 4 : NativeScriptApplication.ensure60FPS ? 8 : delay;
			setTimeout(loop, NativeScriptApplication.ensure60FPS ? 8 : time);
		}
	};
	setTimeout(loop, 0);
}

export default class Application {
	static delegate: AppDelegate;
	static application: NSApplication;
	static rootView: any;
	static window: Window;
	static appMenu: NSMenu;
	static ensure60FPS: boolean;
	static ensure120FPS: boolean;
	static initEditMenu: boolean;

	static launch() {
		if (!(document.body instanceof HTMLElement)) {
			throw new Error('document.body instance of NSView');
		}
		Application.rootView = document.body as unknown;
		Application.rootView?.connectedCallback();

		if (NativeScriptApplication.window) {
			NativeScriptApplication.window.open();
		} else {
			throw new Error('Window is not initialized');
		}

		Application.application = NSApplication.sharedApplication;
		Application.delegate = AppDelegate.new();
		Application.delegate.window = NativeScriptApplication.window.nativeView;
		Application.createMenu();
		NSApp.delegate = Application.delegate;
		NSApp.setActivationPolicy(NSApplicationActivationPolicy.Regular);
		NSApp.run();
	}

	static createMenu() {
		if (!Application.appMenu) {
			const menu = NSMenu.new();
			NSApp.mainMenu = menu;
			Application.appMenu = menu;
		}
	}

	static showMainWindow() {
		// override
	}
}

declare global {
	var NativeScriptApplication: typeof Application;
}
globalThis.NativeScriptApplication = Application;

NativeScriptApplication.ensure120FPS = true;
