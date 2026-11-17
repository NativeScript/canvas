import { Observable, EventData, Page } from '@nativescript/core';
import { DemoSharedCanvasPixi, runPixiTextBench, runTextUploadBench } from '@demo/shared';
import { launchArgs } from '../launch-args';

export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new DemoModel();
}

/**
 * Text upload benchmarks, picked by launch argument:
 *   --demo=canvas-pixi --suite=txup:webgl|webgl2|webgpu        (TXUP| lines)
 *   --demo=canvas-pixi --suite=pxtext:webgl|webgpu:gpu|cpu|static[:count]   (PXTEXT| lines)
 */
export class DemoModel extends DemoSharedCanvasPixi {
	private benchRan = false;

	canvasLoaded(args) {
		const suite = launchArgs.suite;
		if (!suite || !(suite.startsWith('txup:') || suite.startsWith('pxtext:'))) {
			super.canvasLoaded(args);
			return;
		}
		if (this.benchRan) return;
		this.benchRan = true;

		const canvas = args.object;
		const [kind, a, b, c] = suite.split(':');
		// Wait for layout: the canvas has no size when `loaded` fires.
		const start = () => {
			if (!(canvas.clientWidth > 0 && canvas.clientHeight > 0)) {
				setTimeout(start, 100);
				return;
			}
			canvas.width = canvas.clientWidth * window.devicePixelRatio;
			canvas.height = canvas.clientHeight * window.devicePixelRatio;
			console.log(`BENCH|canvas ${canvas.width}x${canvas.height}`);
			const run = kind === 'txup' ? runTextUploadBench(canvas, a as any) : runPixiTextBench(canvas, a as any, b as any, c ? parseInt(c, 10) : undefined);
			run.catch((e) => console.log(`BENCH|error|${e?.message ?? e}|${e?.stack ?? ''}`));
		};
		setTimeout(start, 1000);
	}
}
