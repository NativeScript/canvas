import { DemoSharedBase } from '../utils';
import { Video } from '@nativescript/canvas-media';
import { GridLayout } from '@nativescript/core';

export class DemoSharedCanvasMedia extends DemoSharedBase {
	loaded(event) {
		//@ts-ignore
		const video = Video.createCustomView();
		video.loop = true;
		video.autoplay = true;
		video.controls = true;
		const view = event.object as GridLayout;
		video.src = 'https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4';
		view.addChild(video);
	}
}
