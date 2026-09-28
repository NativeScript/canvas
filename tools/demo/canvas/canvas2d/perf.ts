import { Canvas } from '@nativescript/canvas';

/**
 * Canvas 2D binding-layer benchmark.
 *
 * Measures the JS -> native cost of the 2D context surface so the V8 fast-API
 * work can be judged against something other than a frame counter. Every result
 * is emitted as a single `PERF|...` line so a run can be scraped off logcat:
 *
 *   PERF|<group>|<name>|<iterations>|<best ms>|<ns per op>|<opt status>
 *
 * The three groups are the point of the whole file:
 *   wrapper  - `ctx.foo()`, i.e. what demo/user code actually calls. Goes through
 *              the CanvasRenderingContext2D TS class, which forwards to `ctx.native`.
 *   native   - `ctx.native.foo()`, the same call with the TS class taken out of the
 *              path. wrapper-minus-native is the cost of the forwarding layer.
 *   setter   - property accessors, which are *not* registered on the fast API.
 */

const MAGLEV = 1 << 5;
const TURBOFAN = 1 << 6;

let getOptimizationStatus: ((f: Function) => number) | null = null;
try {
	// --allow-natives-syntax is set in nativescript.config.ts. Built through
	// `new Function` so the bundler never has to parse the `%` syntax, and so a
	// runtime without the flag just leaves this null instead of failing to load.
	getOptimizationStatus = new Function('f', 'return %GetOptimizationStatus(f);') as (f: Function) => number;
	getOptimizationStatus(function () {});
} catch (e) {
	getOptimizationStatus = null;
}

function tier(fn: Function): string {
	if (!getOptimizationStatus) {
		return 'unknown';
	}
	try {
		const status = getOptimizationStatus(fn);
		if (status & TURBOFAN) {
			return 'turbofan';
		}
		if (status & MAGLEV) {
			return 'maglev';
		}
		return `tier:${status}`;
	} catch (e) {
		return 'unknown';
	}
}

interface BenchOptions {
	/** calls per timed run */
	iters?: number;
	/** timed runs; the best one is reported */
	reps?: number;
	/**
	 * Number of native calls a single loop iteration makes. Lets a bench that has
	 * to pair calls (save/restore) still report a per-call number.
	 */
	callsPerIter?: number;
}

const results: Array<{ group: string; name: string; nsPerOp: number; tier: string }> = [];

function bench(group: string, name: string, fn: (n: number) => void, opts: BenchOptions = {}) {
	const iters = opts.iters ?? 100_000;
	const reps = opts.reps ?? 5;
	const callsPerIter = opts.callsPerIter ?? 1;

	// Warm up into the optimizing tier. Fast-API calls are only emitted from
	// optimized code, so measuring before this point measures the interpreter.
	fn(Math.min(iters, 20_000));
	fn(Math.min(iters, 20_000));

	let best = Infinity;
	for (let r = 0; r < reps; r++) {
		const start = performance.now();
		fn(iters);
		const elapsed = performance.now() - start;
		if (elapsed < best) {
			best = elapsed;
		}
	}

	const nsPerOp = (best * 1e6) / (iters * callsPerIter);
	const t = tier(fn);
	results.push({ group, name, nsPerOp, tier: t });
	console.log(`PERF|${group}|${name}|${iters * callsPerIter}|${best.toFixed(3)}|${nsPerOp.toFixed(1)}|${t}`);
}

export function runCanvasPerf(canvas: Canvas, willReadFrequently = false) {
	// `willReadFrequently` swaps the GPU surface for a CPU raster one, which is
	// the only way getImageData/putImageData stop costing a GPU round trip. Run
	// the suite both ways to see what that trade actually buys and costs.
	const ctx = canvas.getContext('2d', willReadFrequently ? { willReadFrequently: true } : undefined) as any;
	if (!ctx) {
		console.log('PERF|error|no 2d context');
		return;
	}
	console.log(`PERF|meta|willReadFrequently|${willReadFrequently}`);
	const nat = ctx.native;
	const width = (canvas as any).width;
	const height = (canvas as any).height;

	console.log(`PERF|meta|canvas|${width}x${height}`);
	console.log(`PERF|meta|natives|${getOptimizationStatus ? 'available' : 'unavailable'}`);
	console.log(`PERF|header|group|name|calls|bestMs|nsPerOp|tier`);

	let sink = 0;

	// ---------------------------------------------------------------- transforms
	// save/restore and translate/rotate touch no pixels, so these come closest to
	// isolating the call overhead itself.

	bench(
		'wrapper',
		'save+restore',
		(n) => {
			for (let i = 0; i < n; i++) {
				ctx.save();
				ctx.restore();
			}
		},
		{ callsPerIter: 2 },
	);

	bench(
		'native',
		'save+restore',
		(n) => {
			for (let i = 0; i < n; i++) {
				nat.save();
				nat.restore();
			}
		},
		{ callsPerIter: 2 },
	);

	bench('wrapper', 'translate', (n) => {
		for (let i = 0; i < n; i++) {
			ctx.translate(0.0001, 0);
		}
	});

	bench('native', 'translate', (n) => {
		for (let i = 0; i < n; i++) {
			nat.translate(0.0001, 0);
		}
	});

	ctx.resetTransform();

	bench('wrapper', 'rotate', (n) => {
		for (let i = 0; i < n; i++) {
			ctx.rotate(0.0001);
		}
	});

	bench('native', 'rotate', (n) => {
		for (let i = 0; i < n; i++) {
			nat.rotate(0.0001);
		}
	});

	ctx.resetTransform();

	bench('wrapper', 'setTransform', (n) => {
		for (let i = 0; i < n; i++) {
			ctx.setTransform(1, 0, 0, 1, 0, 0);
		}
	});

	bench('native', 'setTransform', (n) => {
		for (let i = 0; i < n; i++) {
			nat.setTransform(1, 0, 0, 1, 0, 0);
		}
	});

	// ---------------------------------------------------------------- path build
	// beginPath is hoisted out of the inner loop so the path does not grow without
	// bound; at 1000 lineTo per beginPath it costs 0.1% of the measurement.

	bench('wrapper', 'lineTo', (n) => {
		const chunks = n / 1000;
		for (let c = 0; c < chunks; c++) {
			ctx.beginPath();
			ctx.moveTo(0, 0);
			for (let i = 0; i < 1000; i++) {
				ctx.lineTo(i & 255, i & 127);
			}
		}
	});

	bench('native', 'lineTo', (n) => {
		const chunks = n / 1000;
		for (let c = 0; c < chunks; c++) {
			nat.beginPath();
			nat.moveTo(0, 0);
			for (let i = 0; i < 1000; i++) {
				nat.lineTo(i & 255, i & 127);
			}
		}
	});

	bench('wrapper', 'beginPath', (n) => {
		for (let i = 0; i < n; i++) {
			ctx.beginPath();
		}
	});

	bench('native', 'beginPath', (n) => {
		for (let i = 0; i < n; i++) {
			nat.beginPath();
		}
	});

	// ---------------------------------------------------------------- rasterizing
	// These do real Skia work, so the wrapper/native gap is a smaller share of the
	// total -- which is itself the useful signal.

	bench(
		'wrapper',
		'fillRect',
		(n) => {
			for (let i = 0; i < n; i++) {
				ctx.fillRect(0, 0, 10, 10);
			}
		},
		{ iters: 50_000 },
	);

	bench(
		'native',
		'fillRect',
		(n) => {
			for (let i = 0; i < n; i++) {
				nat.fillRect(0, 0, 10, 10);
			}
		},
		{ iters: 50_000 },
	);

	bench(
		'wrapper',
		'strokeRect',
		(n) => {
			for (let i = 0; i < n; i++) {
				ctx.strokeRect(0, 0, 10, 10);
			}
		},
		{ iters: 50_000 },
	);

	bench(
		'native',
		'strokeRect',
		(n) => {
			for (let i = 0; i < n; i++) {
				nat.strokeRect(0, 0, 10, 10);
			}
		},
		{ iters: 50_000 },
	);

	bench(
		'wrapper',
		'clearRect',
		(n) => {
			for (let i = 0; i < n; i++) {
				ctx.clearRect(0, 0, 10, 10);
			}
		},
		{ iters: 50_000 },
	);

	bench(
		'native',
		'clearRect',
		(n) => {
			for (let i = 0; i < n; i++) {
				nat.clearRect(0, 0, 10, 10);
			}
		},
		{ iters: 50_000 },
	);

	// ---------------------------------------------------------------- accessors
	// Not on the fast API. Two colours are alternated so any single-value cache in
	// the setter cannot make this look better than real drawing code does.

	bench(
		'setter',
		'fillStyle (string)',
		(n) => {
			for (let i = 0; i < n; i++) {
				ctx.fillStyle = i & 1 ? '#00ff00' : '#ff0000';
			}
		},
		{ iters: 50_000 },
	);

	bench(
		'setter',
		'fillStyle (string, native)',
		(n) => {
			for (let i = 0; i < n; i++) {
				nat.fillStyle = i & 1 ? '#00ff00' : '#ff0000';
			}
		},
		{ iters: 50_000 },
	);

	bench(
		'setter',
		'strokeStyle (string)',
		(n) => {
			for (let i = 0; i < n; i++) {
				ctx.strokeStyle = i & 1 ? '#4060A3' : '#A36040';
			}
		},
		{ iters: 50_000 },
	);

	bench('setter', 'lineWidth (number)', (n) => {
		for (let i = 0; i < n; i++) {
			ctx.lineWidth = i & 1 ? 2 : 1;
		}
	});

	bench('setter', 'lineWidth (number, native)', (n) => {
		for (let i = 0; i < n; i++) {
			nat.lineWidth = i & 1 ? 2 : 1;
		}
	});

	bench('setter', 'globalAlpha (number)', (n) => {
		for (let i = 0; i < n; i++) {
			ctx.globalAlpha = i & 1 ? 1 : 0.5;
		}
	});

	bench(
		'setter',
		'font (string)',
		(n) => {
			for (let i = 0; i < n; i++) {
				ctx.font = i & 1 ? '12px Arial' : '16px Arial';
			}
		},
		{ iters: 20_000 },
	);

	// ---------------------------------------------------------------- text

	bench(
		'wrapper',
		'measureText',
		(n) => {
			for (let i = 0; i < n; i++) {
				sink += ctx.measureText('Hello world').width;
			}
		},
		{ iters: 20_000 },
	);

	bench(
		'native',
		'measureText',
		(n) => {
			for (let i = 0; i < n; i++) {
				sink += nat.measureText('Hello world').width;
			}
		},
		{ iters: 20_000 },
	);

	bench(
		'wrapper',
		'fillText',
		(n) => {
			for (let i = 0; i < n; i++) {
				ctx.fillText('Hello world', 10, 10);
			}
		},
		{ iters: 20_000 },
	);

	// The worst case for the shaped-layout cache: a string that is never drawn
	// twice, so every call re-shapes. A frame counter or a clock looks like this.
	bench(
		'wrapper',
		'fillText (unique string)',
		(n) => {
			for (let i = 0; i < n; i++) {
				ctx.fillText('frame ' + i, 10, 10);
			}
		},
		{ iters: 5_000 },
	);

	// Alternating two strings, which is what an HUD drawing a label and a value
	// does; it also proves the cache is not a single-entry memo.
	bench(
		'wrapper',
		'fillText (two strings)',
		(n) => {
			for (let i = 0; i < n; i++) {
				ctx.fillText(i & 1 ? 'Hello world' : 'Goodbye world', 10, 10);
			}
		},
		{ iters: 20_000 },
	);

	if (sink === Number.MIN_VALUE) {
		console.log('PERF|sink', sink);
	}

	// ---------------------------------------------------------------- paths
	// What the particle demos repeat: a fresh sub-path per object.

	bench(
		'wrapper',
		'arc',
		(n) => {
			for (let i = 0; i < n; i++) {
				ctx.arc(10, 10, 5, 0, 6.28);
			}
		},
		{ iters: 50_000 },
	);

	bench(
		'wrapper',
		'beginPath+arc+fill',
		(n) => {
			for (let i = 0; i < n; i++) {
				ctx.beginPath();
				ctx.arc(10, 10, 5, 0, 6.28);
				ctx.fill();
			}
		},
		{ iters: 20_000, callsPerIter: 3 },
	);

	// One two-point stroked sub-path -- the swarm demo's inner loop.
	bench(
		'wrapper',
		'beginPath+moveTo+lineTo+stroke',
		(n) => {
			for (let i = 0; i < n; i++) {
				ctx.beginPath();
				ctx.moveTo(0, 0);
				ctx.lineTo(10, 10);
				ctx.stroke();
			}
		},
		{ iters: 20_000, callsPerIter: 4 },
	);

	// ---------------------------------------------------------------- clearRect
	// A whole-canvas clear is what every animation frame starts with, and it is a
	// different shape of work from clearing a small rect.
	bench(
		'wrapper',
		'clearRect (full canvas)',
		(n) => {
			for (let i = 0; i < n; i++) {
				ctx.clearRect(0, 0, width, height);
			}
		},
		{ iters: 20_000 },
	);

	// ---------------------------------------------------------------- images
	const IMG = 64;
	const asset = new (global as any).ImageAsset();
	const pixels = new Uint8Array(IMG * IMG * 4);
	for (let i = 0; i < pixels.length; i += 4) {
		pixels[i] = i & 0xff;
		pixels[i + 1] = (i >> 8) & 0xff;
		pixels[i + 2] = 0x80;
		pixels[i + 3] = 0xff;
	}
	const loaded = asset.loadFromBytesSync(IMG, IMG, pixels);
	console.log(`PERF|meta|imageAsset|${loaded}|${asset.width}x${asset.height}`);

	if (loaded) {
		bench(
			'wrapper',
			'drawImage (dx,dy)',
			(n) => {
				for (let i = 0; i < n; i++) {
					ctx.drawImage(asset, 10, 10);
				}
			},
			{ iters: 20_000 },
		);

		bench(
			'wrapper',
			'drawImage (9 args)',
			(n) => {
				for (let i = 0; i < n; i++) {
					ctx.drawImage(asset, 0, 0, IMG, IMG, 10, 10, IMG, IMG);
				}
			},
			{ iters: 20_000 },
		);
	}

	// ---------------------------------------------------------------- pixels
	// GPU-backed surfaces have to read back, so these are expected to be orders
	// of magnitude above everything else; the point is to know by how much.
	bench(
		'wrapper',
		'createImageData (64x64)',
		(n) => {
			for (let i = 0; i < n; i++) {
				sink += ctx.createImageData(IMG, IMG).width;
			}
		},
		{ iters: 5_000 },
	);

	// 1x1 allocates 4 bytes, so whatever this still costs is the wrapper object,
	// not the pixel buffer.
	bench(
		'wrapper',
		'createImageData (1x1)',
		(n) => {
			for (let i = 0; i < n; i++) {
				sink += ctx.createImageData(1, 1).width;
			}
		},
		{ iters: 5_000 },
	);

	// The same construction pattern through other wrapper types.
	bench(
		'wrapper',
		'createLinearGradient',
		(n) => {
			for (let i = 0; i < n; i++) {
				sink += ctx.createLinearGradient(0, 0, 10, 10) ? 1 : 0;
			}
		},
		{ iters: 10_000 },
	);

	const imageData = ctx.createImageData(IMG, IMG);

	bench(
		'wrapper',
		'putImageData (64x64)',
		(n) => {
			for (let i = 0; i < n; i++) {
				ctx.putImageData(imageData, 0, 0);
			}
		},
		{ iters: 2_000 },
	);

	bench(
		'wrapper',
		'getImageData (64x64)',
		(n) => {
			for (let i = 0; i < n; i++) {
				sink += ctx.getImageData(0, 0, IMG, IMG).width;
			}
		},
		{ iters: 500 },
	);

	// ---------------------------------------------------------------- real scene
	// The rn-skia comparison scene: 450 rotated, filled and stroked rects. This is
	// the number that maps onto a frame budget.
	sceneBench(ctx, width, height);

	// Visual check that the font path still resolves typefaces: several distinct
	// shorthands, drawn last so they survive on screen. Measurements are already
	// taken, so this costs nothing.
	drawFontProof(ctx, height);

	summary();
}

function drawFontProof(ctx: any, height: number) {
	const samples = ['16px sans-serif', 'bold 28px sans-serif', 'italic 24px serif', '20px monospace', 'bold italic 22px serif'];

	ctx.fillStyle = '#111111';
	let y = height * 0.55;
	for (const font of samples) {
		ctx.font = font;
		ctx.fillText(`${font} — Sphinx of black quartz, judge my vow 0123`, 20, y);
		y += 60;
	}
	ctx.font = '10px sans-serif';
}

function sceneBench(ctx: any, width: number, height: number) {
	const numberOfBoxes = 450;
	const Size = 25;
	const SizeWidth = Size;
	const SizeHeight = Size * 0.45;
	const pos = { x: width / 2, y: height * 0.25 };

	const rects = new Array(numberOfBoxes).fill(0).map((_, i) => ({
		x: 5 + ((i * Size) % width),
		y: 25 + Math.floor(i / (width / Size)) * Size,
		width: SizeWidth,
		height: SizeHeight,
	}));

	const drawScene = (frames: number) => {
		for (let f = 0; f < frames; f++) {
			ctx.clearRect(0, 0, width, height);
			for (let r = 0; r < rects.length; r++) {
				const rect = rects[r];
				ctx.save();
				ctx.translate(rect.x, rect.y);
				ctx.rotate(Math.atan2(pos.y - rect.y, pos.x - rect.x));
				ctx.fillStyle = '#00ff00';
				ctx.fillRect(0, 0, rect.width, rect.height);
				ctx.strokeStyle = '#4060A3';
				ctx.lineWidth = 2;
				ctx.strokeRect(0, 0, rect.width, rect.height);
				ctx.restore();
			}
		}
	};

	drawScene(20);

	let best = Infinity;
	const frames = 60;
	for (let r = 0; r < 5; r++) {
		const start = performance.now();
		drawScene(frames);
		const elapsed = performance.now() - start;
		if (elapsed < best) {
			best = elapsed;
		}
	}

	const msPerFrame = best / frames;
	// 8 canvas calls per box plus one clearRect per frame.
	const callsPerFrame = numberOfBoxes * 8 + 1;
	console.log(`PERF|scene|450 rotated rects|${callsPerFrame}|${msPerFrame.toFixed(3)}|${((msPerFrame * 1e6) / callsPerFrame).toFixed(1)}|${tier(drawScene)}`);
	console.log(`PERF|scene|ms per frame|${msPerFrame.toFixed(3)}|fps ceiling|${(1000 / msPerFrame).toFixed(1)}`);
}

function summary() {
	const byName = new Map<string, { wrapper?: number; native?: number }>();
	for (const r of results) {
		if (r.group !== 'wrapper' && r.group !== 'native') {
			continue;
		}
		const entry = byName.get(r.name) ?? {};
		entry[r.group] = r.nsPerOp;
		byName.set(r.name, entry);
	}

	console.log('PERF|--- wrapper overhead (ns/op) ---');
	for (const [name, entry] of byName) {
		if (entry.wrapper === undefined || entry.native === undefined) {
			continue;
		}
		const delta = entry.wrapper - entry.native;
		const pct = (delta / entry.native) * 100;
		console.log(`PERF|overhead|${name}|${entry.native.toFixed(1)}|${entry.wrapper.toFixed(1)}|${delta.toFixed(1)}|${pct.toFixed(1)}%`);
	}
	console.log('PERF|done');
}
