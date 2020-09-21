import { registerNativeViewElement } from 'svelte-native/dom'

registerNativeViewElement("canvas", () => require("@nativescript/canvas").Canvas);
