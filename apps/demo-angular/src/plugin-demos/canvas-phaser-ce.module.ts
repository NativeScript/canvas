import { NgModule, NO_ERRORS_SCHEMA } from '@angular/core';
import { NativeScriptCommonModule, NativeScriptRouterModule } from '@nativescript/angular';
import { CanvasPhaserCeComponent } from './canvas-phaser-ce.component';

@NgModule({
	imports: [NativeScriptCommonModule, NativeScriptRouterModule.forChild([{ path: '', component: CanvasPhaserCeComponent }])],
	declarations: [CanvasPhaserCeComponent],
	schemas: [NO_ERRORS_SCHEMA],
})
export class CanvasPhaserCeModule {}
