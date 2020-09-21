import { NgModule, NO_ERRORS_SCHEMA } from '@angular/core';
import { NativeScriptCommonModule, NativeScriptRouterModule } from '@nativescript/angular';
import { CanvasBabylonComponent } from './canvas-babylon.component';

@NgModule({
	imports: [NativeScriptCommonModule, NativeScriptRouterModule.forChild([{ path: '', component: CanvasBabylonComponent }])],
	declarations: [CanvasBabylonComponent],
	schemas: [NO_ERRORS_SCHEMA],
})
export class CanvasBabylonModule {}
