/**
 * WebGL 1 and 2: context attributes, the constant table, error reporting, object
 * lifecycle, and end-to-end draws whose pixels are read back.
 */

import { suite, test, ok, equal, notEqual, closeTo, oneOf, arrayEqual, throws, makeCanvas } from './harness';

function makeGL(version: 1 | 2, options?: any) {
	const canvas = makeCanvas(64, 64);
	const gl = canvas.getContext(version === 1 ? 'webgl' : 'webgl2', options) as any;
	return { canvas, gl };
}

const VERTEX = `
attribute vec2 aPos;
void main() { gl_Position = vec4(aPos, 0.0, 1.0); }
`;

const FRAGMENT = `
precision mediump float;
uniform vec4 uColor;
void main() { gl_FragColor = uColor; }
`;

function buildProgram(gl: any, vertexSource = VERTEX, fragmentSource = FRAGMENT) {
	const vs = gl.createShader(gl.VERTEX_SHADER);
	gl.shaderSource(vs, vertexSource);
	gl.compileShader(vs);
	if (!gl.getShaderParameter(vs, gl.COMPILE_STATUS)) {
		throw new Error(`vertex shader: ${gl.getShaderInfoLog(vs)}`);
	}
	const fs = gl.createShader(gl.FRAGMENT_SHADER);
	gl.shaderSource(fs, fragmentSource);
	gl.compileShader(fs);
	if (!gl.getShaderParameter(fs, gl.COMPILE_STATUS)) {
		throw new Error(`fragment shader: ${gl.getShaderInfoLog(fs)}`);
	}
	const program = gl.createProgram();
	gl.attachShader(program, vs);
	gl.attachShader(program, fs);
	gl.linkProgram(program);
	if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
		throw new Error(`link: ${gl.getProgramInfoLog(program)}`);
	}
	return program;
}

function registerCommon(version: 1 | 2) {
	const label = version === 1 ? 'webgl' : 'webgl2';

	suite(`${label}.context`, () => {
		test('the context is available', () => {
			const { gl } = makeGL(version);
			ok(gl, `no ${label} context`);
		});

		test('getContext is idempotent', () => {
			const canvas = makeCanvas(32, 32);
			const name = version === 1 ? 'webgl' : 'webgl2';
			equal(canvas.getContext(name as any), canvas.getContext(name as any));
		});

		test('gl.canvas points back at the canvas', () => {
			const { canvas, gl } = makeGL(version);
			equal(gl.canvas, canvas);
		});

		test('drawingBufferWidth/Height match the canvas', () => {
			const { gl } = makeGL(version);
			equal(gl.drawingBufferWidth, 64, 'width');
			equal(gl.drawingBufferHeight, 64, 'height');
		});

		test('getContextAttributes reflects the request', () => {
			const canvas = makeCanvas(32, 32);
			const gl = canvas.getContext(version === 1 ? 'webgl' : ('webgl2' as any), { alpha: false, stencil: true, antialias: false }) as any;
			const attrs = gl.getContextAttributes();
			ok(attrs, 'getContextAttributes() returned nothing');
			equal(attrs.alpha, false, 'alpha');
			equal(attrs.stencil, true, 'stencil');
			equal(attrs.antialias, false, 'antialias');
		});

		test('the constant table has the spec values', () => {
			const { gl } = makeGL(version);
			const expected: Array<[string, number]> = [
				['DEPTH_BUFFER_BIT', 0x00000100],
				['STENCIL_BUFFER_BIT', 0x00000400],
				['COLOR_BUFFER_BIT', 0x00004000],
				['POINTS', 0x0000],
				['TRIANGLES', 0x0004],
				['ARRAY_BUFFER', 0x8892],
				['ELEMENT_ARRAY_BUFFER', 0x8893],
				['STATIC_DRAW', 0x88e4],
				['FLOAT', 0x1406],
				['UNSIGNED_BYTE', 0x1401],
				['RGBA', 0x1908],
				['TEXTURE_2D', 0x0de1],
				['TEXTURE0', 0x84c0],
				['VERTEX_SHADER', 0x8b31],
				['FRAGMENT_SHADER', 0x8b30],
				['COMPILE_STATUS', 0x8b81],
				['LINK_STATUS', 0x8b82],
				['NO_ERROR', 0],
				['INVALID_ENUM', 0x0500],
				['INVALID_VALUE', 0x0501],
				['INVALID_OPERATION', 0x0502],
				['BLEND', 0x0be2],
				['DEPTH_TEST', 0x0b71],
				['CULL_FACE', 0x0b44],
				['SRC_ALPHA', 0x0302],
				['ONE_MINUS_SRC_ALPHA', 0x0303],
			];
			const wrong: string[] = [];
			for (const [name, value] of expected) {
				if (gl[name] !== value) {
					wrong.push(`${name}=${gl[name]} (expected 0x${value.toString(16)})`);
				}
			}
			equal(wrong.length, 0, wrong.join(', '));
		});

		test('getError starts at NO_ERROR', () => {
			const { gl } = makeGL(version);
			equal(gl.getError(), gl.NO_ERROR);
		});

		test('a bad enum sets INVALID_ENUM', () => {
			const { gl } = makeGL(version);
			gl.getError();
			gl.enable(0x1234);
			equal(gl.getError(), gl.INVALID_ENUM);
		});

		test('getError clears the flag', () => {
			const { gl } = makeGL(version);
			gl.enable(0x1234);
			gl.getError();
			equal(gl.getError(), gl.NO_ERROR, 'the error flag was not cleared by the read');
		});

		test('getParameter returns the string parameters', () => {
			const { gl } = makeGL(version);
			for (const name of ['VERSION', 'VENDOR', 'RENDERER', 'SHADING_LANGUAGE_VERSION']) {
				const value = gl.getParameter(gl[name]);
				equal(typeof value, 'string', `${name} should be a string, got ${typeof value}`);
				ok((value as string).length > 0, `${name} is empty`);
			}
			const version_ = gl.getParameter(gl.VERSION) as string;
			ok(version_.indexOf('WebGL') === 0, `VERSION should start with "WebGL", got ${version_}`);
		});

		test('getParameter returns the numeric limits', () => {
			const { gl } = makeGL(version);
			for (const name of ['MAX_TEXTURE_SIZE', 'MAX_VERTEX_ATTRIBS', 'MAX_TEXTURE_IMAGE_UNITS', 'MAX_RENDERBUFFER_SIZE']) {
				const value = gl.getParameter(gl[name]);
				equal(typeof value, 'number', `${name} should be a number, got ${typeof value}`);
				ok(value > 0, `${name} is ${value}`);
			}
		});

		test('getParameter(VIEWPORT) returns an Int32Array of 4', () => {
			const { gl } = makeGL(version);
			const viewport = gl.getParameter(gl.VIEWPORT);
			ok(viewport, 'no viewport');
			equal(viewport.length, 4, 'length');
			arrayEqual(viewport, [0, 0, 64, 64]);
		});

		test('getParameter(COLOR_CLEAR_VALUE) round trips clearColor', () => {
			const { gl } = makeGL(version);
			gl.clearColor(0.25, 0.5, 0.75, 1);
			const value = gl.getParameter(gl.COLOR_CLEAR_VALUE);
			equal(value.length, 4, 'length');
			closeTo(value[0], 0.25, 1e-3, 'r');
			closeTo(value[1], 0.5, 1e-3, 'g');
			closeTo(value[2], 0.75, 1e-3, 'b');
		});

		test('isEnabled tracks enable/disable', () => {
			const { gl } = makeGL(version);
			equal(gl.isEnabled(gl.DEPTH_TEST), false, 'DEPTH_TEST is off by default');
			gl.enable(gl.DEPTH_TEST);
			equal(gl.isEnabled(gl.DEPTH_TEST), true, 'after enable');
			gl.disable(gl.DEPTH_TEST);
			equal(gl.isEnabled(gl.DEPTH_TEST), false, 'after disable');
		});

		test('getSupportedExtensions returns a list of strings', () => {
			const { gl } = makeGL(version);
			const list = gl.getSupportedExtensions();
			ok(Array.isArray(list), 'not an array');
			ok(list.length > 0, 'no extensions reported');
			for (const name of list) {
				equal(typeof name, 'string', 'extension names must be strings');
			}
		});

		test('getExtension returns null for an unknown extension', () => {
			const { gl } = makeGL(version);
			equal(gl.getExtension('NOT_A_REAL_EXTENSION'), null);
		});
	});

	suite(`${label}.objects`, () => {
		test('createBuffer / isBuffer / deleteBuffer', () => {
			const { gl } = makeGL(version);
			const buffer = gl.createBuffer();
			ok(buffer, 'createBuffer returned nothing');
			equal(gl.isBuffer(buffer), false, 'a buffer is not a buffer until it is bound');
			gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
			equal(gl.isBuffer(buffer), true, 'after bind');
			gl.deleteBuffer(buffer);
			equal(gl.isBuffer(buffer), false, 'after delete');
		});

		test('bufferData sets BUFFER_SIZE', () => {
			const { gl } = makeGL(version);
			const buffer = gl.createBuffer();
			gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
			gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([0, 1, 2, 3, 4, 5]), gl.STATIC_DRAW);
			equal(gl.getBufferParameter(gl.ARRAY_BUFFER, gl.BUFFER_SIZE), 24, 'BUFFER_SIZE');
			equal(gl.getBufferParameter(gl.ARRAY_BUFFER, gl.BUFFER_USAGE), gl.STATIC_DRAW, 'BUFFER_USAGE');
		});

		test('bufferData with a size allocates', () => {
			const { gl } = makeGL(version);
			const buffer = gl.createBuffer();
			gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
			gl.bufferData(gl.ARRAY_BUFFER, 64, gl.STATIC_DRAW);
			equal(gl.getBufferParameter(gl.ARRAY_BUFFER, gl.BUFFER_SIZE), 64);
		});

		test('a broken shader reports COMPILE_STATUS false and a log', () => {
			const { gl } = makeGL(version);
			const shader = gl.createShader(gl.FRAGMENT_SHADER);
			gl.shaderSource(shader, 'this is not glsl');
			gl.compileShader(shader);
			equal(gl.getShaderParameter(shader, gl.COMPILE_STATUS), false, 'COMPILE_STATUS');
			const log = gl.getShaderInfoLog(shader);
			equal(typeof log, 'string', 'getShaderInfoLog');
			ok(log.length > 0, 'a failed compile must produce an info log');
		});

		test('getShaderSource round trips', () => {
			const { gl } = makeGL(version);
			const shader = gl.createShader(gl.VERTEX_SHADER);
			gl.shaderSource(shader, VERTEX);
			equal(gl.getShaderSource(shader), VERTEX);
		});

		test('a program links and reports its attributes and uniforms', () => {
			const { gl } = makeGL(version);
			const program = buildProgram(gl);
			equal(gl.getProgramParameter(program, gl.LINK_STATUS), true, 'LINK_STATUS');
			equal(gl.getProgramParameter(program, gl.ACTIVE_ATTRIBUTES), 1, 'ACTIVE_ATTRIBUTES');
			equal(gl.getProgramParameter(program, gl.ACTIVE_UNIFORMS), 1, 'ACTIVE_UNIFORMS');
			ok(gl.getAttribLocation(program, 'aPos') >= 0, 'aPos should have a location');
			equal(gl.getAttribLocation(program, 'nope'), -1, 'an unknown attribute must be -1');
			ok(gl.getUniformLocation(program, 'uColor'), 'uColor should have a location');
			equal(gl.getUniformLocation(program, 'nope'), null, 'an unknown uniform must be null');
		});

		test('getActiveAttrib / getActiveUniform describe the program', () => {
			const { gl } = makeGL(version);
			const program = buildProgram(gl);
			const attrib = gl.getActiveAttrib(program, 0);
			ok(attrib, 'no active attrib');
			equal(attrib.name, 'aPos', 'name');
			equal(attrib.size, 1, 'size');
			equal(attrib.type, gl.FLOAT_VEC2, 'type');

			const uniform = gl.getActiveUniform(program, 0);
			ok(uniform, 'no active uniform');
			equal(uniform.name, 'uColor', 'name');
			equal(uniform.type, gl.FLOAT_VEC4, 'type');
		});

		test('getUniform reads back what uniform4f wrote', () => {
			const { gl } = makeGL(version);
			const program = buildProgram(gl);
			gl.useProgram(program);
			const loc = gl.getUniformLocation(program, 'uColor');
			gl.uniform4f(loc, 0.1, 0.2, 0.3, 0.4);
			const value = gl.getUniform(program, loc);
			equal(value.length, 4, 'length');
			closeTo(value[0], 0.1, 1e-3, 'x');
			closeTo(value[3], 0.4, 1e-3, 'w');
		});

		test('createTexture / isTexture / deleteTexture', () => {
			const { gl } = makeGL(version);
			const texture = gl.createTexture();
			gl.bindTexture(gl.TEXTURE_2D, texture);
			equal(gl.isTexture(texture), true, 'after bind');
			gl.deleteTexture(texture);
			equal(gl.isTexture(texture), false, 'after delete');
		});

		test('getShaderPrecisionFormat describes the precision', () => {
			const { gl } = makeGL(version);
			const format = gl.getShaderPrecisionFormat(gl.FRAGMENT_SHADER, gl.MEDIUM_FLOAT);
			ok(format, 'no precision format');
			equal(typeof format.rangeMin, 'number', 'rangeMin');
			equal(typeof format.rangeMax, 'number', 'rangeMax');
			equal(typeof format.precision, 'number', 'precision');
		});
	});

	suite(`${label}.draw`, () => {
		test('clear + readPixels round trip an exact colour', () => {
			const { gl } = makeGL(version);
			gl.clearColor(1, 0, 0, 1);
			gl.clear(gl.COLOR_BUFFER_BIT);
			const pixels = new Uint8Array(4);
			gl.readPixels(32, 32, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, pixels);
			arrayEqual(pixels, [255, 0, 0, 255]);
		});

		test('scissor limits the clear', () => {
			const { gl } = makeGL(version);
			gl.clearColor(0, 0, 1, 1);
			gl.clear(gl.COLOR_BUFFER_BIT);
			gl.enable(gl.SCISSOR_TEST);
			gl.scissor(0, 0, 10, 10);
			gl.clearColor(0, 1, 0, 1);
			gl.clear(gl.COLOR_BUFFER_BIT);
			gl.disable(gl.SCISSOR_TEST);

			const inside = new Uint8Array(4);
			gl.readPixels(5, 5, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, inside);
			arrayEqual(inside, [0, 255, 0, 255], 'inside the scissor rect');

			const outside = new Uint8Array(4);
			gl.readPixels(40, 40, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, outside);
			arrayEqual(outside, [0, 0, 255, 255], 'outside the scissor rect');
		});

		test('drawArrays paints the fragment colour', () => {
			const { gl } = makeGL(version);
			gl.viewport(0, 0, 64, 64);
			gl.clearColor(0, 0, 0, 1);
			gl.clear(gl.COLOR_BUFFER_BIT);

			const program = buildProgram(gl);
			gl.useProgram(program);
			const buffer = gl.createBuffer();
			gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
			// A full-screen triangle pair as one strip.
			gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 1, -1, -1, 1, 1, 1]), gl.STATIC_DRAW);
			const pos = gl.getAttribLocation(program, 'aPos');
			gl.enableVertexAttribArray(pos);
			gl.vertexAttribPointer(pos, 2, gl.FLOAT, false, 0, 0);
			gl.uniform4f(gl.getUniformLocation(program, 'uColor'), 0, 1, 0, 1);
			gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);

			equal(gl.getError(), gl.NO_ERROR, 'the draw raised an error');
			const pixels = new Uint8Array(4);
			gl.readPixels(32, 32, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, pixels);
			arrayEqual(pixels, [0, 255, 0, 255]);
		});

		test('drawElements paints the fragment colour', () => {
			const { gl } = makeGL(version);
			gl.viewport(0, 0, 64, 64);
			gl.clearColor(0, 0, 0, 1);
			gl.clear(gl.COLOR_BUFFER_BIT);

			const program = buildProgram(gl);
			gl.useProgram(program);
			const buffer = gl.createBuffer();
			gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
			gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 1, -1, -1, 1, 1, 1]), gl.STATIC_DRAW);
			const pos = gl.getAttribLocation(program, 'aPos');
			gl.enableVertexAttribArray(pos);
			gl.vertexAttribPointer(pos, 2, gl.FLOAT, false, 0, 0);

			const indices = gl.createBuffer();
			gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indices);
			gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Uint16Array([0, 1, 2, 2, 1, 3]), gl.STATIC_DRAW);

			gl.uniform4f(gl.getUniformLocation(program, 'uColor'), 0, 0, 1, 1);
			gl.drawElements(gl.TRIANGLES, 6, gl.UNSIGNED_SHORT, 0);

			equal(gl.getError(), gl.NO_ERROR, 'the draw raised an error');
			const pixels = new Uint8Array(4);
			gl.readPixels(32, 32, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, pixels);
			arrayEqual(pixels, [0, 0, 255, 255]);
		});

		test('texImage2D from a typed array then sample it', () => {
			const { gl } = makeGL(version);
			gl.viewport(0, 0, 64, 64);
			const texture = gl.createTexture();
			gl.bindTexture(gl.TEXTURE_2D, texture);
			gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 1, 1, 0, gl.RGBA, gl.UNSIGNED_BYTE, new Uint8Array([255, 128, 0, 255]));
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
			equal(gl.getError(), gl.NO_ERROR, 'texImage2D raised an error');

			const program = buildProgram(gl, `attribute vec2 aPos; void main() { gl_Position = vec4(aPos, 0.0, 1.0); }`, `precision mediump float; uniform sampler2D uTex; void main() { gl_FragColor = texture2D(uTex, vec2(0.5)); }`);
			gl.useProgram(program);
			const buffer = gl.createBuffer();
			gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
			gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 1, -1, -1, 1, 1, 1]), gl.STATIC_DRAW);
			const pos = gl.getAttribLocation(program, 'aPos');
			gl.enableVertexAttribArray(pos);
			gl.vertexAttribPointer(pos, 2, gl.FLOAT, false, 0, 0);
			gl.activeTexture(gl.TEXTURE0);
			gl.uniform1i(gl.getUniformLocation(program, 'uTex'), 0);
			gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);

			const pixels = new Uint8Array(4);
			gl.readPixels(32, 32, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, pixels);
			arrayEqual(pixels, [255, 128, 0, 255]);
		});

		test('texImage2D accepts an ImageData', () => {
			const { gl } = makeGL(version);
			const data = new ImageData(2, 2);
			for (let i = 0; i < data.data.length; i += 4) {
				data.data[i] = 200;
				data.data[i + 3] = 255;
			}
			const texture = gl.createTexture();
			gl.bindTexture(gl.TEXTURE_2D, texture);
			gl.getError();
			gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, data);
			equal(gl.getError(), gl.NO_ERROR, 'texImage2D(ImageData) raised an error');
		});

		test('a framebuffer round trips its attachment', () => {
			const { gl } = makeGL(version);
			const framebuffer = gl.createFramebuffer();
			gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
			const texture = gl.createTexture();
			gl.bindTexture(gl.TEXTURE_2D, texture);
			gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 16, 16, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
			gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, texture, 0);
			equal(gl.checkFramebufferStatus(gl.FRAMEBUFFER), gl.FRAMEBUFFER_COMPLETE, 'FRAMEBUFFER_COMPLETE');

			gl.viewport(0, 0, 16, 16);
			gl.clearColor(1, 1, 0, 1);
			gl.clear(gl.COLOR_BUFFER_BIT);
			const pixels = new Uint8Array(4);
			gl.readPixels(8, 8, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, pixels);
			arrayEqual(pixels, [255, 255, 0, 255]);
			gl.bindFramebuffer(gl.FRAMEBUFFER, null);
		});

		test('blending composites source over destination', () => {
			const { gl } = makeGL(version);
			gl.viewport(0, 0, 64, 64);
			gl.clearColor(0, 0, 0, 1);
			gl.clear(gl.COLOR_BUFFER_BIT);
			gl.enable(gl.BLEND);
			gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

			const program = buildProgram(gl);
			gl.useProgram(program);
			const buffer = gl.createBuffer();
			gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
			gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 1, -1, -1, 1, 1, 1]), gl.STATIC_DRAW);
			const pos = gl.getAttribLocation(program, 'aPos');
			gl.enableVertexAttribArray(pos);
			gl.vertexAttribPointer(pos, 2, gl.FLOAT, false, 0, 0);
			gl.uniform4f(gl.getUniformLocation(program, 'uColor'), 1, 0, 0, 0.5);
			gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);

			const pixels = new Uint8Array(4);
			gl.readPixels(32, 32, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, pixels);
			ok(Math.abs(pixels[0] - 128) <= 2, `expected a half-blended red, got [${Array.from(pixels).join(', ')}]`);
		});
	});
}

export function registerWebGLSpec() {
	registerCommon(1);
	registerCommon(2);

	suite('webgl2.only', () => {
		test('WebGL2 reports a WebGL 2.0 version string', () => {
			const { gl } = makeGL(2);
			const version = gl.getParameter(gl.VERSION) as string;
			ok(version.indexOf('WebGL 2') === 0, `expected a WebGL 2 version string, got ${version}`);
		});

		test('vertex array objects', () => {
			const { gl } = makeGL(2);
			const vao = gl.createVertexArray();
			ok(vao, 'createVertexArray returned nothing');
			gl.bindVertexArray(vao);
			equal(gl.isVertexArray(vao), true, 'after bind');
			gl.bindVertexArray(null);
			gl.deleteVertexArray(vao);
			equal(gl.isVertexArray(vao), false, 'after delete');
		});

		test('uniform buffer objects report their limits', () => {
			const { gl } = makeGL(2);
			const max = gl.getParameter(gl.MAX_UNIFORM_BUFFER_BINDINGS);
			equal(typeof max, 'number', 'MAX_UNIFORM_BUFFER_BINDINGS');
			ok(max > 0, `MAX_UNIFORM_BUFFER_BINDINGS is ${max}`);
		});

		test('drawArraysInstanced draws without error', () => {
			const { gl } = makeGL(2);
			gl.viewport(0, 0, 64, 64);
			gl.clearColor(0, 0, 0, 1);
			gl.clear(gl.COLOR_BUFFER_BIT);
			const program = buildProgram(gl, `#version 300 es\nin vec2 aPos;\nvoid main() { gl_Position = vec4(aPos, 0.0, 1.0); }`, `#version 300 es\nprecision mediump float;\nuniform vec4 uColor;\nout vec4 fragColor;\nvoid main() { fragColor = uColor; }`);
			gl.useProgram(program);
			const buffer = gl.createBuffer();
			gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
			gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 1, -1, -1, 1, 1, 1]), gl.STATIC_DRAW);
			const pos = gl.getAttribLocation(program, 'aPos');
			gl.enableVertexAttribArray(pos);
			gl.vertexAttribPointer(pos, 2, gl.FLOAT, false, 0, 0);
			gl.uniform4f(gl.getUniformLocation(program, 'uColor'), 1, 0, 1, 1);
			gl.drawArraysInstanced(gl.TRIANGLE_STRIP, 0, 4, 1);
			equal(gl.getError(), gl.NO_ERROR, 'drawArraysInstanced raised an error');
			const pixels = new Uint8Array(4);
			gl.readPixels(32, 32, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, pixels);
			arrayEqual(pixels, [255, 0, 255, 255]);
		});

		test('getFragDataLocation resolves an output', () => {
			const { gl } = makeGL(2);
			const program = buildProgram(gl, `#version 300 es\nin vec2 aPos;\nvoid main() { gl_Position = vec4(aPos, 0.0, 1.0); }`, `#version 300 es\nprecision mediump float;\nout vec4 fragColor;\nvoid main() { fragColor = vec4(1.0); }`);
			equal(gl.getFragDataLocation(program, 'fragColor'), 0, 'fragColor should be output 0');
			equal(gl.getFragDataLocation(program, 'nope'), -1, 'an unknown output must be -1');
		});

		test('texStorage2D allocates immutable storage', () => {
			const { gl } = makeGL(2);
			const texture = gl.createTexture();
			gl.bindTexture(gl.TEXTURE_2D, texture);
			gl.getError();
			gl.texStorage2D(gl.TEXTURE_2D, 1, gl.RGBA8, 16, 16);
			equal(gl.getError(), gl.NO_ERROR, 'texStorage2D raised an error');
		});
	});
}
