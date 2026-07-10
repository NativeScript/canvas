import { NativeScriptConfig } from '@nativescript/core';

export default {
	ios: {
		SPMPackages: [
			{
				name: 'CanvasNative',
				libs: ['NativeScriptV8'],
				// Header-only SwiftPM shim (V8 headers + empty stub) shipped inside
				// this npm package — same shim @nativescript/canvas ships. The CLI
				// dedupes same-name packages across plugins, so installing both
				// plugins adds it only once. Referenced by local path on purpose: a
				// repositoryURL reference makes SwiftPM clone the canvas repo's
				// entire multi-GB git history just to read Package.swift.
				path: 'node_modules/@nativescript/audio-context/platforms/ios/NativeScriptV8',
			},
		],
	},
	visionos: {
		SPMPackages: [
			{
				name: 'CanvasNative',
				libs: ['NativeScriptV8'],
				path: 'node_modules/@nativescript/audio-context/platforms/ios/NativeScriptV8',
			},
		],
	},
} as NativeScriptConfig;
