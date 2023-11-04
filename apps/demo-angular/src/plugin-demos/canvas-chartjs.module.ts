import { NgModule, NO_ERRORS_SCHEMA } from '@angular/core';
import { NativeScriptCommonModule, NativeScriptRouterModule } from '@nativescript/angular';
import { CanvasChartjsComponent } from './canvas-chartjs.component';

@NgModule({
	imports: [NativeScriptCommonModule, NativeScriptRouterModule.forChild([{ path: '', component: CanvasChartjsComponent }])],
  declarations: [CanvasChartjsComponent],
  schemas: [ NO_ERRORS_SCHEMA]
})
export class CanvasChartjsModule {}
