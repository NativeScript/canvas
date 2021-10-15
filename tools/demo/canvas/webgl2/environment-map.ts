import {createProgramFromScripts} from "../webgl/utils";
import {ImageAsset} from '@nativescript/canvas';
import * as m4 from "../webgl/m4";
import {ImageSource} from "@nativescript/core";

let LAF = 0;

export function environmentMap(canvas, drawingBufferWidth?, drawingBufferHeight?, nativeCanvas?) {
  var vertexShaderSource = `#version 300 es
in vec4 a_position;
in vec3 a_normal;

uniform mat4 u_projection;
uniform mat4 u_view;
uniform mat4 u_world;

out vec3 v_worldPosition;
out vec3 v_worldNormal;

void main() {
  gl_Position = u_projection * u_view * u_world * a_position;
  v_worldPosition = (u_world * a_position).xyz;
  v_worldNormal = mat3(u_world) * a_normal;
}
`;

  var fragmentShaderSource =
    `#version 300 es
precision highp float;
in vec3 v_worldPosition;
in vec3 v_worldNormal;

uniform samplerCube u_texture;


uniform vec3 u_worldCameraPosition;

out vec4 outColor;

void main() {
  vec3 worldNormal = normalize(v_worldNormal);
  vec3 eyeToSurfaceDir = normalize(v_worldPosition - u_worldCameraPosition);
  vec3 direction = reflect(eyeToSurfaceDir,worldNormal);
  outColor = texture(u_texture, direction);
}
`;

  function main() {
    var gl = canvas.getContext ? canvas.getContext("webgl2") : canvas;
    if (!gl) {
      return;
    }


    drawingBufferWidth = drawingBufferWidth || gl.drawingBufferWidth;
    drawingBufferHeight = drawingBufferHeight || gl.drawingBufferHeight;
    // Use our boilerplate utils to compile the shaders and link into a program
    var program = createProgramFromScripts(gl,
      [{src: vertexShaderSource, type: 'vertex'}, {src: fragmentShaderSource, type: 'fragment'}]);

    // look up where the vertex data needs to go.
    var positionLocation = gl.getAttribLocation(program, "a_position");
    var normalLocation = gl.getAttribLocation(program, "a_normal");

    // lookup uniforms
    var projectionLocation = gl.getUniformLocation(program, "u_projection");
    var viewLocation = gl.getUniformLocation(program, "u_view");
    var worldLocation = gl.getUniformLocation(program, "u_world");
    var textureLocation = gl.getUniformLocation(program, "u_texture");
    var worldCameraPositionLocation = gl.getUniformLocation(program, "u_worldCameraPosition");

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

    // Turn on the position attribute
    gl.enableVertexAttribArray(positionLocation);

    // Tell the position attribute how to get data out of positionBuffer (ARRAY_BUFFER)
    var size = 3;          // 3 components per iteration
    var type = gl.FLOAT;   // the data is 32bit floats
    var normalize = false; // don't normalize the data
    var stride = 0;        // 0 = move forward size * sizeof(type) each iteration to get the next position
    var offset = 0;        // start at the beginning of the buffer
    gl.vertexAttribPointer(
      positionLocation, size, type, normalize, stride, offset);


    // Create a buffer to put normals in
    var normalBuffer = gl.createBuffer();
    // Bind it to ARRAY_BUFFER (think of it as ARRAY_BUFFER = normalBuffer)
    gl.bindBuffer(gl.ARRAY_BUFFER, normalBuffer);
    // Put normals data into buffer
    setNormals(gl);

    // Turn on the normal attribute
    gl.enableVertexAttribArray(normalLocation);

    // Tell the attribute how to get data out of normalBuffer (ARRAY_BUFFER)
    var size = 3;          // 3 components per iteration
    var type = gl.FLOAT;   // the data is 32bit floating point values
    var normalize = false; // normalize the data (convert from 0-255 to 0-1)
    var stride = 0;        // 0 = move forward size * sizeof(type) each iteration to get the next position
    var offset = 0;        // start at the beginning of the buffer
    gl.vertexAttribPointer(
      normalLocation, size, type, normalize, stride, offset);

    // Create a texture.
    var texture = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_CUBE_MAP, texture);

    // const faceInfos = [
    //   {
    //     target: gl.TEXTURE_CUBE_MAP_POSITIVE_X,
    //     url: 'https://webgl2fundamentals.org/webgl/resources/images/computer-history-museum/pos-x.jpg',
    //   },
    //   {
    //     target: gl.TEXTURE_CUBE_MAP_NEGATIVE_X,
    //     url: 'https://webgl2fundamentals.org/webgl/resources/images/computer-history-museum/neg-x.jpg',
    //   },
    //   {
    //     target: gl.TEXTURE_CUBE_MAP_POSITIVE_Y,
    //     url: 'https://webgl2fundamentals.org/webgl/resources/images/computer-history-museum/pos-y.jpg',
    //   },
    //   {
    //     target: gl.TEXTURE_CUBE_MAP_NEGATIVE_Y,
    //     url: 'https://webgl2fundamentals.org/webgl/resources/images/computer-history-museum/neg-y.jpg',
    //   },
    //   {
    //     target: gl.TEXTURE_CUBE_MAP_POSITIVE_Z,
    //     url: 'https://webgl2fundamentals.org/webgl/resources/images/computer-history-museum/pos-z.jpg',
    //   },
    //   {
    //     target: gl.TEXTURE_CUBE_MAP_NEGATIVE_Z,
    //     url: 'https://webgl2fundamentals.org/webgl/resources/images/computer-history-museum/neg-z.jpg',
    //   },
    // ];

    const faceInfos = [
      {
        target: gl.TEXTURE_CUBE_MAP_POSITIVE_X,
        url: '~/assets/file-assets/webgl2/images/computer-history-museum/pos-x.jpg',
      },
      {
        target: gl.TEXTURE_CUBE_MAP_NEGATIVE_X,
        url: '~/assets/file-assets/webgl2/images/computer-history-museum/neg-x.jpg',
      },
      {
        target: gl.TEXTURE_CUBE_MAP_POSITIVE_Y,
        url: '~/assets/file-assets/webgl2/images/computer-history-museum/pos-y.jpg',
      },
      {
        target: gl.TEXTURE_CUBE_MAP_NEGATIVE_Y,
        url: '~/assets/file-assets/webgl2/images/computer-history-museum/neg-y.jpg',
      },
      {
        target: gl.TEXTURE_CUBE_MAP_POSITIVE_Z,
        url: '~/assets/file-assets/webgl2/images/computer-history-museum/pos-z.jpg',
      },
      {
        target: gl.TEXTURE_CUBE_MAP_NEGATIVE_Z,
        url: '~/assets/file-assets/webgl2/images/computer-history-museum/neg-z.jpg',
      },
    ];


    faceInfos.forEach((faceInfo) => {
      const {target, url} = faceInfo;

      // Upload the canvas to the cubemap face.
      const level = 0;
      const internalFormat = gl.RGBA;
      const width = 512;
      const height = 512;
      const format = gl.RGBA;
      const type = gl.UNSIGNED_BYTE;

      // setup each face so it's immediately renderable
      gl.texImage2D(target, level, internalFormat, width, height, 0, format, type, null);

      // Asynchronously load an image
      const asset = new ImageAsset();
      asset.loadFileAsync(url).then(() => {
        gl.bindTexture(gl.TEXTURE_CUBE_MAP, texture);
        gl.texImage2D(target, level, internalFormat, format, type, asset);
        gl.generateMipmap(gl.TEXTURE_CUBE_MAP);
      }).catch(e => {
        console.log('Image failed: ', e)
      });

      /*ImageSource.fromFile(url)
        .then((image) => {
          gl.bindTexture(gl.TEXTURE_CUBE_MAP, texture);
          gl.texImage2D(target, level, internalFormat, format, type, image);
          gl.generateMipmap(gl.TEXTURE_CUBE_MAP);
        }).catch(e => {
        console.log('Image failed: ', e)
      });*/
    });
    gl.generateMipmap(gl.TEXTURE_CUBE_MAP);
    gl.texParameteri(gl.TEXTURE_CUBE_MAP, gl.TEXTURE_MIN_FILTER, gl.LINEAR_MIPMAP_LINEAR);

    function degToRad(d) {
      return d * Math.PI / 180;
    }

    var fieldOfViewRadians = degToRad(60);
    var modelXRotationRadians = degToRad(0);
    var modelYRotationRadians = degToRad(0);

    // Get the starting time.
    var then = 0;
    if (!nativeCanvas) {

    } else {}
    requestAnimationFrame(drawScene);

    // Draw the scene.
    function drawScene(time) {
      // convert to seconds
      time *= 0.001;
      // Subtract the previous time from the current time
      var deltaTime = time - then;
      // Remember the current time for the next frame.
      then = time;

      //webglUtils.resizeCanvasToDisplaySize(gl.canvas);

      // Tell WebGL how to convert from clip space to pixels

      gl.viewport(0, 0, drawingBufferWidth, drawingBufferHeight);

      gl.enable(gl.CULL_FACE);
      gl.enable(gl.DEPTH_TEST);

      // Animate the rotation
      modelYRotationRadians += -0.7 * deltaTime;
      modelXRotationRadians += -0.4 * deltaTime;

      // Clear the canvas AND the depth buffer.
      gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

      // Tell it to use our program (pair of shaders)
      gl.useProgram(program);

      // Bind the attribute/buffer set we want.
      gl.bindVertexArray(vao);

      // Compute the projection matrix
      var aspect = drawingBufferWidth / drawingBufferHeight;
      var projectionMatrix =
        m4.perspective(fieldOfViewRadians, aspect, 1, 2000);
      gl.uniformMatrix4fv(projectionLocation, false, projectionMatrix);

      var cameraPosition = [0, 0, 2];
      var target = [0, 0, 0];
      var up = [0, 1, 0];
      // Compute the camera's matrix using look at.
      var cameraMatrix = m4.lookAt(cameraPosition, target, up);

      // Make a view matrix from the camera matrix.
      var viewMatrix = m4.inverse(cameraMatrix);

      var worldMatrix = m4.xRotation(modelXRotationRadians);
      worldMatrix = m4.yRotate(worldMatrix, modelYRotationRadians);

      // Set the uniforms
      gl.uniformMatrix4fv(projectionLocation, false, projectionMatrix);
      gl.uniformMatrix4fv(viewLocation, false, viewMatrix);
      gl.uniformMatrix4fv(worldLocation, false, worldMatrix);
      gl.uniform3fv(worldCameraPositionLocation, cameraPosition);

      // Tell the shader to use texture unit 0 for u_texture
      gl.uniform1i(textureLocation, 0);

      // Draw the geometry.
      gl.drawArrays(gl.TRIANGLES, 0, 6 * 6);

      if (!nativeCanvas) {

      } else {}
      LAF = requestAnimationFrame(drawScene);
    }
  }

// Fill the buffer with the values that define a cube.
  function setGeometry(gl) {
    var positions = new Float32Array(
      [
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

// Fill the buffer with normals for cube
  function setNormals(gl) {
    var normals = new Float32Array(
      [
        0, 0, -1,
        0, 0, -1,
        0, 0, -1,
        0, 0, -1,
        0, 0, -1,
        0, 0, -1,

        0, 0, 1,
        0, 0, 1,
        0, 0, 1,
        0, 0, 1,
        0, 0, 1,
        0, 0, 1,

        0, 1, 0,
        0, 1, 0,
        0, 1, 0,
        0, 1, 0,
        0, 1, 0,
        0, 1, 0,

        0, -1, 0,
        0, -1, 0,
        0, -1, 0,
        0, -1, 0,
        0, -1, 0,
        0, -1, 0,

        -1, 0, 0,
        -1, 0, 0,
        -1, 0, 0,
        -1, 0, 0,
        -1, 0, 0,
        -1, 0, 0,

        1, 0, 0,
        1, 0, 0,
        1, 0, 0,
        1, 0, 0,
        1, 0, 0,
        1, 0, 0,
      ]);
    gl.bufferData(gl.ARRAY_BUFFER, normals, gl.STATIC_DRAW);
  }

  main();
}

export function cancelEnvironmentMap() {
  cancelAnimationFrame(LAF);
  LAF = 0;
}
