// No Windows media backend yet (MediaPlayer/MediaFoundation). Importing fails loudly, so callers
// that probe for it (canvas-polyfill) fall back, and nothing half-works.
throw new Error('@nativescript/canvas-media is not supported on Windows yet');
