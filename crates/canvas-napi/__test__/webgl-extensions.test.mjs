// WebGL extensions on ANGLE (Windows): what each WebGL version offers, enabling on getExtension,
// and the extension objects' members packages/canvas reads (ext_name, constants, methods).
import assert from 'node:assert/strict';
import test from 'node:test';

import { CanvasModule } from './addon.mjs';

const skip = process.platform !== 'win32' && 'ANGLE on Windows only';

const TEXTURE_2D = 0x0de1;
const RGBA = 0x1908;
const FLOAT = 0x1406;
const NO_ERROR = 0;

function webgl(version) {
	return version === 2 ? CanvasModule.createWebGL2Context({ version: 2 }, 16, 16) : CanvasModule.createWebGLContext({ version: 1 }, 16, 16);
}

test('getSupportedExtensions lists WebGL names, per version', { skip }, () => {
	const v1 = webgl(1).getSupportedExtensions();
	const v2 = webgl(2).getSupportedExtensions();
	assert.ok(v1.every((name) => !name.startsWith('GL_')), `GL names leaked: ${v1}`);
	for (const name of ['ANGLE_instanced_arrays', 'OES_vertex_array_object', 'OES_texture_float', 'WEBGL_lose_context']) {
		assert.ok(v1.includes(name), `${name} missing from WebGL 1`);
	}
	// WebGL 2 has these in core.
	for (const name of ['ANGLE_instanced_arrays', 'OES_vertex_array_object', 'OES_texture_float', 'WEBGL_draw_buffers']) {
		assert.ok(!v2.includes(name), `${name} offered on WebGL 2`);
	}
	assert.ok(v2.includes('EXT_color_buffer_float'));
	assert.ok(!v1.includes('EXT_color_buffer_float'));
});

test('every supported extension comes back with its ext_name', { skip }, () => {
	for (const version of [1, 2]) {
		const gl = webgl(version);
		for (const name of gl.getSupportedExtensions()) {
			assert.equal(gl.getExtension(name)?.ext_name, name, `WebGL ${version} ${name}`);
		}
		assert.equal(gl.getExtension('NOT_an_extension'), null);
	}
});

test('getExtension enables the extension (float textures after OES_texture_float)', { skip }, () => {
	const gl = webgl(1);
	gl.bindTexture(TEXTURE_2D, gl.createTexture());
	gl.texImage2D(TEXTURE_2D, 0, RGBA, 1, 1, 0, RGBA, FLOAT, new Float32Array(4));
	assert.notEqual(gl.getError(), NO_ERROR);
	assert.ok(gl.getExtension('OES_texture_float'));
	gl.texImage2D(TEXTURE_2D, 0, RGBA, 1, 1, 0, RGBA, FLOAT, new Float32Array(4));
	assert.equal(gl.getError(), NO_ERROR);
});

test('WebGL 1 extension objects work on an ES 2 context', { skip }, () => {
	const gl = webgl(1);
	const vao = gl.getExtension('OES_vertex_array_object');
	assert.equal(vao.VERTEX_ARRAY_BINDING_OES, 0x85b5);
	const array = vao.createVertexArrayOES();
	vao.bindVertexArrayOES(array);
	assert.equal(vao.isVertexArrayOES(array), true);
	vao.deleteVertexArrayOES(array);

	const instanced = gl.getExtension('ANGLE_instanced_arrays');
	assert.equal(instanced.VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE, 0x88fe);
	instanced.vertexAttribDivisorANGLE(0, 1);

	const drawBuffers = gl.getExtension('WEBGL_draw_buffers');
	assert.equal(drawBuffers.COLOR_ATTACHMENT1_EXT, 0x8ce1);
	assert.equal(drawBuffers.DRAW_BUFFER0_EXT, 0x8825);
	// The default framebuffer takes BACK (COLOR_ATTACHMENTi is for framebuffer objects).
	drawBuffers.drawBuffersWEBGL([0x0405]);

	const minmax = gl.getExtension('EXT_blend_minmax');
	assert.deepEqual([minmax.MIN_EXT, minmax.MAX_EXT], [0x8007, 0x8008]);
	assert.equal(gl.getExtension('OES_texture_half_float').HALF_FLOAT_OES, 0x8d61);
	assert.equal(gl.getExtension('EXT_texture_filter_anisotropic').TEXTURE_MAX_ANISOTROPY_EXT, 0x84fe);
	assert.equal(gl.getError(), NO_ERROR);
});
