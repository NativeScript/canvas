declare var global: any;
require('@nativescript/canvas-polyfill');
import * as THREE from 'three';
(global as any).THREE = global.window.THREE = global.THREE || THREE;
