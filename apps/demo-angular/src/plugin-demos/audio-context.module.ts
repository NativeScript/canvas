import { NgModule, NO_ERRORS_SCHEMA } from '@angular/core';
import { NativeScriptCommonModule, NativeScriptRouterModule } from '@nativescript/angular';
import { AudioContextComponent } from './audio-context.component';

@NgModule({
	imports: [NativeScriptCommonModule, NativeScriptRouterModule.forChild([{ path: '', component: AudioContextComponent }])],
	declarations: [AudioContextComponent],
	schemas: [NO_ERRORS_SCHEMA],
})
export class AudioContextModule {}
