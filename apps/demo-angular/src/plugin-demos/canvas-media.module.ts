import { NgModule, NO_ERRORS_SCHEMA } from '@angular/core';
import { NativeScriptCommonModule, NativeScriptRouterModule } from '@nativescript/angular';
import { CanvasMediaComponent } from './canvas-media.component';

@NgModule({
	imports: [NativeScriptCommonModule, NativeScriptRouterModule.forChild([{ path: '', component: CanvasMediaComponent }])],
	declarations: [CanvasMediaComponent],
	schemas: [NO_ERRORS_SCHEMA],
})
export class CanvasMediaModule {}
