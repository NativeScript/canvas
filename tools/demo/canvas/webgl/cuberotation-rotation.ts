import * as m4 from './m4';

let LAF = 0;

export function cubeRotationRotation(canvas) {
	function main() {
		var gl = canvas.getContext('webgl2') as WebGL2RenderingContext;
		if (!gl) {
			return;
		}

		const vertexShader3d = `
        attribute vec4 a_position;
        attribute vec2 a_texcoord;
        uniform mat4 u_matrix;
        varying vec2 v_texcoord;

        void main() {
            // Multiply the position by the matrix.
            gl_Position = u_matrix * a_position;

            // Pass the texcoord to the fragment shader.
            v_texcoord = a_texcoord;
        }`;

		const fragmentShader3d = `
        precision mediump float;
        // Passed in from the vertex shader.
        varying vec2 v_texcoord;
        // The texture.
        uniform sampler2D u_texture;

        void main() {
            gl_FragColor = texture2D(u_texture, v_texcoord);
        }`;

		// setup GLSL program

		// Create the shader object
		const vertexShader = gl.createShader(gl.VERTEX_SHADER);

		// Load the shader source
		gl.shaderSource(vertexShader, vertexShader3d);

		// Compile the shader
		gl.compileShader(vertexShader);

		// Check the compile status
		let compiled = gl.getShaderParameter(vertexShader, gl.COMPILE_STATUS);
		if (!compiled) {
			// Something went wrong during compilation; get the error
			const lastError = gl.getShaderInfoLog(vertexShader);
			console.log("*** Error compiling shader '" + vertexShader + "':" + lastError);
			gl.deleteShader(vertexShader);
			return null;
		}

		// Create the shader object
		const fragmentShader = gl.createShader(gl.FRAGMENT_SHADER);

		// Load the shader source
		gl.shaderSource(fragmentShader, fragmentShader3d);

		// Compile the shader
		gl.compileShader(fragmentShader);

		// Check the compile status
		compiled = gl.getShaderParameter(fragmentShader, gl.COMPILE_STATUS);
		if (!compiled) {
			// Something went wrong during compilation; get the error
			const lastError = gl.getShaderInfoLog(fragmentShader);
			console.log("*** Error compiling shader '" + fragmentShader + "':" + lastError);
			gl.deleteShader(fragmentShader);
			return null;
		}

		const program = gl.createProgram();

		gl.attachShader(program, vertexShader);

		gl.attachShader(program, fragmentShader);

		gl.linkProgram(program);

		// Check the link status
		const linked = gl.getProgramParameter(program, gl.LINK_STATUS);
		if (!linked) {
			// something went wrong with the link
			const lastError = gl.getProgramInfoLog(program);
			console.log('Error in program linking:' + lastError);

			gl.deleteProgram(program);
			return null;
		}

		// look up where the vertex data needs to go.
		var positionLocation = gl.getAttribLocation(program, 'a_position');
		var texcoordLocation = gl.getAttribLocation(program, 'a_texcoord');

		// lookup uniforms
		var matrixLocation = gl.getUniformLocation(program, 'u_matrix');
		var textureLocation = gl.getUniformLocation(program, 'u_texture');

		// Create a buffer for positions
		var positionBuffer = gl.createBuffer();
		// Bind it to ARRAY_BUFFER (think of it as ARRAY_BUFFER = positionBuffer)
		gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
		// Put the positions in the buffer
		setGeometry(gl);

		// provide texture coordinates for the rectangle.
		var texcoordBuffer = gl.createBuffer();
		gl.bindBuffer(gl.ARRAY_BUFFER, texcoordBuffer);
		// Set Texcoords.
		setTexcoords(gl);

		// Create a texture.
		var texture = gl.createTexture();
		gl.bindTexture(gl.TEXTURE_2D, texture);

		{
			// fill texture with 3x2 pixels
			const level = 0;
			const internalFormat = gl.LUMINANCE;
			const width = 3;
			const height = 2;
			const border = 0;
			const format = gl.LUMINANCE;
			const type = gl.UNSIGNED_BYTE;
			const data = new Uint8Array([128, 64, 128, 0, 192, 0]);
			const alignment = 1;
			gl.pixelStorei(gl.UNPACK_ALIGNMENT, alignment);
			gl.texImage2D(gl.TEXTURE_2D, level, internalFormat, width, height, border, format, type, data);

			// set the filtering so we don't need mips and it's not filtered
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
		}

		// Create a texture to render to
		const targetTextureWidth = 256 * 3;
		const targetTextureHeight = 256 * 3;
		const targetTexture = gl.createTexture();
		gl.bindTexture(gl.TEXTURE_2D, targetTexture);

		{
			// define size and format of level 0
			const level = 0;
			const internalFormat = gl.RGBA;
			const border = 0;
			const format = gl.RGBA;
			const type = gl.UNSIGNED_BYTE;
			const data = null;
			gl.texImage2D(gl.TEXTURE_2D, level, internalFormat, targetTextureWidth, targetTextureHeight, border, format, type, data);

			// set the filtering so we don't need mips
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
			gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
		}

		// Create and bind the framebuffer
		const fb = gl.createFramebuffer();
		gl.bindFramebuffer(gl.FRAMEBUFFER, fb);

		// attach the texture as the first color attachment
		const attachmentPoint = gl.COLOR_ATTACHMENT0;
		const level = 0;
		gl.framebufferTexture2D(gl.FRAMEBUFFER, attachmentPoint, gl.TEXTURE_2D, targetTexture, level);

		function degToRad(d) {
			return (d * Math.PI) / 180;
		}

		var fieldOfViewRadians = degToRad(60);
		var modelXRotationRadians = degToRad(0);
		var modelYRotationRadians = degToRad(0);

		// Get the starting time.
		var then = 0;

		requestAnimationFrame(drawScene);

		function drawCube(aspect) {
			// Tell it to use our program (pair of shaders)
			gl.useProgram(program);

			// Turn on the position attribute
			gl.enableVertexAttribArray(positionLocation);

			// Bind the position buffer.
			gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

			// Tell the position attribute how to get data out of positionBuffer (ARRAY_BUFFER)
			var size = 3; // 3 components per iteration
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

			// Compute the projection matrix
			var projectionMatrix = m4.perspective(fieldOfViewRadians, aspect, 1, 2000);

			var cameraPosition = [0, 0, 2];
			var up = [0, 1, 0];
			var target = [0, 0, 0];

			// Compute the camera's matrix using look at.
			var cameraMatrix = m4.lookAt(cameraPosition, target, up);

			// Make a view matrix from the camera matrix.
			var viewMatrix = m4.inverse(cameraMatrix);

			var viewProjectionMatrix = m4.multiply(projectionMatrix, viewMatrix);

			var matrix = m4.xRotate(viewProjectionMatrix, modelXRotationRadians);
			matrix = m4.yRotate(matrix, modelYRotationRadians);

			// Set the matrix.
			gl.uniformMatrix4fv(matrixLocation, false, matrix);

			// Tell the shader to use texture unit 0 for u_texture

			gl.uniform1i(textureLocation, 0);

			// Draw the geometry.
			gl.drawArrays(gl.TRIANGLES, 0, 6 * 6);
		}

		// Draw the scene.
		function drawScene(time) {
			// convert to seconds
			time *= 0.001;
			// Subtract the previous time from the current time
			var deltaTime = time - then;
			// Remember the current time for the next frame.
			then = time;

			// Animate the rotation
			modelYRotationRadians += -0.7 * deltaTime;
			modelXRotationRadians += -0.4 * deltaTime;

			//webglUtils.resizeCanvasToDisplaySize(gl.canvas);

			gl.enable(gl.CULL_FACE);
			gl.enable(gl.DEPTH_TEST);

			{
				// render to our targetTexture by binding the framebuffer
				gl.bindFramebuffer(gl.FRAMEBUFFER, fb);

				// render cube with our 3x2 texture
				gl.bindTexture(gl.TEXTURE_2D, texture);

				// Tell WebGL how to convert from clip space to pixels
				gl.viewport(0, 0, targetTextureWidth, targetTextureHeight);

				// Clear the attachment(s).
				gl.clearColor(0, 0, 1, 1); // clear to blue
				gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

				const aspect = targetTextureWidth / targetTextureHeight;
				drawCube(aspect);
			}

			{
				// render to the canvas
				gl.bindFramebuffer(gl.FRAMEBUFFER, null);

				// render the cube with the texture we just rendered to
				gl.bindTexture(gl.TEXTURE_2D, targetTexture);
				// Tell WebGL how to convert from clip space to pixels
				gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);

				// Clear the canvas AND the depth buffer.
				gl.clearColor(1, 1, 1, 1); // clear to white
				gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

				const aspect = gl.drawingBufferWidth / gl.drawingBufferHeight;
				drawCube(aspect);
			}

			LAF = requestAnimationFrame(drawScene);
		}
	}

	// Fill the buffer with the values that define a cube.
	function setGeometry(gl) {
		var positions = new Float32Array([
			-0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5,

			-0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5,

			-0.5, 0.5, -0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5,

			-0.5, -0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5,

			-0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5,

			0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5,
		]);
		gl.bufferData(gl.ARRAY_BUFFER, positions, gl.STATIC_DRAW);
	}

	// Fill the buffer with texture coordinates the cube.
	function setTexcoords(gl) {
		gl.bufferData(
			gl.ARRAY_BUFFER,
			new Float32Array([
				0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0,

				0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1,

				0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0,

				0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1,

				0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0,

				0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1,
			]),
			gl.STATIC_DRAW
		);
	}

	main();
}

export function cancelCubeRotationRotation() {
	cancelAnimationFrame(LAF);
	LAF = 0;
}
