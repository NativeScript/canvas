import { NativeScriptConfig } from '@nativescript/core';

export default {
	ios: {
		SPMPackages: [
			{
				name: 'CanvasNative',
				libs: ['NativeScriptV8'],
				// Header-only SwiftPM shim (V8 headers + empty stub) shipped inside
				// this npm package. Referenced by local path on purpose: a
				// repositoryURL reference makes SwiftPM clone this repo's entire
				// multi-GB git history just to read Package.swift, which takes
				// 20+ minutes on first fetch. The real native code
				// (CanvasNative.xcframework) also ships in this package under
				// platforms/ios.
				path: 'node_modules/@nativescript/canvas/platforms/ios/NativeScriptV8',
			},
		],
	},
} as NativeScriptConfig;
