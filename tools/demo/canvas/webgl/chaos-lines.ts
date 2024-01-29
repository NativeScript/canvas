import { Canvas } from '@nativescript/canvas';

export function createChaosLines(canvas: HTMLCanvasElement) {
	function createShader(gl, type, source) {
		var shader = gl.createShader(type);
		gl.shaderSource(shader, source);
		gl.compileShader(shader);

		if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
			console.log('An error occurred compiling the shaders: ' + gl.getShaderInfoLog(shader));
			gl.deleteShader(shader);
			return null;
		}

		return shader;
	}

	function initWebGL(canvas) {
		const gl = canvas.getContext('webgl2');

		if (!gl) {
			alert('Unable to initialize WebGL. Your browser may not support it.');
			return;
		}

		const vertexShaderSource = `
    attribute vec4 aVertexPosition;
    void main() {
        gl_Position = aVertexPosition;
    }`;

		const fragmentShaderSource = `
    precision mediump float;

    uniform float u_time;
    uniform vec2 u_resolution;

    float hash(float n) {
        return fract(sin(n) * 753.5453123);
    }

    float noise(vec2 p) {
        vec2 i = floor(p);
        vec2 f = fract(p);
        f = f * f * (3.0 - 2.0 * f);
        float n = i.x + i.y * 157.0;
        return mix(mix(hash(n + 0.0), hash(n + 1.0), f.x),
                   mix(hash(n + 157.0), hash(n + 158.0), f.x), f.y);
    }

    float fbm(vec2 p, vec3 a) {
        float v = 0.0;
        v += noise(p * a.x) * 0.50;
        v += noise(p * a.y) * 1.50;
        v += noise(p * a.z) * 0.125 * 0.1;
        return v;
    }

    vec3 drawLines(vec2 uv, vec3 fbmOffset, vec3 color1, vec3 colorSet[4], float secs) {
        float timeVal = secs * 0.05;
        vec3 finalColor = vec3(0.0);
        for (int i = 0; i < 4; ++i) {
            float indexAsFloat = float(i);
            float amp = 80.0 + indexAsFloat * 0.0;
            float period = 2.0 + indexAsFloat + 2.0;
            float thickness = mix(0.4, 0.2, noise(uv * 2.0));
            float t = abs(1.0 / (sin(uv.y + fbm(uv + timeVal * period, fbmOffset)) * amp) * thickness);
            finalColor += t * colorSet[i];
        }

        for (int i = 0; i < 4; ++i) {
            float indexAsFloat = float(i);
            float amp = 40.0 + indexAsFloat * 5.0;
            float period = 9.0 + indexAsFloat + 2.0;
            float thickness = mix(0.1, 0.1, noise(uv * 12.0));
            float t = abs(1.0 / (sin(uv.y + fbm(uv + timeVal * period, fbmOffset)) * amp) * thickness);
            finalColor += t * colorSet[i] * color1;
        }
        return finalColor;
    }

    void main() {
        vec2 uv = (gl_FragCoord.xy / u_resolution) * 0.75 - 2.0;
        uv.x *= u_resolution.x / u_resolution.y;

        vec3 lineColor1 = vec3(1.0, 0.0, 0.5);
        vec3 lineColor2 = vec3(0.3, 0.5, 1.5);

        float spread = 1.0;
        vec3 finalColor = vec3(0.0);
        vec3 colorSet[4];
        colorSet[0] = vec3(0.7, 0.05, 1.0);
        colorSet[1] = vec3(1.0, 0.19, 0.0);
        colorSet[2] = vec3(0.0, 1.0, 0.3);
        colorSet[3] = vec3(0.0, 0.38, 1.0);
        float t = sin(u_time) * 0.5 + 0.5;
        float pulse = mix(0.05, 0.20, t);

        finalColor = drawLines(uv, vec3(65.2, 40.0, 4.0), lineColor1, colorSet, u_time * 0.1) * pulse;
        finalColor += drawLines(uv, vec3(5.0 * spread / 2.0, 2.1 * spread, 1.0), lineColor2, colorSet, u_time);

        gl_FragColor = vec4(finalColor, 1.0);
    }
`;

		const vertexShader = createShader(gl, gl.VERTEX_SHADER, vertexShaderSource);
		const fragmentShader = createShader(gl, gl.FRAGMENT_SHADER, fragmentShaderSource);
		if (!vertexShader || !fragmentShader) {
			console.error('Shader creation failed.');
			return;
		}

		const shaderProgram = gl.createProgram();
		gl.attachShader(shaderProgram, vertexShader);
		gl.attachShader(shaderProgram, fragmentShader);
		gl.linkProgram(shaderProgram);

		if (!gl.getProgramParameter(shaderProgram, gl.LINK_STATUS)) {
			console.error('Unable to initialize the shader program: ' + gl.getProgramInfoLog(shaderProgram));
			return;
		}

		const positionBuffer = gl.createBuffer();
		gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
		const positions = [1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0];
		gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);

		const vertexPosition = gl.getAttribLocation(shaderProgram, 'aVertexPosition');
		gl.vertexAttribPointer(vertexPosition, 2, gl.FLOAT, false, 0, 0);
		gl.enableVertexAttribArray(vertexPosition);

		gl.useProgram(shaderProgram);
		const u_time = gl.getUniformLocation(shaderProgram, 'u_time');
		const u_resolution = gl.getUniformLocation(shaderProgram, 'u_resolution');

		const width = canvas.width;
		const height = canvas.height;
		let start = 0;
		function drawScene(now) {
			if (start === 0) {
				start = now;
			}
			// now *= 0.001;
			now = (now - start) * 0.0001;
			requestAnimationFrame(drawScene);
			gl.uniform1f(u_time, now);
			gl.uniform2f(u_resolution, width, height);
			gl.clearColor(0.0, 0.0, 0.0, 1.0);
			gl.clear(gl.COLOR_BUFFER_BIT);
			gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
			gl.vertexAttribPointer(vertexPosition, 2, gl.FLOAT, false, 0, 0);
			gl.enableVertexAttribArray(vertexPosition);
			gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
		}

		requestAnimationFrame(drawScene);
	}

	initWebGL(canvas);
}

/*

export function createChaosLines(canvas) {
	const gl = canvas.getContext('webgl2');

	if (!gl) {
		alert('Unable to initialize WebGL. Your browser may not support it.');
		return;
	}

	const vertexShaderSource = `
        attribute vec4 aVertexPosition;
        void main() {
            gl_Position = aVertexPosition;
        }`;

	const fragmentShaderSource = `
        precision highp float;

        uniform float u_time;
        uniform vec2 u_resolution;

        float hash(float n) {
            return fract(sin(n) * 753.5453123);
        }

        float noise(vec2 p) {
            vec2 i = floor(p);
            vec2 f = fract(p);
            f = f * f * (3.0 - 2.0 * f);
            float n = i.x + i.y * 157.0;
            return mix(mix(hash(n + 0.0), hash(n + 1.0), f.x),
                       mix(hash(n + 157.0), hash(n + 158.0), f.x), f.y);
        }

        float fbm(vec2 p, vec3 a) {
            float v = 0.0;
            v += noise(p * a.x) * 0.50;
            v += noise(p * a.y) * 1.50;
            v += noise(p * a.z) * 0.125 * 0.1;
            return v;
        }

        vec3 drawLines(vec2 uv, vec3 fbmOffset, vec3 color1, vec3 colorSet[4], float secs) {
            float timeVal = secs * 0.05;
            vec3 finalColor = vec3(0.0);

            for (int i = 0; i < 4; ++i) {
                float indexAsFloat = float(i);
                float amp = 80.0 + indexAsFloat * 0.0;
                float period = 2.0 + indexAsFloat + 2.0;
                float thickness = mix(0.4, 0.2, noise(uv * 2.0));
                float t = abs(1.0 / (sin(uv.y + fbm(uv + timeVal * period, fbmOffset)) * amp) * thickness);
                finalColor += t * colorSet[i];
            }

            for (int i = 0; i < 4; ++i) {
                float indexAsFloat = float(i);
                float amp = 40.0 + indexAsFloat * 5.0;
                float period = 9.0 + indexAsFloat + 2.0;
                float thickness = mix(0.1, 0.1, noise(uv * 12.0));
                float t = abs(1.0 / (sin(uv.y + fbm(uv + timeVal * period, fbmOffset)) * amp) * thickness);
                finalColor += t * colorSet[i] * color1;
            }

            return finalColor;
        }

        void main() {
            vec2 uv = (gl_FragCoord.xy / u_resolution) * 0.75 - vec2(2.0);
            uv.x *= u_resolution.x / u_resolution.y;

            vec3 lineColor1 = vec3(1.0, 0.0, 0.5);
            vec3 lineColor2 = vec3(0.3, 0.5, 1.5);

            float spread = 1.0;
            vec3 finalColor = vec3(0.0);
           

            vec3 colorSet[4];
colorSet[0] = vec3(0.7, 0.05, 1.0);
colorSet[1] = vec3(1.0, 0.19, 0.0);
colorSet[2] = vec3(0.0, 1.0, 0.3);
colorSet[3] = vec3(0.0, 0.38, 1.0);

            float t = sin(u_time) * 0.5 + 0.5;
            float pulse = mix(0.05, 0.20, t);

            finalColor = drawLines(uv, vec3(65.2, 40.0, 4.0), lineColor1, colorSet, u_time * 0.1) * pulse;
            finalColor += drawLines(uv, vec3(5.0 * spread / 2.0, 2.1 * spread, 1.0), lineColor2, colorSet, u_time);

            gl_FragColor = vec4(finalColor, 1.0);
        }
    `;

	const vertexShader = createShader(gl, gl.VERTEX_SHADER, vertexShaderSource);
	const fragmentShader = createShader(gl, gl.FRAGMENT_SHADER, fragmentShaderSource);

	if (!vertexShader || !fragmentShader) {
		console.error('Shader creation failed.');
		return;
	}

	const shaderProgram = gl.createProgram();
	gl.attachShader(shaderProgram, vertexShader);
	gl.attachShader(shaderProgram, fragmentShader);
	gl.linkProgram(shaderProgram);

	if (!gl.getProgramParameter(shaderProgram, gl.LINK_STATUS)) {
		console.error('Unable to initialize the shader program: ' + gl.getProgramInfoLog(shaderProgram));
		return;
	}

	const positionBuffer = gl.createBuffer();
	const positions = new Float32Array([1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0]);

	gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
	gl.bufferData(gl.ARRAY_BUFFER, positions, gl.STATIC_DRAW);

	const vertexPosition = gl.getAttribLocation(shaderProgram, 'aVertexPosition');
	gl.vertexAttribPointer(vertexPosition, 2, gl.FLOAT, false, 0, 0);
	gl.enableVertexAttribArray(vertexPosition);

	gl.useProgram(shaderProgram);

	const u_time = gl.getUniformLocation(shaderProgram, 'u_time');
	const u_resolution = gl.getUniformLocation(shaderProgram, 'u_resolution');

	const width = canvas.width;
	const height = canvas.height;
    const time = java.lang.System.currentTimeMillis() * 1000;
	function drawScene(now) {
		now = (now *= 0.001)  / 1000000;
		requestAnimationFrame(drawScene);

		gl.uniform1f(u_time, now);
		gl.uniform2f(u_resolution, width, height);

		gl.clearColor(0.0, 0.0, 0.0, 1.0);
		gl.clear(gl.COLOR_BUFFER_BIT);

		gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
	}

	requestAnimationFrame(drawScene);
}

function createShader(gl, type, source) {
	const shader = gl.createShader(type);
	gl.shaderSource(shader, source);
	gl.compileShader(shader);

	if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
		console.log('An error occurred compiling the shaders: ' + gl.getShaderInfoLog(shader));
		gl.deleteShader(shader);
		return null;
	}

	return shader;
}
*/
