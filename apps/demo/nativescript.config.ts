import { NativeScriptConfig } from '@nativescript/core';

export default {
	id: 'org.nativescript.plugindemo',
	appResourcesPath: '../../tools/assets/App_Resources',
	android: {
		v8Flags: '--expose_gc',
		markingMode: 'none',
		enableMultithreadedJavascript: true
	},
	appPath: 'src',
	ios: {
		discardUncaughtJsExceptions: false
	},
	cli: {
		packageManager: 'npm'
	}
} as NativeScriptConfig;
