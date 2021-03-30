import { NgModule } from '@angular/core';
import { Routes } from '@angular/router';
import { NativeScriptRouterModule } from '@nativescript/angular';

import { HomeComponent } from './home.component';

const routes: Routes = [
   { path: '', redirectTo: '/home', pathMatch: 'full' },
   { path: 'home', component: HomeComponent },
	{ path: 'canvas', loadChildren: () => import('./plugin-demos/canvas.module').then(m => m.CanvasModule) },
	{ path: 'canvas-babylon', loadChildren: () => import('./plugin-demos/canvas-babylon.module').then(m => m.CanvasBabylonModule) },
	{ path: 'canvas-media', loadChildren: () => import('./plugin-demos/canvas-media.module').then(m => m.CanvasMediaModule) },
	{ path: 'canvas-phaser', loadChildren: () => import('./plugin-demos/canvas-phaser.module').then(m => m.CanvasPhaserModule) },
	{ path: 'canvas-phaser-ce', loadChildren: () => import('./plugin-demos/canvas-phaser-ce.module').then(m => m.CanvasPhaserCeModule) },
	{ path: 'canvas-pixi', loadChildren: () => import('./plugin-demos/canvas-pixi.module').then(m => m.CanvasPixiModule) },
	{ path: 'canvas-polyfill', loadChildren: () => import('./plugin-demos/canvas-polyfill.module').then(m => m.CanvasPolyfillModule) },
	{ path: 'canvas-three', loadChildren: () => import('./plugin-demos/canvas-three.module').then(m => m.CanvasThreeModule) }
];

@NgModule({
	imports: [NativeScriptRouterModule.forRoot(routes)],
	exports: [NativeScriptRouterModule],
})
export class AppRoutingModule {}
