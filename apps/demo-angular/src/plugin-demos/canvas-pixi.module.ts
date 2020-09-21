import { NgModule, NO_ERRORS_SCHEMA } from '@angular/core';
import { NativeScriptCommonModule, NativeScriptRouterModule } from '@nativescript/angular';
import { CanvasPixiComponent } from './canvas-pixi.component';

@NgModule({
	imports: [NativeScriptCommonModule, NativeScriptRouterModule.forChild([{ path: '', component: CanvasPixiComponent }])],
	declarations: [CanvasPixiComponent],
	schemas: [NO_ERRORS_SCHEMA],
})
export class CanvasPixiModule {}
