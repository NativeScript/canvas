// No Windows audio backend yet (AudioGraph). Importing fails loudly, so callers that probe for it
// (canvas-polyfill) leave AudioContext undefined and apps feature-detect it as absent.
throw new Error('@nativescript/audio-context is not supported on Windows yet');
