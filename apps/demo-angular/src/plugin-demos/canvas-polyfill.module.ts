import { NgModule, NO_ERRORS_SCHEMA } from '@angular/core';
import { NativeScriptCommonModule, NativeScriptRouterModule } from '@nativescript/angular';
import { CanvasPolyfillComponent } from './canvas-polyfill.component';

@NgModule({
	imports: [NativeScriptCommonModule, NativeScriptRouterModule.forChild([{ path: '', component: CanvasPolyfillComponent }])],
	declarations: [CanvasPolyfillComponent],
	schemas: [NO_ERRORS_SCHEMA],
})
export class CanvasPolyfillModule {}
