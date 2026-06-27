import { NativeScriptConfig } from '@nativescript/core';

export default {
	ios: {
		SPMPackages: [
			{
				name: 'CanvasNative',
				libs: ['NativeScriptV8'],
				version: '2.1.0',
				repositoryURL: 'https://github.com/NativeScript/canvas.git',
			},
		],
	},
} as NativeScriptConfig;
