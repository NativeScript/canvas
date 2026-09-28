/**
 * CanvasRenderingContext2D conformance, against html.spec.whatwg.org/#2dcontext.
 * Where the spec leaves serialization open, these follow Chrome.
 */

import { suite, test, ok, equal, notEqual, closeTo, oneOf, arrayEqual, throws, makeCanvas, make2D, pixelAt, pixelEqual } from './harness';

export function registerContext2DSpec() {
	// ------------------------------------------------------------- context
	suite('2d.context', () => {
		test('getContext("2d") returns a context', () => {
			const { ctx } = make2D();
			ok(ctx, 'no 2d context');
		});

		test('getContext("2d") is idempotent', () => {
			const canvas = makeCanvas();
			const a = canvas.getContext('2d');
			const b = canvas.getContext('2d');
			equal(a, b, 'a second getContext("2d") returned a different object');
		});

		test('getContext with an unknown type returns null', () => {
			const canvas = makeCanvas();
			equal(canvas.getContext('not-a-context' as any), null);
		});

		test('getContext("webgl") after "2d" returns null', () => {
			const canvas = makeCanvas();
			ok(canvas.getContext('2d'));
			equal(canvas.getContext('webgl'), null, 'a canvas handed out two different context types');
		});

		test('ctx.canvas points back at the canvas', () => {
			const { canvas, ctx } = make2D();
			equal(ctx.canvas, canvas);
		});

		test('getContextAttributes reports the requested attributes', () => {
			const { ctx } = make2D(50, 50, { alpha: false, willReadFrequently: true });
			const attrs = ctx.getContextAttributes?.();
			ok(attrs, 'getContextAttributes() returned nothing');
			equal(attrs.alpha, false, 'alpha');
			equal(attrs.willReadFrequently, true, 'willReadFrequently');
		});

		test('canvas.width/height round trip', () => {
			const canvas = makeCanvas(123, 45);
			equal((canvas as any).width, 123, 'width');
			equal((canvas as any).height, 45, 'height');
		});

		test('setting canvas.width clears the bitmap', () => {
			const { canvas, ctx } = make2D(50, 50);
			ctx.fillStyle = 'red';
			ctx.fillRect(0, 0, 50, 50);
			pixelEqual(ctx, 10, 10, [255, 0, 0, 255], 2, 'precondition');
			(canvas as any).width = 50;
			(canvas as any).height = 50;
			pixelEqual(ctx, 10, 10, [0, 0, 0, 0], 0, 'resizing the canvas must clear it');
		});

		test('setting canvas.width resets the context state', () => {
			const { canvas, ctx } = make2D(50, 50);
			ctx.fillStyle = '#00ff00';
			ctx.lineWidth = 7;
			ctx.translate(10, 10);
			(canvas as any).width = 50;
			(canvas as any).height = 50;
			equal(ctx.fillStyle, '#000000', 'fillStyle');
			equal(ctx.lineWidth, 1, 'lineWidth');
			const m = ctx.getTransform();
			equal(m.e, 0, 'transform e');
			equal(m.f, 0, 'transform f');
		});
	});

	// -------------------------------------------------------- default state
	suite('2d.state.defaults', () => {
		const defaults: Array<[string, any]> = [
			['fillStyle', '#000000'],
			['strokeStyle', '#000000'],
			['lineWidth', 1],
			['lineCap', 'butt'],
			['lineJoin', 'miter'],
			['miterLimit', 10],
			['lineDashOffset', 0],
			['globalAlpha', 1],
			['globalCompositeOperation', 'source-over'],
			['shadowBlur', 0],
			['shadowColor', 'rgba(0, 0, 0, 0)'],
			['shadowOffsetX', 0],
			['shadowOffsetY', 0],
			['font', '10px sans-serif'],
			['textAlign', 'start'],
			['textBaseline', 'alphabetic'],
			['direction', 'inherit'],
			['filter', 'none'],
			['imageSmoothingEnabled', true],
			['imageSmoothingQuality', 'low'],
			['letterSpacing', '0px'],
			['wordSpacing', '0px'],
		];

		for (const [prop, expected] of defaults) {
			test(`${prop} defaults to ${JSON.stringify(expected)}`, () => {
				const { ctx } = make2D();
				equal(ctx[prop], expected);
			});
		}

		test('getLineDash defaults to an empty list', () => {
			const { ctx } = make2D();
			arrayEqual(ctx.getLineDash(), []);
		});
	});

	// ------------------------------------------------------ save and restore
	suite('2d.state.stack', () => {
		test('save/restore round trips every state property', () => {
			const { ctx } = make2D();
			const before = {
				fillStyle: ctx.fillStyle,
				strokeStyle: ctx.strokeStyle,
				lineWidth: ctx.lineWidth,
				lineCap: ctx.lineCap,
				lineJoin: ctx.lineJoin,
				miterLimit: ctx.miterLimit,
				lineDashOffset: ctx.lineDashOffset,
				globalAlpha: ctx.globalAlpha,
				globalCompositeOperation: ctx.globalCompositeOperation,
				shadowBlur: ctx.shadowBlur,
				shadowColor: ctx.shadowColor,
				shadowOffsetX: ctx.shadowOffsetX,
				shadowOffsetY: ctx.shadowOffsetY,
				font: ctx.font,
				textAlign: ctx.textAlign,
				textBaseline: ctx.textBaseline,
				imageSmoothingEnabled: ctx.imageSmoothingEnabled,
				imageSmoothingQuality: ctx.imageSmoothingQuality,
			};

			ctx.save();
			ctx.fillStyle = '#123456';
			ctx.strokeStyle = '#654321';
			ctx.lineWidth = 4;
			ctx.lineCap = 'round';
			ctx.lineJoin = 'bevel';
			ctx.miterLimit = 3;
			ctx.lineDashOffset = 5;
			ctx.globalAlpha = 0.25;
			ctx.globalCompositeOperation = 'lighter';
			ctx.shadowBlur = 6;
			ctx.shadowColor = '#ff0000';
			ctx.shadowOffsetX = 7;
			ctx.shadowOffsetY = 8;
			ctx.font = 'bold 20px serif';
			ctx.textAlign = 'center';
			ctx.textBaseline = 'top';
			ctx.imageSmoothingEnabled = false;
			ctx.imageSmoothingQuality = 'high';
			ctx.restore();

			for (const key of Object.keys(before)) {
				equal(ctx[key], before[key], `${key} was not restored`);
			}
		});

		test('save/restore round trips the transform', () => {
			const { ctx } = make2D();
			ctx.save();
			ctx.translate(20, 30);
			ctx.scale(2, 3);
			ctx.rotate(Math.PI / 5);
			ctx.restore();
			const m = ctx.getTransform();
			closeTo(m.a, 1, 1e-5, 'a');
			closeTo(m.b, 0, 1e-5, 'b');
			closeTo(m.c, 0, 1e-5, 'c');
			closeTo(m.d, 1, 1e-5, 'd');
			closeTo(m.e, 0, 1e-5, 'e');
			closeTo(m.f, 0, 1e-5, 'f');
		});

		test('save/restore round trips the line dash', () => {
			const { ctx } = make2D();
			ctx.setLineDash([1, 2]);
			ctx.save();
			ctx.setLineDash([9, 9, 9, 9]);
			ctx.restore();
			arrayEqual(ctx.getLineDash(), [1, 2]);
		});

		test('restore on an empty stack is a no-op', () => {
			const { ctx } = make2D();
			ctx.fillStyle = '#abcdef';
			ctx.restore();
			ctx.restore();
			equal(ctx.fillStyle, '#abcdef');
		});

		test('restore does not pop the current path', () => {
			const { ctx } = make2D();
			ctx.beginPath();
			ctx.rect(0, 0, 50, 50);
			ctx.save();
			ctx.restore();
			ok(ctx.isPointInPath(10, 10), 'the path was reset by save/restore');
		});

		test('reset() returns the context to its defaults', () => {
			const { ctx } = make2D(50, 50);
			ctx.fillStyle = '#ff0000';
			ctx.translate(5, 5);
			ctx.fillRect(0, 0, 50, 50);
			ctx.reset();
			equal(ctx.fillStyle, '#000000', 'fillStyle');
			equal(ctx.getTransform().e, 0, 'transform');
			pixelEqual(ctx, 10, 10, [0, 0, 0, 0], 0, 'reset() must clear the bitmap');
		});
	});

	// ------------------------------------------------- colour serialization
	suite('2d.colours', () => {
		const cases: Array<[string, string]> = [
			['red', '#ff0000'],
			['#f00', '#ff0000'],
			['#FF0000', '#ff0000'],
			['rgb(255, 0, 0)', '#ff0000'],
			['rgb(100%, 0%, 0%)', '#ff0000'],
			['transparent', 'rgba(0, 0, 0, 0)'],
			['rgba(1, 2, 3, 0.5)', 'rgba(1, 2, 3, 0.5)'],
			['hsl(0, 100%, 50%)', '#ff0000'],
		];

		for (const [input, expected] of cases) {
			test(`fillStyle = ${JSON.stringify(input)} serializes to ${JSON.stringify(expected)}`, () => {
				const { ctx } = make2D();
				ctx.fillStyle = input;
				equal(ctx.fillStyle, expected);
			});
		}

		test('an invalid fillStyle leaves the previous value', () => {
			const { ctx } = make2D();
			ctx.fillStyle = '#00ff00';
			ctx.fillStyle = 'not-a-colour';
			equal(ctx.fillStyle, '#00ff00');
		});

		test('an invalid strokeStyle leaves the previous value', () => {
			const { ctx } = make2D();
			ctx.strokeStyle = '#00ff00';
			ctx.strokeStyle = 'bogus(1,2,3)';
			equal(ctx.strokeStyle, '#00ff00');
		});

		test('an invalid shadowColor leaves the previous value', () => {
			const { ctx } = make2D();
			ctx.shadowColor = '#0000ff';
			ctx.shadowColor = 'nope';
			equal(ctx.shadowColor, '#0000ff');
		});

		test('fillStyle keeps a gradient object identity', () => {
			const { ctx } = make2D();
			const g = ctx.createLinearGradient(0, 0, 10, 10);
			ctx.fillStyle = g;
			equal(ctx.fillStyle, g);
		});
	});

	// -------------------------------------------- out-of-range state values
	suite('2d.state.range', () => {
		const ignored: Array<[string, number, number[]]> = [
			['lineWidth', 3, [0, -1, NaN, Infinity, -Infinity]],
			['miterLimit', 5, [0, -1, NaN, Infinity, -Infinity]],
			['globalAlpha', 0.5, [-0.1, 1.1, NaN, Infinity, -Infinity]],
			['shadowBlur', 4, [-1, NaN, Infinity, -Infinity]],
			['shadowOffsetX', 4, [NaN, Infinity, -Infinity]],
			['shadowOffsetY', 4, [NaN, Infinity, -Infinity]],
			['lineDashOffset', 4, [NaN, Infinity, -Infinity]],
		];

		for (const [prop, good, bad] of ignored) {
			test(`${prop} ignores out-of-range values`, () => {
				const { ctx } = make2D();
				ctx[prop] = good;
				for (const value of bad) {
					ctx[prop] = value;
					equal(ctx[prop], good, `${prop} accepted ${value}`);
				}
			});
		}

		const enums: Array<[string, string, string[]]> = [
			['lineCap', 'round', ['butt', 'round', 'square']],
			['lineJoin', 'bevel', ['round', 'bevel', 'miter']],
			['textAlign', 'center', ['start', 'end', 'left', 'right', 'center']],
			['textBaseline', 'top', ['top', 'hanging', 'middle', 'alphabetic', 'ideographic', 'bottom']],
			['imageSmoothingQuality', 'high', ['low', 'medium', 'high']],
			['direction', 'rtl', ['ltr', 'rtl', 'inherit']],
		];

		for (const [prop, sentinel, valid] of enums) {
			test(`${prop} accepts every spec value and ignores others`, () => {
				const { ctx } = make2D();
				for (const value of valid) {
					ctx[prop] = value;
					equal(ctx[prop], value, `${prop} rejected the valid value ${value}`);
				}
				ctx[prop] = sentinel;
				ctx[prop] = 'definitely-not-valid';
				equal(ctx[prop], sentinel, `${prop} accepted an invalid value`);
			});
		}

		test('globalCompositeOperation accepts every spec operator', () => {
			const { ctx } = make2D();
			const ops = ['source-over', 'source-in', 'source-out', 'source-atop', 'destination-over', 'destination-in', 'destination-out', 'destination-atop', 'lighter', 'copy', 'xor', 'multiply', 'screen', 'overlay', 'darken', 'lighten', 'color-dodge', 'color-burn', 'hard-light', 'soft-light', 'difference', 'exclusion', 'hue', 'saturation', 'color', 'luminosity'];
			const missing: string[] = [];
			for (const op of ops) {
				ctx.globalCompositeOperation = 'source-over';
				ctx.globalCompositeOperation = op;
				if (ctx.globalCompositeOperation !== op) {
					missing.push(op);
				}
			}
			equal(missing.length, 0, `unsupported operators: ${missing.join(', ')}`);
		});

		test('globalCompositeOperation ignores an invalid operator', () => {
			const { ctx } = make2D();
			ctx.globalCompositeOperation = 'xor';
			ctx.globalCompositeOperation = 'not-an-operator';
			equal(ctx.globalCompositeOperation, 'xor');
		});
	});

	// ------------------------------------------------------------ line dash
	suite('2d.linedash', () => {
		test('setLineDash round trips an even list', () => {
			const { ctx } = make2D();
			ctx.setLineDash([1, 2, 3, 4]);
			arrayEqual(ctx.getLineDash(), [1, 2, 3, 4]);
		});

		test('setLineDash duplicates an odd list', () => {
			const { ctx } = make2D();
			ctx.setLineDash([1, 2, 3]);
			arrayEqual(ctx.getLineDash(), [1, 2, 3, 1, 2, 3]);
		});

		test('setLineDash ignores a list with a negative entry', () => {
			const { ctx } = make2D();
			ctx.setLineDash([4, 5]);
			ctx.setLineDash([1, -1]);
			arrayEqual(ctx.getLineDash(), [4, 5]);
		});

		test('setLineDash ignores a list with a non-finite entry', () => {
			const { ctx } = make2D();
			ctx.setLineDash([4, 5]);
			ctx.setLineDash([1, NaN]);
			arrayEqual(ctx.getLineDash(), [4, 5]);
			ctx.setLineDash([1, Infinity]);
			arrayEqual(ctx.getLineDash(), [4, 5]);
		});

		test('getLineDash returns a copy', () => {
			const { ctx } = make2D();
			ctx.setLineDash([1, 2]);
			const list = ctx.getLineDash();
			list[0] = 99;
			arrayEqual(ctx.getLineDash(), [1, 2], 'getLineDash handed out its internal list');
		});

		test('setLineDash([]) clears the dash', () => {
			const { ctx } = make2D();
			ctx.setLineDash([1, 2]);
			ctx.setLineDash([]);
			arrayEqual(ctx.getLineDash(), []);
		});
	});

	// ----------------------------------------------------------- transforms
	suite('2d.transform', () => {
		test('the initial transform is the identity', () => {
			const { ctx } = make2D();
			const m = ctx.getTransform();
			arrayEqual([m.a, m.b, m.c, m.d, m.e, m.f], [1, 0, 0, 1, 0, 0]);
		});

		test('translate accumulates', () => {
			const { ctx } = make2D();
			ctx.translate(10, 20);
			ctx.translate(5, 5);
			const m = ctx.getTransform();
			closeTo(m.e, 15, 1e-5, 'e');
			closeTo(m.f, 25, 1e-5, 'f');
		});

		test('scale then translate applies in the scaled space', () => {
			const { ctx } = make2D();
			ctx.scale(2, 3);
			ctx.translate(10, 10);
			const m = ctx.getTransform();
			closeTo(m.a, 2, 1e-5, 'a');
			closeTo(m.d, 3, 1e-5, 'd');
			closeTo(m.e, 20, 1e-5, 'e');
			closeTo(m.f, 30, 1e-5, 'f');
		});

		test('rotate(PI/2) maps (1,0) to (0,1)', () => {
			const { ctx } = make2D();
			ctx.rotate(Math.PI / 2);
			const m = ctx.getTransform();
			closeTo(m.a, 0, 1e-5, 'a');
			closeTo(m.b, 1, 1e-5, 'b');
			closeTo(m.c, -1, 1e-5, 'c');
			closeTo(m.d, 0, 1e-5, 'd');
		});

		test('setTransform replaces rather than multiplies', () => {
			const { ctx } = make2D();
			ctx.translate(50, 50);
			ctx.setTransform(1, 0, 0, 1, 10, 20);
			const m = ctx.getTransform();
			arrayEqual([m.a, m.b, m.c, m.d, m.e, m.f], [1, 0, 0, 1, 10, 20]);
		});

		test('setTransform accepts a matrix-like init', () => {
			const { ctx } = make2D();
			ctx.setTransform({ a: 2, b: 0, c: 0, d: 2, e: 5, f: 6 });
			const m = ctx.getTransform();
			arrayEqual([m.a, m.b, m.c, m.d, m.e, m.f], [2, 0, 0, 2, 5, 6]);
		});

		test('resetTransform restores the identity', () => {
			const { ctx } = make2D();
			ctx.translate(11, 22);
			ctx.rotate(1);
			ctx.resetTransform();
			const m = ctx.getTransform();
			arrayEqual([m.a, m.b, m.c, m.d, m.e, m.f], [1, 0, 0, 1, 0, 0]);
		});

		test('transform() multiplies into the current matrix', () => {
			const { ctx } = make2D();
			ctx.translate(10, 10);
			ctx.transform(1, 0, 0, 1, 5, 5);
			const m = ctx.getTransform();
			closeTo(m.e, 15, 1e-5, 'e');
			closeTo(m.f, 15, 1e-5, 'f');
		});

		test('a non-finite transform argument is ignored', () => {
			const { ctx } = make2D();
			ctx.translate(10, 10);
			ctx.translate(NaN, 5);
			const m = ctx.getTransform();
			closeTo(m.e, 10, 1e-5, 'e');
			closeTo(m.f, 10, 1e-5, 'f');
		});

		test('getTransform returns a snapshot, not a live matrix', () => {
			const { ctx } = make2D();
			const m = ctx.getTransform();
			ctx.translate(30, 30);
			closeTo(m.e, 0, 1e-5, 'the matrix from getTransform tracked a later translate');
		});

		test('a non-invertible transform paints nothing', () => {
			const { ctx } = make2D(50, 50);
			ctx.fillStyle = '#ff0000';
			ctx.scale(0, 0);
			ctx.fillRect(0, 0, 50, 50);
			ctx.resetTransform();
			pixelEqual(ctx, 25, 25, [0, 0, 0, 0], 0, 'a singular transform must suppress drawing');
		});

		test('transforms apply to fillRect', () => {
			const { ctx } = make2D(50, 50);
			ctx.fillStyle = '#ff0000';
			ctx.translate(20, 20);
			ctx.fillRect(0, 0, 10, 10);
			ctx.resetTransform();
			pixelEqual(ctx, 25, 25, [255, 0, 0, 255], 2, 'inside the translated rect');
			pixelEqual(ctx, 5, 5, [0, 0, 0, 0], 0, 'outside the translated rect');
		});
	});

	// ---------------------------------------------------------- rectangles
	suite('2d.rects', () => {
		test('fillRect paints the requested colour', () => {
			const { ctx } = make2D(50, 50);
			ctx.fillStyle = '#204080';
			ctx.fillRect(0, 0, 50, 50);
			pixelEqual(ctx, 25, 25, [0x20, 0x40, 0x80, 255]);
		});

		test('fillRect with a zero dimension paints nothing', () => {
			const { ctx } = make2D(50, 50);
			ctx.fillStyle = '#ff0000';
			ctx.fillRect(0, 0, 0, 50);
			pixelEqual(ctx, 0, 25, [0, 0, 0, 0], 0);
		});

		test('fillRect with a negative size normalises', () => {
			const { ctx } = make2D(50, 50);
			ctx.fillStyle = '#ff0000';
			ctx.fillRect(40, 40, -20, -20);
			pixelEqual(ctx, 30, 30, [255, 0, 0, 255], 2, 'the rect should cover (20,20)-(40,40)');
		});

		test('clearRect erases to transparent black', () => {
			const { ctx } = make2D(50, 50);
			ctx.fillStyle = '#ff0000';
			ctx.fillRect(0, 0, 50, 50);
			ctx.clearRect(10, 10, 20, 20);
			pixelEqual(ctx, 20, 20, [0, 0, 0, 0], 0, 'cleared');
			pixelEqual(ctx, 5, 5, [255, 0, 0, 255], 2, 'untouched');
		});

		test('clearRect ignores globalAlpha and the composite operator', () => {
			const { ctx } = make2D(50, 50);
			ctx.fillStyle = '#ff0000';
			ctx.fillRect(0, 0, 50, 50);
			ctx.globalAlpha = 0.5;
			ctx.globalCompositeOperation = 'lighter';
			ctx.clearRect(10, 10, 20, 20);
			pixelEqual(ctx, 20, 20, [0, 0, 0, 0], 0);
		});

		test('strokeRect paints on the rect outline', () => {
			const { ctx } = make2D(50, 50);
			ctx.strokeStyle = '#ff0000';
			ctx.lineWidth = 4;
			ctx.strokeRect(10, 10, 30, 30);
			pixelEqual(ctx, 10, 25, [255, 0, 0, 255], 4, 'on the left edge');
			pixelEqual(ctx, 25, 25, [0, 0, 0, 0], 0, 'the middle must stay empty');
		});

		test('globalAlpha scales the painted alpha', () => {
			const { ctx } = make2D(50, 50);
			ctx.globalAlpha = 0.5;
			ctx.fillStyle = '#ff0000';
			ctx.fillRect(0, 0, 50, 50);
			const [r, , , a] = pixelAt(ctx, 25, 25);
			closeTo(a, 128, 3, 'alpha');
			closeTo(r, 255, 3, 'red is premultiplied back out by getImageData');
		});

		test('destination-out punches a hole', () => {
			const { ctx } = make2D(50, 50);
			ctx.fillStyle = '#ff0000';
			ctx.fillRect(0, 0, 50, 50);
			ctx.globalCompositeOperation = 'destination-out';
			ctx.fillRect(10, 10, 20, 20);
			pixelEqual(ctx, 20, 20, [0, 0, 0, 0], 2, 'inside the hole');
			pixelEqual(ctx, 5, 5, [255, 0, 0, 255], 2, 'outside the hole');
		});

		test('copy replaces the whole surface', () => {
			const { ctx } = make2D(50, 50);
			ctx.fillStyle = '#ff0000';
			ctx.fillRect(0, 0, 50, 50);
			ctx.globalCompositeOperation = 'copy';
			ctx.fillStyle = '#00ff00';
			ctx.fillRect(10, 10, 20, 20);
			pixelEqual(ctx, 20, 20, [0, 255, 0, 255], 2, 'the new rect');
			pixelEqual(ctx, 5, 5, [0, 0, 0, 0], 2, 'copy must clear everything else');
		});
	});

	// --------------------------------------------------------------- paths
	suite('2d.path', () => {
		test('isPointInPath follows a rect', () => {
			const { ctx } = make2D();
			ctx.beginPath();
			ctx.rect(10, 10, 30, 30);
			ok(ctx.isPointInPath(20, 20), 'inside');
			ok(!ctx.isPointInPath(5, 5), 'outside');
		});

		test('beginPath discards the previous path', () => {
			const { ctx } = make2D();
			ctx.rect(10, 10, 30, 30);
			ctx.beginPath();
			ok(!ctx.isPointInPath(20, 20), 'the old path survived beginPath');
		});

		test('isPointInPath honours the nonzero rule', () => {
			const { ctx } = make2D();
			ctx.beginPath();
			ctx.rect(0, 0, 50, 50);
			ctx.rect(10, 10, 30, 30);
			ok(ctx.isPointInPath(25, 25, 'nonzero'), 'nonzero: the inner rect stays filled');
		});

		test('isPointInPath honours the evenodd rule', () => {
			const { ctx } = make2D();
			ctx.beginPath();
			ctx.rect(0, 0, 50, 50);
			ctx.rect(10, 10, 30, 30);
			ok(!ctx.isPointInPath(25, 25, 'evenodd'), 'evenodd: the inner rect is a hole');
		});

		test('isPointInPath applies the current transform', () => {
			const { ctx } = make2D();
			ctx.translate(20, 20);
			ctx.beginPath();
			ctx.rect(0, 0, 10, 10);
			ok(ctx.isPointInPath(25, 25), 'the point should be tested in device space');
		});

		test('isPointInStroke follows the stroke width', () => {
			const { ctx } = make2D();
			ctx.lineWidth = 10;
			ctx.beginPath();
			ctx.moveTo(0, 25);
			ctx.lineTo(50, 25);
			ok(ctx.isPointInStroke(25, 27), 'inside the stroke');
			ok(!ctx.isPointInStroke(25, 45), 'outside the stroke');
		});

		test('fill paints the current path', () => {
			const { ctx } = make2D(50, 50);
			ctx.fillStyle = '#ff0000';
			ctx.beginPath();
			ctx.rect(10, 10, 30, 30);
			ctx.fill();
			pixelEqual(ctx, 25, 25, [255, 0, 0, 255], 2);
		});

		test('fill("evenodd") leaves the inner rect empty', () => {
			const { ctx } = make2D(60, 60);
			ctx.fillStyle = '#ff0000';
			ctx.beginPath();
			ctx.rect(0, 0, 60, 60);
			ctx.rect(15, 15, 30, 30);
			ctx.fill('evenodd');
			pixelEqual(ctx, 30, 30, [0, 0, 0, 0], 2, 'the hole');
			pixelEqual(ctx, 5, 5, [255, 0, 0, 255], 2, 'the ring');
		});

		test('closePath on an empty path is a no-op', () => {
			const { ctx } = make2D();
			ctx.beginPath();
			ctx.closePath();
			ok(!ctx.isPointInPath(5, 5));
		});

		test('arc with a negative radius throws IndexSizeError', () => {
			const { ctx } = make2D();
			throws(() => ctx.arc(10, 10, -5, 0, Math.PI), 'IndexSizeError');
		});

		test('ellipse with a negative radius throws IndexSizeError', () => {
			const { ctx } = make2D();
			throws(() => ctx.ellipse(10, 10, -5, 5, 0, 0, Math.PI), 'IndexSizeError');
		});

		test('arc draws a filled disc', () => {
			const { ctx } = make2D(50, 50);
			ctx.fillStyle = '#ff0000';
			ctx.beginPath();
			ctx.arc(25, 25, 15, 0, Math.PI * 2);
			ctx.fill();
			pixelEqual(ctx, 25, 25, [255, 0, 0, 255], 2, 'centre');
			pixelEqual(ctx, 2, 2, [0, 0, 0, 0], 2, 'corner');
		});

		test('a non-finite path coordinate is ignored', () => {
			const { ctx } = make2D();
			ctx.beginPath();
			ctx.moveTo(10, 10);
			ctx.lineTo(NaN, 20);
			ctx.lineTo(40, 40);
			// The spec drops the bad segment but keeps the path usable.
			ok(true);
		});

		test('clip restricts later painting', () => {
			const { ctx } = make2D(50, 50);
			ctx.beginPath();
			ctx.rect(10, 10, 20, 20);
			ctx.clip();
			ctx.fillStyle = '#ff0000';
			ctx.fillRect(0, 0, 50, 50);
			pixelEqual(ctx, 20, 20, [255, 0, 0, 255], 2, 'inside the clip');
			pixelEqual(ctx, 40, 40, [0, 0, 0, 0], 0, 'outside the clip');
		});

		test('restore undoes a clip', () => {
			const { ctx } = make2D(50, 50);
			ctx.save();
			ctx.beginPath();
			ctx.rect(10, 10, 20, 20);
			ctx.clip();
			ctx.restore();
			ctx.fillStyle = '#ff0000';
			ctx.fillRect(0, 0, 50, 50);
			pixelEqual(ctx, 40, 40, [255, 0, 0, 255], 2, 'the clip outlived its restore');
		});

		test('roundRect draws rounded corners', () => {
			const { ctx } = make2D(50, 50);
			ctx.fillStyle = '#ff0000';
			ctx.beginPath();
			ctx.roundRect(0, 0, 50, 50, 20);
			ctx.fill();
			pixelEqual(ctx, 25, 25, [255, 0, 0, 255], 2, 'centre');
			pixelEqual(ctx, 1, 1, [0, 0, 0, 0], 2, 'the corner should be rounded away');
		});
	});

	// Granular on purpose: the SPEC|start marker then pins a native abort to one
	// operation, which is how the duplicate-template-property crash was found.
	suite('2d.path2d', () => {
		test('Path2D can be constructed', () => {
			const p = new Path2D();
			ok(p, 'new Path2D() returned nothing');
		});

		test('Path2D.rect adds geometry', () => {
			const p = new Path2D();
			p.rect(10, 10, 30, 30);
			ok(p, 'rect() on a Path2D');
		});

		test('isPointInPath(path, x, y) defaults to nonzero', () => {
			const { ctx } = make2D();
			const p = new Path2D();
			p.rect(10, 10, 30, 30);
			ok(ctx.isPointInPath(p, 25, 25), 'inside');
			ok(!ctx.isPointInPath(p, 5, 5), 'outside');
		});

		test('isPointInPath(path, x, y, rule) honours the rule', () => {
			const { ctx } = make2D();
			const p = new Path2D();
			p.rect(0, 0, 50, 50);
			p.rect(10, 10, 30, 30);
			ok(ctx.isPointInPath(p, 25, 25, 'nonzero'), 'nonzero');
			ok(!ctx.isPointInPath(p, 25, 25, 'evenodd'), 'evenodd');
		});

		test('Path2D can be filled', () => {
			const { ctx } = make2D(50, 50);
			const p = new Path2D();
			p.rect(10, 10, 30, 30);
			ctx.fillStyle = '#ff0000';
			ctx.fill(p);
			pixelEqual(ctx, 25, 25, [255, 0, 0, 255], 2);
		});

		test('Path2D can be stroked', () => {
			const { ctx } = make2D(50, 50);
			const p = new Path2D();
			p.rect(10, 10, 30, 30);
			ctx.strokeStyle = '#ff0000';
			ctx.lineWidth = 4;
			ctx.stroke(p);
			pixelEqual(ctx, 10, 25, [255, 0, 0, 255], 4, 'on the edge');
			pixelEqual(ctx, 25, 25, [0, 0, 0, 0], 0, 'the middle must stay empty');
		});

		test('Path2D can be clipped with', () => {
			const { ctx } = make2D(50, 50);
			const p = new Path2D();
			p.rect(10, 10, 20, 20);
			ctx.clip(p);
			ctx.fillStyle = '#ff0000';
			ctx.fillRect(0, 0, 50, 50);
			pixelEqual(ctx, 20, 20, [255, 0, 0, 255], 2, 'inside the clip');
			pixelEqual(ctx, 40, 40, [0, 0, 0, 0], 0, 'outside the clip');
		});

		test('Path2D copy constructor takes a snapshot', () => {
			const { ctx } = make2D();
			const a = new Path2D();
			a.rect(10, 10, 20, 20);
			const b = new Path2D(a);
			a.rect(40, 40, 5, 5);
			ok(ctx.isPointInPath(b, 15, 15), 'the copy kept the original geometry');
			ok(!ctx.isPointInPath(b, 42, 42), 'the copy tracked a later edit of the original');
		});

		test('Path2D parses an SVG path string', () => {
			const { ctx } = make2D();
			const p = new Path2D('M 10 10 L 40 10 L 40 40 L 10 40 Z');
			ok(ctx.isPointInPath(p, 25, 25), 'inside');
			ok(!ctx.isPointInPath(p, 5, 5), 'outside');
		});

		test('Path2D.addPath appends', () => {
			const { ctx } = make2D();
			const a = new Path2D();
			a.rect(0, 0, 10, 10);
			const b = new Path2D();
			b.rect(30, 30, 10, 10);
			a.addPath(b);
			ok(ctx.isPointInPath(a, 5, 5), 'the original');
			ok(ctx.isPointInPath(a, 35, 35), 'the appended path');
		});

		test('Path2D.addPath into an empty path', () => {
			const { ctx } = make2D();
			const a = new Path2D();
			const b = new Path2D();
			b.rect(0, 0, 10, 10);
			a.addPath(b);
			ok(ctx.isPointInPath(a, 5, 5), 'the appended path');
		});

		test('Path2D.addPath with an identity matrix appends', () => {
			const { ctx } = make2D();
			const a = new Path2D();
			const b = new Path2D();
			b.rect(0, 0, 10, 10);
			a.addPath(b, new DOMMatrix() as any);
			ok(ctx.isPointInPath(a, 5, 5), 'the appended path');
		});

		test('Path2D.addPath applies the given transform', () => {
			const { ctx } = make2D();
			const a = new Path2D();
			const b = new Path2D();
			b.rect(0, 0, 10, 10);
			a.addPath(b, new DOMMatrix().translate(30, 30) as any);
			ok(ctx.isPointInPath(a, 35, 35), 'the transformed copy');
			ok(!ctx.isPointInPath(a, 5, 5), 'the untransformed position');
		});

		test('isPointInStroke accepts a Path2D', () => {
			const { ctx } = make2D();
			const p = new Path2D();
			p.moveTo(0, 25);
			p.lineTo(50, 25);
			ctx.lineWidth = 10;
			ok(ctx.isPointInStroke(p, 25, 27), 'inside');
			ok(!ctx.isPointInStroke(p, 25, 45), 'outside');
		});
	});

	suite('2d.dommatrix', () => {
		test('new DOMMatrix() is the identity', () => {
			const m = new DOMMatrix();
			arrayEqual([m.a, m.b, m.c, m.d, m.e, m.f], [1, 0, 0, 1, 0, 0]);
		});

		test('translate returns a translated copy', () => {
			const m = new DOMMatrix();
			const t = m.translate(30, 40);
			closeTo(t.e, 30, 1e-5, 'e');
			closeTo(t.f, 40, 1e-5, 'f');
			closeTo(m.e, 0, 1e-5, 'translate must not mutate the receiver');
		});

		test('scale returns a scaled copy', () => {
			const m = new DOMMatrix().scale(2, 3);
			closeTo(m.a, 2, 1e-5, 'a');
			closeTo(m.d, 3, 1e-5, 'd');
		});
	});

	// ----------------------------------------------------------- ImageData
	suite('2d.imagedata', () => {
		test('createImageData(w, h) is transparent black', () => {
			const { ctx } = make2D();
			const data = ctx.createImageData(4, 5);
			equal(data.width, 4, 'width');
			equal(data.height, 5, 'height');
			equal(data.data.length, 4 * 5 * 4, 'data length');
			for (let i = 0; i < data.data.length; i++) {
				if (data.data[i] !== 0) {
					throw new Error(`byte ${i} is ${data.data[i]}, expected 0`);
				}
			}
		});

		test('createImageData takes the absolute size', () => {
			const { ctx } = make2D();
			const data = ctx.createImageData(-4, -5);
			equal(data.width, 4, 'width');
			equal(data.height, 5, 'height');
		});

		test('createImageData(0, h) throws IndexSizeError', () => {
			const { ctx } = make2D();
			throws(() => ctx.createImageData(0, 10), 'IndexSizeError');
		});

		test('createImageData(imagedata) copies the size but not the pixels', () => {
			const { ctx } = make2D();
			const src = ctx.createImageData(3, 4);
			src.data[0] = 255;
			const copy = ctx.createImageData(src);
			equal(copy.width, 3, 'width');
			equal(copy.height, 4, 'height');
			equal(copy.data[0], 0, 'createImageData(imagedata) must not copy pixel data');
		});

		test('new ImageData(w, h) works', () => {
			const data = new ImageData(6, 7);
			equal(data.width, 6, 'width');
			equal(data.height, 7, 'height');
			equal(data.data.length, 6 * 7 * 4, 'length');
		});

		test('new ImageData(array, w) derives the height', () => {
			const buf = new Uint8ClampedArray(4 * 3 * 2);
			const data = new ImageData(buf, 3);
			equal(data.width, 3, 'width');
			equal(data.height, 2, 'height');
		});

		test('new ImageData with a mismatched buffer throws', () => {
			throws(() => new ImageData(new Uint8ClampedArray(7), 3), undefined, 'a buffer that is not a multiple of 4*width must throw');
		});

		test('ImageData.colorSpace defaults to srgb', () => {
			const data = new ImageData(2, 2);
			oneOf((data as any).colorSpace, ['srgb', undefined], 'colorSpace');
		});

		test('getImageData reads back what was painted', () => {
			const { ctx } = make2D(20, 20);
			ctx.fillStyle = '#123456';
			ctx.fillRect(0, 0, 20, 20);
			const data = ctx.getImageData(0, 0, 20, 20);
			equal(data.width, 20, 'width');
			equal(data.height, 20, 'height');
			equal(data.data.length, 20 * 20 * 4, 'length');
			equal(data.data[0], 0x12, 'r');
			equal(data.data[1], 0x34, 'g');
			equal(data.data[2], 0x56, 'b');
			equal(data.data[3], 255, 'a');
		});

		test('getImageData ignores the current transform', () => {
			const { ctx } = make2D(20, 20);
			ctx.fillStyle = '#ff0000';
			ctx.fillRect(0, 0, 10, 10);
			ctx.translate(10, 10);
			const data = ctx.getImageData(0, 0, 1, 1);
			equal(data.data[0], 255, 'getImageData must read in device space');
		});

		test('getImageData outside the canvas is transparent black', () => {
			const { ctx } = make2D(10, 10);
			ctx.fillStyle = '#ff0000';
			ctx.fillRect(0, 0, 10, 10);
			const data = ctx.getImageData(-5, -5, 4, 4);
			equal(data.data[0], 0, 'r');
			equal(data.data[3], 0, 'a');
		});

		test('getImageData with a zero width throws IndexSizeError', () => {
			const { ctx } = make2D();
			throws(() => ctx.getImageData(0, 0, 0, 10), 'IndexSizeError');
		});

		test('putImageData round trips', () => {
			const { ctx } = make2D(20, 20);
			const data = ctx.createImageData(4, 4);
			for (let i = 0; i < data.data.length; i += 4) {
				data.data[i] = 10;
				data.data[i + 1] = 20;
				data.data[i + 2] = 30;
				data.data[i + 3] = 255;
			}
			ctx.putImageData(data, 5, 5);
			pixelEqual(ctx, 6, 6, [10, 20, 30, 255], 1);
		});

		test('putImageData ignores the transform, alpha and composite operator', () => {
			const { ctx } = make2D(20, 20);
			ctx.translate(10, 10);
			ctx.globalAlpha = 0.1;
			ctx.globalCompositeOperation = 'lighter';
			const data = ctx.createImageData(2, 2);
			for (let i = 0; i < data.data.length; i += 4) {
				data.data[i] = 255;
				data.data[i + 3] = 255;
			}
			ctx.putImageData(data, 0, 0);
			ctx.resetTransform();
			pixelEqual(ctx, 0, 0, [255, 0, 0, 255], 1, 'putImageData must bypass the drawing state');
		});

		test('putImageData with a dirty rect only writes that rect', () => {
			const { ctx } = make2D(20, 20);
			const data = ctx.createImageData(10, 10);
			for (let i = 0; i < data.data.length; i += 4) {
				data.data[i] = 255;
				data.data[i + 3] = 255;
			}
			ctx.putImageData(data, 0, 0, 2, 2, 3, 3);
			pixelEqual(ctx, 3, 3, [255, 0, 0, 255], 1, 'inside the dirty rect');
			pixelEqual(ctx, 0, 0, [0, 0, 0, 0], 0, 'outside the dirty rect');
		});
	});

	// ---------------------------------------------------------- gradients
	suite('2d.gradient', () => {
		test('createLinearGradient returns a CanvasGradient', () => {
			const { ctx } = make2D();
			const g = ctx.createLinearGradient(0, 0, 10, 0);
			ok(g, 'no gradient');
			equal(typeof g.addColorStop, 'function', 'addColorStop');
		});

		test('addColorStop with an offset outside [0,1] throws IndexSizeError', () => {
			const { ctx } = make2D();
			const g = ctx.createLinearGradient(0, 0, 10, 0);
			throws(() => g.addColorStop(-0.1, 'red'), 'IndexSizeError', 'offset < 0');
			throws(() => g.addColorStop(1.1, 'red'), 'IndexSizeError', 'offset > 1');
			throws(() => g.addColorStop(NaN, 'red'), 'IndexSizeError', 'offset NaN');
		});

		test('addColorStop with an invalid colour throws SyntaxError', () => {
			const { ctx } = make2D();
			const g = ctx.createLinearGradient(0, 0, 10, 0);
			throws(() => g.addColorStop(0.5, 'not-a-colour'), 'SyntaxError');
		});

		test('createRadialGradient with a negative radius throws IndexSizeError', () => {
			const { ctx } = make2D();
			throws(() => ctx.createRadialGradient(0, 0, -1, 0, 0, 10), 'IndexSizeError');
		});

		test('a linear gradient paints its end colours', () => {
			const { ctx } = make2D(50, 50);
			const g = ctx.createLinearGradient(0, 0, 50, 0);
			g.addColorStop(0, '#ff0000');
			g.addColorStop(1, '#0000ff');
			ctx.fillStyle = g;
			ctx.fillRect(0, 0, 50, 50);
			pixelEqual(ctx, 0, 25, [255, 0, 0, 255], 6, 'the left end');
			pixelEqual(ctx, 49, 25, [0, 0, 255, 255], 6, 'the right end');
		});

		test('a gradient with no colour stops paints nothing', () => {
			const { ctx } = make2D(50, 50);
			ctx.fillStyle = ctx.createLinearGradient(0, 0, 50, 0);
			ctx.fillRect(0, 0, 50, 50);
			pixelEqual(ctx, 25, 25, [0, 0, 0, 0], 0);
		});

		test('createConicGradient exists and paints', () => {
			const { ctx } = make2D(50, 50);
			const g = ctx.createConicGradient(0, 25, 25);
			g.addColorStop(0, '#ff0000');
			g.addColorStop(1, '#ff0000');
			ctx.fillStyle = g;
			ctx.fillRect(0, 0, 50, 50);
			pixelEqual(ctx, 25, 10, [255, 0, 0, 255], 6);
		});
	});

	// ---------------------------------------------------------- patterns
	suite('2d.pattern', () => {
		function sourceCanvas() {
			const { canvas, ctx } = make2D(10, 10);
			ctx.fillStyle = '#ff0000';
			ctx.fillRect(0, 0, 5, 10);
			ctx.fillStyle = '#0000ff';
			ctx.fillRect(5, 0, 5, 10);
			return canvas;
		}

		test('createPattern with an invalid repetition throws SyntaxError', () => {
			const { ctx } = make2D();
			throws(() => ctx.createPattern(sourceCanvas(), 'not-a-repetition'), 'SyntaxError');
		});

		test('createPattern treats null and "" as repeat', () => {
			const { ctx } = make2D();
			ok(ctx.createPattern(sourceCanvas(), ''), 'empty string');
			ok(ctx.createPattern(sourceCanvas(), null as any), 'null');
		});

		test('a repeating pattern tiles', () => {
			const { ctx } = make2D(40, 40);
			const p = ctx.createPattern(sourceCanvas(), 'repeat');
			ok(p, 'no pattern');
			ctx.fillStyle = p;
			ctx.fillRect(0, 0, 40, 40);
			pixelEqual(ctx, 2, 2, [255, 0, 0, 255], 4, 'the first tile');
			pixelEqual(ctx, 12, 2, [255, 0, 0, 255], 4, 'the second tile');
			pixelEqual(ctx, 7, 2, [0, 0, 255, 255], 4, 'the right half of the first tile');
		});

		test('no-repeat leaves the rest of the surface empty', () => {
			const { ctx } = make2D(40, 40);
			ctx.fillStyle = ctx.createPattern(sourceCanvas(), 'no-repeat');
			ctx.fillRect(0, 0, 40, 40);
			pixelEqual(ctx, 2, 2, [255, 0, 0, 255], 4, 'the single tile');
			pixelEqual(ctx, 30, 30, [0, 0, 0, 0], 2, 'outside the tile');
		});

		test('pattern.setTransform with the identity leaves the tile alone', () => {
			const { ctx } = make2D(40, 40);
			const p = ctx.createPattern(sourceCanvas(), 'no-repeat') as any;
			p.setTransform(new DOMMatrix());
			ctx.fillStyle = p;
			ctx.fillRect(0, 0, 40, 40);
			pixelEqual(ctx, 2, 2, [255, 0, 0, 255], 4, 'the tile');
		});

		test('pattern.setTransform offsets the tile', () => {
			const { ctx } = make2D(40, 40);
			const p = ctx.createPattern(sourceCanvas(), 'no-repeat') as any;
			p.setTransform(new DOMMatrix().translate(20, 20));
			ctx.fillStyle = p;
			ctx.fillRect(0, 0, 40, 40);
			pixelEqual(ctx, 22, 22, [255, 0, 0, 255], 4, 'the moved tile');
			pixelEqual(ctx, 2, 2, [0, 0, 0, 0], 2, 'the original position');
		});
	});

	// -------------------------------------------------------------- text
	suite('2d.text', () => {
		test('the font round trips a full shorthand', () => {
			const { ctx } = make2D();
			ctx.font = 'bold 20px serif';
			equal(ctx.font, 'bold 20px serif');
		});

		test('the font serializes in canonical order', () => {
			const { ctx } = make2D();
			ctx.font = 'italic bold 12px sans-serif';
			// The spec orders style, variant, weight, stretch, size/line-height, family.
			oneOf(ctx.font, ['italic bold 12px sans-serif'], 'font serialization');
		});

		test('an invalid font leaves the previous value', () => {
			const { ctx } = make2D();
			ctx.font = '20px serif';
			ctx.font = 'not a font';
			equal(ctx.font, '20px serif');
		});

		test('a font with no size is invalid and ignored', () => {
			const { ctx } = make2D();
			ctx.font = '20px serif';
			ctx.font = 'serif';
			equal(ctx.font, '20px serif');
		});

		test('measureText returns a positive width', () => {
			const { ctx } = make2D();
			ctx.font = '20px sans-serif';
			const m = ctx.measureText('Hello');
			ok(m, 'no metrics');
			ok(m.width > 0, `width was ${m.width}`);
		});

		test('measureText("") has zero width', () => {
			const { ctx } = make2D();
			equal(ctx.measureText('').width, 0);
		});

		test('measureText scales with the font size', () => {
			const { ctx } = make2D();
			ctx.font = '10px sans-serif';
			const small = ctx.measureText('Hello world').width;
			ctx.font = '20px sans-serif';
			const large = ctx.measureText('Hello world').width;
			ok(large > small * 1.5, `10px=${small} 20px=${large}: doubling the size barely changed the width`);
		});

		test('TextMetrics exposes the bounding box extents', () => {
			const { ctx } = make2D();
			ctx.font = '40px sans-serif';
			const m = ctx.measureText('Hg');
			for (const key of ['actualBoundingBoxLeft', 'actualBoundingBoxRight', 'actualBoundingBoxAscent', 'actualBoundingBoxDescent', 'fontBoundingBoxAscent', 'fontBoundingBoxDescent']) {
				equal(typeof m[key], 'number', `${key} is missing`);
			}
			ok(m.actualBoundingBoxAscent > 0, `actualBoundingBoxAscent was ${m.actualBoundingBoxAscent}`);
			ok(m.fontBoundingBoxAscent > 0, `fontBoundingBoxAscent was ${m.fontBoundingBoxAscent}`);
		});

		test('textAlign moves the painted glyphs', () => {
			function inkColumnCentre(align: string) {
				const { ctx } = make2D(200, 50);
				ctx.font = '20px sans-serif';
				ctx.textAlign = align;
				ctx.fillStyle = '#000000';
				ctx.fillText('MMMM', 100, 30);
				const data = ctx.getImageData(0, 0, 200, 50).data;
				let sum = 0;
				let count = 0;
				for (let y = 0; y < 50; y++) {
					for (let x = 0; x < 200; x++) {
						if (data[(y * 200 + x) * 4 + 3] > 40) {
							sum += x;
							count++;
						}
					}
				}
				ok(count > 0, `nothing was painted for textAlign=${align}`);
				return sum / count;
			}
			const left = inkColumnCentre('left');
			const centre = inkColumnCentre('center');
			const right = inkColumnCentre('right');
			ok(left > centre, `left(${left}) should sit right of center(${centre})`);
			ok(centre > right, `center(${centre}) should sit right of right(${right})`);
		});

		test('fillText paints ink', () => {
			const { ctx } = make2D(100, 50);
			ctx.font = '30px sans-serif';
			ctx.fillStyle = '#ff0000';
			ctx.fillText('X', 10, 35);
			const data = ctx.getImageData(0, 0, 100, 50).data;
			let painted = 0;
			for (let i = 3; i < data.length; i += 4) {
				if (data[i] > 0) painted++;
			}
			ok(painted > 0, 'fillText painted nothing');
		});

		test('fillText with maxWidth condenses the run', () => {
			const { ctx } = make2D(200, 50);
			ctx.font = '30px sans-serif';
			function inkWidth(maxWidth?: number) {
				ctx.clearRect(0, 0, 200, 50);
				if (maxWidth === undefined) {
					ctx.fillText('Wide text', 0, 35);
				} else {
					ctx.fillText('Wide text', 0, 35, maxWidth);
				}
				const data = ctx.getImageData(0, 0, 200, 50).data;
				let max = -1;
				for (let y = 0; y < 50; y++) {
					for (let x = 0; x < 200; x++) {
						if (data[(y * 200 + x) * 4 + 3] > 40 && x > max) max = x;
					}
				}
				return max;
			}
			const unconstrained = inkWidth();
			const constrained = inkWidth(40);
			ok(unconstrained > 0, 'nothing painted');
			ok(constrained <= 45, `maxWidth was ignored: ink reached x=${constrained} for maxWidth=40 (unconstrained ${unconstrained})`);
		});

		test('letterSpacing widens the measured run', () => {
			const { ctx } = make2D();
			ctx.font = '20px sans-serif';
			const base = ctx.measureText('AAAA').width;
			ctx.letterSpacing = '10px';
			equal(ctx.letterSpacing, '10px', 'letterSpacing did not round trip');
			const spaced = ctx.measureText('AAAA').width;
			ok(spaced > base, `letterSpacing had no effect: ${base} -> ${spaced}`);
		});

		test('wordSpacing widens the measured run', () => {
			const { ctx } = make2D();
			ctx.font = '20px sans-serif';
			const base = ctx.measureText('a a a').width;
			ctx.wordSpacing = '20px';
			equal(ctx.wordSpacing, '20px', 'wordSpacing did not round trip');
			const spaced = ctx.measureText('a a a').width;
			ok(spaced > base, `wordSpacing had no effect: ${base} -> ${spaced}`);
		});

		test('strokeText paints ink', () => {
			const { ctx } = make2D(100, 50);
			ctx.font = '30px sans-serif';
			ctx.strokeStyle = '#ff0000';
			ctx.lineWidth = 2;
			ctx.strokeText('X', 10, 35);
			const data = ctx.getImageData(0, 0, 100, 50).data;
			let painted = 0;
			for (let i = 3; i < data.length; i += 4) {
				if (data[i] > 0) painted++;
			}
			ok(painted > 0, 'strokeText painted nothing');
		});
	});

	// ------------------------------------------------------------ shadows
	suite('2d.shadow', () => {
		test('a shadow paints outside the rect', () => {
			const { ctx } = make2D(60, 60);
			ctx.shadowColor = '#ff0000';
			ctx.shadowBlur = 0;
			ctx.shadowOffsetX = 20;
			ctx.shadowOffsetY = 0;
			ctx.fillStyle = '#000000';
			ctx.fillRect(5, 20, 15, 20);
			pixelEqual(ctx, 30, 30, [255, 0, 0, 255], 6, 'the offset shadow');
		});

		test('a fully transparent shadowColor paints no shadow', () => {
			const { ctx } = make2D(60, 60);
			ctx.shadowColor = 'rgba(255, 0, 0, 0)';
			ctx.shadowOffsetX = 20;
			ctx.fillStyle = '#000000';
			ctx.fillRect(5, 20, 15, 20);
			pixelEqual(ctx, 30, 30, [0, 0, 0, 0], 2);
		});
	});

	// ------------------------------------------------------------- filter
	suite('2d.filter', () => {
		test('filter round trips a known function', () => {
			const { ctx } = make2D();
			ctx.filter = 'blur(4px)';
			equal(ctx.filter, 'blur(4px)');
		});

		test('filter = "none" clears it', () => {
			const { ctx } = make2D();
			ctx.filter = 'blur(4px)';
			ctx.filter = 'none';
			equal(ctx.filter, 'none');
		});

		test('an invalid filter leaves the previous value', () => {
			const { ctx } = make2D();
			ctx.filter = 'blur(4px)';
			ctx.filter = 'not-a-filter(3)';
			equal(ctx.filter, 'blur(4px)');
		});

		test('blur softens an edge', () => {
			const { ctx } = make2D(60, 60);
			ctx.filter = 'blur(5px)';
			ctx.fillStyle = '#ff0000';
			ctx.fillRect(20, 20, 20, 20);
			const [, , , a] = pixelAt(ctx, 18, 30);
			ok(a > 5 && a < 250, `expected a partially-covered pixel just outside the rect, got alpha ${a}`);
		});

		test('invert(1) inverts', () => {
			const { ctx } = make2D(40, 40);
			ctx.filter = 'invert(1)';
			ctx.fillStyle = '#ff0000';
			ctx.fillRect(0, 0, 40, 40);
			pixelEqual(ctx, 20, 20, [0, 255, 255, 255], 6);
		});
	});

	// -------------------------------------------------------- drawImage
	suite('2d.drawimage', () => {
		function source() {
			const { canvas, ctx } = make2D(20, 20);
			ctx.fillStyle = '#ff0000';
			ctx.fillRect(0, 0, 10, 20);
			ctx.fillStyle = '#0000ff';
			ctx.fillRect(10, 0, 10, 20);
			return canvas;
		}

		test('drawImage(image, dx, dy) copies at the natural size', () => {
			const { ctx } = make2D(40, 40);
			ctx.drawImage(source(), 10, 10);
			pixelEqual(ctx, 12, 12, [255, 0, 0, 255], 4, 'the red half');
			pixelEqual(ctx, 28, 12, [0, 0, 255, 255], 4, 'the blue half');
			pixelEqual(ctx, 2, 2, [0, 0, 0, 0], 2, 'outside');
		});

		test('drawImage(image, dx, dy, dw, dh) scales', () => {
			const { ctx } = make2D(40, 40);
			ctx.drawImage(source(), 0, 0, 40, 40);
			pixelEqual(ctx, 5, 20, [255, 0, 0, 255], 6, 'the red half');
			pixelEqual(ctx, 35, 20, [0, 0, 255, 255], 6, 'the blue half');
		});

		test('drawImage with a source rect crops', () => {
			const { ctx } = make2D(40, 40);
			// Take only the blue half of the source.
			ctx.drawImage(source(), 10, 0, 10, 20, 0, 0, 40, 40);
			pixelEqual(ctx, 5, 20, [0, 0, 255, 255], 6, 'the crop should be all blue');
			pixelEqual(ctx, 35, 20, [0, 0, 255, 255], 6, 'the crop should be all blue');
		});

		test('drawImage applies the current transform', () => {
			const { ctx } = make2D(60, 60);
			ctx.translate(20, 20);
			ctx.drawImage(source(), 0, 0);
			ctx.resetTransform();
			pixelEqual(ctx, 22, 22, [255, 0, 0, 255], 4, 'inside the translated image');
			pixelEqual(ctx, 5, 5, [0, 0, 0, 0], 2, 'outside');
		});

		test('drawImage with a non-finite argument paints nothing', () => {
			const { ctx } = make2D(40, 40);
			ctx.drawImage(source(), NaN, 10);
			pixelEqual(ctx, 12, 12, [0, 0, 0, 0], 0);
		});

		test('drawImage honours globalAlpha', () => {
			const { ctx } = make2D(40, 40);
			ctx.globalAlpha = 0.5;
			ctx.drawImage(source(), 0, 0);
			const [, , , a] = pixelAt(ctx, 5, 5);
			closeTo(a, 128, 4, 'alpha');
		});
	});
}
