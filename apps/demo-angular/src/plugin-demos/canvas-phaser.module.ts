import { NgModule, NO_ERRORS_SCHEMA } from '@angular/core';
import { NativeScriptCommonModule, NativeScriptRouterModule } from '@nativescript/angular';
import { CanvasPhaserComponent } from './canvas-phaser.component';

@NgModule({
	imports: [NativeScriptCommonModule, NativeScriptRouterModule.forChild([{ path: '', component: CanvasPhaserComponent }])],
	declarations: [CanvasPhaserComponent],
	schemas: [NO_ERRORS_SCHEMA],
})
export class CanvasPhaserModule {}
