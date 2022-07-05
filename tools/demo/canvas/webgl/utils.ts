export function createProgramFromScripts(gl, shaderSources) {
    // setup GLSL programs

    const shaders = [];

    if (!Array.isArray(shaderSources)) {
        return;
    }
    shaderSources.forEach((element) => {
        if (element.type === "vertex") {
            // Create the shader object
            const vertexShader = gl.createShader(gl.VERTEX_SHADER);

            console.log('vertexShader', vertexShader);

            console.log(element.src);
            // Load the shader source
            gl.shaderSource(vertexShader, element.src);

            console.log('shaderSource');

            // Compile the shader
            gl.compileShader(vertexShader);

            console.log('compileShader');

            // Check the compile status
            let compiled = gl.getShaderParameter(
                vertexShader,
                gl.COMPILE_STATUS
            );


            console.log('compiled', compiled);

            if (!compiled) {
                // Something went wrong during compilation; get the error
                const lastError = gl.getShaderInfoLog(vertexShader);
                console.log(
                    "*** Error compiling shader '" +
                        vertexShader +
                        "':" +
                        lastError
                );
                gl.deleteShader(vertexShader);
                return;
            }
            shaders.push(vertexShader);
        }

        if (element.type === "fragment") {
            // Create the shader object
            const fragmentShader = gl.createShader(gl.FRAGMENT_SHADER);

            // Load the shader source
            gl.shaderSource(fragmentShader, element.src);

            // Compile the shader
            gl.compileShader(fragmentShader);

            // Check the compile status
            const compiled = gl.getShaderParameter(
                fragmentShader,
                gl.COMPILE_STATUS
            );
            if (!compiled) {
                // Something went wrong during compilation; get the error
                const lastError = gl.getShaderInfoLog(fragmentShader);
                console.log(
                    "*** Error compiling shader '" +
                        fragmentShader +
                        "':" +
                        lastError
                );
                gl.deleteShader(fragmentShader);
                return null;
            }
            shaders.push(fragmentShader);
        }
    });

    const program = gl.createProgram();

    shaders.forEach((shader) => {
        gl.attachShader(program, shader);
    });

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
    return program;
}
