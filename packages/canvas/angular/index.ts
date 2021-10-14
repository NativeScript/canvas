import {NgModule, NO_ERRORS_SCHEMA} from '@angular/core';
import {registerElement} from '@nativescript/angular';

registerElement('Canvas', () => require('@nativescript/canvas').Canvas);


@NgModule({
  schemas: [NO_ERRORS_SCHEMA]
})
// @ts-ignore
export class CanvasModule {
}
