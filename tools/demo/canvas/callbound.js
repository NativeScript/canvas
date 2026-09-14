/**
 * The call-bound half of the WebView A/B.
 *
 * star-warp answered "who rasterizes faster" -- it is fill-bound, and the answer
 * was the WebView by ~10-15%. This answers a different question: **what does one
 * canvas call cost from JS?** Every op here is chosen to touch as few pixels as
 * possible (1x1 rects, pure state changes, path building with no stroke), so the
 * time is dominated by crossing from JS into the canvas implementation rather
 * than by drawing.
 *
 * Scope, stated plainly: for canvas 2D this measures *recording* cost on both
 * sides -- neither implementation rasterizes inside the timed loop. That is the
 * quantity the "in-process FFI is cheaper than the browser's bindings" claim is
 * actually about. Rasterization was already measured by star-warp.
 *
 * This file is loaded verbatim by both halves -- a <script src> in the WebView
 * page and a webpack import on the NativeScript side -- so neither can drift.
 */
(function (root) {
	function median(xs) {
		const s = xs.slice().sort(function (a, b) {
			return a - b;
		});
		return s[Math.floor(s.length / 2)];
	}

	let sink = 0;

	root.__callBound = function (ctx, emit, N, REPS) {
		N = N || 20000;
		REPS = REPS || 7;

		// Every case must actually be honoured by the implementation -- nothing
		// here is dead code a compiler could hoist out of the loop.
		const CASES = [
			{
				name: 'fillRect1x1',
				fn: function () {
					for (let i = 0; i < N; i++) ctx.fillRect(i & 255, (i >> 8) & 255, 1, 1);
				},
			},
			{
				name: 'save+restore',
				fn: function () {
					for (let i = 0; i < N; i++) {
						ctx.save();
						ctx.restore();
					}
				},
			},
			{
				name: 'translate',
				fn: function () {
					for (let i = 0; i < N; i++) ctx.translate(i & 1 ? 1 : -1, 0);
				},
			},
			{
				name: 'setTransform',
				fn: function () {
					for (let i = 0; i < N; i++) ctx.setTransform(1, 0, 0, 1, i & 63, 0);
				},
			},
			{
				name: 'fillStyle(str)',
				fn: function () {
					const colors = ['#ff0000', '#00ff00', '#0000ff', '#ffff00'];
					for (let i = 0; i < N; i++) ctx.fillStyle = colors[i & 3];
				},
			},
			{
				name: 'globalAlpha',
				fn: function () {
					for (let i = 0; i < N; i++) ctx.globalAlpha = (i & 7) / 8 + 0.1;
				},
			},
			{
				name: 'path build',
				fn: function () {
					for (let i = 0; i < N; i++) {
						ctx.beginPath();
						ctx.moveTo(i & 255, 0);
						ctx.lineTo(i & 255, 8);
					}
				},
			},
			{
				// Same string and same `.width` read as the native harness's
				// measureText, so the two numbers are directly comparable.
				name: 'measureText',
				fn: function () {
					let w = 0;
					for (let i = 0; i < N; i++) w += ctx.measureText('Hello world').width;
					sink = w;
				},
			},
		];

		for (let c = 0; c < CASES.length; c++) {
			const kase = CASES[c];
			const times = [];
			kase.fn(); // warm the JIT before any sample is kept
			for (let r = 0; r < REPS; r++) {
				const t0 = performance.now();
				kase.fn();
				times.push(performance.now() - t0);
			}
			const ms = median(times);
			emit(kase.name + '|' + ((ms * 1e6) / N).toFixed(1) + '|' + ms.toFixed(2));
		}
	};
})(typeof globalThis !== 'undefined' ? globalThis : this);
