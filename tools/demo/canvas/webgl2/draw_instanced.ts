import {createProgramFromScripts} from '../webgl/utils';

export function draw_instanced(canvas) {
  const vs = `#version 300 es
    #define POSITION_LOCATION 0
    #define COLOR_LOCATION 1

    precision highp float;
    precision highp int;

    layout(location = POSITION_LOCATION) in vec2 pos;
    layout(location = COLOR_LOCATION) in vec4 color;
    flat out vec4 v_color;

    void main()
    {
        v_color = color;
        gl_Position = vec4(pos + vec2(float(gl_InstanceID) - 0.5, 0.0), 0.0, 1.0);
    }`;

  const fs = `#version 300 es
   precision highp float;
   precision highp int;

   flat in vec4 v_color;
   out vec4 color;

   void main()
   {
       color = v_color;
   }`;

  // -- Init WebGL Context
  var gl = canvas.getContext('webgl2');
  var isWebGL2 = !!gl;
  if (!isWebGL2) {
    alert('WebGL 2 is not available.')
    return;
  }

  // -- Init Program
  var program = createProgramFromScripts(gl, [{type: 'vertex', src: vs}, {type: 'fragment', src: fs}]);
  gl.useProgram(program);


  // -- Init Vertex Array
  var vertexArray = gl.createVertexArray();
  gl.bindVertexArray(vertexArray);

  // -- Init Buffers
  var vertexPosLocation = 0;  // set with GLSL layout qualifier
  var vertices = new Float32Array([
    -0.3, -0.5,
    0.3, -0.5,
    0.0, 0.5
  ]);
  var vertexPosBuffer = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, vertexPosBuffer);
  gl.bufferData(gl.ARRAY_BUFFER, vertices, gl.STATIC_DRAW);
  gl.enableVertexAttribArray(vertexPosLocation);
  gl.vertexAttribPointer(vertexPosLocation, 2, gl.FLOAT, false, 0, 0);

  var vertexColorLocation = 1;  // set with GLSL layout qualifier
  var colors = new Float32Array([
    1.0, 0.5, 0.0,
    0.0, 0.5, 1.0
  ]);
  var vertexColorBuffer = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, vertexColorBuffer);
  gl.bufferData(gl.ARRAY_BUFFER, colors, gl.STATIC_DRAW);
  gl.enableVertexAttribArray(vertexColorLocation);
  gl.vertexAttribPointer(vertexColorLocation, 3, gl.FLOAT, false, 0, 0);
  gl.vertexAttribDivisor(vertexColorLocation, 1); // attribute used once per instance

  gl.bindVertexArray(null);

  // -- Render
  gl.clearColor(0.0, 0.0, 0.0, 1.0);
  gl.clear(gl.COLOR_BUFFER_BIT);

  gl.bindVertexArray(vertexArray);

  gl.drawArraysInstanced(gl.TRIANGLES, 0, 3, 2);

  // -- Delete WebGL resources
  gl.deleteBuffer(vertexPosBuffer);
  gl.deleteBuffer(vertexColorBuffer);
  gl.deleteProgram(program);
  gl.deleteVertexArray(vertexArray);

}
