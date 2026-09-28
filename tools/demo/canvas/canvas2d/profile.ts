/**
 * Frame profiler for the canvas 2D demos.
 *
 * Wraps the real demo's requestAnimationFrame callback to time the work it does
 * per frame (excluding the vsync wait), and counts the 2D calls it makes, so a
 * slow demo can be attributed to JS work vs. canvas calls vs. call volume
 * instead of guessed at. Results go to logcat as `PROF|` lines.
 */

const COUNTED = ['fillRect', 'clearRect', 'strokeRect', 'beginPath', 'closePath', 'moveTo', 'lineTo', 'arc', 'fill', 'stroke', 'save', 'restore', 'translate', 'rotate', 'scale', 'fillText', 'measureText'];

const COUNTED_SETTERS = ['fillStyle', 'strokeStyle', 'lineWidth', 'globalAlpha', 'font'];

export interface ProfileResult {
	label: string;
	frames: number;
	medianMs: number;
	p95Ms: number;
	maxMs: number;
	callsPerFrame: Record<string, number>;
}

/**
 * Counts calls on the live context. `getContext('2d')` caches its wrapper, so
 * patching the object here is enough -- the demo gets this same instance.
 */
function instrument(ctx: any): { counts: Record<string, number>; restore: () => void } {
	const counts: Record<string, number> = {};
	const undo: Array<() => void> = [];

	for (const name of COUNTED) {
		const original = ctx[name];
		if (typeof original !== 'function') {
			continue;
		}
		counts[name] = 0;
		ctx[name] = function (...args: any[]) {
			counts[name]++;
			return original.apply(this, args);
		};
		undo.push(() => {
			delete ctx[name];
		});
	}

	// Accessors live on the prototype; shadow them with a counting own-property.
	for (const name of COUNTED_SETTERS) {
		const proto = Object.getPrototypeOf(ctx);
		const desc = Object.getOwnPropertyDescriptor(proto, name);
		if (!desc?.set) {
			continue;
		}
		counts[name + '='] = 0;
		Object.defineProperty(ctx, name, {
			configurable: true,
			get: desc.get ? () => desc.get.call(ctx) : undefined,
			set: (value) => {
				counts[name + '=']++;
				desc.set.call(ctx, value);
			},
		});
		undo.push(() => {
			delete ctx[name];
		});
	}

	return { counts, restore: () => undo.forEach((f) => f()) };
}

function percentile(sorted: number[], p: number): number {
	if (!sorted.length) {
		return 0;
	}
	const i = Math.min(sorted.length - 1, Math.max(0, Math.round((p / 100) * (sorted.length - 1))));
	return sorted[i];
}

/**
 * Runs `start(canvas)` -- a real demo entry point -- and reports the first
 * `frames` frames it renders.
 */
export function profileDemo(canvas: any, start: (canvas: any) => void, label: string, frames = 180, onDone?: (r: ProfileResult) => void) {
	const ctx = canvas.getContext('2d');
	if (!ctx) {
		console.log('PROF|error|no 2d context');
		return;
	}

	const { counts, restore } = instrument(ctx);

	const durations: number[] = [];
	const callTotals: Record<string, number> = {};
	const originalRaf = (global as any).requestAnimationFrame;
	let finished = false;

	(global as any).requestAnimationFrame = function (cb: FrameRequestCallback) {
		return originalRaf.call(global, (t: number) => {
			if (finished) {
				return;
			}
			for (const k of Object.keys(counts)) {
				counts[k] = 0;
			}

			const startedAt = performance.now();
			cb(t);
			const elapsed = performance.now() - startedAt;

			durations.push(elapsed);
			for (const k of Object.keys(counts)) {
				callTotals[k] = (callTotals[k] ?? 0) + counts[k];
			}

			if (durations.length >= frames && !finished) {
				finished = true;
				report();
			}
		});
	};

	function report() {
		(global as any).requestAnimationFrame = originalRaf;
		restore();

		// Drop the first 20 frames -- shader/pipeline warmup, not steady state.
		const steady = durations.slice(20).sort((a, b) => a - b);
		const n = Math.max(1, durations.length - 20);
		const callsPerFrame: Record<string, number> = {};
		for (const k of Object.keys(callTotals)) {
			if (callTotals[k] > 0) {
				callsPerFrame[k] = callTotals[k] / durations.length;
			}
		}

		const result: ProfileResult = {
			label,
			frames: durations.length,
			medianMs: percentile(steady, 50),
			p95Ms: percentile(steady, 95),
			maxMs: percentile(steady, 100),
			callsPerFrame,
		};

		console.log(`PROF|${label}|frames|${result.frames}|median|${result.medianMs.toFixed(2)}|p95|${result.p95Ms.toFixed(2)}|max|${result.maxMs.toFixed(2)}`);
		const entries = Object.entries(callsPerFrame).sort((a, b) => b[1] - a[1]);
		for (const [name, per] of entries) {
			console.log(`PROF|${label}|calls|${name}|${per.toFixed(1)}`);
		}
		console.log(`PROF|${label}|totalCalls|${entries.reduce((s, [, v]) => s + v, 0).toFixed(1)}`);
		console.log(`PROF|${label}|done`);
		onDone?.(result);
	}

	start(canvas);
}
