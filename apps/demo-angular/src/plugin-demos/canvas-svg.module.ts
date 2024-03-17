import { NgModule, NO_ERRORS_SCHEMA } from '@angular/core';
import { NativeScriptCommonModule, NativeScriptRouterModule } from '@nativescript/angular';
import { CanvasSvgComponent } from './canvas-svg.component';

@NgModule({
	imports: [NativeScriptCommonModule, NativeScriptRouterModule.forChild([{ path: '', component: CanvasSvgComponent }])],
  declarations: [CanvasSvgComponent],
  schemas: [ NO_ERRORS_SCHEMA]
})
export class CanvasSvgModule {}
