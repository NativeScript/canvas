import { normalizeSourcePath } from './common';

declare const __non_webpack_require__: (specifier: string) => any;

// Loaded on import, so a missing module leaves AudioContext undefined in canvas-polyfill's probe.
export const Native = __non_webpack_require__('system_lib://audiocontext.node');

export function nativePath(source: string): string {
	return normalizeSourcePath(source.startsWith('file://') ? decodeURIComponent(source.substring('file://'.length)) : source);
}
