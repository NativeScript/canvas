import { Canvas } from '@nativescript/canvas';
import { createProgramFromScripts } from './utils';
import { ImageSource } from '@nativescript/core';
export function imageProcessing(canvas) {
	const vertexShader2d = {
		type: 'vertex',
		src: `attribute vec2 a_position;
        attribute vec2 a_texCoord;

        uniform vec2 u_resolution;

        varying vec2 v_texCoord;

        void main() {
           // convert the rectangle from pixels to 0.0 to 1.0
           vec2 zeroToOne = a_position / u_resolution;

           // convert from 0->1 to 0->2
           vec2 zeroToTwo = zeroToOne * 2.0;

           // convert from 0->2 to -1->+1 (clipspace)
           vec2 clipSpace = zeroToTwo - 1.0;

           gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);

           // pass the texCoord to the fragment shader
           // The GPU will interpolate this value between points.
           v_texCoord = a_texCoord;
        }`,
	};
	const fragmentShader2d = {
		type: 'fragment',
		src: `precision mediump float;

        // our texture
        uniform sampler2D u_image;

        // the texCoords passed in from the vertex shader.
        varying vec2 v_texCoord;

        void main() {
           gl_FragColor = texture2D(u_image, v_texCoord);
        }`,
	};
	function main() {
		const asset = new global.ImageAsset();
		asset.fromUrl('https://webglfundamentals.org/webgl/resources/leaves.jpg').then((image) => {
			//  render(asset);

			// const c = Canvas.createCustomView();
			// c.width = 512;
			// c.height = 512;
			// const c2d = c.getContext('2d');
			// c2d.drawImage(asset, 0, 0);

			render(asset);
		});
	}

	function render(image) {
		var gl = canvas.getContext('webgl') as WebGLRenderingContext;
		if (!gl) {
			return;
		}

		// setup GLSL program
		var program = createProgramFromScripts(gl, [vertexShader2d, fragmentShader2d]);

		// look up where the vertex data needs to go.
		var positionLocation = gl.getAttribLocation(program, 'a_position');
		var texcoordLocation = gl.getAttribLocation(program, 'a_texCoord');

		// Create a buffer to put three 2d clip space points in
		var positionBuffer = gl.createBuffer();

		// Bind it to ARRAY_BUFFER (think of it as ARRAY_BUFFER = positionBuffer)
		gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
		// Set a rectangle the same size as the image.
		setRectangle(gl, 0, 0, image.width, image.height);

		// provide texture coordinates for the rectangle.
		var texcoordBuffer = gl.createBuffer();
		gl.bindBuffer(gl.ARRAY_BUFFER, texcoordBuffer);
		gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0]), gl.STATIC_DRAW);

		// Create a texture.
		var texture = gl.createTexture();
		gl.bindTexture(gl.TEXTURE_2D, texture);

		// Set the parameters so we can render any size image.
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);

		// Upload the image into the texture.
		//(<WebGL2RenderingContext>gl).texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, image.width, image.height, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);

		//(<WebGL2RenderingContext>gl).texSubImage2D(gl.TEXTURE_2D, 0, 0, 0, gl.RGBA, gl.UNSIGNED_BYTE, image);
		
		gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, image);

		// lookup uniforms
		var resolutionLocation = gl.getUniformLocation(program, 'u_resolution');

		// webglUtils.resizeCanvasToDisplaySize(gl.canvas);

		// Tell WebGL how to convert from clip space to pixels
		gl.viewport(0, 0, canvas.width, canvas.height);

		// Clear the canvas
		gl.clearColor(0, 0, 0, 0);
		gl.clear(gl.COLOR_BUFFER_BIT);

		// Tell it to use our program (pair of shaders)
		gl.useProgram(program);

		// Turn on the position attribute
		gl.enableVertexAttribArray(positionLocation);

		// Bind the position buffer.
		gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

		// Tell the position attribute how to get data out of positionBuffer (ARRAY_BUFFER)
		var size = 2; // 2 components per iteration
		var type = gl.FLOAT; // the data is 32bit floats
		var normalize = false; // don't normalize the data
		var stride = 0; // 0 = move forward size * sizeof(type) each iteration to get the next position
		var offset = 0; // start at the beginning of the buffer
		gl.vertexAttribPointer(positionLocation, size, type, normalize, stride, offset);

		// Turn on the teccord attribute
		gl.enableVertexAttribArray(texcoordLocation);

		// Bind the position buffer.
		gl.bindBuffer(gl.ARRAY_BUFFER, texcoordBuffer);

		// Tell the position attribute how to get data out of positionBuffer (ARRAY_BUFFER)
		var size = 2; // 2 components per iteration
		var type = gl.FLOAT; // the data is 32bit floats
		var normalize = false; // don't normalize the data
		var stride = 0; // 0 = move forward size * sizeof(type) each iteration to get the next position
		var offset = 0; // start at the beginning of the buffer
		gl.vertexAttribPointer(texcoordLocation, size, type, normalize, stride, offset);

		// set the resolution
		gl.uniform2f(resolutionLocation, canvas.width, canvas.height);

		// Draw the rectangle.
		var primitiveType = gl.TRIANGLES;
		var offset = 0;
		var count = 6;
		gl.drawArrays(primitiveType, offset, count);
	}

	function setRectangle(gl, x, y, width, height) {
		var x1 = x;
		var x2 = x + width;
		var y1 = y;
		var y2 = y + height;
		gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2]), gl.STATIC_DRAW);
	}

	main();
}
