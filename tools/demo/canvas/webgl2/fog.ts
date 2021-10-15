import {createProgramFromScripts} from "../webgl/utils";
import {ImageSource} from '@nativescript/core';
import * as m4 from "../webgl/m4";

let LAF = 0;

export function fog(canvas, drawingBufferWidth?, drawingBufferHeight?, nativeCanvas?) {

  var vertexShaderSource = `#version 300 es
in vec4 a_position;
in vec2 a_texcoord;

uniform mat4 u_worldView;
uniform mat4 u_projection;

out vec2 v_texcoord;

void main() {
  // Multiply the position by the matrix.
  gl_Position = u_projection * u_worldView * a_position;

  // Pass the texcoord to the fragment shader.
  v_texcoord = a_texcoord;
}
`;

  var fragmentShaderSource = `#version 300 es
precision highp float;

// Passed in from the vertex shader.
in vec2 v_texcoord;

// The texture.
uniform sampler2D u_texture;
uniform vec4 u_fogColor;
uniform float u_fogNear;
uniform float u_fogFar;

out vec4 outColor;

void main() {
  vec4 color = texture(u_texture, v_texcoord);

  float fogAmount = smoothstep(u_fogNear, u_fogFar, gl_FragCoord.z);

  outColor = mix(color, u_fogColor, fogAmount);
}
`;

  function main() {
    var gl = canvas.getContext ? canvas.getContext("webgl2") : canvas;
    if (!gl) {
      return;
    }

    // Use our boilerplate utils to compile the shaders and link into a program
    var program = createProgramFromScripts(gl,
      [{type: 'vertex', src: vertexShaderSource}, {type: 'fragment', src: fragmentShaderSource}]);

    // look up where the vertex data needs to go.
    var positionAttributeLocation = gl.getAttribLocation(program, "a_position");
    var texcoordAttributeLocation = gl.getAttribLocation(program, "a_texcoord");

    // lookup uniforms
    var projectionLocation = gl.getUniformLocation(program, "u_projection");
    var worldViewLocation = gl.getUniformLocation(program, "u_worldView");
    var textureLocation = gl.getUniformLocation(program, "u_texture");
    var fogColorLocation = gl.getUniformLocation(program, "u_fogColor");
    var fogNearLocation = gl.getUniformLocation(program, "u_fogNear");
    var fogFarLocation = gl.getUniformLocation(program, "u_fogFar");

    // Create a vertex array object (attribute state)
    var vao = gl.createVertexArray();

    // and make it the one we're currently working with
    gl.bindVertexArray(vao);

    // Create a buffer for positions
    var positionBuffer = gl.createBuffer();
    // Bind it to ARRAY_BUFFER (think of it as ARRAY_BUFFER = positionBuffer)
    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
    // Put the positions in the buffer
    setGeometry(gl);

    // Turn on the attribute
    gl.enableVertexAttribArray(positionAttributeLocation);

    // Tell the attribute how to get data out of positionBuffer (ARRAY_BUFFER)
    var size = 3;          // 3 components per iteration
    var type = gl.FLOAT;   // the data is 32bit floats
    var normalize = false; // don't normalize the data
    var stride = 0;        // 0 = move forward size * sizeof(type) each iteration to get the next position
    var offset = 0;        // start at the beginning of the buffer
    gl.vertexAttribPointer(
      positionAttributeLocation, size, type, normalize, stride, offset);

    // provide texture coordinates for the rectangle.
    var texcoordBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, texcoordBuffer);
    // Set Texcoords.
    setTexcoords(gl);

    // Turn on the attribute
    gl.enableVertexAttribArray(texcoordAttributeLocation);

    // Tell the attribute how to get data out of colorBuffer (ARRAY_BUFFER)
    var size = 2;          // 2 components per iteration
    var type = gl.FLOAT;   // the data is 32bit floating point values
    var normalize = true;  // convert from 0-255 to 0.0-1.0
    var stride = 0;        // 0 = move forward size * sizeof(type) each iteration to get the next color
    var offset = 0;        // start at the beginning of the buffer
    gl.vertexAttribPointer(
      texcoordAttributeLocation, size, type, normalize, stride, offset);

    // Create a texture.
    var texture = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, texture);
    // Fill the texture with a 1x1 blue pixel.
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 1, 1, 0, gl.RGBA, gl.UNSIGNED_BYTE,
      new Uint8Array([0, 0, 255, 255]));
    // Asynchronously load an image

    ImageSource.fromUrl("https://webgl2fundamentals.org/webgl/resources/f-texture.png")
      .then(image => {
        // Now that the image has loaded make copy it to the texture.
        gl.bindTexture(gl.TEXTURE_2D, texture);
        gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, image);
        gl.generateMipmap(gl.TEXTURE_2D);
      }).catch(e => {
      console.log('image failed: ', e);
    });

    function radToDeg(r) {
      return r * 180 / Math.PI;
    }

    function degToRad(d) {
      return d * Math.PI / 180;
    }

    var fieldOfViewRadians = degToRad(60);
    var modelXRotationRadians = degToRad(0);
    var modelYRotationRadians = degToRad(0);
    var fogColor = [0.8, 0.9, 1, 1];
    var settings = {
      fogNear: 0.7,
      fogFar: 0.9,
      xOff: 1.1,
      zOff: 1.4,
    };

    /*
    webglLessonsUI.setupUI(document.querySelector("#ui"), settings, [
      { type: "slider",   key: "fogNear",  min: 0, max: 1, precision: 3, step: 0.001, },
      { type: "slider",   key: "fogFar",   min: 0, max: 1, precision: 3, step: 0.001, },
    ]);
    */

    // Get the starting time.
    var then = 0;

    requestAnimationFrame(drawScene);

    // Draw the scene.
    function drawScene(time) {
      // convert to seconds
      time *= 0.001;
      // Subtract the previous time from the current time
      var deltaTime = time - then;
      // Remember the current time for the next frame.
      then = time;

      // webglUtils.resizeCanvasToDisplaySize(gl.canvas);

      // Tell WebGL how to convert from clip space to pixels
      gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);

      gl.enable(gl.CULL_FACE);
      gl.enable(gl.DEPTH_TEST);

      // Animate the rotation
      modelYRotationRadians += -0.7 * deltaTime;
      modelXRotationRadians += -0.4 * deltaTime;

      // Clear the canvas AND the depth buffer.
      gl.clearColor(...fogColor);
      gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

      // Tell it to use our program (pair of shaders)
      gl.useProgram(program);

      // Bind the attribute/buffer set we want.
      gl.bindVertexArray(vao);

      // Compute the projection matrix
      var aspect = gl.drawingBufferWidth / gl.drawingBufferHeight;
      var projectionMatrix =
        m4.perspective(fieldOfViewRadians, aspect, 1, 40);

      var cameraPosition = [0, 0, 2];
      var up = [0, 1, 0];
      var target = [0, 0, 0];

      // Compute the camera's matrix using look at.
      var cameraMatrix = m4.lookAt(cameraPosition, target, up);

      // Make a view matrix from the camera matrix.
      var viewMatrix = m4.inverse(cameraMatrix);

      // Set the projection matrix.
      gl.uniformMatrix4fv(projectionLocation, false, projectionMatrix);

      // Tell the shader to use texture unit 0 for u_texture
      gl.uniform1i(textureLocation, 0);

      // set the fog color and near, far settings
      gl.uniform4fv(fogColorLocation, fogColor);
      gl.uniform1f(fogNearLocation, settings.fogNear);
      gl.uniform1f(fogFarLocation, settings.fogFar);

      const numCubes = 40;
      for (let i = 0; i <= numCubes; ++i) {
        var worldViewMatrix = m4.translate(viewMatrix, -2 + i * settings.xOff, 0, -i * settings.zOff);
        worldViewMatrix = m4.xRotate(worldViewMatrix, modelXRotationRadians + i * 0.1);
        worldViewMatrix = m4.yRotate(worldViewMatrix, modelYRotationRadians + i * 0.1);

        gl.uniformMatrix4fv(worldViewLocation, false, worldViewMatrix);

        // Draw the geometry.
        gl.drawArrays(gl.TRIANGLES, 0, 6 * 6);
      }
      if (!nativeCanvas) {

      } else {}
      LAF = requestAnimationFrame(drawScene);
    }
  }

// Fill the buffer with the values that define a cube.
  function setGeometry(gl) {
    var positions = new Float32Array([
      -0.5, -0.5, -0.5,
      -0.5, 0.5, -0.5,
      0.5, -0.5, -0.5,
      -0.5, 0.5, -0.5,
      0.5, 0.5, -0.5,
      0.5, -0.5, -0.5,

      -0.5, -0.5, 0.5,
      0.5, -0.5, 0.5,
      -0.5, 0.5, 0.5,
      -0.5, 0.5, 0.5,
      0.5, -0.5, 0.5,
      0.5, 0.5, 0.5,

      -0.5, 0.5, -0.5,
      -0.5, 0.5, 0.5,
      0.5, 0.5, -0.5,
      -0.5, 0.5, 0.5,
      0.5, 0.5, 0.5,
      0.5, 0.5, -0.5,

      -0.5, -0.5, -0.5,
      0.5, -0.5, -0.5,
      -0.5, -0.5, 0.5,
      -0.5, -0.5, 0.5,
      0.5, -0.5, -0.5,
      0.5, -0.5, 0.5,

      -0.5, -0.5, -0.5,
      -0.5, -0.5, 0.5,
      -0.5, 0.5, -0.5,
      -0.5, -0.5, 0.5,
      -0.5, 0.5, 0.5,
      -0.5, 0.5, -0.5,

      0.5, -0.5, -0.5,
      0.5, 0.5, -0.5,
      0.5, -0.5, 0.5,
      0.5, -0.5, 0.5,
      0.5, 0.5, -0.5,
      0.5, 0.5, 0.5,

    ]);
    gl.bufferData(gl.ARRAY_BUFFER, positions, gl.STATIC_DRAW);
  }

// Fill the buffer with texture coordinates the cube.
  function setTexcoords(gl) {
    gl.bufferData(
      gl.ARRAY_BUFFER,
      new Float32Array([
        0, 0,
        0, 1,
        1, 0,
        0, 1,
        1, 1,
        1, 0,

        0, 0,
        1, 0,
        0, 1,
        0, 1,
        1, 0,
        1, 1,

        0, 0,
        0, 1,
        1, 0,
        0, 1,
        1, 1,
        1, 0,

        0, 0,
        1, 0,
        0, 1,
        0, 1,
        1, 0,
        1, 1,

        0, 0,
        0, 1,
        1, 0,
        0, 1,
        1, 1,
        1, 0,

        0, 0,
        1, 0,
        0, 1,
        0, 1,
        1, 0,
        1, 1,
      ]),
      gl.STATIC_DRAW);
  }

  main();
}

export function cancelFog() {
  cancelAnimationFrame(LAF);
  LAF = 0;
}
