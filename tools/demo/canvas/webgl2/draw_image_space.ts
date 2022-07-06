import { createProgramFromScripts } from '../webgl/utils';
import { ImageAsset } from '@nativescript/canvas';
import { knownFolders } from '@nativescript/core';

export function draw_image_space(canvas) {
	const vs = `#version 300 es
precision highp float;
precision highp int;

void main()
{
    gl_Position = vec4(2.f * float(uint(gl_VertexID) % 2u) - 1.f, 2.f * float(uint(gl_VertexID) / 2u) - 1.f, 0.0, 1.0);
}`;
	const fs = `#version 300 es
    precision highp float;
    precision highp int;

    uniform sampler2D diffuse;

    uniform vec2 u_imageSize;

    out vec4 color;

    void main()
    {
        color = texture(diffuse, vec2(gl_FragCoord.x, u_imageSize.y - gl_FragCoord.y) / u_imageSize);
    }
    `;

	var gl = canvas.getContext('webgl2');
	var isWebGL2 = !!gl;
	if (!isWebGL2) {
		alert('WebGL 2 is not available.');
		return;
	}

	// -- Init program
	var program = createProgramFromScripts(gl, [
		{ type: 'vertex', src: vs },
		{ type: 'fragment', src: fs },
	]);
	var diffuseLocation = gl.getUniformLocation(program, 'diffuse');
	var imageSizeLocation = gl.getUniformLocation(program, 'u_imageSize');

	// -- Init VertexArray
	var vertexArray = gl.createVertexArray();
	gl.bindVertexArray(vertexArray);
	gl.bindVertexArray(null);
	const asset = new global.ImageAsset();

	asset.loadFileAsync('~/assets/file-assets/webgl2/Di-3d.png').then((done) => {
		if (done) {
			var texture = gl.createTexture();
			gl.activeTexture(gl.TEXTURE0);
			gl.bindTexture(gl.TEXTURE_2D, texture);
			gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, false);
			gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, asset);
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);

			// -- Render
			gl.clearColor(0, 0, 0, 1);
			gl.clear(gl.COLOR_BUFFER_BIT);

			gl.useProgram(program);
			gl.uniform1i(diffuseLocation, 0);
			gl.uniform2f(imageSizeLocation, gl.drawingBufferWidth / 2, gl.drawingBufferHeight / 2);

			gl.bindVertexArray(vertexArray);

			gl.drawArrays(gl.TRIANGLES, 0, 3);

			// Delete WebGL resources
			gl.deleteTexture(texture);
			gl.deleteProgram(program);
			gl.deleteVertexArray(vertexArray);
		}
	});
}
