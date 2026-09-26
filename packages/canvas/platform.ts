/**
 * What the current platform's canvas host can do. Call sites branch on these capabilities, never
 * on a list of platforms, so adding a platform only changes this file.
 */

// Module-local: bundlers without Windows support do not define it (and older core typings do
// not declare it), so it is read through `typeof`, which still folds to a constant where defined.
declare const __WINDOWS__: boolean;

const WINDOWS = typeof __WINDOWS__ !== 'undefined' && __WINDOWS__;

/** The native module is the Node-API addon (crates/canvas-napi), not engine-specific bindings. */
export const NAPI_HOST: boolean = WINDOWS; /* || macOS || Linux */

/** The canvas view creates the rendering context and hands out its pointer. */
export const POINTER_CONTEXT_HOST: boolean = __APPLE__ || NAPI_HOST;
