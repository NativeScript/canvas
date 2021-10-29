require('@nativescript/canvas-polyfill');
import { init } from '@nativescript/canvas';
init();
import { Application } from '@nativescript/core';
Application.run({ moduleName: 'app-root' });
