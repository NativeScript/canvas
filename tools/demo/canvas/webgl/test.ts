import { Canvas } from "@nativescript/canvas";

export function subTest(canvas) {
	const gl = canvas.getContext('webgl2');

	if (!gl) {
		console.error('Unable to initialize WebGL. Your browser may not support it.');
		return;
	}

	// Vertex and fragment shader source code
	const vsSource = `
        attribute vec4 a_position;
        void main() {
            gl_Position = a_position;
        }
    `;

	const fsSource = `
        precision mediump float;
        uniform vec4 u_color;
        void main() {
            gl_FragColor = u_color;
        }
    `;

	// Compile shader and create program
	function compileShader(gl, source, type) {
		const shader = gl.createShader(type);
		gl.shaderSource(shader, source);
		gl.compileShader(shader);

		if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
			console.error('Shader compilation error:', gl.getShaderInfoLog(shader));
			gl.deleteShader(shader);
			return null;
		}

		return shader;
	}

	const vertexShader = compileShader(gl, vsSource, gl.VERTEX_SHADER);
	const fragmentShader = compileShader(gl, fsSource, gl.FRAGMENT_SHADER);

	const program = gl.createProgram();
	gl.attachShader(program, vertexShader);
	gl.attachShader(program, fragmentShader);
	gl.linkProgram(program);

	if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
		console.error('Program linking error:', gl.getProgramInfoLog(program));
		return;
	}

	gl.useProgram(program);

	// Create a texture and load an initial image
	const texture = gl.createTexture();
	gl.bindTexture(gl.TEXTURE_2D, texture);

	const initialImage = new Image();
	initialImage.src = 'https://webglfundamentals.org/webgl/resources/leaves.jpg';

	initialImage.onload = function () {



        const c = Canvas.createCustomView();
        c.width = 512;
        c.height = 512;
        const c2d = c.getContext('2d');
        c2d.drawImage(initialImage, 0, 0);

		gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, c);

		// // Set the clear color and clear the canvas
		// gl.clearColor(0.0, 0.0, 0.0, 1.0);
		// gl.clear(gl.COLOR_BUFFER_BIT);

		// Draw the initial texture
		drawTexture();

		// Update a portion of the texture using texSubImage2D
		//updateTexture();
	};

	// Define the vertex buffer
	const positionBuffer = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
	const positions = new Float32Array([-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0]);
	gl.bufferData(gl.ARRAY_BUFFER, positions, gl.STATIC_DRAW);

	// Get the attribute location and enable it
	const positionAttrib = gl.getAttribLocation(program, 'a_position');
	gl.vertexAttribPointer(positionAttrib, 2, gl.FLOAT, false, 0, 0);
	gl.enableVertexAttribArray(positionAttrib);

	// Define the uniform location for the color
	const colorUniform = gl.getUniformLocation(program, 'u_color');

	// Function to draw the texture
	function drawTexture() {
		gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
	}

	// Function to update a portion of the texture
	function updateTexture() {
		// Assuming the updated portion is a square at (x, y) with size (width, height)
		const x = 50;
		const y = 50;
		const width = 100;
		const height = 100;

		// Create a new image or use another source
		const updatedImage = new Image();
		updatedImage.src = 'https://webglfundamentals.org/webgl/resources/leaves.jpg';

		updatedImage.onload = function () {
			// Update the portion of the texture using texSubImage2D
			gl.texSubImage2D(gl.TEXTURE_2D, 0, x, y, gl.RGBA, gl.UNSIGNED_BYTE, updatedImage);

			// Set the color to white and draw the updated texture
			gl.uniform4f(colorUniform, 1.0, 1.0, 1.0, 1.0);
			drawTexture();
		};
	}
}
