import { NgModule, NO_ERRORS_SCHEMA } from '@angular/core';
import { NativeScriptCommonModule, NativeScriptRouterModule } from '@nativescript/angular';
import { CanvasComponent } from './canvas.component';

@NgModule({
	imports: [NativeScriptCommonModule, NativeScriptRouterModule.forChild([{ path: '', component: CanvasComponent }])],
	declarations: [CanvasComponent],
	schemas: [NO_ERRORS_SCHEMA],
})
export class CanvasModule {}
