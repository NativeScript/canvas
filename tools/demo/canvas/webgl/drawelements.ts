export function drawElements(canvas, drawingBufferWidth?, drawingBufferHeight?, nativeCanvas?) {
  function main() {
    var gl = canvas.getContext ? canvas.getContext("webgl") : canvas;
    if (!gl) {
      return;
    }

    drawingBufferWidth = drawingBufferWidth || gl.drawingBufferWidth;
    drawingBufferHeight = drawingBufferHeight || gl.drawingBufferHeight;
    console.log(drawingBufferWidth, drawingBufferHeight)

    var vertexShaderSrc = `
  attribute vec2 a_position;

  uniform vec2 u_resolution;

  void main() {
     vec2 zeroToOne = a_position / u_resolution;

     vec2 zeroToTwo = zeroToOne * 2.0;

     vec2 clipSpace = zeroToTwo - 1.0;

     gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
    }
`;

    var fragmentShaderSrc = `
    precision mediump float;
    uniform vec4 u_color;
    void main() {
        gl_FragColor = u_color;
    }
     `;

    // setup GLSL program

    var vertexShader = gl.createShader(gl.VERTEX_SHADER);
    gl.shaderSource(vertexShader, vertexShaderSrc);
    gl.compileShader(vertexShader);

    let compiled = gl.getShaderParameter(vertexShader, gl.COMPILE_STATUS);
    if (!compiled) {
      // Something went wrong during compilation; get the error
      const lastError = gl.getShaderInfoLog(vertexShader);
      console.log(
        "*** Error compiling vertexShader '" + vertexShader + "':" + lastError
      );
      gl.deleteShader(vertexShader);
      return null;
    }

    var fragmentShader = gl.createShader(gl.FRAGMENT_SHADER);
    gl.shaderSource(fragmentShader, fragmentShaderSrc);
    gl.compileShader(fragmentShader);

    compiled = gl.getShaderParameter(fragmentShader, gl.COMPILE_STATUS);
    if (!compiled) {
      // Something went wrong during compilation; get the error
      const lastError = gl.getShaderInfoLog(fragmentShader);
      console.log(
        "*** Error compiling fragmentShader '" +
        fragmentShader +
        "':" +
        lastError
      );
      gl.deleteShader(fragmentShader);
      return null;
    }

    var program = gl.createProgram();

    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    gl.linkProgram(program);

    // Check the link status
    const linked = gl.getProgramParameter(program, gl.LINK_STATUS);
    if (!linked) {
      // something went wrong with the link
      const lastError = gl.getProgramInfoLog(program);
      console.log("Error in program linking:" + lastError);

      gl.deleteProgram(program);
      return null;
    }

    // look up where the vertex data needs to go.
    var positionAttributeLocation = gl.getAttribLocation(program, "a_position");

    // look up uniform locations
    var resolutionUniformLocation = gl.getUniformLocation(
      program,
      "u_resolution"
    );
    var colorUniformLocation = gl.getUniformLocation(program, "u_color");

    // Create a buffer to put three 2d clip space points in
    var positionBuffer = gl.createBuffer();

    // Bind it to ARRAY_BUFFER (think of it as ARRAY_BUFFER = positionBuffer)
    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);


    // Tell WebGL how to convert from clip space to pixels
    gl.viewport(0, 0, drawingBufferWidth, drawingBufferHeight);

    // Clear the canvas
    gl.clearColor(0, 0, 0, 0);
    gl.clear(gl.COLOR_BUFFER_BIT);

    // Tell it to use our program (pair of shaders)
    gl.useProgram(program);

    // Bind the position buffer.
    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

    // create the buffer
    const indexBuffer = gl.createBuffer();

    // make this buffer the current 'ELEMENT_ARRAY_BUFFER'
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer);

    // Fill the current element array buffer with data
    const indices = [
      0,
      1,
      2, // first triangle
      2,
      1,
      3, // second triangle
    ];
    gl.bufferData(
      gl.ELEMENT_ARRAY_BUFFER,
      new Uint16Array(indices),
      gl.STATIC_DRAW
    );

    // code above this line is initialization code
    // --------------------------------
    // code below this line is rendering code

    // Turn on the attribute
    gl.enableVertexAttribArray(positionAttributeLocation);

    // Tell the attribute how to get data out of positionBuffer (ARRAY_BUFFER)
    var size = 2; // 2 components per iteration
    var type = gl.FLOAT; // the data is 32bit floats
    var normalize = false; // don't normalize the data
    var stride = 0; // 0 = move forward size * sizeof(type) each iteration to get the next position
    var offset = 0; // start at the beginning of the buffer
    gl.vertexAttribPointer(
      positionAttributeLocation,
      size,
      type,
      normalize,
      stride,
      offset
    );

    // bind the buffer containing the indices
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer);

    // set the resolution
    gl.uniform2f(
      resolutionUniformLocation,
      drawingBufferWidth,
      drawingBufferHeight
    );

    // draw 50 random rectangles in random colors
    for (var ii = 0; ii < 50; ++ii) {
      // Setup a random rectangle
      // This will write to positionBuffer because
      // its the last thing we bound on the ARRAY_BUFFER
      // bind point
      setRectangle(
        gl,
        randomInt(300),
        randomInt(300),
        randomInt(300),
        randomInt(300)
      );

      // Set a random color.
      gl.uniform4f(
        colorUniformLocation,
        Math.random(),
        Math.random(),
        Math.random(),
        1
      );

      // Draw the rectangle.
      var primitiveType = gl.TRIANGLES;
      var offset = 0;
      var count = 6;
      var indexType = gl.UNSIGNED_SHORT;
      gl.drawElements(primitiveType, count, indexType, offset);
    }
  }

  // Returns a random integer from 0 to range - 1.
  function randomInt(range) {
    return Math.floor(Math.random() * range);
  }

  // Fill the buffer with the values that define a rectangle.
  function setRectangle(gl, x, y, width, height) {
    var x1 = x;
    var x2 = x + width;
    var y1 = y;
    var y2 = y + height;
    gl.bufferData(
      gl.ARRAY_BUFFER,
      new Float32Array([x1, y1, x2, y1, x1, y2, x2, y2]),
      gl.STATIC_DRAW
    );

  }

  main();
}

export function drawElementsBasic(canvas) {
  var gl = canvas.getContext("webgl");
  if (!gl) {
    return;
  }
  var vertices = [
    -0.5,
    -0.5,
    0.0,
    -0.25,
    0.5,
    0.0,
    0.0,
    -0.5,
    0.0,
    0.25,
    0.5,
    0.0,
    0.5,
    -0.5,
    0.0,
  ];

  var indices = [0, 1, 2, 2, 3, 4];

  gl.drawElements(gl.TRIANGLES, indices.length, gl.UNSIGNED_SHORT, 0);
}

export function scaleTriangle(canvas) {
  /*=================Creating a canvas=========================*/
  var gl = canvas.getContext("experimental-webgl");

  /*===========Defining and storing the geometry==============*/
  var vertices = [-0.5, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0];

  //Create an empty buffer object and store vertex data

  var vertex_buffer = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer);
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW);
  gl.bindBuffer(gl.ARRAY_BUFFER, null);

  /*========================Shaders============================*/

  //Vertex shader source code
  var vertCode =
    "attribute vec4 coordinates;" +
    "uniform mat4 u_xformMatrix;" +
    "void main() {" +
    "  gl_Position = u_xformMatrix * coordinates;" +
    "}";

  //Create a vertex shader program object and compile it
  var vertShader = gl.createShader(gl.VERTEX_SHADER);
  gl.shaderSource(vertShader, vertCode);
  gl.compileShader(vertShader);

  let compiled = gl.getShaderParameter(vertShader, gl.COMPILE_STATUS);

  if (!compiled) {
    // Something went wrong during compilation; get the error
    const lastError = gl.getShaderInfoLog(vertShader);
    console.log(
      "*** Error compiling vertexShader '" + vertShader + "':" + lastError
    );
    gl.deleteShader(vertShader);
    return null;
  }

  //fragment shader source code
  var fragCode =
    "void main() {" + "   gl_FragColor = vec4(0.0, 0.0, 0.0, 0.1);" + "}";

  //Create a fragment shader program object and compile it
  var fragShader = gl.createShader(gl.FRAGMENT_SHADER);
  gl.shaderSource(fragShader, fragCode);
  gl.compileShader(fragShader);

  compiled = gl.getShaderParameter(fragShader, gl.COMPILE_STATUS);
  if (!compiled) {
    // Something went wrong during compilation; get the error
    const lastError = gl.getShaderInfoLog(fragShader);
    console.log(
      "*** Error compiling vertexShader '" + fragShader + "':" + lastError
    );
    gl.deleteShader(fragShader);
    return null;
  }

  //Create and use combiened shader program
  var shaderProgram = gl.createProgram();
  gl.attachShader(shaderProgram, vertShader);
  gl.attachShader(shaderProgram, fragShader);
  gl.linkProgram(shaderProgram);

  gl.useProgram(shaderProgram);

  const linked = gl.getProgramParameter(shaderProgram, gl.LINK_STATUS);
  if (!linked) {
    // Something went wrong during compilation; get the error
    const lastError = gl.getProgramInfoLog(shaderProgram);
    console.log(
      "*** Error linking shaderProgram '" + shaderProgram + "':" + lastError
    );

    return null;
  }

  /*===================scaling==========================*/

  var Sx = 1.0,
    Sy = 1.5,
    Sz = 1.0;
  var xformMatrix = new Float32Array([
    Sx,
    0.0,
    0.0,
    0.0,
    0.0,
    Sy,
    0.0,
    0.0,
    0.0,
    0.0,
    Sz,
    0.0,
    0.0,
    0.0,
    0.0,
    1.0,
  ]);

  var u_xformMatrix = gl.getUniformLocation(shaderProgram, "u_xformMatrix");
  gl.uniformMatrix4fv(u_xformMatrix, false, xformMatrix);

  /* ===========Associating shaders to buffer objects============*/
  gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer);

  var coordinatesVar = gl.getAttribLocation(shaderProgram, "coordinates");
  gl.vertexAttribPointer(coordinatesVar, 3, gl.FLOAT, false, 0, 0);
  gl.enableVertexAttribArray(coordinatesVar);

  /*=================Drawing the Quad========================*/
  gl.clearColor(0.5, 0.5, 0.5, 0.2);
  //gl.enable(gl.DEPTH_TEST);

  gl.clear(gl.COLOR_BUFFER_BIT);
  gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);
  gl.drawArrays(gl.TRIANGLES, 0, 3);
}

export function triangle(canvas) {
  /*============== Creating a canvas ====================*/
  var gl = canvas.getContext("experimental-webgl");

  /*======== Defining and storing the geometry ===========*/

  var vertices = [-0.5, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0];

  var indices = [0, 1, 2];

  // Create an empty buffer object to store vertex buffer
  var vertex_buffer = gl.createBuffer();

  // Bind appropriate array buffer to it
  gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer);

  // Pass the vertex data to the buffer
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW);

  // Unbind the buffer
  gl.bindBuffer(gl.ARRAY_BUFFER, null);

  // Create an empty buffer object to store Index buffer
  var Index_Buffer = gl.createBuffer();

  // Bind appropriate array buffer to it
  gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, Index_Buffer);

  // Pass the vertex data to the buffer
  gl.bufferData(
    gl.ELEMENT_ARRAY_BUFFER,
    new Uint16Array(indices),
    gl.STATIC_DRAW
  );

  // Unbind the buffer
  gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null);

  /*================ Shaders ====================*/

  // Vertex shader source code
  var vertCode =
    "attribute vec3 coordinates;" +
    "void main(void) {" +
    " gl_Position = vec4(coordinates, 1.0);" +
    "}";

  // Create a vertex shader object
  var vertShader = gl.createShader(gl.VERTEX_SHADER);

  // Attach vertex shader source code
  gl.shaderSource(vertShader, vertCode);

  // Compile the vertex shader
  gl.compileShader(vertShader);

  //fragment shader source code
  var fragCode =
    "void main(void) {" + " gl_FragColor = vec4(0.0, 0.0, 0.0, 0.1);" + "}";

  // Create fragment shader object
  var fragShader = gl.createShader(gl.FRAGMENT_SHADER);

  // Attach fragment shader source code
  gl.shaderSource(fragShader, fragCode);

  // Compile the fragmentt shader
  gl.compileShader(fragShader);

  // Create a shader program object to store
  // the combined shader program
  var shaderProgram = gl.createProgram();

  // Attach a vertex shader
  gl.attachShader(shaderProgram, vertShader);

  // Attach a fragment shader
  gl.attachShader(shaderProgram, fragShader);

  // Link both the programs
  gl.linkProgram(shaderProgram);

  // Use the combined shader program object
  gl.useProgram(shaderProgram);

  /*======= Associating shaders to buffer objects =======*/

  // Bind vertex buffer object
  gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer);

  // Bind index buffer object
  gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, Index_Buffer);

  // Get the attribute location
  var coord = gl.getAttribLocation(shaderProgram, "coordinates");

  // Point an attribute to the currently bound VBO
  gl.vertexAttribPointer(coord, 3, gl.FLOAT, false, 0, 0);

  // Enable the attribute
  gl.enableVertexAttribArray(coord);

  /*=========Drawing the triangle===========*/

  // Clear the canvas
  gl.clearColor(0.5, 0.5, 0.5, 0.9);

  // Enable the depth test
  //gl.enable(gl.DEPTH_TEST);

  // Clear the color buffer bit
  gl.clear(gl.COLOR_BUFFER_BIT);

  // Set the view port
  gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);

  // Draw the triangle
  gl.drawElements(gl.TRIANGLES, indices.length, gl.UNSIGNED_SHORT, 0);

}

export function points(canvas) {
  /*================Creating a canvas=================*/

  var gl = canvas.getContext("experimental-webgl");

  /*==========Defining and storing the geometry=======*/

  var vertices = [-0.5, 0.5, 0.0, 0.0, 0.5, 0.0, -0.25, 0.25, 0.0];

  // Create an empty buffer object to store the vertex buffer
  var vertex_buffer = gl.createBuffer();

  //Bind appropriate array buffer to it
  gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer);

  // Pass the vertex data to the buffer
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW);

  // Unbind the buffer
  gl.bindBuffer(gl.ARRAY_BUFFER, null);

  /*=========================Shaders========================*/

  // vertex shader source code
  var vertCode =
    "attribute vec3 coordinates;" +
    "void main(void) {" +
    " gl_Position = vec4(coordinates, 1.0);" +
    "gl_PointSize = 10.0;" +
    "}";

  // Create a vertex shader object
  var vertShader = gl.createShader(gl.VERTEX_SHADER);

  // Attach vertex shader source code
  gl.shaderSource(vertShader, vertCode);

  // Compile the vertex shader
  gl.compileShader(vertShader);

  // fragment shader source code
  var fragCode =
    "void main(void) {" + " gl_FragColor = vec4(0.0, 0.0, 0.0, 0.1);" + "}";

  // Create fragment shader object
  var fragShader = gl.createShader(gl.FRAGMENT_SHADER);

  // Attach fragment shader source code
  gl.shaderSource(fragShader, fragCode);

  // Compile the fragmentt shader
  gl.compileShader(fragShader);

  // Create a shader program object to store
  // the combined shader program
  var shaderProgram = gl.createProgram();

  // Attach a vertex shader
  gl.attachShader(shaderProgram, vertShader);

  // Attach a fragment shader
  gl.attachShader(shaderProgram, fragShader);

  // Link both programs
  gl.linkProgram(shaderProgram);

  // Use the combined shader program object
  gl.useProgram(shaderProgram);

  /*======== Associating shaders to buffer objects ========*/

  // Bind vertex buffer object
  gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer);

  // Get the attribute location
  var coord = gl.getAttribLocation(shaderProgram, "coordinates");

  // Point an attribute to the currently bound VBO
  gl.vertexAttribPointer(coord, 3, gl.FLOAT, false, 0, 0);

  // Enable the attribute
  gl.enableVertexAttribArray(coord);

  /*============= Drawing the primitive ===============*/

  // Clear the canvas
  gl.clearColor(0.5, 0.5, 0.5, 0.9);

  // Enable the depth test
  //gl.enable(gl.DEPTH_TEST);

  // Clear the color buffer bit
  gl.clear(gl.COLOR_BUFFER_BIT);

  // Set the view port
  gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);

  // Draw the triangle
  gl.drawArrays(gl.POINTS, 0, 3);
}


export function drawModes(canvas, mode: 'line' | 'points' | 'line_strip' | 'triangle_strip' | 'triangle_fan' | 'triangles' | 'line_loop' = "line") {
  /*======= Creating a canvas =========*/

  var gl = canvas.getContext("experimental-webgl") as WebGLRenderingContext;

  /*======= Defining and storing the geometry ======*/

  var vertices = [
    -0.7,
    -0.1,
    0,
    -0.3,
    0.6,
    0,
    -0.3,
    -0.3,
    0,
    0.2,
    0.6,
    0,
    0.3,
    -0.3,
    0,
    0.7,
    0.6,
    0,
  ];

  // Create an empty buffer object
  var vertex_buffer = gl.createBuffer();

  // Bind appropriate array buffer to it
  gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer);

  // Pass the vertex data to the buffer
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW);

  // Unbind the buffer
  gl.bindBuffer(gl.ARRAY_BUFFER, null);

  /*=================== Shaders ====================*/

  // Vertex shader source code
  var vertCode =
    "attribute vec3 coordinates;" +
    "void main(void) {" +
    " gl_Position = vec4(coordinates, 1.0);" +
    "}";

  // Create a vertex shader object
  var vertShader = gl.createShader(gl.VERTEX_SHADER);

  // Attach vertex shader source code
  gl.shaderSource(vertShader, vertCode);

  // Compile the vertex shader
  gl.compileShader(vertShader);

  // Fragment shader source code
  var fragCode =
    "void main(void) {" + "gl_FragColor = vec4(0.0, 0.0, 0.0, 0.1);" + "}";

  // Create fragment shader object
  var fragShader = gl.createShader(gl.FRAGMENT_SHADER);

  // Attach fragment shader source code
  gl.shaderSource(fragShader, fragCode);

  // Compile the fragmentt shader
  gl.compileShader(fragShader);

  // Create a shader program object to store
  // the combined shader program
  var shaderProgram = gl.createProgram();

  // Attach a vertex shader
  gl.attachShader(shaderProgram, vertShader);

  // Attach a fragment shader
  gl.attachShader(shaderProgram, fragShader);

  // Link both the programs
  gl.linkProgram(shaderProgram);

  // Use the combined shader program object
  gl.useProgram(shaderProgram);

  /*======= Associating shaders to buffer objects ======*/

  // Bind vertex buffer object
  gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer);

  // Get the attribute location
  var coord = gl.getAttribLocation(shaderProgram, "coordinates");


  // Point an attribute to the currently bound VBO
  gl.vertexAttribPointer(coord, 3, gl.FLOAT, false, 0, 0);

  // Enable the attribute
  gl.enableVertexAttribArray(coord);

  /*============ Drawing the triangle =============*/

  // Clear the canvas
  gl.clearColor(0.5, 0.5, 0.5, 0.9);

  // Enable the depth test
  gl.enable(gl.DEPTH_TEST);

  // Clear the color and depth buffer
  gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

  // Set the view port
  gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);
  // Draw the triangle
  var m = gl.LINES;
  switch (mode) {
    case "points":
      m = gl.POINTS;
      break;
    case "line_strip":
      m = gl.LINE_STRIP;
      break;
    case "line_loop":
      m = gl.LINE_LOOP;
      break;
    case "triangle_strip":
      m = gl.TRIANGLE_STRIP;
      break;
    case "triangle_fan":
      m = gl.TRIANGLE_FAN;
      break;
    case "triangles":
      m = gl.TRIANGLES;
      break;
    default:
      m = gl.LINES;
      break;
  }
  gl.drawArrays(m, 0, 6);
  // POINTS, LINE_STRIP, LINE_LOOP, LINES,
  // TRIANGLE_STRIP,TRIANGLE_FAN, TRIANGLES
}
