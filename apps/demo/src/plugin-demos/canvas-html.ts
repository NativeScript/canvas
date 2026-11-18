import { EventData, Observable, Page, View } from '@nativescript/core';
import type { Canvas } from '@nativescript/canvas';

export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new HtmlInCanvasModel();
}

class HtmlInCanvasModel extends Observable {
	private raf = 0;
	private paints = 0;

	constructor() {
		super();
		this.set('name', '');
		this.set('tilt', 8);
		this.set('taps', 0);
		this.set('status', 'waiting for the first paint…');
	}

	tapped() {
		this.set('taps', this.get('taps') + 1);
	}

	canvasLoaded(args: EventData) {
		const canvas = args.object as Canvas & any;
		const form = canvas.getViewById('form') as View;
		const ctx = canvas.getContext('2d');
		let ready = false;

		canvas.on('paint', (event: any) => {
			ready = true;
			this.paints++;
			this.set('status', `paint #${this.paints}: ${event.changedElements.length} changed`);
		});

		const draw = (time: number) => {
			this.raf = requestAnimationFrame(draw);
			if (!ready) {
				return;
			}
			const width = canvas.width;
			const height = canvas.height;
			ctx.setTransform(1, 0, 0, 1, 0, 0);
			const sky = ctx.createLinearGradient(0, 0, width, height);
			sky.addColorStop(0, '#4f46e5');
			sky.addColorStop(1, '#ec4899');
			ctx.fillStyle = sky;
			ctx.fillRect(0, 0, width, height);

			const size = canvas.getElementImageDefaultSize(form);
			const angle = ((this.get('tilt') + Math.sin(time / 900) * 4) * Math.PI) / 180;
			ctx.translate(width / 2, height / 2);
			ctx.rotate(angle);
			ctx.shadowColor = 'rgba(0,0,0,0.35)';
			ctx.shadowBlur = 30;
			ctx.drawElementImage(form, -size.width / 2, -size.height / 2);
			ctx.shadowBlur = 0;
		};
		this.raf = requestAnimationFrame(draw);
	}

	canvasUnloaded() {
		cancelAnimationFrame(this.raf);
	}
}
