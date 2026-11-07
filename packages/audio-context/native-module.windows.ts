import { normalizeSourcePath } from './common';

declare const __non_webpack_require__: (specifier: string) => any;

// Loaded on import, so a missing module leaves AudioContext undefined in canvas-polyfill's probe.
export const Native = __non_webpack_require__('system_lib://audiocontext.node');

/** `file:///C:/x` and `/C:/x` name a drive path on Windows. */
export function nativePath(source: string): string {
	const path = normalizeSourcePath(source.startsWith('file:///') ? source.substring('file://'.length) : source);
	return /^\/[a-zA-Z]:[\\/]/.test(path) ? path.substring(1) : path;
}
