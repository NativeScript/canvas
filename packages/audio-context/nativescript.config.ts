import { NativeScriptConfig } from '@nativescript/core';

// Local path packages on purpose: a repositoryURL makes SwiftPM clone this repo's
// multi-GB history before it reads Package.swift.
//  - CanvasNative: header-only V8 shim shared by the canvas plugins. The CLI keeps
//    the first plugin package of a given name, so every plugin ships the same one.
//  - AudioContextNativeBinary: the xcframework. Published packages point it at the
//    GitHub release zip, so SwiftPM downloads only the framework.
const SPMPackages = [
	{
		name: 'CanvasNative',
		libs: ['NativeScriptV8'],
		path: 'node_modules/@nativescript/audio-context/platforms/ios/NativeScriptV8',
	},
	{
		name: 'AudioContextNativeBinary',
		libs: ['AudioContextNative'],
		path: 'node_modules/@nativescript/audio-context/platforms/ios/AudioContextNativeBinary',
	},
];

export default {
	ios: { SPMPackages },
	visionos: { SPMPackages },
	tvos: { SPMPackages },
} as NativeScriptConfig;
