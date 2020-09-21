import UIKit
import CanvasNative
@available(iOS 13.0, *)
class ViewController: UIViewController {
    var canvas1: TNSCanvas?
    var canvas2: TNSCanvas?
    var imageView: UIImageView?
    let PI: Float = .pi
    let TWO_PI: Float = .pi * 2
    @IBOutlet weak var first: UIView!
    @IBOutlet weak var second: UIView!
    @IBOutlet weak var third: UIView!
    var tapped = 0
    

    override func viewDidLoad() {
        super.viewDidLoad()
        scale = Int(UIScreen.main.scale)
        // Do any additional setup after loading the view.
        canvas1 = TNSCanvas(frame: first.bounds, useGL: true)
        canvas1!.backgroundColor = .green
        first.addSubview(canvas1!)
        //let matrix = Canvas.createSVGMatrix()
        //matrix.a = 3.0
    }
    
    var drawn = false
    var count = 0
    
    var vertexShaderSource = """
        void main() {
            gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
            gl_PointSize = 256.0;
        }
    """
    var fragmentShaderSource = """
        precision mediump float;
            void main() {
                vec2 fragmentPosition = 2.0*gl_PointCoord - 1.0;
                float distance = length(fragmentPosition);
                float distanceSqrd = distance * distance;
                gl_FragColor = vec4(
                    0.2/distanceSqrd,
                    0.1/distanceSqrd,
                    0.0, 1.0 );
                }
    """
    
    
    
    var vertCode = """
    attribute vec3 position;
    uniform mat4 Pmatrix;
    uniform mat4 Vmatrix;
    uniform mat4 Mmatrix;
    attribute vec3 color;
    varying vec3 vColor;
    void main() {
    gl_Position = Pmatrix*Vmatrix*Mmatrix*vec4(position, 1.0);
    vColor = color;
    }
    """
    
    var fragCode = """
    precision mediump float;
    varying vec3 vColor;
    void main() {
    gl_FragColor = vec4(vColor, 1.0);
    }
    """
    var buffer: UInt32?
    var gl: TNSWebGLRenderingContext?
    var program : UInt32?
    func initializeAttributes(gl: TNSWebGLRenderingContext?) {
        gl!.enableVertexAttribArray(0)
        buffer = gl!.createBuffer()
        gl!.bindBuffer(gl!.ARRAY_BUFFER, buffer!)
        print(CACurrentMediaTime())
        let data:[Float32] = [0.0, 0.0]
        gl!.bufferData(gl!.ARRAY_BUFFER,f32: data, gl!.STATIC_DRAW)
        gl!.vertexAttribPointer(0, 2, gl!.FLOAT, false, 0, 0);
    }
    
    func cleanup(gl: TNSWebGLRenderingContext?){
        gl!.useProgram(0)
        if (buffer != nil){
            gl!.deleteBuffer(buffer!)
        }
        if (program != nil){
            gl!.deleteProgram(program!)
        }
        
    }
    
    
    
    
    func drawGL(canvas: TNSCanvas){
        let gl = canvas.getContext("webgl") as! TNSWebGLRenderingContext
        gl.viewport(0, 0,
                    gl.drawingBufferWidth, gl.drawingBufferHeight)
        
        let vertexShader = gl.createShader(gl.VERTEX_SHADER)
        gl.shaderSource(vertexShader,vertexShaderSource)
        gl.compileShader(vertexShader)
        let fragmentShader = gl.createShader(gl.FRAGMENT_SHADER)
        gl.shaderSource(fragmentShader,fragmentShaderSource)
        gl.compileShader(fragmentShader)
        let program = gl.createProgram()
        gl.attachShader(program, vertexShader)
        gl.attachShader(program, fragmentShader)
        gl.linkProgram(program)
        gl.detachShader(program, vertexShader)
        gl.detachShader(program, fragmentShader)
        gl.deleteShader(vertexShader)
        gl.deleteShader(fragmentShader)
        
        
        if (!(gl.getProgramParameter(program, gl.LINK_STATUS) as! Bool)) {
            let linkErrLog = gl.getProgramInfoLog(program)
        print("error", linkErrLog)
            cleanup(gl: gl)
        }
        
        initializeAttributes(gl: gl)
        gl.useProgram(program)
        gl.drawArrays(gl.POINTS, 0, 1)
        cleanup(gl: gl)
    }
    
    func clearExample(ctx: TNSCanvasRenderingContext2D) {
        ctx.beginPath()
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(fromString: "#ff6"))
        ctx.fillRect(0, 0, canvas1!.width, canvas1!.height)

        // Draw blue triangle
        ctx.beginPath();
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(fromString: "blue"))
        ctx.moveTo(20, 20)
        ctx.lineTo(180, 20)
        ctx.lineTo(130, 130)
        ctx.closePath()
        ctx.fill()

        // Clear part of the canvas
        ctx.clearRect(10, 10, 120, 100)
    }
    
    
    func drawAll() {
       // canvas1?.handleInvalidationManually = true
        
//        let q = DispatchQueue(label: "bg", qos: .background, attributes: .concurrent, autoreleaseFrequency: .inherit, target: nil)
//        q.async {
//            self.gl = (self.canvas1?.getContext("webgl")  as! TNSWebGLRenderingContext)
//            self.drawPoints(canvas: self.canvas1!)
//        }
        
        
        
      // drawRotatingCube(gl: gl!)
        
       // drawRotatingCube(gl: gl!)
       // drawGL(canvas: canvas1!) // sun
        
        // drawTextures(canvas: canvas1!)


        
        
       // canvas1?.handleInvalidationManually = true
        
        //drawPoints(canvas: canvas1!)
      // canvas1?.handleInvalidationManually = true
       /* canvas1?.moveOffMain()
        DispatchQueue.global(qos: .default).async {
            self.canvas1?.handleMoveOffMain()
            //let ctx = self.canvas1?.getContext("2d") as! TNSCanvasRenderingContext2D
            //self.doSolarAnimation(ctx: ctx)
            //self.drawFace(ctx: ctx)
            self.gl = (self.canvas1?.getContext("webgl")  as! TNSWebGLRenderingContext)
           // self.drawTextures(canvas: self.canvas1!)
            
        }
        */
        //let ctx = canvas1?.getContext("2d") as! TNSCanvasRenderingContext2D
       //clearExample(ctx: ctx)
        //drawImageExample(ctx: ctx)
       // canvas1?.flush()
        //drawImageBlock(ctx: ctx)
         //doSolarAnimation(ctx: ctx)
        //  drawFace(ctx: ctx)
        // fontExample(ctx: ctx)
       // arcToAnimationExample(ctx: ctx)
        //  saveRestoreExample(ctx: ctx)
        //ballExample(ctx: ctx)
        
        //ctx.fillRect(x: 200, y: 10, width: 200, height: 200);
        // scaleTransformation(ctx: ctx)
       // particleAnimation(ctx: ctx)
        //        canvas1!.toDataURLAsync { (data) in
        //           print("data: ", data)
        //        }
        
        //drawPatterWithCanvas(canvas: canvas1!)
        //ellipseExample(ctx: ctx)
        
    }
    
    
    func drawPatterWithCanvas(canvas: TNSCanvas){
        let patternCanvas = TNSCanvas(frame: .zero, useGL: true)
        let patternContext = patternCanvas.getContext("2d") as! TNSCanvasRenderingContext2D
        let scale = UIScreen.main.scale
        let width = 50
        let height = 50
        var frame = CGRect()
        frame.size.width = CGFloat(width)
        frame.size.height = CGFloat(height)
        patternCanvas.frame = frame
        patternCanvas.setNeedsLayout()
        patternCanvas.layoutIfNeeded()


    // Give the pattern a background color and draw an arc
        patternContext.fillStyle = "#fec"
        patternContext.fillRect(0, 0, patternCanvas.width, patternCanvas.height)
        patternContext.arc(0, 0, Float(50 * scale), 0, (0.5 * PI))
        patternContext.stroke()
        print("done")
    // Create our primary canvas and fill it with the pattern

        let ctx = canvas.getContext("2d") as! TNSCanvasRenderingContext2D
        let pattern = ctx.createPattern(patternCanvas, .Repeat)
        ctx.fillStyle = pattern
        ctx.fillRect(0, 0, canvas.width, canvas.height)
        }
    
    var vertCode2 = """
                   attribute vec3 coordinates;
                       void main() {
                       gl_Position = vec4(coordinates, 1.0);
                       gl_PointSize = 10.0;
       }
"""
    
    var fragCode2 = """
    void main() {
       gl_FragColor = vec4(0.0, 0.0, 0.0, 0.1);
    }
"""
    
    func drawPoints(canvas: TNSCanvas){
        let gl = canvas.getContext("webgl") as! TNSWebGLRenderingContext

                /*==========Defining and storing the geometry=======*/

        let vertices: [Float] = [
                   -0.5,0.5,0.0,
                   0.0,0.5,0.0,
                   -0.25,0.25,0.0,
                ]

                // Create an empty buffer object to store the vertex buffer
        let vertex_buffer = gl.createBuffer();
     

                //Bind appropriate array buffer to it
        gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer);
        

                // Pass the vertex data to the buffer
        gl.bufferData(gl.ARRAY_BUFFER,f32: vertices, gl.STATIC_DRAW);

                // Unbind the buffer
        gl.bindBuffer(gl.ARRAY_BUFFER, 0)

                /*=========================Shaders========================*/

                // vertex shader source code
           

                // Create a vertex shader object
        let vertShader = gl.createShader(gl.VERTEX_SHADER);
                
                // Attach vertex shader source code
        gl.shaderSource(vertShader, vertCode2);

                // Compile the vertex shader
        gl.compileShader(vertShader);

                // fragment shader source code
                

                // Create fragment shader object
        let fragShader = gl.createShader(gl.FRAGMENT_SHADER);

                // Attach fragment shader source code
        gl.shaderSource(fragShader, fragCode2);

                // Compile the fragmentt shader
        gl.compileShader(fragShader);
                
                // Create a shader program object to store
                // the combined shader program
        let shaderProgram = gl.createProgram();

                // Attach a vertex shader
        gl.attachShader(shaderProgram, vertShader);

                // Attach a fragment shader
        gl.attachShader(shaderProgram, fragShader);

                // Link both programs
        gl.linkProgram(shaderProgram);
        
        
        let linked = gl.getProgramParameter(shaderProgram, gl.LINK_STATUS) as! Bool
               if (!linked) {
                 // something went wrong with the link
                let lastError = gl.getProgramInfoLog(shaderProgram);
                 print("Error in program linking:" + lastError);

                gl.deleteProgram(shaderProgram);
                 return
               }

                // Use the combined shader program object
        gl.useProgram(shaderProgram);

                /*======== Associating shaders to buffer objects ========*/

                // Bind vertex buffer object
        gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer);

                // Get the attribute location
        let coord = gl.getAttribLocation(shaderProgram, "coordinates");
                // Point an attribute to the currently bound VBO
        gl.vertexAttribPointer(UInt32(coord), 3, gl.FLOAT, false, 0, 0)

                // Enable the attribute
        gl.enableVertexAttribArray(UInt32(coord))

                /*============= Drawing the primitive ===============*/

                // Clear the canvas
        gl.clearColor(0.5, 0.5, 0.5, 0.9);

                // Enable the depth test
        gl.enable(gl.DEPTH_TEST);
        
                // Clear the color buffer bit
        gl.clear(UInt32(gl.COLOR_BUFFER_BIT));
   
                // Set the view port
        gl.viewport(0,0,gl.drawingBufferWidth,gl.drawingBufferHeight);

                // Draw the triangle
        gl.drawArrays(gl.POINTS, 0, 3);
    }
    
    
    
    func drawTextures(canvas: TNSCanvas) {
        let gl = canvas.getContext("webgl") as! TNSWebGLRenderingContext
    
        let vertexShaderSrc = """
      attribute vec2 a_position;
      uniform vec2 u_resolution;
      void main() {
         vec2 zeroToOne = a_position / u_resolution;
         vec2 zeroToTwo = zeroToOne * 2.0;
         vec2 clipSpace = zeroToTwo - 1.0;
         gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
        }
    """

        let fragmentShaderSrc = """
         precision mediump float;
         uniform vec4 u_color;
         void main() {
            gl_FragColor = u_color;
         }
         """

        // setup GLSL program

        let vertexShader = gl.createShader(gl.VERTEX_SHADER);
        gl.shaderSource(vertexShader, vertexShaderSrc);
        gl.compileShader(vertexShader);

        var compiled = gl.getShaderParameter(vertexShader, gl.COMPILE_STATUS) as! Bool
        if (!compiled) {
          // Something went wrong during compilation; get the error
            let lastError = gl.getShaderInfoLog(vertexShader);
          print(
            "*** Error compiling vertexShader ", vertexShader, ":" , lastError
          )
            gl.deleteShader(vertexShader)
          return
        }

        let fragmentShader = gl.createShader(gl.FRAGMENT_SHADER);
        gl.shaderSource(fragmentShader, fragmentShaderSrc);
        gl.compileShader(fragmentShader);

        compiled = gl.getShaderParameter(fragmentShader, gl.COMPILE_STATUS) as! Bool
        if (!compiled) {
          // Something went wrong during compilation; get the error
            let lastError = gl.getShaderInfoLog(fragmentShader);
          print(
            "*** Error compiling fragmentShader ",
              fragmentShader,
              ":",
              lastError
          );
            gl.deleteShader(fragmentShader);
          return;
        }

        let program = gl.createProgram();

        gl.attachShader(program, vertexShader);
        gl.attachShader(program, fragmentShader);
        gl.linkProgram(program);

        // Check the link status
        let linked = gl.getProgramParameter(program, gl.LINK_STATUS) as! Bool
        if (!linked) {
          // something went wrong with the link
            let lastError = gl.getProgramInfoLog(program);
          print("Error in program linking:" + lastError);

            gl.deleteProgram(program);
          return
        }

        // look up where the vertex data needs to go.
        let positionAttributeLocation = gl.getAttribLocation(program, "a_position");

        // look up uniform locations
        let resolutionUniformLocation = gl.getUniformLocation(
            program,
            "u_resolution"
        );
   
        let colorUniformLocation = gl.getUniformLocation(program, "u_color");

        // Create a buffer to put three 2d clip space points in
        let positionBuffer = gl.createBuffer();

        // Bind it to ARRAY_BUFFER (think of it as ARRAY_BUFFER = positionBuffer)
        gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

        // Tell WebGL how to convert from clip space to pixels
        gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);

        // Clear the canvas
        gl.clearColor(1, 1, 1, 1);
        gl.clear(gl.COLOR_BUFFER_BIT);

        // Tell it to use our program (pair of shaders)
        gl.useProgram(program);

        // Bind the position buffer.
        gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

        // create the buffer
        let indexBuffer = gl.createBuffer();
  

        // make this buffer the current 'ELEMENT_ARRAY_BUFFER'
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer);

        // Fill the current element array buffer with data
        let indices : [UInt16] = [
                    0,
                     1,
                     2, // first triangle
                     2,
                     1,
                     3, // second triangle
        ]
            
        
        gl.bufferData(
            gl.ELEMENT_ARRAY_BUFFER,
            u16: indices,
            gl.STATIC_DRAW
        )

        // code above this line is initialization code
        // --------------------------------
        // code below this line is rendering code

        // Turn on the attribute
        gl.enableVertexAttribArray(UInt32(positionAttributeLocation));

        // Tell the attribute how to get data out of positionBuffer (ARRAY_BUFFER)
        let size = 2; // 2 components per iteration
        let type = gl.FLOAT; // the data is 32bit floats
        let normalize = false; // don't normalize the data
        let stride = 0; // 0 = move forward size * sizeof(type) each iteration to get the next position
        let offset = 0; // start at the beginning of the buffer
        
        gl.vertexAttribPointer(
            UInt32(positionAttributeLocation),
            Int32(size),
            type,
            normalize,
            Int32(stride),
            offset
        );

        // bind the buffer containing the indices
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer);

        // set the resolution
        gl.uniform2f(
            resolutionUniformLocation,
            Float(gl.drawingBufferWidth),
            Float(gl.drawingBufferHeight)
        );
        

        // draw 50 random rectangles in random colors
        for _ in 0 ... 50 {
            // Setup a random rectangle
            // This will write to positionBuffer because
            // its the last thing we bound on the ARRAY_BUFFER
            // bind point
            setRectangle(
                gl: gl,
                x: randomInt(range: 300),
                y: randomInt(range: 300),
                width: randomInt(range: 300),
                height: randomInt(range: 300)
            );

            // Set a random color.
            gl.uniform4f(
                colorUniformLocation,
                Float.random(in: 0 ... 1),
                Float.random(in: 0 ... 1),
                Float.random(in: 0 ... 1),
                1
            );

            // Draw the rectangle.
            let primitiveType = gl.TRIANGLES
            let offset = 0
            let count = 6
            let indexType = gl.UNSIGNED_SHORT
            gl.drawElements(primitiveType, Int32(count), indexType, Int(offset))
        }
      }

      // Returns a random integer from 0 to range - 1.
    func randomInt(range: Float) -> Float{
        return floor(Float.random(in: -1 ... 0) * range)
      }

      // Fill the buffer with the values that define a rectangle.
    func setRectangle(gl: TNSWebGLRenderingContext, x: Float, y: Float, width: Float, height: Float) {
        let x1 = x;
        let x2 = x + width;
        let y1 = y;
        let y2 = y + height;
        gl.bufferData(
            gl.ARRAY_BUFFER,
            f32: [x1, y1, x2, y1, x1, y2, x2, y2],
            gl.STATIC_DRAW
        );
      }

    
    
    
    func drawRotatingCube(gl: TNSWebGLRenderingContext){
        let width = gl.getCanvas().width
        let height = gl.getCanvas().height
        let vertices: [Float32] = [
            -1,-1,-1, 1,-1,-1, 1, 1,-1, -1, 1,-1,
            -1,-1, 1, 1,-1, 1, 1, 1, 1, -1, 1, 1,
            -1,-1,-1, -1, 1,-1, -1, 1, 1, -1,-1, 1,
            1,-1,-1, 1, 1,-1, 1, 1, 1, 1,-1, 1,
            -1,-1,-1, -1,-1, 1, 1,-1, 1, 1,-1,-1,
            -1, 1,-1, -1, 1, 1, 1, 1, 1, 1, 1,-1,
        ]
        let colors: [Float32] = [
            5,3,7, 5,3,7, 5,3,7, 5,3,7,
            1,1,3, 1,1,3, 1,1,3, 1,1,3,
            0,0,1, 0,0,1, 0,0,1, 0,0,1,
            1,0,0, 1,0,0, 1,0,0, 1,0,0,
            1,1,0, 1,1,0, 1,1,0, 1,1,0,
            0,1,0, 0,1,0, 0,1,0, 0,1,0
        ]
        
        indices = [
            0,1,2, 0,2,3, 4,5,6, 4,6,7,
            8,9,10, 8,10,11, 12,13,14, 12,14,15,
            16,17,18, 16,18,19, 20,21,22, 20,22,23
        ]
        
        
        
        // Create and store data into vertex buffer
        let vertex_buffer = gl.createBuffer()
        gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer)
        gl.bufferData(gl.ARRAY_BUFFER, f32: vertices, gl.STATIC_DRAW)
        
        // Create and store data into color buffer
        let color_buffer = gl.createBuffer ()
        gl.bindBuffer(gl.ARRAY_BUFFER, color_buffer)
        gl.bufferData(gl.ARRAY_BUFFER,f32: colors, gl.STATIC_DRAW)
        
        // Create and store data into index buffer
        index_buffer = gl.createBuffer()
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, index_buffer)
        gl.bufferData(gl.ELEMENT_ARRAY_BUFFER,u16: indices, gl.STATIC_DRAW)
    
        
        let vertShader = gl.createShader(gl.VERTEX_SHADER)
        gl.shaderSource(vertShader, vertCode)
        gl.compileShader(vertShader)
        
        
        let fragShader = gl.createShader(gl.FRAGMENT_SHADER)
        gl.shaderSource(fragShader, fragCode)
        gl.compileShader(fragShader)
        
        
        let shaderProgram = gl.createProgram()
        gl.attachShader(shaderProgram, vertShader)
        gl.attachShader(shaderProgram, fragShader)
        gl.linkProgram(shaderProgram)
        
        /* ====== Associating attributes to vertex shader =====*/
        Pmatrix = gl.getUniformLocation(shaderProgram, "Pmatrix")
        Vmatrix = gl.getUniformLocation(shaderProgram, "Vmatrix")
        Mmatrix = gl.getUniformLocation(shaderProgram, "Mmatrix")

        
        gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer)
        let position = gl.getAttribLocation(shaderProgram, "position")
        gl.vertexAttribPointer(UInt32(position), 3, gl.FLOAT, false,0,0)
        
        // Position
        gl.enableVertexAttribArray(UInt32(position))
        gl.bindBuffer(gl.ARRAY_BUFFER, color_buffer)
        let color = gl.getAttribLocation(shaderProgram, "color")
        gl.vertexAttribPointer(UInt32(color), 3, gl.FLOAT, false,0,0)
        
        // Color
        gl.enableVertexAttribArray(UInt32(color))
        gl.useProgram(shaderProgram)
        
        
        proj_matrix = get_projection(angle: 40, a: Int32(width / height), zMin: 1, zMax: 100)
        mov_matrix = [1,0,0,0, 0,1,0,0, 0,0,1,0, 0,0,0,1]
        
        view_matrix = [1,0,0,0, 0,1,0,0, 0,0,1,0, 0,0,0,1]
        
        // translating z
        view_matrix[14] = view_matrix[14]-6;//zoom
        
          
        cubeRotationAnimation(gl: gl, time: 0)
        
    }
    
    
    
    func cubeRotationAnimation(gl: TNSWebGLRenderingContext,time: Float) {
        let width = gl.drawingBufferWidth
        let height = gl.drawingBufferHeight
        let dt = time - time_old
        rotateZ(matrix: &mov_matrix, angle: dt*0.005) //time
        rotateY(matrix: &mov_matrix, angle: dt*0.002)
        rotateX(matrix: &mov_matrix, angle: dt*0.003)
        time_old = time
        
        gl.enable(gl.DEPTH_TEST)
        gl.depthFunc(gl.LEQUAL)
        gl.clearColor(0.5, 0.5, 0.5, 0.9)
        gl.clearDepth(1.0)
        gl.viewport(0, 0, Int32(width), Int32(height))
        gl.clear(UInt32(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT))
        gl.uniformMatrix4fv(Pmatrix, false, proj_matrix)
        gl.uniformMatrix4fv(Vmatrix, false, view_matrix)
        gl.uniformMatrix4fv(Mmatrix, false, mov_matrix)
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, index_buffer)
        gl.drawElements(gl.TRIANGLES, Int32(indices.count), gl.UNSIGNED_SHORT, 0)
        TNSAnimationFrame.requestAnimationFrame { (t) in
            self.cubeRotationAnimation(gl: gl, time: t)
        }
    }
    
    var index_buffer: UInt32 = 0
    var indices: [UInt16] = []
    var Pmatrix: Int32 = 0
    var Vmatrix: Int32 = 0
    var Mmatrix: Int32 = 0
    var proj_matrix: [Float32] = []
    var mov_matrix: [Float32] = []
    var view_matrix: [Float32] = []
    var time_old: Float = 0
    
    func get_projection(angle: Float, a: Int32, zMin: Int32, zMax: Int32) -> [Float32] {
        let ang = tan((angle * Float(0.5)) * PI/180) //angle*.5
        return [
            0.5 / ang, 0.0 , 0.0, 0.0,
            0.0, Float(0.5) * (Float(a)/ang), 0.0, 0.0,
            0.0, 0.0, -(Float(zMax) + Float(zMin)) / (Float(zMax) - Float(zMin)), -1.0,
            0.0, 0.0, (-2 * Float(zMax) * Float(zMin))/(Float(zMax) - Float(zMin)), 0.0
        ]
    }
    
    
    
    func rotateZ(matrix: inout [Float32], angle: Float) {
        let c = cos(angle)
        let s = sin(angle)
        var m = matrix
        let mv0 = m[0], mv4 = m[4], mv8 = m[8]
        
        m[0] = c*m[0]-s*m[1]
        m[4] = c*m[4]-s*m[5]
        m[8] = c*m[8]-s*m[9]
        
        m[1]=c*m[1]+s*mv0
        m[5]=c*m[5]+s*mv4
        m[9]=c*m[9]+s*mv8
    }
    
    func rotateX(matrix: inout [Float32], angle: Float) {
        let c = cos(angle)
        let s = sin(angle)
        var m = matrix
        let mv1 = m[1], mv5 = m[5], mv9 = m[9]
        
        m[1] = m[1]*c-m[2]*s
        m[5] = m[5]*c-m[6]*s
        m[9] = m[9]*c-m[10]*s
        
        m[2] = m[2]*c+mv1*s
        m[6] = m[6]*c+mv5*s
        m[10] = m[10]*c+mv9*s
    }
    
    func rotateY(matrix:inout [Float32], angle: Float) {
        let c = cos(angle)
        let s = sin(angle)
        var m = matrix
        let mv0 = m[0], mv4 = m[4], mv8 = m[8]
        
        m[0] = c*m[0]+s*m[2]
        m[4] = c*m[4]+s*m[6]
        m[8] = c*m[8]+s*m[10]
        
        m[2] = c*m[2]-s*mv0
        m[6] = c*m[6]-s*mv4
        m[10] = c*m[10]-s*mv8
    }
    
    
    
    func scaleTransformation(ctx: TNSCanvasRenderingContext2D){
        // Scaled rectangle
        ctx.scale(9, 3);
        ctx.fillStyle = TNSColorStyle.Color(color: .red);
        ctx.fillRect(10, 10, 8, 20);
        
        // Reset current transformation matrix to the identity matrix
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        
        // Non-scaled rectangle
        ctx.fillStyle = TNSColorStyle.Color(color: .gray);
        ctx.fillRect(10, 10, 8, 20);
    }
    
    func drawNight(ctx: TNSCanvasRenderingContext2D) {
        ctx.fillRect(0, 0, 150, 150);
        ctx.translate(75, 75);
        
        // Create a circular clipping path
        ctx.beginPath();
        ctx.arc(0, 0,  60,  0, PI * 2, true);
        ctx.clip();
        
        // draw background
        let lingrad = ctx.createLinearGradient(0, -75, 0, 75);
        lingrad.addColorStop(offset: 0, color: UIColor(fromString: "#232256"));
        lingrad.addColorStop(offset: 1, color: UIColor(fromString: "#143778"));
        
        ctx.fillStyle = lingrad;
        ctx.fillRect(-75, -75, 150, 150);
        
        // draw stars
        for _ in 0 ... 49 {
            ctx.save()
            ctx.fillStyle = TNSColorStyle.Color.init(color: .white)
            ctx.translate(75 - floor(Float.random(in: 0.0...1.0) * 150),
                          75 - floor(Float.random(in: 0.0...1.0) * 150));
            drawStar(ctx: ctx, r: floor(Float.random(in: 0.0...1.0) * 4) + 2);
            ctx.restore();
        }
        
    }
    
    func drawStar(ctx: TNSCanvasRenderingContext2D, r: Float) {
        ctx.save();
        ctx.beginPath();
        ctx.moveTo(r, 0)
        for i in 0 ... 8 {
            ctx.rotate(PI / 5);
            if (i % 2 == 0) {
                ctx.lineTo((r / 0.525731) * 0.200811, 0);
            } else {
                ctx.lineTo(r, 0);
            }
        }
        ctx.closePath();
        ctx.fill();
        ctx.restore();
    }
    
    func rotateSquare(ctx: TNSCanvasRenderingContext2D) {
        // left rectangles, rotate from canvas origin
        ctx.save();
        // blue rect
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(fromString: "#0095DD"));
        ctx.fillRect(30, 30, 100, 100);
        ctx.rotate((PI / 180) * 25);
        // grey rect
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(fromString: "#4D4E53"));
        ctx.fillRect(30, 30, 100, 100);
        ctx.restore();
        
        // right rectangles, rotate from rectangle center
        // draw blue rect
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(fromString: "#0095DD"));
        ctx.fillRect(150, 30, 100, 100);
        
        ctx.translate(200, 80); // translate to rectangle center
        // x = x + 0.5 * width
        // y = y + 0.5 * height
        ctx.rotate((PI / 180) * 25); // rotate
        ctx.translate(-200, -80); // translate back
        
        // draw grey rect
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(fromString: "#4D4E53"));
        ctx.fillRect(150, 30, 100, 100);
    }
    
    func drawImageBlock(ctx: TNSCanvasRenderingContext2D){
        
        do {
            let home = URL(fileURLWithPath:NSTemporaryDirectory())
            let rhino = home.appendingPathComponent("rhino.jpg")
            let image: UIImage?
            if(!FileManager.default.fileExists(atPath: rhino.path)){
                let data = try Data(contentsOf: URL(string: "https://mdn.mozillademos.org/files/5397/rhino.jpg")!)
                try data.write(to: rhino)
                image = UIImage(data: data)
            }else {
                image = UIImage(contentsOfFile: rhino.path)
            }
            let s = Float(UIScreen.main.scale) * 2
            
            for i in 0...3{
                for j in 0 ... 3 {
                    ctx.drawImage(image!, Float(j * 50) * s, Float(i * 38) * s, 50 * s, 38 * s);
                }
            }
        } catch  {
            print(error)
            
        }
    }
    
    func radialGradient(ctx: TNSCanvasRenderingContext2D){
        // Create gradients
        let radgrad = ctx.createRadialGradient(45, 45, 10, 52, 50, 30);
        radgrad.addColorStop(offset: 0, color: UIColor(fromString: "#A7D30C"))
        radgrad.addColorStop(offset: 0.9, color: UIColor(fromString: "#019F62"));
        radgrad.addColorStop(offset: 1, color: UIColor(fromString: "rgba(1, 159, 98, 0)"));
        
        let radgrad2 = ctx.createRadialGradient(105, 105, 20, 112, 120, 50);
        radgrad2.addColorStop(offset: 0, color: UIColor(fromString: "#FF5F98"));
        radgrad2.addColorStop(offset: 0.75, color: UIColor(fromString: "#FF0188"));
        radgrad2.addColorStop(offset: 1, color: UIColor(fromString: "rgba(255, 1, 136, 0)"));
        
        let radgrad3 = ctx.createRadialGradient(95, 15, 15, 102, 20, 40);
        radgrad3.addColorStop(offset: 0, color: UIColor(fromString: "#00C9FF"));
        radgrad3.addColorStop(offset: 0.8, color: UIColor(fromString: "#00B5E2"));
        radgrad3.addColorStop(offset: 1, color: UIColor(fromString: "rgba(0, 201, 255, 0)"));
        
        let radgrad4 = ctx.createRadialGradient(0, 150, 50, 0, 140, 90);
        radgrad4.addColorStop(offset: 0, color: UIColor(fromString: "#F4F201"));
        radgrad4.addColorStop(offset: 0.8, color: UIColor(fromString: "#E4C700"));
        radgrad4.addColorStop(offset: 1, color: UIColor(fromString: "rgba(228, 199, 0, 0)"));
        
        // draw shapes
        ctx.fillStyle = radgrad4;
        ctx.fillRect(0, 0, 150, 150);
        ctx.fillStyle = radgrad3;
        ctx.fillRect(0, 0, 150, 150);
        ctx.fillStyle = radgrad2;
        ctx.fillRect(0, 0, 150, 150);
        ctx.fillStyle = radgrad;
        ctx.fillRect(0, 0, 150, 150);
    }
    
    func drawLinearGradient(ctx: TNSCanvasRenderingContext2D) {
        // Create gradients
        let lingrad = ctx.createLinearGradient(0, 0, 0, 150);
        lingrad.addColorStop(offset: 0, color: UIColor(fromHex: "#00ABEB"))
        lingrad.addColorStop(offset: 0.5, color: UIColor(fromHex: "#fff"));
        lingrad.addColorStop(offset: 0.5, color: UIColor(fromHex: "#26C000"));
        lingrad.addColorStop(offset: 1, color: UIColor(fromHex: "#fff"));
        
        let lingrad2 = ctx.createLinearGradient(0, 50, 0, 95);
        lingrad2.addColorStop(offset: 0.5, color: UIColor(fromHex: "#000"));
        lingrad2.addColorStop(offset: 1, color: UIColor(red: 0, green: 0, blue: 0, alpha: 0));
        
        // assign gradients to fill and stroke styles
        ctx.fillStyle = lingrad;
        ctx.strokeStyle = lingrad2;
        
        // draw shapes
        ctx.fillRect(10, 10, 130, 130);
        ctx.strokeRect(50, 50, 50, 50);
    }
    
    
    
    
    func drawPM(ctx: TNSCanvasRenderingContext2D) {
        let s = Float(UIScreen.main.scale)
        roundedRect(ctx: ctx, x: 12 * s, y: 12 * s, width: 150 * s, height: 150 * s, radius: 15 * s);
        roundedRect(ctx: ctx, x: 19 * s, y: 19 * s, width: 150 * s, height: 150 * s, radius: 9 * s);
        roundedRect(ctx: ctx, x: 53 * s, y: 53 * s, width: 49 * s, height: 33 * s, radius: 10 * s);
        roundedRect(ctx: ctx, x: 53 * s, y: 119 * s, width: 49 * s, height: 16 * s, radius: 6 * s);
        roundedRect(ctx: ctx, x: 135 * s, y: 53 * s, width: 49 * s, height: 33 * s, radius: 10 * s);
        roundedRect(ctx: ctx, x: 135 * s, y: 119 * s, width: 25 * s, height: 49 * s, radius: 10 * s);
        
        ctx.beginPath();
        ctx.arc(37 * s, 37 * s, 13 * s, PI / 7, -PI / 7, false);
        ctx.lineTo(31 * s, 37 * s);
        ctx.fill();
        
        for i in 0...7{
            ctx.fillRect(Float(51 + i * 16)  * s, 35 * s, 4 * s, 4 * s);
        }
        
        for i in 0...5{
            ctx.fillRect(115 * s, Float(51 + i * 16) * s, 4 * s, 4 * s);
        }
        
        for i in 0...7{
            ctx.fillRect(Float(51 + i * 16) * s, 99 * s, 4 * s, 4 * s);
        }
        
        
        ctx.beginPath();
        ctx.moveTo(83 * s, 116 * s);
        ctx.lineTo(83 * s, 102 * s);
        ctx.bezierCurveTo(83 * s, 94 * s, 89 * s, 88 * s, 97 * s, 88 * s);
        ctx.bezierCurveTo(105 * s, 88 * s, 111 * s, 94 * s, 111 * s, 102 * s);
        ctx.lineTo(111 * s, 116 * s);
        ctx.lineTo(106.333 * s, 111.333 * s);
        ctx.lineTo(101.666 * s, 116 * s);
        ctx.lineTo(97 * s, 111.333 * s);
        ctx.lineTo(92.333 * s, 116 * s);
        ctx.lineTo(87.666 * s, 111.333 * s);
        ctx.lineTo(83 * s, 116 * s);
        ctx.fill();
        
        ctx.fillStyle = TNSColorStyle.Color(color: .white);
        ctx.beginPath();
        ctx.moveTo(91 * s, 96 * s);
        ctx.bezierCurveTo(88 * s, 96 * s, 87 * s, 99 * s, 87 * s, 101 * s);
        ctx.bezierCurveTo(87 * s, 103 * s, 88 * s, 106 * s, 91 * s, 106 * s);
        ctx.bezierCurveTo(94 * s, 106 * s, 95 * s, 103 * s, 95 * s, 101 * s);
        ctx.bezierCurveTo(95 * s, 99 * s, 94 * s, 96 * s, 91 * s, 96 * s);
        ctx.moveTo(103 * s, 96 * s);
        ctx.bezierCurveTo(100 * s, 96 * s, 99 * s, 99 * s, 99 * s, 101 * s);
        ctx.bezierCurveTo(99 * s, 103 * s, 100 * s, 106 * s, 103 * s, 106 * s);
        ctx.bezierCurveTo(106 * s, 106 * s, 107 * s, 103 * s, 107 * s, 101 * s);
        ctx.bezierCurveTo(107 * s, 99 * s, 106 * s, 96 * s, 103 * s, 96 * s);
        ctx.fill();
        
        ctx.fillStyle = TNSColorStyle.Color(color: .black);
        ctx.beginPath();
        ctx.arc( 101 * s, 102 * s, 2 * s, 0 * s, PI * 2, true);
        ctx.fill();
        
        ctx.beginPath();
        ctx.arc(89 * s, 102 * s, 2 * s, 0 * s, PI * 2, true);
        ctx.fill();
        
    }
    
    // A utility function to draw a rectangle with rounded corners.
    
    func roundedRect(ctx: TNSCanvasRenderingContext2D, x: Float, y: Float, width: Float, height:Float, radius:Float) {
        ctx.beginPath();
        ctx.moveTo(x, y + radius);
        ctx.lineTo(x, y + height - radius);
        ctx.arcTo(x, y + height, x + radius, y + height, radius);
        ctx.lineTo(x + width - radius, y + height);
        ctx.arcTo(x + width, y + height, x + width, y + height-radius, radius);
        ctx.lineTo(x + width, y + radius);
        ctx.arcTo(x + width, y, x + width - radius, y, radius);
        ctx.lineTo(x + radius, y);
        ctx.arcTo(x, y, x, y + radius, radius);
        ctx.stroke();
    }
    
    func drawWindow(ctx: TNSCanvasRenderingContext2D){
        // draw background
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(fromHex: "#FD0"));
        ctx.fillRect(0, 0, Float(75 * scale), Float(75 * scale));
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(fromHex: "#6C0"));
        ctx.fillRect(Float(75 * scale), 0, Float(75 * scale), Float(75 * scale));
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(fromHex: "#09F"));
        ctx.fillRect(0, Float(75 * scale), Float(75 * scale), Float(75 * scale));
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(fromHex: "#F30"));
        ctx.fillRect(Float(75 * scale), Float(75 * scale), Float(75 * scale), Float(75 * scale));
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(fromHex: "#FFF"));
        
        // set transparency value
        ctx.globalAlpha = 0.2;
        
        // Draw semi transparent circles
        
        for i in 0...6 {
            ctx.beginPath();
            ctx.arc(Float(75 * scale),Float(75 * scale), Float((10 + 10 * i) * scale), 0, PI * 2, true);
            ctx.fill();
        }
    }
    
    func draw1() {
        let ctx = canvas1?.getContext("2d") as! TNSCanvasRenderingContext2D
        ctx.beginPath()
        ctx.arc(240, 20, 40, 0, PI)
        ctx.moveTo(100, 20)
        ctx.arc(60, 20, 40, 0, PI)
        ctx.moveTo(215, 80)
        ctx.arc(150, 80, 65, 0, PI)
        ctx.closePath()
        ctx.lineWidth = 6
        ctx.stroke()
    }
    
    
    func scaleText(ctx: TNSCanvasRenderingContext2D){
        ctx.scale(-1, 1);
        ctx.font = "48px serif";
        ctx.fillText("Hello world!", -280, 90);
        ctx.setTransform(1, 0, 0, 1, 0, 0);
    }
    func saveRestoreExample(ctx: TNSCanvasRenderingContext2D){
        // Save the default state
        ctx.save();
        let s = Float(UIScreen.main.scale)
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(red: 0, green: 128/255, blue: 0, alpha: 1.0));
        ctx.fillRect(10 * s, 10 * s, 100 * s, 100 * s);
        
        // Restore the default state
        ctx.restore();
        
        ctx.fillRect(150 * s, 40 * s, 100 * s, 100 * s);
        
    }
    func closePathExample(ctx: TNSCanvasRenderingContext2D){
        ctx.beginPath();
        ctx.arc(240, 20, 40, 0, PI);
        ctx.moveTo(100, 20);
        ctx.arc(60, 20, 40, 0, PI);
        ctx.moveTo(215, 80);
        ctx.arc(150, 80, 65, 0, PI);
        ctx.closePath();
        ctx.lineWidth = 6;
        ctx.stroke();
    }
    
    func bezierCurveToExample(ctx: TNSCanvasRenderingContext2D) {
        // Define the points as {x, y}
        let start = KeyValue(x: 50, y: 20)
        let cp1 = KeyValue(x: 230, y: 30)
        let cp2 = KeyValue(x: 150, y: 80)
        let end = KeyValue(x: 250, y: 100)
        
        // Cubic Bézier curve
        ctx.beginPath()
        ctx.moveTo(start.x, start.y)
        ctx.bezierCurveTo(cp1.x, cp1.y, cp2.x, cp2.y, end.x, end.y)
        ctx.stroke()
        
        // Start and end points
        ctx.fillStyle = TNSColorStyle.Color(color: .blue)
        ctx.beginPath()
        ctx.arc(start.x, start.y, 5, 0, TWO_PI)  // Start point
        ctx.arc(end.x, end.y, 5, 0, TWO_PI)      // End point
        ctx.fill()
        
        // Control points
        ctx.fillStyle = TNSColorStyle.Color(color: .red)
        ctx.beginPath()
        ctx.arc(cp1.x, cp1.y, 5, 0, TWO_PI)  // Control point one
        ctx.arc(cp2.x, cp2.y, 5, 0, TWO_PI)  // Control point two
        ctx.fill()
    }
    
    func drawHouse(ctx: TNSCanvasRenderingContext2D){
        ctx.lineWidth = 10
        
        // Wall
        ctx.strokeRect(75, 140, 150, 110)
        
        // Door
        ctx.fillRect(130, 190, 40, 60)
        
        // Roof
        ctx.moveTo(50, 140)
        ctx.lineTo(150, 60)
        ctx.lineTo(250, 140)
        ctx.closePath()
        ctx.stroke()
    }
    
    
    func arcExample(ctx: TNSCanvasRenderingContext2D){
        for i in 0...3    {
            for j in 0...2{
                ctx.beginPath();
                let x             = Float32(25 + j * 50);                 // x coordinate
                let y             = Float32(25 + i * 50);                 // y coordinate
                let radius        = Float32(20);                          // Arc radius
                let startAngle    = Float32(0);                           // Starting point on circle
                let endAngle      = Float32((PI + (PI * Float(j)) / 2)) // End point on circle
                let anticlockwise = i % 2 == 1;                  // Draw anticlockwise
                
                ctx.arc(x, y, radius, startAngle, endAngle, anticlockwise);
                if (i > 1) {
                    ctx.fill();
                } else {
                    ctx.stroke();
                }
            }
        }
    }
    
    func ellipseExample(ctx: TNSCanvasRenderingContext2D){
       // Draw the ellipse
       ctx.beginPath()
        ctx.ellipse(100, 100, 50, 75, PI / 4, 0, TWO_PI)
       ctx.stroke()

       // Draw the ellipse's line of reflection
       ctx.beginPath()
        ctx.setLineDash([5, 5])
        ctx.moveTo(0, 200)
        ctx.lineTo(200, 0)
       ctx.stroke()
    }
    
    func getCanvasWidth(canvas: TNSCanvas) -> Float32 {
        return Float32(canvas.frame.width )
    }
    
    func getCanvasHeight(canvas: TNSCanvas) -> Float32 {
        return Float32(canvas.frame.height )
    }
    
    func strokeExample(ctx: TNSCanvasRenderingContext2D){
        // First sub-path
        ctx.lineWidth = 26
        ctx.strokeStyle = TNSColorStyle.Color(color: UIColor(red: 1, green: 165/255, blue: 0, alpha: 1.0))
        ctx.moveTo(20, 20)
        ctx.lineTo(160, 20)
        ctx.stroke()
        
        // Second sub-path
        ctx.lineWidth = 14
        ctx.strokeStyle = TNSColorStyle.Color(color: UIColor(red: 0, green: 128/255, blue: 0, alpha: 1.0))
        ctx.moveTo(20, 80)
        ctx.lineTo(220, 80)
        ctx.stroke()
        
        // Third sub-path
        ctx.lineWidth = 4
        ctx.strokeStyle = TNSColorStyle.Color(color: UIColor(red: 255/255, green: 192/255, blue: 203/255, alpha: 1.0))
        ctx.moveTo(20, 140)
        ctx.lineTo(280, 140)
        ctx.stroke()
    }
    
    @IBAction func firstTap(_ sender: UITapGestureRecognizer) {
        drawAll()
    }
    
    class KeyValue{
        let x: Float
        let y: Float
        init(x: Float, y: Float) {
            self.x = x
            self.y = y
        }
    }
    @IBAction func secondTap(_ sender: UITapGestureRecognizer) {
        let ctx = canvas2?.getContext("2d") as! TNSCanvasRenderingContext2D
        //_ = canvas2?.ensureIsContextIsCurrent()
        drawHouse(ctx: ctx)
    }
    
    
    private func textPoint(ctx: TNSCanvasRenderingContext2D,p: KeyValue, offset: KeyValue,i: Int){
        let x = offset.x
        let y = offset.y
        ctx.beginPath()
        ctx.arc(p.x, p.y, 2, 0, TWO_PI)
        ctx.fill()
        var text = String(i) + ":"
        text = text + String(Int(p.x)) + ","
        text = text + String(Int(p.y))
        ctx.fillText(text, p.x + x, p.y + y);
    }
    
    private func drawPoints(ctx: TNSCanvasRenderingContext2D, points: [KeyValue]){
        for i in 0...points.count - 1 {
            autoreleasepool {
                textPoint(ctx: ctx,p: points[i], offset: KeyValue( x: 0, y: -20 ) , i: i)
            }
        }
    }
    
    private func drawArc(ctx: TNSCanvasRenderingContext2D, points: [KeyValue], r: Float){
        ctx.beginPath()
        p1 = points[0]
        p2 = points[1]
        p3 = points[2]
        
        ctx.moveTo(p1.x, p1.y)
        ctx.arcTo(p2.x, p2.y, p3.x, p3.y, r)
        ctx.lineTo(p3.x, p3.y)
        ctx.stroke()
    }
    
    
    
    func drawFace(ctx: TNSCanvasRenderingContext2D) {
        ctx.beginPath()
        ctx.arc(75,75, 50,0, TWO_PI, true) // Outer circle
        ctx.moveTo(110, 75)
        ctx.arc(75, 75, 35, 0, PI, false) // Mouth (clockwise)
        ctx.moveTo(65, 65)
        ctx.arc(60, 65, 5, 0, TWO_PI, true)  // Left eye
        ctx.moveTo(95, 65);
        ctx.arc(90, 65, 5, 0, TWO_PI, true)  // Right eye
        ctx.strokeStyle = TNSColorStyle.Color(color: UIColor(fromString: "blue"))
        ctx.stroke();
        
    }
    var last = 0
    private func loop(ctx: TNSCanvasRenderingContext2D, t: Float){
        let PI2 = TWO_PI
        let points = [p1,p2,p3]
        let t0 = t / 1000
        let a  = t0.truncatingRemainder(dividingBy: PI2);
        let rr = abs(cos(a) * r)
        ctx.clearRect(0, 0, ctx.getCanvas().width, ctx.getCanvas().height);
        drawArc(ctx: ctx,points: points, r: rr)
        drawPoints(ctx: ctx,points: points)
        TNSAnimationFrame.requestAnimationFrame(toLoop: { (id) in
            self.loop(ctx: ctx, t: Float(id))
        })
    }
    
    class Ball {
        var x = 100.0
        var y = 100.0
        var vx = 5.0
        var vy = 2.0
        var radius: Float = 25
        var color = UIColor.blue
        init() {
            
        }
        func draw(ctx: TNSCanvasRenderingContext2D){
            ctx.beginPath();
            ctx.arc(Float(x), Float(y),radius, 0, .pi * 2, true);
            ctx.closePath();
            ctx.fillStyle = TNSColorStyle.Color(color: color)
            ctx.fill();
        }
    }
    
    
    
    func draw(ctx: TNSCanvasRenderingContext2D) {
        let canvas = ctx.getCanvas()
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(fromString: "rgba(255, 255, 255, 0.3)"))
        let width = canvas.width
        let height = canvas.height
        ctx.fillRect(0, 0, Float(width), Float(height));
        ball.draw(ctx: ctx)
        ball.x += ball.vx;
        ball.y += ball.vy;
        ball.vy *= 0.99;
        ball.vy += 0.25;
        
        if (Int(ball.y + ball.vy) > Int(height) ||
            ball.y + ball.vy < 0) {
            ball.vy = -ball.vy;
        }
        if (Int(ball.x + ball.vx) > Int(width) ||
            ball.x + ball.vx < 0) {
            ball.vx = -ball.vx;
        }
        
        TNSAnimationFrame.requestAnimationFrame(toLoop: { (timer) in
            self.draw(ctx: ctx)
        })
    }
    
    var ball = Ball()
    
    func ballExample(ctx: TNSCanvasRenderingContext2D) {
        TNSAnimationFrame.requestAnimationFrame(toLoop:{ (timer) in
            self.draw(ctx: ctx)
        })
    }
    
    
    func globalCompositeOperationExample(ctx: TNSCanvasRenderingContext2D){
        ctx.globalCompositeOperation = TNSCompositeOperationType(rawValue: "xor")!
        
        ctx.fillStyle = TNSColorStyle.Color(color: .blue)
        ctx.fillRect(10, 10, 100, 100)
        
        ctx.fillStyle = TNSColorStyle.Color(color: .red)
        ctx.fillRect(50, 50, 100, 100)
    }
    
    
    func createRadialGradientExample(ctx: TNSCanvasRenderingContext2D){
        // Create a radial gradient
        // The inner circle is at x=110, y=90, with radius=30
        // The outer circle is at x=100, y=100, with radius=70
        let gradient = ctx.createRadialGradient(110,90,30, 100,100,70);
        // Add three color stops
        //gradient.addColorStop(offset: 0, color: UIColor(red: 1.0, green: 192/255, blue: 203/255, alpha: 1.0));
        // gradient.addColorStop(offset: 0.9, color: .white);
        gradient.addColorStop(offset: 1, color: UIColor(red: 0.0, green: 128/255, blue: 0.0, alpha: 1.0));
        
        // Set the fill style and draw a rectangle
        ctx.fillStyle = gradient;
        ctx.fillRect(20, 20, 160, 160);
    }
    
    
    func createLinearGradientExample(ctx: TNSCanvasRenderingContext2D) {
        // Create a linear gradient
        // The start gradient point is at x=20, y=0
        // The end gradient point is at x=220, y=0
        let gradient = ctx.createLinearGradient(20,0, 220,0);
        
        // Add three color stops
        let green = UIColor(red: 0.0, green: 128/255, blue: 0.0, alpha: 1.0)
        gradient.addColorStop(offset: 0, color: green);
        gradient.addColorStop(offset: 0.5, color: UIColor(red: 0.0, green: 1.0, blue: 1.0, alpha: 1.0));
        gradient.addColorStop(offset: 1, color: green);
        
        // Set the fill style and draw a rectangle
        ctx.fillStyle = gradient;
        ctx.fillRect(20, 20, 200, 100);
    }
    var scale = 0
    var r  = Float(100.0) * 3 ; // Radius
    let p0 = KeyValue( x: 0, y: 50 )
    
    var p1 = KeyValue( x: 100 , y: 100 )
    var p2 = KeyValue( x: 150 , y: 50 )
    var p3 = KeyValue( x: 200 , y: 100 )
    var t = Float(0.0)
    
    
    func arcToAnimationExample(ctx: TNSCanvasRenderingContext2D){
        TNSAnimationFrame.requestAnimationFrame(toLoop:{ (timer) in
            self.loop(ctx: ctx, t: timer)
        })
    }
    
    func arcToExample(ctx: TNSCanvasRenderingContext2D) {
        // Tangential lines
        ctx.beginPath()
        ctx.strokeStyle = TNSColorStyle.Color(color: .darkGray)
        ctx.moveTo(200, 20)
        ctx.lineTo(200, 130)
        ctx.lineTo(50, 20)
        ctx.stroke()
        
        // Arc
        ctx.beginPath()
        ctx.strokeStyle = TNSColorStyle.Color(color: .black)
        ctx.lineWidth = 5
        ctx.moveTo(200, 20)
        ctx.arcTo(200, 130, 50, 20, 40)
        ctx.stroke()
        
        // Start point
        ctx.beginPath()
        ctx.fillStyle = TNSColorStyle.Color(color: .blue)
        ctx.arc(200, 20, 5, 0, (TWO_PI))
        ctx.fill()
        
        // Control points
        ctx.beginPath()
        ctx.fillStyle = TNSColorStyle.Color(color: .red)
        ctx.arc(200, 130, 5, 0, TWO_PI) // Control point one
        ctx.arc(50, 20, 5, 0, TWO_PI)   // Control point two
        ctx.fill()
    }
    
    func clearRectExample(ctx: TNSCanvasRenderingContext2D){
        // Draw yellow background
        let width = ctx.getCanvas().width
        let height = ctx.getCanvas().height
        ctx.beginPath();
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(fromHex: "#ff6"))
        ctx.fillRect(0, 0, width, height);
        
        // Draw blue triangle
        ctx.beginPath();
        ctx.fillStyle = TNSColorStyle.Color(color: .blue);
        ctx.moveTo(20, 20);
        ctx.lineTo(180, 20);
        ctx.lineTo(130, 130);
        ctx.closePath();
        ctx.fill();
        
        // Clear part of the canvas
        ctx.clearRect(10, 10, 120, 100);
    }
    
    func fontExample(ctx: TNSCanvasRenderingContext2D){
        ctx.font = "bold 48px serif"
        ctx.strokeText("Hello world", 50, 100);
        
        ctx.font = "50px serif"
        ctx.fillText("Hello world", 50, 190);
    }
    
    func setTransformExample(ctx: TNSCanvasRenderingContext2D){
        ctx.setTransform(1, 0.2, 0.8, 1, 0, 0);
        ctx.fillRect(0, 0, 100, 100);
    }
    
    func scaleExample(ctx: TNSCanvasRenderingContext2D){
        // Scaled rectangle
        ctx.scale(9, 3)
        ctx.fillStyle = TNSColorStyle.Color(color: .red)
        ctx.fillRect(10, 10, 8, 20);
        
        // Reset current transformation matrix to the identity matrix
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        
        // Non-scaled rectangle
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(red: 128/255, green: 128/255, blue: 128/255, alpha: 1.0))
        ctx.fillRect(10, 10, 8, 20);
    }
    
    func rotateAngleExample(ctx: TNSCanvasRenderingContext2D){
        // Point of transform origin
        ctx.arc(0, 0, 5, 0, TWO_PI);
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0));
        ctx.fill();
        
        // Non-rotated rectangle
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(red: 128/255, green: 128/255, blue: 128/255, alpha: 1.0))
        ctx.fillRect(100, 0, 80, 20);
        
        // Rotated rectangle
        ctx.rotate(45 * PI / 180);
        ctx.fillStyle =  TNSColorStyle.Color(color: .red)
        ctx.fillRect(100, 0, 80, 20);
        
        // Reset transformation matrix to the identity matrix
        ctx.setTransform(1, 0, 0, 1, 0, 0);
    }
    
    func secondRotateAngleExample(ctx: TNSCanvasRenderingContext2D) {
        // Non-rotated rectangle
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(red: 128/255, green: 128/255, blue: 128/255, alpha: 1.0))
        ctx.fillRect(80, 60, 140, 30);
        
        // Matrix transformation
        ctx.translate(150, 75);
        ctx.rotate(PI / 2);
        ctx.translate(-150, -75);
        
        // Rotated rectangle
        ctx.fillStyle = TNSColorStyle.Color(color: .red)
        ctx.fillRect(80, 60, 140, 30);
    }
    
    func translateExample(ctx: TNSCanvasRenderingContext2D){
        ctx.translate(110, 30);
        ctx.fillStyle = TNSColorStyle.Color(color: .red)
        ctx.fillRect(0, 0, 80, 80);
        
        // Reset current transformation matrix to the identity matrix
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        
        // Unmoved square
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(red: 128/255, green: 128/255, blue: 128/255, alpha: 1.0))
        ctx.fillRect(0, 0, 80, 80);
    }
    
    func quadraticCurveToExample(ctx: TNSCanvasRenderingContext2D){
        // Quadratic Bézier curve
        ctx.beginPath();
        ctx.moveTo(50, 20);
        ctx.quadraticCurveTo(230, 30, 50, 100);
        ctx.stroke();
        
        // Start and end points
        ctx.fillStyle = TNSColorStyle.Color(color: .blue)
        ctx.beginPath();
        ctx.arc(50, 20, 5, 0, TWO_PI);   // Start point
        ctx.arc(50, 100, 5, 0, TWO_PI);  // End point
        ctx.fill();
        
        // Control point
        ctx.fillStyle = TNSColorStyle.Color(color: .red)
        ctx.beginPath();
        ctx.arc(230, 30, 5, 0, TWO_PI);
        ctx.fill();
    }
    
    func drawImageExample(ctx: TNSCanvasRenderingContext2D) {
        do {
            let home = URL(fileURLWithPath:NSTemporaryDirectory())
            let rhino = home.appendingPathComponent("rhino.jpg")
            let image: UIImage?
            if(!FileManager.default.fileExists(atPath: rhino.path)){
                let data = try Data(contentsOf: URL(string: "https://mdn.mozillademos.org/files/5397/rhino.jpg")!)
                try data.write(to: rhino)
                image = UIImage(data: data)
            }else {
                image = UIImage(contentsOfFile: rhino.path)
            }
            
            ctx.drawImage(image!, 0,0);
        } catch  {
            print(error)
        }
    }
    
    func doSolarAnimation(ctx: TNSCanvasRenderingContext2D){
        TNSAnimationFrame.requestAnimationFrame(toLoop: { (id) in
            self.solarSystemExample(ctx: ctx)
        })
    }
    var didScaleSolar = false
    
    var sun: UIImage?
    var moon: UIImage?
    var earth: UIImage?
    func solarSystemExample(ctx: TNSCanvasRenderingContext2D){
        do{
            if sun == nil {
                let sun_url = NSURL(fileURLWithPath: NSTemporaryDirectory() + "/Canvas_sun.png")
                let sun_data = try Data(contentsOf: URL(string: "https://mdn.mozillademos.org/files/1456/Canvas_sun.png")!)
                try sun_data.write(to: sun_url.absoluteURL!, options: .atomicWrite)
                sun = UIImage(data: sun_data)
            }
            if moon == nil {
                let moon_url = NSURL(fileURLWithPath: NSTemporaryDirectory() + "/Canvas_moon.png")
                let moon_data = try Data(contentsOf: URL(string: "https://mdn.mozillademos.org/files/1443/Canvas_moon.png")!)
                try moon_data.write(to: moon_url.absoluteURL!, options: .atomicWrite)
                moon = UIImage(data: moon_data)
            }
            
            if earth == nil {
                let earth_url = NSURL(fileURLWithPath: NSTemporaryDirectory() + "/Canvas_earth.png")
                let earth_data = try Data(contentsOf: URL(string: "https://mdn.mozillademos.org/files/1429/Canvas_earth.png")!)
                try earth_data.write(to: earth_url.absoluteURL!, options: .atomicWrite)
                earth = UIImage(data: earth_data)
            }
            
            
            ctx.globalCompositeOperation = TNSCompositeOperationType.DestinationOver
            ctx.clearRect(0, 0, 300, 300); // clear canvas
            
            ctx.fillStyle = TNSColorStyle.Color(color: UIColor(red: 0, green: 0, blue: 0, alpha: 0.4))
            ctx.strokeStyle = TNSColorStyle.Color(color: UIColor(red: 0.0, green: 153/255, blue: 1.0, alpha: 0.4))
            ctx.save();
            ctx.translate(150, 150);
            
            // Earth
            let time = Date()
            let calendar = Calendar.current
            let components = calendar.dateComponents([.second,.nanosecond], from: time)
            _ = components.second!
            let seconds = components.second!
            let milliseconds = seconds * 1000
            let part_one = (TWO_PI / 60) * Float(seconds)
            let part_two = (TWO_PI / 60000) * Float(milliseconds)
            let first_angle = part_one + part_two
            
        
            ctx.rotate(first_angle)
            ctx.translate(105, 0);
            ctx.fillRect(0, -12, 40, 24); // Shadow
            ctx.drawImage(earth!, -12, -12);
            
            
            // Moon
            ctx.save();
            let part_one_second = (TWO_PI / 6) * Float(seconds)
            let part_two_second = (TWO_PI / 60000) * Float(milliseconds)
            let second_angle = part_one_second + part_two_second
            ctx.rotate(second_angle)
            ctx.translate(0, 28.5);
            ctx.drawImage(moon!, -3.5, -3.5);
            ctx.restore();
            
            ctx.restore();
            
            ctx.beginPath();
            ctx.arc(150, 150, 105, 0, TWO_PI, false); // Earth orbit
            ctx.stroke();
            
            ctx.drawImage(sun!, 0, 0, 300, 300);
            
            if(!didScaleSolar){
              //  ctx.scale(x: ctx.getCanvas().width / 300, y: ctx.getCanvas().height / 300)
                didScaleSolar = true
            }
            
            TNSAnimationFrame.requestAnimationFrame(toLoop:{ id in
                self.solarSystemExample(ctx: ctx)
            })
            
        }catch {
            print(error)
        }
    }
    
    
    func clockAnimation(ctx: TNSCanvasRenderingContext2D){
        TNSAnimationFrame.requestAnimationFrame(toLoop: { (timer) in
            self.clock(ctx: ctx)
        })
    }
    
    func x2Clock(ctx: TNSCanvasRenderingContext2D){
        let date = Date()
        let calendar = Calendar.current
        let components = calendar.dateComponents([.second,.minute,.hour], from: date)
        let seconds = components.second!
        let minutes = components.minute!
        let hours = components.hour!
        ctx.save();
        let scale = UIScreen.main.scale
        ctx.clearRect(0, 0, Float(150 ), Float(150 ));
        ctx.translate(Float(75 ), Float(75 ));
        ctx.scale(0.4, 0.4);
        ctx.rotate(-PI / 2);
        ctx.strokeStyle = TNSColorStyle.Color(color: .black);
        ctx.fillStyle = TNSColorStyle.Color(color: .white);
        ctx.lineWidth = Float(8  );
        ctx.lineCap = TNSLineCap.Round;
        
        // Hour marks
        ctx.save();
        for _ in 0...11{
            ctx.beginPath();
            ctx.rotate(PI / 6);
            ctx.moveTo(Float(100 ), 0);
            ctx.lineTo(Float(120 ), 0);
            ctx.stroke();
        }
        ctx.restore();
        
        // Minute marks
        ctx.save();
        ctx.lineWidth = Float(5 );
        for i in 0...59{
            if (i % 5 != 0) {
                ctx.beginPath();
                ctx.moveTo(Float(117 ), 0);
                ctx.lineTo(Float(120 ), 0);
                ctx.stroke();
            }
            ctx.rotate(PI / 30);
        }
        ctx.restore();
        
        let sec = Float(seconds)
        let min = Float(minutes)
        var hr  = Float(hours)
        hr = hr >= 12 ? hr - 12 : hr;
        
        ctx.fillStyle = TNSColorStyle.Color(color: .black);
        
        // write Hours
        ctx.save();
        let first = hr * (PI / 6)
        let second = first + (PI / 360) * min
        let third = second  + (PI / 21600) * sec
        ctx.rotate(third)
        ctx.lineWidth = Float(14 );
        ctx.beginPath();
        ctx.moveTo(Float(-20 ), 0);
        ctx.lineTo(Float(80 ), 0);
        ctx.stroke();
        ctx.restore();
        
        // write Minutes
        ctx.save();
        ctx.rotate((PI / 30) * min + (PI / 1800) * sec);
        ctx.lineWidth = Float(10 );
        ctx.beginPath();
        ctx.moveTo(Float(-28 ), 0);
        ctx.lineTo(Float(112 ), 0);
        ctx.stroke();
        ctx.restore();
        
        // Write seconds
        ctx.save();
        ctx.rotate(sec * PI / 30);
        ctx.strokeStyle = TNSColorStyle.Color(color: UIColor(fromHex: "#D40000"));
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(fromHex: "#D40000"));
        ctx.lineWidth = Float(6 );
        ctx.beginPath();
        ctx.moveTo(Float(-30 ), 0);
        ctx.lineTo(Float(83 ), 0);
        ctx.stroke();
        ctx.beginPath();
        ctx.arc( 0, 0, Float(10 ), 0, PI * 2, true);
        ctx.fill();
        ctx.beginPath();
        ctx.arc(Float(95 ), 0, Float(10 ), 0, PI * 2, true);
        ctx.stroke();
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(red: 0, green: 0, blue: 0, alpha: 0));
        ctx.arc(0, 0, Float(3 ), 0, PI * 2, true);
        ctx.fill();
        ctx.restore();
        
        ctx.beginPath();
        ctx.lineWidth = Float(14 );
        ctx.strokeStyle = TNSColorStyle.Color(color: UIColor(fromHex: "#325FA2"));
        ctx.arc(0, 0, Float(142 ), 0, PI * 2, true);
        ctx.stroke();
        
        ctx.restore();
        
        TNSAnimationFrame.requestAnimationFrame(toLoop: { (timer) in
            self.clock(ctx: ctx)
        })
    }
    
    func clock(ctx: TNSCanvasRenderingContext2D){
        let date = Date()
        let calendar = Calendar.current
        let components = calendar.dateComponents([.second,.minute,.hour], from: date)
        let seconds = components.second!
        let minutes = components.minute!
        let hours = components.hour!
        ctx.save();
        ctx.clearRect(0, 0, 150, 150);
        ctx.translate(75, 75);
        ctx.scale(0.5, 0.5);
        ctx.rotate(-PI / 2);
        ctx.strokeStyle = TNSColorStyle.Color(color: .black);
        ctx.fillStyle = TNSColorStyle.Color(color: .white);
        ctx.lineWidth = 8;
        ctx.lineCap = TNSLineCap.Round;
        
        // Hour marks
        ctx.save();
        for _ in 0...11{
            ctx.beginPath();
            ctx.rotate(PI / 6);
            ctx.moveTo(100, 0);
            ctx.lineTo(120, 0);
            ctx.stroke();
        }
        ctx.restore();
        
        // Minute marks
        ctx.save();
        ctx.lineWidth = 5;
        for i in 0...59{
            if (i % 5 != 0) {
                ctx.beginPath();
                ctx.moveTo(117, 0);
                ctx.lineTo(120, 0);
                ctx.stroke();
            }
            ctx.rotate(PI / 30);
        }
        ctx.restore();
        
        let sec = Float(seconds)
        let min = Float(minutes)
        var hr  = Float(hours)
        hr = hr >= 12 ? hr - 12 : hr;
        
        ctx.fillStyle = TNSColorStyle.Color(color: .black);
        
        // write Hours
        ctx.save();
        let first = hr * (PI / 6)
        let second = first + (PI / 360) * min
        let third = second  + (PI / 21600) * sec
        ctx.rotate(third)
        ctx.lineWidth = 14;
        ctx.beginPath();
        ctx.moveTo(-20, 0);
        ctx.lineTo(80, 0);
        ctx.stroke();
        ctx.restore();
        
        // write Minutes
        ctx.save();
        ctx.rotate((PI / 30) * min + (PI / 1800) * sec);
        ctx.lineWidth = 10;
        ctx.beginPath();
        ctx.moveTo(-28, 0);
        ctx.lineTo(112, 0);
        ctx.stroke();
        ctx.restore();
        
        // Write seconds
        ctx.save();
        ctx.rotate(sec * PI / 30);
        ctx.strokeStyle = TNSColorStyle.Color(color: UIColor(fromHex: "#D40000"));
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(fromHex: "#D40000"));
        ctx.lineWidth = 6;
        ctx.beginPath();
        ctx.moveTo(-30, 0);
        ctx.lineTo(83, 0);
        ctx.stroke();
        ctx.beginPath();
        ctx.arc(0,0, 10, 0, PI * 2, true);
        ctx.fill();
        ctx.beginPath();
        ctx.arc(95, 0, 10, 0, PI * 2, true);
        ctx.stroke();
        ctx.fillStyle = TNSColorStyle.Color(color: UIColor(red: 0, green: 0, blue: 0, alpha: 0));
        ctx.arc(0,0, 3, 0,PI * 2, true);
        ctx.fill();
        ctx.restore();
        
        ctx.beginPath();
        ctx.lineWidth = 14;
        ctx.strokeStyle = TNSColorStyle.Color(color: UIColor(fromHex: "#325FA2"));
        ctx.arc(0,0, 142, 0, PI * 2, true);
        ctx.stroke();
        
        ctx.restore();
        
        TNSAnimationFrame.requestAnimationFrame(toLoop: { (timer) in
            self.clock(ctx: ctx)
        })
    }
    
    var particleCount = 400
    var particleLoopCount = 399
    var particles : [Particle] = []
    var minDist: Float = 70
    var dist: Float = 0
    
    
    var W: Int = 0
    var H: Int = 0
    
    // Function to paint the canvas black
    func paintCanvas(ctx: TNSCanvasRenderingContext2D) {
        ctx.fillStyle = blackColor
        ctx.fillRect(0,0,Float(W),Float(H));
    }
    
    
    class Particle: NSObject{
        var x: Float
        var y: Float
        var vx: Float
        var vy: Float
        var radius: Float = 4
        var W: Int = 0
        var H: Int = 0
        var color: TNSColorStyle.Color
        static var PI_TWO = Float.pi * 2
        init(width: Int, height: Int, color: TNSColorStyle.Color) {
            W = width
            H = height
            x = Float.random(in: 0...1) * Float(W)
            y = Float.random(in: 0...1) * Float(H)
            vx = -1 + Float.random(in: 0...1) * 2;
            vy = -1 + Float.random(in: 0...1) * 2;
            self.color = color
        }
        
        
        
        func draw (ctx: TNSCanvasRenderingContext2D) {
            ctx.fillStyle = color
            ctx.beginPath()
            ctx.arc(x, y,  Float(radius),  0, Particle.PI_TWO, false);
            ctx.closePath()
            ctx.fill()
        }
    }
    
    func increaseParticles() {
        if(particleCount == 300){
            return
        }
        particleCount += 20
        let count = (20 - 1)
        for _ in 0...count {
            particles.append(Particle(width: W, height: H, color: whiteColor))
        }
        
    }
    
    func particle_draw(ctx: TNSCanvasRenderingContext2D) {
        // increaseParticles()
        // Call the paintCanvas function here so that our canvas
        // will get re-painted in each next frame
        paintCanvas(ctx: ctx)
        
       // Call the function that will draw the balls using a loop
        let count = particleLoopCount
        for i in 0...count {
            p = particles[i]
            p.draw(ctx: ctx)
        }
        
        
        
        //Finally call the update function
        update(ctx: ctx)
        
        
    }
    
    var p: Particle = Particle(width: 0, height: 0, color: TNSColorStyle.Color(color: UIColor(fromString: "white")))
    func update(ctx: TNSCanvasRenderingContext2D) {
        
        // In this function, we are first going to update every
        // particle's position according to their velocities
        let count = particleLoopCount
        for i in 0...count {
            p = particles[i]
            
            // Change the velocities
            p.x += p.vx;
            p.y += p.vy
            
            // We don't want to make the particles leave the
            // area, so just change their position when they
            // touch the walls of the window
            if(Int(p.x + p.radius) > W){
                p.x = p.radius;
            }else if(p.x - p.radius < 0) {
                p.x = Float(W) - p.radius;
            }
            
            if(Int(p.y + p.radius) > H){
                p.y = p.radius;
            }else if(p.y - p.radius < 0) {
                p.y = Float(H) - p.radius;
            }
            
            // Now we need to make them attract each other
            // so first, we'll check the distance between
            // them and compare it to the minDist we have
            // already set
            
            // We will need another loop so that each
            // particle can be compared to every other particle
            // except itself
            
            for j in i...count{
                let p2 = self.particles[j]
                distance(ctx: ctx,p1: self.p, p2: p2)
            }
            
        }
    }
    
    let whiteColor = TNSColorStyle.Color(color: UIColor(fromString: "white"))
    let blackColor = TNSColorStyle.Color(color: UIColor(fromString: "black"))
    func distance(ctx:TNSCanvasRenderingContext2D,p1: Particle, p2: Particle) {
        var colorIndex = 0
        let dx = p1.x - p2.x;
        let dy = p1.y - p2.y;
        
        //dist = Float.squareRoot(dx*dx + dy*dy)()
        dist = sqrt(dx*dx + dy*dy)
        
        // Draw the line when distance is smaller
        // then the minimum distance
        if(dist <= minDist) {
            // Draw the line
            ctx.beginPath()
            colorIndex = Int((100.0 * dist/minDist)) + 25
             ctx.strokeStyle = whiteColor
//            ctx.strokeStyle = CanvasColorStyle.Color(color: UIColor(hue: 2/360, saturation: CGFloat(colorIndex/100), brightness: 0.5, alpha: (CGFloat(1.2-dist/minDist))))
            ctx.moveTo(p1.x, p1.y)
            ctx.lineTo(p2.x, p2.y)
            ctx.closePath()
            ctx.stroke()
            
            // Some acceleration for the partcles
            // depending upon their distance
            let ax = dx/2000,
            ay = dy/2000;
            
            // Apply the acceleration on the particles
            p1.vx -= ax;
            p1.vy -= ay;
            
            p2.vx += ax;
            p2.vy += ay;
        }
        
    }
    
    
    func animloop(ctx: TNSCanvasRenderingContext2D) {
         TNSAnimationFrame.requestAnimationFrame(toLoop: {(_) in self.animloop(ctx: ctx)})
        particle_draw(ctx: ctx)
    }
    
    func particleAnimation(ctx: TNSCanvasRenderingContext2D){
        ctx.getCanvas().handleInvalidationManually =  true
        W = Int(ctx.getCanvas().width)
        H = Int(ctx.getCanvas().height)
        //  particles =  Array(repeating: Particle(width: W, height: H, color: whiteColor), count: particleCount)
        let count = (particleCount - 1)
        for _ in 0...count {
            particles.append(Particle(width: W, height: H, color: whiteColor))
        }
        animloop(ctx:ctx)
    }
    
    
    
    class Ellipses {
        var angle: Float = 0
        
        
        func draw(ctx: TNSCanvasRenderingContext2D) {
            let width = ctx.getCanvas().width
            let height = ctx.getCanvas().height
            ctx.clearRect(0, 0, width, height);
            var count = 0;
            
            for i in stride(from: 0, to: 360, by: 6) {
                ctx.beginPath();
                ctx.ellipse(
                    Float(width/2),
                    Float(height/2),
                    15, 235,
                    Float(Int(angle) + i) * Float.pi / 180,
                    0, 2 * .pi
                );
                
                switch(count) {
                case 0:
                    ctx.strokeStyle = TNSColorStyle.Color(color: UIColor(fromHex: "#008000"))
                case 1:
                    ctx.strokeStyle = TNSColorStyle.Color(color: UIColor(fromHex: "#0000ff"))
                case 2:
                    ctx.strokeStyle = TNSColorStyle.Color(color: UIColor(fromHex: "#ff0000"))
                case 3:
                    ctx.strokeStyle = TNSColorStyle.Color(color: UIColor(fromHex: "#800080"))
                default: break
                    
                }
                
                ctx.stroke()
                ctx.closePath()
                
                if (count == 3){
                    count = 0;
                } else {
                    count+=1
                }
            }
            
            angle+=1;
            
            if (angle == 361) {angle = 0;}
        };
    }
    
    func runEllipse(ctx: TNSCanvasRenderingContext2D) {
        let ellipse = Ellipses()
        TNSAnimationFrame.requestAnimationFrame(toLoop: { (_) in
            self.ellipseAnimation(ctx: ctx, ellipse: ellipse)
        })
    }
    func ellipseAnimation(ctx: TNSCanvasRenderingContext2D, ellipse: Ellipses){
        ellipse.draw(ctx: ctx)
        TNSAnimationFrame.requestAnimationFrame(toLoop: { (_) in
            self.ellipseAnimation(ctx: ctx, ellipse: ellipse)
        })
    }
    
    
    
    class ColorSquares{
        
        
        func draw(ctx: TNSCanvasRenderingContext2D) {
            let width = ctx.getCanvas().width
            let height = ctx.getCanvas().height
            ctx.clearRect(0, 0, width, height);
            
            for i in 0...23 {
                for j in 0...23 {
                    drawRect(ctx: ctx,x: Float(i), y: Float(j));
                }
            }
        }
        
        func drawRect(ctx: TNSCanvasRenderingContext2D, x: Float, y:Float) {
            let red = randomHue()/255
            let green = randomHue()/255
            let blue = randomHue()/255
            
            ctx.beginPath();
            ctx.rect((x * 25) , (y * 25) , Float(24 ) , Float(24 ));
            ctx.fillStyle = TNSColorStyle.Color(color: UIColor(red: CGFloat(red), green: CGFloat(green), blue: CGFloat(blue), alpha: 1.0))
            ctx.fill();
        }
        
        func randomHue() -> Double {
            
            let num = floor(.random(in: 0...1) * 256) + 1
            return num
        }
    }
    func colorSquares(ctx: TNSCanvasRenderingContext2D){
        let squares = ColorSquares()
        squares.draw(ctx: ctx)
    }
}
