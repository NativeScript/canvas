const vertexShaderSrc = `
#version 100
precision highp float;
attribute vec2 position;
void main() {
  gl_Position = vec4(position, 0.0, 1.0);
  gl_PointSize = 128.0;
}`;

const fragmentShaderSrc = `
#version 100
precision mediump float;
void main() {
  vec2 fragmentPosition = 2.0*gl_PointCoord - 1.0;
  float distance = length(fragmentPosition);
  float distanceSqrd = distance * distance;
  gl_FragColor = vec4(
    0.2/distanceSqrd,
    0.1/distanceSqrd,
    0.0, 1.0 );
}`;


var gl: WebGLRenderingContext, program;

export function textures(canvas) {
  function setupWebGL() {
    if (!(gl = getRenderingContext()))
      return;
    var source = vertexShaderSrc;
    var vertexShader = gl.createShader(gl.VERTEX_SHADER);
    gl.shaderSource(vertexShader, source);
    gl.compileShader(vertexShader);
    source = fragmentShaderSrc;
    var fragmentShader = gl.createShader(gl.FRAGMENT_SHADER);
    gl.shaderSource(fragmentShader, source);
    gl.compileShader(fragmentShader);
    program = gl.createProgram();

    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    gl.linkProgram(program);
    gl.detachShader(program, vertexShader);
    gl.detachShader(program, fragmentShader);
    gl.deleteShader(vertexShader);
    gl.deleteShader(fragmentShader);
    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
      var linkErrLog = gl.getProgramInfoLog(program);
      cleanup();
      console.log(
        'Shader program did not link successfully. '
        + 'Error log: ' + linkErrLog);
      return;
    }
    initializeAttributes();
    gl.useProgram(program);
    gl.drawArrays(gl.POINTS, 0, 1);


    console.log('getParameter', gl.getParameter(gl.VIEWPORT));

    android.util.Log.d('JS',canvas.toDataURL());


    cleanup();
  }

  var buffer;

  function initializeAttributes() {
    gl.enableVertexAttribArray(0);
    buffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([0, 0]), gl.STATIC_DRAW);
    gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);
  }

  function cleanup() {
    gl.useProgram(null);
    if (buffer)
      gl.deleteBuffer(buffer);
    if (program)
      gl.deleteProgram(program);
  }


  function getRenderingContext() {
    var gl = canvas.getContext('webgl2')
      || canvas.getContext('experimental-webgl');
    if (!gl) {
      console.log('Failed to get WebGL context.'
        + 'Your device may not support WebGL.');
      return null;
    }
    gl.viewport(0, 0,
      gl.drawingBufferWidth, gl.drawingBufferHeight);
    gl.clearColor(0, 0, 0, 1.0);
    gl.clear(gl.COLOR_BUFFER_BIT);
    return gl;
  }

  setupWebGL();
}
