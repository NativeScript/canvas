//
//  NSCRender.swift
//  CanvasNative
//
//  Created by Osei Fortune on 10/04/2021.
//

import Foundation
import OpenGLES
import CoreVideo
import UIKit


@objcMembers
@objc(NSCRender)
public class NSCRender: NSObject {
    var mProgram = UInt32()
    var rbo = UInt32()
    var fbo = UInt32()
    var width: Int = -1
    var height: Int = -1
    var ab = UInt32()
    var pos = Int32(-1)
    var samplerPos = Int32()
    var vextexBuf: [Float32]
    static let SIZE_OF_FLOAT = 4
    static let VERTEX_SHADER = """
    precision highp float;
    attribute vec4 aPosition;
    uniform mat4 uTextureMatrix;
    varying vec2 TexCoord;
    void main(){
    vec2 clipSpace = (1.0 - 2.0 * aPosition.xy);
    TexCoord = aPosition.xy;
    gl_Position = vec4(clipSpace, 0.0, 1.0);
    }
    """
    
    private static let FRAGMENT_SHADER = """
    precision highp float;
    varying vec2 TexCoord;
    uniform sampler2D uSampler;
    void main(){
    gl_FragColor = texture2D(uSampler, TexCoord);
    }
    """
    
    var surface: IOSurface? = nil
    var videoTexture: CVOpenGLESTexture? = nil
    var videoTextureCache: CVOpenGLESTextureCache? = nil
    
    var pixelBuf: Unmanaged<CVPixelBuffer>?
    public override init() {
        vextexBuf = [
            0, 0,
            1, 0,
            0, 1,
            1, 1,
        ]
    }
    func destroy(){
        glDeleteProgram(mProgram)
        glDeleteRenderbuffers(1, &rbo)
        glDeleteFramebuffers(1, &fbo)
        glDeleteBuffers(1, &ab)
        ab = 0
        mProgram = 0
        rbo = 0
        fbo = 0
        width = -1
        height = -1
        pos = -1
    }
    
    
    static func getPixelFormat(_ cgImage: CGImage) -> CGBitmapInfo? {
        return cgImage.bitmapInfo.intersection(.byteOrderMask)
    }

    public func texImage2D(_ target: Int32 , _ level: Int32, _ internalFormat: Int32, _ format: Int32, _ type: Int32, _ source: NSCCanvas, _ dest: NSCCanvas,_ flipYWebGL: Bool){
        
        canvas_native_gl_make_current(source.nativeGL)

        let source_texture = canvas_native_context_get_texture_from_2d(source.native2DContext)
        
        var textureID: GLuint = canvas_native_context_backend_texture_get_id(source_texture)
    
        canvas_native_gl_make_current(dest.nativeGL)
        
        var previousViewPort: [Int32] = Array(repeating: 0, count: 4);
        var previousActiveTexture = GLint()
        var previousTexture = GLint()
        var previousProgram = GLint()
        var previousFrameBuffer = GLint()
        var previousRenderBuffer = GLint()
        var previousVertexArray = GLint()
        
        glGetIntegerv(GLenum(GL_VIEWPORT), &previousViewPort)
        glGetIntegerv(
            GLenum(GL_ACTIVE_TEXTURE),
            &previousActiveTexture
        )
        glGetIntegerv(
            GLenum(GL_TEXTURE_BINDING_2D),
            &previousTexture
        )
        glGetIntegerv(
            GLenum(GL_CURRENT_PROGRAM),
            &previousProgram
        )
        glGetIntegerv(
            GLenum(GL_FRAMEBUFFER_BINDING),
            &previousFrameBuffer
        )
        
        glGetIntegerv(
            GLenum(GL_RENDERBUFFER_BINDING),
            &previousRenderBuffer
        )
        
        glGetIntegerv(
            GLenum(GL_VERTEX_ARRAY_BINDING),
            &previousVertexArray
        )
        
        glBindFramebuffer(GLenum(GL_FRAMEBUFFER), fbo)
        glBindRenderbuffer(GLenum(GL_RENDERBUFFER), rbo)
        
        let width = Int(source.width)
        let height = Int(source.height)
        if (self.width != width || self.height != height) {
            glRenderbufferStorage(GLenum(GL_RENDERBUFFER), GLenum(GL_DEPTH24_STENCIL8), GLsizei(width), GLsizei(height))
            glFramebufferRenderbuffer(GLenum(GL_FRAMEBUFFER), GLenum(GL_DEPTH_STENCIL_ATTACHMENT), GLenum(GL_RENDERBUFFER), rbo)
            glBindTexture(GLenum(GL_TEXTURE_2D), GLuint(previousTexture))
            
            
            glTexImage2D(
                GLenum(GL_TEXTURE_2D),
                0,
                internalFormat,
                GLsizei(width),
                GLsizei(height),
                0,
                GLenum(format),
                GLenum(GL_UNSIGNED_BYTE),
                nil
            )
            
            glTexParameteri(
                GLenum(GL_TEXTURE_2D),
                GLenum(GL_TEXTURE_MAG_FILTER),
                GL_LINEAR
            )
            
            glTexParameteri(
                GLenum(GL_TEXTURE_2D),
                GLenum(GL_TEXTURE_MIN_FILTER),
                GL_LINEAR
            )
            
            glTexParameteri(
                GLenum(GL_TEXTURE_2D),
                GLenum(GL_TEXTURE_WRAP_S),
                GL_CLAMP_TO_EDGE
            )
            
            glTexParameteri(
                GLenum(GL_TEXTURE_2D),
                GLenum(GL_TEXTURE_WRAP_T),
                GL_CLAMP_TO_EDGE
            )
            
            glFramebufferTexture2D(
                GLenum(GL_FRAMEBUFFER),
                GLenum(GL_COLOR_ATTACHMENT0),
                GLenum(GL_TEXTURE_2D),
                GLuint(previousTexture),
                0
            )
            
            if (glCheckFramebufferStatus(GLenum(GL_FRAMEBUFFER))
                    != GL_FRAMEBUFFER_COMPLETE)
            {
                print("TextureRender Error: Could not setup frame buffer.")
            }
            
            self.width = Int(width)
            self.height = Int(height)
            
        }
        
        glClearColor(0, 0, 0, 1)
        glClear(GLbitfield(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT))
        
        
        glUseProgram(mProgram)
        glBindBuffer(GLenum(GL_ARRAY_BUFFER), ab)
        
        glVertexAttribPointer(
            GLuint(pos),
            2,
            GLenum(GL_FLOAT),
            0,
            GLsizei(2 * MemoryLayout.size(ofValue: Float.self)),
            nil
        )
        
        glEnableVertexAttribArray(GLuint(pos))
        
        
        glViewport(0, 0, GLsizei(width), GLsizei(height))
        
 
        
        glBindTexture(GLenum(GL_TEXTURE_2D), textureID)
        
        
        glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_MIN_FILTER), GL_LINEAR)
        glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_MAG_FILTER), GL_LINEAR)
        glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_WRAP_S), GL_CLAMP_TO_EDGE)
        glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_WRAP_T), GL_CLAMP_TO_EDGE)
        
        glUniform1i(
            samplerPos,
            previousActiveTexture - GL_TEXTURE0
        )
        
        
        glViewport(
            0,
            0,
            GLsizei(width),
            GLsizei(height)
        )
        
        glDrawArrays(GLenum(GL_TRIANGLE_STRIP), 0, 4)
        
       glFinish()
        
        glBindRenderbuffer(GLenum(GL_RENDERBUFFER), GLuint(previousRenderBuffer))
        glBindFramebuffer(GLenum(GL_FRAMEBUFFER), GLuint(previousFrameBuffer))
        
        glViewport(
            previousViewPort[0],
            previousViewPort[1],
            previousViewPort[2],
            previousViewPort[3]
        )
        
        glBindTexture(GLenum(GL_TEXTURE_2D), GLuint(previousTexture))
        
        glUseProgram(GLuint(previousProgram))
        
        glBindVertexArray(GLuint(previousVertexArray))
        
        canvas_native_context_backend_texture_destroy(source_texture)
        
        
    }
    
        /*

    public func texImage2D(_ target: Int32 , _ level: Int32, _ internalFormat: Int32, _ format: Int32, _ type: Int32, _ source: NSCCanvas, _ dest: NSCCanvas,_ flipYWebGL: Bool){
       canvas_native_gl_make_current(source.nativeGL)
    
        
        // Check for the OES_mapbuffer extension
        let extensions = String(cString: glGetString(GLenum(GL_EXTENSIONS)))
        
        if extensions.contains("GL_OES_mapbuffer") {
            // OES_mapbuffer extension is supported, you can use glMapBufferOES
            print("OES_mapbuffer is supported.")
        } else {
            // OES_mapbuffer extension is not supported
            print("OES_mapbuffer is not supported.")
        }
        
        
        let sourceWidth = source.drawingBufferWidth
        let sourceHeight = source.drawingBufferHeight
        
       
       // var start = CFAbsoluteTimeGetCurrent()
        
        var previous_framebuffer: GLint = 0
    
        
        glGetIntegerv(
            GLenum(GL_FRAMEBUFFER_BINDING),
            &previous_framebuffer
        )
        
        
        var previous_texture: GLint = 0
        
        glGetIntegerv(
            GLenum(GL_TEXTURE_BINDING_2D),
            &previous_texture
        )
        
        
        var previous_pixel_pack_buffer: GLint = 0
        
        glGetIntegerv(
            GLenum(GL_PIXEL_PACK_BUFFER_BINDING),
            &previous_pixel_pack_buffer
        )
        
        
        
        // Create a framebuffer
        var framebuffer: GLuint = 0
        glGenFramebuffers(1, &framebuffer)
        glBindFramebuffer(GLenum(GL_FRAMEBUFFER), framebuffer)

        // Create a texture to render into
        var texture: GLuint = 0
        glGenTextures(1, &texture)
        glBindTexture(GLenum(GL_TEXTURE_2D), texture)
        glTexImage2D(GLenum(GL_TEXTURE_2D), 0, GL_RGBA, GLsizei(sourceWidth), GLsizei(sourceHeight), 0, GLenum(GL_RGBA), GLenum(GL_UNSIGNED_BYTE), nil)
        glFramebufferTexture2D(GLenum(GL_FRAMEBUFFER), GLenum(GL_COLOR_ATTACHMENT0), GLenum(GL_TEXTURE_2D), texture, 0)

        // Check framebuffer status (optional)
        if glCheckFramebufferStatus(GLenum(GL_FRAMEBUFFER)) != GLenum(GL_FRAMEBUFFER_COMPLETE) {
            print("Framebuffer is not complete.")
        }

        // Unbind framebuffer
        glBindFramebuffer(GLenum(GL_FRAMEBUFFER), GLuint(previous_framebuffer))

        // Create PBOs
        var pbos: [GLuint] = Array(repeating: 0, count: 2
        )
        glGenBuffers(GLsizei(pbos.count), &pbos)
    

        // Allocate storage for PBOs
        let bufferSize: GLsizei = GLsizei(sourceWidth * sourceHeight * 4)
        for pbo in pbos {
            glBindBuffer(GLenum(GL_PIXEL_PACK_BUFFER), pbo)
            glBufferData(GLenum(GL_PIXEL_PACK_BUFFER), GLsizeiptr(bufferSize), nil, GLenum(GL_STREAM_READ))
        }

        // Bind the framebuffer
        glBindFramebuffer(GLenum(GL_FRAMEBUFFER), framebuffer)

        // Use PBOs with glReadPixels for asynchronous pixel transfers
        for pbo in pbos {
            glBindBuffer(GLenum(GL_PIXEL_PACK_BUFFER), pbo)
            glReadPixels(0, 0, GLsizei(sourceWidth), GLsizei(sourceHeight), GLenum(GL_RGBA), GLenum(GL_UNSIGNED_BYTE), nil)
        }

        // Unbind the framebuffer
        glBindFramebuffer(GLenum(GL_FRAMEBUFFER), GLuint(previous_framebuffer))
        
        
        var combinedData = UnsafeMutableRawPointer.allocate(byteCount: Int(bufferSize) * pbos.count, alignment: 1)
        
        // Map the PBOs for reading (asynchronously)
        var count = 0
        for pbo in pbos {
            glBindBuffer(GLenum(GL_PIXEL_PACK_BUFFER), pbo)
          //  let mappedData = glMapBufferOES(GLenum(GL_PIXEL_PACK_BUFFER), GLenum(GL_MAP_READ_BIT_EXT))
            let mappedData = glMapBufferRange(GLenum(GL_PIXEL_PACK_BUFFER), 0, GLsizeiptr(bufferSize), GLenum(GL_MAP_READ_BIT))
            

            if mappedData != nil {
                if(count == 0){
                    memcpy(combinedData, mappedData, Int(bufferSize))
                }else {
                    memcpy(combinedData.advanced(by: Int(bufferSize)), mappedData, Int(bufferSize))
                }
                glUnmapBuffer(GLenum(GL_PIXEL_PACK_BUFFER))
            } else {
                print("Failed to map buffer.")
            }
            
            let error = glGetError()
            if error != GLenum(GL_NO_ERROR) {
                print("OpenGL error: \(error)")
            }
            
            count += 1
        }
        
       

        glBindBuffer(GLenum(GL_PIXEL_PACK_BUFFER), GLuint(previous_pixel_pack_buffer))
        glBindBuffer(GLenum(GL_FRAMEBUFFER), GLuint(previous_framebuffer))
        glBindBuffer(GLenum(GL_TEXTURE_2D), GLuint(previous_texture))


        glDeleteFramebuffers(1, &framebuffer)
        glDeleteTextures(1, &texture)
        glDeleteBuffers(GLsizei(pbos.count), &pbos)
        
      //  print("glMapBufferOES", CFAbsoluteTimeGetCurrent() - start)
        
      //  start = CFAbsoluteTimeGetCurrent()
        canvas_native_gl_make_current(dest.nativeGL)
        
        

//        var unpack: GLint = 0
//        var pack: GLint = 0
//        glGetIntegerv(GLenum(GL_UNPACK_ALIGNMENT), &unpack)
//        glGetIntegerv(GLenum(GL_PACK_ALIGNMENT), &pack)
//
//        print(unpack, pack)
//        glPixelStorei(GLenum(GL_UNPACK_ALIGNMENT), 1)
//        glPixelStorei(GLenum(GL_PACK_ALIGNMENT), 1)
        
    
       
    
        
        glTexImage2D(GLenum(target), level, internalFormat, GLsizei(sourceWidth) , GLsizei(sourceHeight), 0, GLenum(format), GLenum(type), combinedData)
        
        
        combinedData.deallocate()
        
//        glPixelStorei(GLenum(GL_UNPACK_ALIGNMENT), unpack)
//        glPixelStorei(GLenum(GL_PACK_ALIGNMENT), pack)
        
   //     print("glTexImage2D \(CFAbsoluteTimeGetCurrent() - start)")
        
        let error = glGetError()
        if error != GLenum(GL_NO_ERROR) {
            print("OpenGL error: \(error)")
        }
        
    }
    */
    
    func drawFrame(buffer:CVPixelBuffer, width: Int, height: Int, internalFormat: Int32,
                   format: Int32,
                   flipYWebGL: Bool) {
        
        if videoTexture != nil {
            videoTexture = nil
        }
        
        if(videoTextureCache != nil){
            CVOpenGLESTextureCacheFlush(videoTextureCache!, 0)
        }
        
        var previousViewPort: [Int32] = Array(repeating: 0, count: 4);
        var previousActiveTexture = GLint()
        var previousTexture = GLint()
        var previousProgram = GLint()
        var previousFrameBuffer = GLint()
        var previousRenderBuffer = GLint()
        var previousVertexArray = GLint()
        
        glGetIntegerv(GLenum(GL_VIEWPORT), &previousViewPort)
        glGetIntegerv(
            GLenum(GL_ACTIVE_TEXTURE),
            &previousActiveTexture
        )
        glGetIntegerv(
            GLenum(GL_TEXTURE_BINDING_2D),
            &previousTexture
        )
        glGetIntegerv(
            GLenum(GL_CURRENT_PROGRAM),
            &previousProgram
        )
        glGetIntegerv(
            GLenum(GL_FRAMEBUFFER_BINDING),
            &previousFrameBuffer
        )
        
        glGetIntegerv(
            GLenum(GL_RENDERBUFFER_BINDING),
            &previousRenderBuffer
        )
        
        glGetIntegerv(
            GLenum(GL_VERTEX_ARRAY_BINDING),
            &previousVertexArray
        )
        
        glBindFramebuffer(GLenum(GL_FRAMEBUFFER), fbo)
        glBindRenderbuffer(GLenum(GL_RENDERBUFFER), rbo)
        
        if (self.width != width || self.height != height) {
            glRenderbufferStorage(GLenum(GL_RENDERBUFFER), GLenum(GL_DEPTH24_STENCIL8), GLsizei(width), GLsizei(height))
            glFramebufferRenderbuffer(GLenum(GL_FRAMEBUFFER), GLenum(GL_DEPTH_STENCIL_ATTACHMENT), GLenum(GL_RENDERBUFFER), rbo)
            glBindTexture(GLenum(GL_TEXTURE_2D), GLuint(previousTexture))
            
            
            glTexImage2D(
                GLenum(GL_TEXTURE_2D),
                0,
                internalFormat,
                GLsizei(width),
                GLsizei(height),
                0,
                GLenum(format),
                GLenum(GL_UNSIGNED_BYTE),
                nil
            )
            
            glTexParameteri(
                GLenum(GL_TEXTURE_2D),
                GLenum(GL_TEXTURE_MAG_FILTER),
                GL_LINEAR
            )
            
            glTexParameteri(
                GLenum(GL_TEXTURE_2D),
                GLenum(GL_TEXTURE_MIN_FILTER),
                GL_LINEAR
            )
            
            glTexParameteri(
                GLenum(GL_TEXTURE_2D),
                GLenum(GL_TEXTURE_WRAP_S),
                GL_CLAMP_TO_EDGE
            )
            
            glTexParameteri(
                GLenum(GL_TEXTURE_2D),
                GLenum(GL_TEXTURE_WRAP_T),
                GL_CLAMP_TO_EDGE
            )
            
            glFramebufferTexture2D(
                GLenum(GL_FRAMEBUFFER),
                GLenum(GL_COLOR_ATTACHMENT0),
                GLenum(GL_TEXTURE_2D),
                GLuint(previousTexture),
                0
            )
            
            if (glCheckFramebufferStatus(GLenum(GL_FRAMEBUFFER))
                    != GL_FRAMEBUFFER_COMPLETE)
            {
                print("TextureRender Error: Could not setup frame buffer.")
            }
            
            self.width = width
            self.height = height
            
        }
        
        glClearColor(0, 0, 0, 1)
        glClear(GLbitfield(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT))
        
        
        glUseProgram(mProgram)
        glBindBuffer(GLenum(GL_ARRAY_BUFFER), ab)
        
        glVertexAttribPointer(
            GLuint(pos),
            2,
            GLenum(GL_FLOAT),
            0,
            GLsizei(2 * MemoryLayout.size(ofValue: Float.self)),
            nil
        )
        
        glEnableVertexAttribArray(GLuint(pos))
        
        
        glViewport(0, 0, GLsizei(width), GLsizei(height))
        
        CVPixelBufferLockBaseAddress(buffer,CVPixelBufferLockFlags(rawValue: 0))
        let err = CVOpenGLESTextureCacheCreateTextureFromImage(
            kCFAllocatorDefault,
            videoTextureCache!,
            buffer,
            nil,
            GLenum(GL_TEXTURE_2D),
            GL_RGBA,
            GLsizei(width),
            GLsizei(height),
            GLenum(GL_BGRA),
            UInt32(GL_UNSIGNED_BYTE),
            0,
            &videoTexture)
        
        if err != kCVReturnSuccess  {
            glBindRenderbuffer(GLenum(GL_RENDERBUFFER), GLuint(previousRenderBuffer))
            glBindFramebuffer(GLenum(GL_FRAMEBUFFER), GLuint(previousFrameBuffer))
            
            glViewport(
                previousViewPort[0],
                previousViewPort[1],
                previousViewPort[2],
                previousViewPort[3]
            )
            
            glBindTexture(GLenum(GL_TEXTURE_2D), GLuint(previousTexture))
            
            glUseProgram(GLuint(previousProgram))
            
            glBindVertexArray(GLuint(previousVertexArray))
            return
        }
        
        var textureID: GLuint = GLuint()
        textureID = CVOpenGLESTextureGetName(videoTexture!)
        glBindTexture(GLenum(GL_TEXTURE_2D), textureID)
        
        
        glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_MIN_FILTER), GL_LINEAR)
        glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_MAG_FILTER), GL_LINEAR)
        glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_WRAP_S), GL_CLAMP_TO_EDGE)
        glTexParameteri(GLenum(GL_TEXTURE_2D), GLenum(GL_TEXTURE_WRAP_T), GL_CLAMP_TO_EDGE)
        
        glUniform1i(
            samplerPos,
            previousActiveTexture - GL_TEXTURE0
        )
        
        CVPixelBufferUnlockBaseAddress(buffer, CVPixelBufferLockFlags(rawValue: 0))
        
        glViewport(
            0,
            0,
            GLsizei(width),
            GLsizei(height)
        )
        
        glDrawArrays(GLenum(GL_TRIANGLE_STRIP), 0, 4)
        
       // glFinish()
        
        glBindRenderbuffer(GLenum(GL_RENDERBUFFER), GLuint(previousRenderBuffer))
        glBindFramebuffer(GLenum(GL_FRAMEBUFFER), GLuint(previousFrameBuffer))
        
        glViewport(
            previousViewPort[0],
            previousViewPort[1],
            previousViewPort[2],
            previousViewPort[3]
        )
        
        glBindTexture(GLenum(GL_TEXTURE_2D), GLuint(previousTexture))
        
        glUseProgram(GLuint(previousProgram))
        
        glBindVertexArray(GLuint(previousVertexArray))
    }
    
    public func createSurface(){
        let context = EAGLContext.current()
        if context != nil {
            CVOpenGLESTextureCacheCreate(kCFAllocatorDefault, nil, context!, nil, &videoTextureCache)
        }
        
        mProgram = glCreateProgram()
        let vs = glCreateShader(GLenum(GL_VERTEX_SHADER))
        var vs_ptr = (NSCRender.VERTEX_SHADER as NSString).cString(using: String.Encoding.utf8.rawValue)
    
        glShaderSource(vs, 1, &vs_ptr, nil)
        
        let fs = glCreateShader(GLenum(GL_FRAGMENT_SHADER))
        var fs_ptr = (NSCRender.FRAGMENT_SHADER as NSString).cString(using: String.Encoding.utf8.rawValue)
        
        glShaderSource(fs, 1, &fs_ptr, nil)
        
        glCompileShader(vs)
        glCompileShader(fs)
        
        glAttachShader(mProgram, vs)
        glAttachShader(mProgram, fs)
        
        glLinkProgram(mProgram)
        
        glGenBuffers(1, &ab)
        glGenRenderbuffers(1, &rbo)
        glGenFramebuffers(1, &fbo)
        var previousArrayBuffer = GLint(0)
        glGetIntegerv(
            GLenum(GL_ARRAY_BUFFER_BINDING),
            &previousArrayBuffer
        )
        
        /*
         var index = GLuint()
         var array_enabled = GLint()
         var array_size = GLint()
         var array_type = GLint()
         var array_normalized = GLint()
         var array_stride = GLint()
         var array_pointer: UnsafeMutableRawPointer?
         var array_binding = GLint()
         var array_vertex_attr: [Float] = Array(repeating: 0, count: 4)
         
         glGetVertexAttribiv(
         index, GLenum(GL_VERTEX_ATTRIB_ARRAY_ENABLED), &array_enabled)
         glGetVertexAttribiv(
         index, GLenum(GL_VERTEX_ATTRIB_ARRAY_SIZE), &array_size)
         glGetVertexAttribiv(
         index, GLenum(GL_VERTEX_ATTRIB_ARRAY_TYPE), &array_type)
         glGetVertexAttribiv(
         index, GLenum(GL_VERTEX_ATTRIB_ARRAY_NORMALIZED), &array_normalized)
         glGetVertexAttribiv(
         index, GLenum(GL_VERTEX_ATTRIB_ARRAY_STRIDE), &array_stride)
         glGetVertexAttribPointerv(
         index, GLenum(GL_VERTEX_ATTRIB_ARRAY_POINTER), &array_pointer)
         glGetVertexAttribiv(index,
         GLenum(GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING),
         &array_binding)
         glGetVertexAttribfv(
         index, GLenum(GL_CURRENT_VERTEX_ATTRIB), &array_vertex_attr)
         
         */
        
        
        
        glBindBuffer(GLenum(GL_ARRAY_BUFFER), ab)
        glBufferData(
            GLenum(GL_ARRAY_BUFFER),
            Int(vextexBuf.count * NSCRender.SIZE_OF_FLOAT),
            vextexBuf,
            GLenum(GL_STATIC_DRAW)
        )
        
        
        samplerPos = glGetUniformLocation(mProgram, "uSampler")
        pos = glGetAttribLocation(mProgram, "aPosition")
        
        glVertexAttribPointer(GLuint(pos), 2, GLenum(GL_FLOAT), GLboolean(0), GLsizei(2 * NSCRender.SIZE_OF_FLOAT), nil)
        glEnableVertexAttribArray(GLuint(pos))
        
    }
}
