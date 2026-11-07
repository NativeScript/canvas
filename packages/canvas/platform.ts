/**
 * What the current platform's canvas host can do. Call sites branch on these capabilities, never
 * on a list of platforms, so adding a platform only changes this file.
 */

// Module-local: bundlers without Windows / Linux support do not define them (and older core
// typings do not declare them), so they are read through `typeof`, which still folds to a
// constant where defined.
declare const __WINDOWS__: boolean;
declare const __LINUX__: boolean;

const WINDOWS = typeof __WINDOWS__ !== 'undefined' && __WINDOWS__;
const LINUX = typeof __LINUX__ !== 'undefined' && __LINUX__;

/** The native module is the Node-API addon (crates/canvas-napi), not engine-specific bindings. */
export const NAPI_HOST: boolean = WINDOWS || LINUX; /* || macOS */

/** The canvas view creates the rendering context and hands out its pointer. */
export const POINTER_CONTEXT_HOST: boolean = __APPLE__ || NAPI_HOST;
