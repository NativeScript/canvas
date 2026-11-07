import { NativeScriptConfig } from '@nativescript/core';

export default {
	id: 'org.nativescript.plugindemo',
	appResourcesPath: '../../tools/assets/App_Resources',
	android: {
		v8Flags: '--expose_gc --allow-natives-syntax --turbo-fast-api-calls',
		markingMode: 'none',
		discardUncaughtJsExceptions: false,
	},
	// Light like the Android app theme (Theme.AppCompat.Light), whatever the desktop uses
	linux: {
		appearance: 'light',
	},
	appPath: 'src',
	ios: {
		discardUncaughtJsExceptions: false,
		SPMPackages: [
			{
				name: 'CanvasNative',
				libs: ['NativeScriptV8'],
				path: '../../nativescript-v8',
			},
		],
	},
	visionos: {
		discardUncaughtJsExceptions: false,
		SPMPackages: [
			{
				name: 'CanvasNative',
				libs: ['NativeScriptV8'],
				path: '../../nativescript-v8',
			},
		],
	},
	tvos: {
		discardUncaughtJsExceptions: false,
		SPMPackages: [
			{
				name: 'CanvasNative',
				libs: ['NativeScriptV8'],
				path: '../../nativescript-v8',
			},
		],
	},
	cli: {
		packageManager: 'npm',
	},
} as NativeScriptConfig;
