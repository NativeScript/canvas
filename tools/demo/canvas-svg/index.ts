import { GridLayout, Screen } from '@nativescript/core';
import { DemoSharedBase } from '../utils';
import { Svg } from '@nativescript/canvas-svg';
import { Canvas, ImageAsset } from '@nativescript/canvas';
require('@nativescript/canvas-polyfill');

const SVG_NS = 'http://www.w3.org/2000/svg';

function el(tag: string, attrs: Record<string, string> = {}): any {
	const node = document.createElementNS(SVG_NS, tag) as any;
	for (const key in attrs) {
		node.setAttribute(key, attrs[key]);
	}
	return node;
}

function mountSvg(view, row: number, col: number, width = 150, height = 150): any {
	const svg = el('svg', { width: String(width), height: String(height) });
	const svgView: Svg = svg.nativeElement;
	svgView.width = String(width);
	svgView.height = String(height);
	(svgView as any).row = row;
	(svgView as any).col = col;
	view.addChild(svgView);
	return svg;
}

function mountCanvas(view, row: number, col: number, onReady: (canvas: Canvas) => void): Canvas {
	const canvas = new Canvas();
	(canvas as any).row = row;
	(canvas as any).col = col;
	// Layout size goes through style: on Canvas, `width`/`height` are the backing store.
	canvas.style.width = '100%' as any;
	canvas.style.height = '100%' as any;
	canvas.on('ready', () => {
		// Like a web canvas, the backing store starts at 300x150 whatever the layout size.
		canvas.width = Math.round(canvas.clientWidth * Screen.mainScreen.scale) as any;
		canvas.height = Math.round(canvas.clientHeight * Screen.mainScreen.scale) as any;
		onReady(canvas);
	});
	view.addChild(canvas);
	return canvas;
}

function mountSvgWithSrc(view, row: number, col: number, src: string, gpu = true, threaded = false): Svg {
	const svgView = new Svg();
	(svgView as any).row = row;
	(svgView as any).col = col;
	svgView.width = '100%';
	svgView.height = '100%';
	svgView.gpu = gpu;
	(svgView as any).threaded = threaded;
	svgView.src = src;
	view.addChild(svgView);
	return svgView;
}

/**
 * One of each animation element, small enough to check by eye:
 * a circle whose radius pulses, a square that spins about its own centre, a dot running the
 * path, and a bar that turns red exactly once, two seconds in, and stays red.
 */
const SMIL_SOURCE = `<svg xmlns="http://www.w3.org/2000/svg" width="150" height="150" viewBox="0 0 150 150">
	<circle cx="30" cy="30" r="10" fill="crimson">
		<animate attributeName="r" values="6;18;6" dur="1.5s" repeatCount="indefinite"/>
	</circle>
	<rect x="90" y="15" width="30" height="30" fill="seagreen">
		<animateTransform attributeName="transform" type="rotate" from="0 105 30" to="360 105 30"
		                  dur="3s" repeatCount="indefinite"/>
	</rect>
	<path d="M10 100 Q75 60 140 100" stroke="#999" fill="none"/>
	<circle r="6" fill="darkorange">
		<animateMotion path="M10 100 Q75 60 140 100" dur="2s" rotate="auto" repeatCount="indefinite"/>
	</circle>
	<rect x="10" y="125" width="130" height="14" fill="steelblue">
		<set attributeName="fill" to="orangered" begin="2s" fill="freeze"/>
	</rect>
</svg>`;

export class DemoSharedCanvasSvg extends DemoSharedBase {
	private svg: Svg;
	private svg2: Svg;
	private svg3: Svg;
	private svg4: Svg;

	viewLoaded(args) {
		const view = args.object;

		// Test 1 (row 0, col 0): basic shape breadth: rect, circle, ellipse, line, polygon, polyline.
		const t1 = mountSvg(view, 0, 0);
		t1.appendChild(el('rect', { x: '5', y: '5', width: '50', height: '35', fill: 'green' }));
		t1.appendChild(el('circle', { cx: '110', cy: '25', r: '20', fill: 'gold' }));
		t1.appendChild(el('ellipse', { cx: '30', cy: '90', rx: '25', ry: '15', fill: 'purple' }));
		t1.appendChild(el('line', { x1: '70', y1: '70', x2: '140', y2: '110', stroke: 'red', 'stroke-width': '3' }));
		t1.appendChild(el('polygon', { points: '75,120 100,145 50,145', fill: 'lime', stroke: 'black' }));
		t1.appendChild(el('polyline', { points: '5,148 20,130 35,148 50,130', fill: 'none', stroke: 'blue', 'stroke-width': '2' }));

		// Test 2 (row 0, col 1): path + <g> with multiple children + id/<use> resolution.
		const t2 = mountSvg(view, 0, 1);
		t2.appendChild(el('path', { d: 'M10 10 L70 10 L40 60 Z', fill: 'teal' }));
		const g = el('g', { transform: 'translate(80,10)' });
		g.appendChild(el('path', { d: 'M0 0 L60 0', stroke: 'red' }));
		g.appendChild(el('path', { d: 'M0 15 L60 15', stroke: 'black' }));
		g.appendChild(el('path', { d: 'M0 30 L60 30', stroke: 'blue' }));
		t2.appendChild(g);
		const base = el('circle', { cx: '20', cy: '110', r: '10', fill: 'orange' });
		base.id = 'sharedCircle';
		t2.appendChild(base);
		const use1 = el('use', { href: '#sharedCircle', x: '30', fill: 'orange' });
		t2.appendChild(use1);
		const use2 = el('use', { href: '#sharedCircle', x: '60', fill: 'none', stroke: 'red' });
		t2.appendChild(use2);

		// Test 3 (row 1, col 0): text.
		const t3 = mountSvg(view, 1, 0);
		const text = el('text', { x: '5', y: '30', 'font-size': '16', fill: 'darkblue' });
		text.appendChild(document.createTextNode('I love SVG!'));
		t3.appendChild(text);
		const tspanHost = el('text', { x: '5', y: '60', 'font-size': '14' });
		const tspan1 = el('tspan', { fill: 'red' });
		tspan1.appendChild(document.createTextNode('red '));
		const tspan2 = el('tspan', { fill: 'green' });
		tspan2.appendChild(document.createTextNode('green'));
		tspanHost.appendChild(tspan1);
		tspanHost.appendChild(tspan2);
		t3.appendChild(tspanHost);

		// Test 4 (row 1, col 1): live mutation: setAttribute after mount, and removeChild.
		const t4 = mountSvg(view, 1, 1);
		const mutRect = el('rect', { x: '5', y: '5', width: '40', height: '40', fill: 'blue' });
		t4.appendChild(mutRect);
		const toRemove = el('circle', { cx: '110', cy: '25', r: '20', fill: 'red' });
		t4.appendChild(toRemove);
		const label = el('text', { x: '5', y: '90', 'font-size': '12', fill: 'black' });
		label.appendChild(document.createTextNode('before mutation'));
		t4.appendChild(label);

		// Test 5 (row 2, col 0): SMIL. Skia parses none of these elements (they are lifted out
		// of the source and driven by our own clock), so this tile is the whole feature: an
		// interpolated attribute, a transform, motion along a path, and a discrete `set`.
		const smil = mountSvgWithSrc(view, 2, 0, SMIL_SOURCE);
		smil.on('animationEnd', () => console.log('[canvas-svg test] smil animation ended'));

		// Test 6 (row 2, col 1): the real thing: an exported animation of 200+ elements, which
		// renders as one frozen frame without the SMIL engine.
		// The heaviest tile, rasterized on its own thread: the rest of the UI should stay at the
		// display's rate while this one paces itself.
		const rocket = mountSvgWithSrc(view, 2, 1, '~/assets/file-assets/svg/rocket.svg', true, true);

		// Test 7 (row 3, col 0): the same shape of export as the rocket, but animated with CSS
		// @keyframes rather than SMIL, driven by the same clock.
		const solar = mountSvgWithSrc(view, 3, 0, '~/assets/file-assets/svg/solar-system-animation.svg');
		solar.on('animationEnd', () => console.log('[canvas-svg test] solar animation ended'));

		// Test 8 (row 3, col 1): four views of one source share a document and clock, so they
		// move in lockstep.
		const grid = new GridLayout();
		grid.rows = '*,*' as any;
		grid.columns = '*,*' as any;
		(grid as any).row = 3;
		(grid as any).col = 1;
		view.addChild(grid);
		for (let i = 0; i < 4; i++) {
			mountSvgWithSrc(grid, Math.floor(i / 2), i % 2, SMIL_SOURCE, true, i % 2 === 1);
		}

		// Test 9 (row 4, col 0): drawImage(svgView) every frame, whole and a 3x crop; tracks tile 5.
		mountCanvas(view, 4, 0, (canvas) => {
			const ctx = canvas.getContext('2d') as any;
			let frames = 0;
			let spent = 0;
			const draw = () => {
				const w = canvas.width as number;
				const h = canvas.height as number;
				const side = Math.min(w / 2, h);
				ctx.clearRect(0, 0, w, h);
				const start = Date.now();
				ctx.drawImage(smil, 0, 0, side, side);
				ctx.drawImage(smil, 0, 0, 75, 75, w / 2, 0, side, side);
				spent += Date.now() - start;
				if (++frames === 120) {
					console.log(`[canvas-svg test] drawImage(svg) x2: ${(spent / frames).toFixed(2)} ms/frame`);
					frames = spent = 0;
				}
				requestAnimationFrame(draw);
			};
			requestAnimationFrame(draw);
		});

		// Test 10 (row 4, col 1): ImageAsset.loadSvg from a file, as a pattern, and sized by width.
		mountCanvas(view, 4, 1, (canvas) => {
			const ctx = canvas.getContext('2d') as any;
			const w = canvas.width as number;
			const h = canvas.height as number;
			const dot = `<svg xmlns="${SVG_NS}" width="16" height="16"><circle cx="8" cy="8" r="5" fill="#ccc"/></svg>`;
			const tile = new ImageAsset();
			const sized = new ImageAsset();
			console.log('[canvas-svg test] loadSvgSync pattern:', tile.loadSvgSync(dot, { scale: Screen.mainScreen.scale }), tile.width, 'x', tile.height);
			console.log('[canvas-svg test] loadSvgSync width-only:', sized.loadSvgSync(SMIL_SOURCE, { width: 60, time: 2.5 }), sized.width, 'x', sized.height);
			const rocketAsset = new ImageAsset();
			rocketAsset
				.loadSvg('~/assets/file-assets/svg/rocket.svg', { scale: Screen.mainScreen.scale })
				.then((ok) => {
					console.log('[canvas-svg test] loadSvg rocket:', ok, rocketAsset.width, 'x', rocketAsset.height);
					ctx.fillStyle = ctx.createPattern(tile, 'repeat');
					ctx.fillRect(0, 0, w, h);
					const k = Math.min(w / rocketAsset.width, h / rocketAsset.height);
					ctx.drawImage(rocketAsset, 0, 0, rocketAsset.width * k, rocketAsset.height * k);
					ctx.drawImage(sized, w - sized.width, h - sized.height);
				})
				.catch((error) => console.log('[canvas-svg test] loadSvg rocket failed:', error));
		});

		// Both tiles also stand in for the GPU context-loss path: losing a context mid-animation
		// is the case that matters, because a static picture hides a failure to recover.
		for (const tile of [smil, rocket]) {
			tile.on('contextLost', () => console.log('[canvas-svg test] gpu context lost, on the cpu raster now'));
			tile.on('contextRestored', () => console.log('[canvas-svg test] gpu context restored'));
		}
		setTimeout(() => {
			console.log('[canvas-svg test] forcing a gpu context loss; the animation should not stutter or blank');
			(smil as any).debugLoseContext?.();
		}, 4000);

		setTimeout(() => {
			console.log('[canvas-svg test] mutating rect fill blue->orange, width 40->100');
			mutRect.setAttribute('fill', 'orange');
			mutRect.setAttribute('width', '100');
			console.log('[canvas-svg test] removing red circle via removeChild');
			t4.removeChild(toRemove);
			label.textContent = 'after mutation';
			(t4.nativeElement as any).__redraw();
		}, 2000);

		return;
	}

	testIt() {
		console.log('test canvas-svg!');
	}
}
