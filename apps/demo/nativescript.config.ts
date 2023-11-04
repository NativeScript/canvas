import { NativeScriptConfig } from '@nativescript/core';

export default {
	id: 'org.nativescript.plugindemo',
	appResourcesPath: '../../tools/assets/App_Resources',
	android: {
		v8Flags: '--expose_gc --allow-natives-syntax --turbo-fast-api-calls',
		markingMode: 'none',
		discardUncaughtJsExceptions: false,
	},
	appPath: 'src',
	ios: {
		discardUncaughtJsExceptions: false
	},
	cli: {
		packageManager: 'npm'
	}
} as NativeScriptConfig;
