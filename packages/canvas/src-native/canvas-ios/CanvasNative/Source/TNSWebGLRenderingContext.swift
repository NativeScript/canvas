//
//  TNSWebGLRenderingContext.swift
//  CanvasNative
//
//  Created by Osei Fortune on 4/16/20.
//

import Foundation
import UIKit
import GLKit
@objcMembers
@objc(TNSWebGLRenderingContext)
public class TNSWebGLRenderingContext: TNSCanvasRenderingContext {
    var SIZE_OF_BYTE = MemoryLayout<GLubyte>.size
    var SIZE_OF_SHORT = MemoryLayout<GLshort>.size
    var SIZE_OF_FLOAT = MemoryLayout<GLfloat>.size
    var SIZE_OF_INT = MemoryLayout<GLint>.size
    var SIZE_OF_DOUBLE = MemoryLayout<Double>.size

    private var alpha: Bool = true
    private var antialias: Bool = false
    private var depth:Bool = true
    private var failIfMajorPerformanceCaveat:Bool = false
    private var powerPreference:String = "default"
    private var premultipliedAlpha: Bool = false
    private var preserveDrawingBuffer:Bool = false
    private var stencil:Bool = false
    private var desynchronized:Bool = false
    var flipYWebGL: Bool = false
    var premultiplyAlphaWebGL: Bool = false
    var colorSpaceConversionWebGL: UInt32 = 0

    var canvas: TNSCanvas
    public init(_ canvas: TNSCanvas) {
        self.canvas = canvas
        super.init()
        canvas.renderer.ensureIsContextIsCurrent()
    }

    public init(_ canvas: TNSCanvas,_ attrs: [String: Any]) {
        self.canvas = canvas
        super.init()
        canvas.renderer.ensureIsContextIsCurrent()
        /*
         for (key, val)  in attrs {
         switch key {
         case "alpha":
         if(val) as! Bool{
         self.enable(cap: self.BLEND)
         self.blendFunc(sfactor: self.SRC_ALPHA, dfactor: self.ONE_MINUS_SRC_ALPHA)
         }else {
         self.disable(cap: self.BLEND)
         self.blendFunc(sfactor: self.SRC_ALPHA, dfactor: self.ONE_MINUS_SRC_ALPHA)
         }
         case "antialias":
         case "depth":
         if()
         case "failIfMajorPerformanceCaveat":
         case "powerPreference":
         case "premultipliedAlpha":
         case "preserveDrawingBuffer":
         case "stencil":
         case "desynchronized":
         default:

         }
         }
         */
    }

    public func getCanvas() -> TNSCanvas {
        return canvas
    }



    let _GL_UNSIGNED_BYTE: UInt32 = 0x1401;
    let _GL_FLOAT: UInt32 = 0x1406;
    let _GL_HALF_FLOAT: UInt32 = 0x140B;
    let _GL_UNSIGNED_SHORT_5_6_5: UInt32 = 0x8363;
    let _GL_UNSIGNED_SHORT_4_4_4_4: UInt32 = 0x8033;
    let _GL_UNSIGNED_SHORT_5_5_5_1: UInt32 = 0x8034;
    let _GL_LUMINANCE: UInt32 = 0x1909;
    let _GL_ALPHA: UInt32 = 0x1906;
    let _GL_LUMINANCE_ALPHA: UInt32 = 0x190A;
    let _GL_RGB: UInt32 = 0x1907;
    let _GL_RGBA: UInt32 = 0x1908;

    func bytes_per_pixel(pixel_type: UInt32, format: UInt32) -> Int32 {
        var bytes_per_component: Int32 = 0
        switch pixel_type {
        case _GL_UNSIGNED_BYTE:
                bytes_per_component = 1
                break
        case _GL_FLOAT:
                bytes_per_component = 4
                break
        case _GL_HALF_FLOAT:
                bytes_per_component = 2
            break
        case _GL_UNSIGNED_SHORT_5_6_5, _GL_UNSIGNED_SHORT_4_4_4_4 , _GL_UNSIGNED_SHORT_5_5_5_1:
            return 2
        default:
            break
        }

        switch format {
        case _GL_LUMINANCE , _GL_ALPHA:
            return 1 * bytes_per_component
        case _GL_LUMINANCE_ALPHA:
            return 2 * bytes_per_component
        case _GL_RGB:
            return 3 * bytes_per_component
        case _GL_RGBA:
            return 4 * bytes_per_component
        default:
            break
        }
        return 0
    }


    public var drawingBufferWidth: Int32 {
        get {
            return Int32(canvas.renderer.drawingBufferWidth)
        }
    }

    public var drawingBufferHeight: Int32 {
        get{
            return Int32(canvas.renderer.drawingBufferHeight)
        }
    }

    public func activeTexture(_ texture: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glActiveTexture(texture)
    }

    public func attachShader(_ program: UInt32, _ shader: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glAttachShader(program, shader)
    }

    public func bindAttribLocation(_ program: UInt32, _ index: UInt32, _ name: String){
        canvas.renderer.ensureIsContextIsCurrent()
        let bindName = (name as NSString).utf8String
        glBindAttribLocation(program, index, bindName)
    }

    public func bindBuffer(_ target: UInt32, _ buffer: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glBindBuffer(target, buffer)
    }

    public func bindFramebuffer(_ target: UInt32,_ framebuffer: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        if(framebuffer == 0 && target == GL_FRAMEBUFFER){
            glBindFramebuffer(target, canvas.renderer.displayFramebuffer)
            return
        }
        glBindFramebuffer(target, framebuffer)
    }

    public func bindRenderbuffer(_ target: UInt32, _ renderbuffer: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        if(renderbuffer == 0 && target == GL_RENDERER){
            glBindRenderbuffer(target, canvas.renderer.displayRenderbuffer)
            return
        }
        glBindRenderbuffer(target, renderbuffer)
    }

    public func bindTexture(_ target: UInt32,_ texture: UInt32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glBindTexture(target, texture)
    }

    public func blendColor(_ red: Float32,_ green: Float32,_ blue: Float32,_ alpha: Float32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glBlendColor(red, green, blue, alpha)
    }

    public func blendEquation(_ mode: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glBlendEquation(mode)
    }

    public func blendEquationSeparate(_ modeRGB: UInt32,_ modeAlpha: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glBlendEquationSeparate(modeRGB, modeAlpha)
    }

    public func blendFunc(_ sfactor: UInt32,_ dfactor: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glBlendFunc(sfactor, dfactor)
    }

    public func blendFuncSeparate(_ srcRGB: UInt32,_ dstRGB: UInt32,_ srcAlpha: UInt32,_ dstAlpha: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glBlendFuncSeparate(srcRGB, dstRGB, srcAlpha, dstAlpha)
    }


    public func bufferData(_ target: UInt32, size: Int,_ usage: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glBufferData(target, size, nil, usage)
    }

    public func bufferData(_ target: UInt32, srcData: NSNull,_ usage: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glBufferData(target, 0, nil, usage)
    }

    public func bufferData(_ target: UInt32, i8 srcData: [Int8], _ usage: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        glBufferData(target, buffer.count * SIZE_OF_BYTE, &buffer, usage)
    }
    
    
    
    public func bufferData(_ target: UInt32, srcData: UnsafeMutableRawPointer, size: Int, _ usage: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glBufferData(target, size , srcData, usage)
    }

    

    public func bufferData(_ target: UInt32, u8 srcData: [UInt8], _ usage: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        glBufferData(target, buffer.count * SIZE_OF_BYTE, &buffer, usage)
    }
    
    

    public func bufferData(_ target: UInt32, i16 srcData: [Int16], _ usage: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        glBufferData(target, buffer.count * SIZE_OF_SHORT, &buffer, usage)
    }

    public func bufferData(_ target: UInt32, u16 srcData: [UInt16], _ usage: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        glBufferData(target, buffer.count * SIZE_OF_SHORT, &buffer, usage)
    }


    public func bufferData(_ target: UInt32, i32 srcData: [Int32], _ usage: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        glBufferData(target, buffer.count * SIZE_OF_INT, &buffer, usage)
    }

    public func bufferData(_ target: UInt32, u32 srcData: [UInt32], _ usage: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        glBufferData(target, buffer.count * SIZE_OF_INT, &buffer, usage)
    }


    public func bufferData(_ target: UInt32, f32 srcData: [Float32], _ usage: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        glBufferData(target, buffer.count * SIZE_OF_FLOAT, &buffer, usage)
    }

    public func bufferData(_ target: UInt32, f64 srcData: [Float64], _ usage: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        glBufferData(target, buffer.count * SIZE_OF_DOUBLE, &buffer, usage)
    }


    public func bufferSubData(_ target: UInt32,_ offset: Int,_ srcData: NSNull){
        canvas.renderer.ensureIsContextIsCurrent()
        glBufferSubData(target, offset, 0, nil)
    }


    public func bufferSubData(_ target: UInt32,_ offset: Int,i8 srcData: [Int8]){
        canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        glBufferSubData(target, offset, buffer.count, &buffer)
    }
    
    
    
    public func bufferSubData(_ target: UInt32,_ offset: Int, srcData: UnsafeMutableRawPointer, size: Int){
        canvas.renderer.ensureIsContextIsCurrent()
        glBufferSubData(target, offset, size, srcData)
    }
    
    

    public func bufferSubData(_ target: UInt32,_ offset: Int,u8 srcData: [UInt8]){
        canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        glBufferSubData(target, offset, buffer.count, &buffer)
    }


    public func bufferSubData(_ target: UInt32,_ offset: Int,i16 srcData: [Int16]){
        canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        glBufferSubData(target, offset, buffer.count * SIZE_OF_SHORT, &buffer)
    }

    public func bufferSubData(_ target: UInt32,_ offset: Int,u16 srcData: [UInt16]){
        canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        glBufferSubData(target, offset, buffer.count * SIZE_OF_SHORT, &buffer)
    }

    public func bufferSubData(_ target: UInt32,_ offset: Int,i32 srcData: [Int32]){
        canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        glBufferSubData(target, offset, buffer.count * SIZE_OF_INT, &buffer)
    }

    public func bufferSubData(_ target: UInt32,_ offset: Int,u32 srcData: [UInt32]){
        canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        glBufferSubData(target, offset, buffer.count * SIZE_OF_INT, &buffer)
    }

    public func bufferSubData(_ target: UInt32,_ offset: Int,f32 srcData: [Float32]){
        canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        glBufferSubData(target, offset, buffer.count * SIZE_OF_FLOAT, &buffer)
    }

    public func bufferSubData(_ target: UInt32,_ offset: Int,f64 srcData: [Float64]){
        canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        glBufferSubData(target, offset, buffer.count * SIZE_OF_DOUBLE, &buffer)
    }


    public func checkFramebufferStatus(_ target: UInt32) -> UInt32 {
        canvas.renderer.ensureIsContextIsCurrent()
        return glCheckFramebufferStatus(target)
    }

    public func clear(_ mask: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
       if(clearIfComposited(mask) != .CombinedClear){
            glClear(mask)
        }
       canvas.doDraw()
    }

    public func clearColor(_ red: Float32,_ green: Float32,_ blue: Float32,_ alpha: Float32){
        canvas.renderer.ensureIsContextIsCurrent()
        canvas.mClearColor[0] = red
        canvas.mClearColor[1] = green
        canvas.mClearColor[2] = blue
        canvas.mClearColor[3] = alpha
        glClearColor(red, green, blue,canvas.contextAlpha ? alpha : 1)
    }

    public func clearDepth(_ depth: Float32){
        canvas.renderer.ensureIsContextIsCurrent()
        canvas.mClearDepth = depth
        glClearDepthf(depth)
    }

    public func clearStencil(_ stencil: Int32){
        canvas.renderer.ensureIsContextIsCurrent()
        canvas.mClearStencil = stencil
        glClearStencil(stencil)
    }

    func boolConverter(_ value: Bool) -> UInt8 {
        if(value){
            return UInt8(GL_TRUE)
        }
        return UInt8(GL_FALSE)
    }

    func toBool(value: Int32) -> Bool {
        return value == GL_TRUE
    }

    public func colorMask(_ red: Bool, _ green: Bool, _ blue: Bool, _ alpha: Bool){
        canvas.renderer.ensureIsContextIsCurrent()
        canvas.mColorMask[0] = red
        canvas.mColorMask[1] = green
        canvas.mColorMask[2] = blue
        canvas.mColorMask[3] = alpha
        glColorMask(boolConverter(red), boolConverter(green), boolConverter(blue), boolConverter(alpha))
    }

    func reset(){
        glDisable(GLenum(GL_SCISSOR_TEST))
        glClearColor(0, 0, 0, canvas.contextAlpha ? 0 : 1)
        glColorMask(GLboolean(GL_TRUE), GLboolean(GL_TRUE), GLboolean(GL_TRUE), GLboolean(GL_TRUE))
        var clearMask = GL_COLOR_BUFFER_BIT
        if (canvas.contextDepth) {
            glClearDepthf(1)
            clearMask |= GL_DEPTH_BUFFER_BIT
            glDepthMask(GLboolean(GL_TRUE))
        }
        if (canvas.contextStencil) {
            glClearStencil(0)
            clearMask |= GL_STENCIL_BUFFER_BIT
            glStencilMaskSeparate(GLenum(GL_FRONT), 0xFFFFFFFF)
        }
    }



    func restoreStateAfterClear()
    {
        // Restore the state that the context set.
        if (canvas.mScissorEnabled){
            glEnable(GLenum(GL_SCISSOR_TEST))
        }
        glClearColor(canvas.mClearColor[0], canvas.mClearColor[1], canvas.mClearColor[2], canvas.contextAlpha ? canvas.mClearColor[3] : 1)
        glColorMask(boolConverter(canvas.mColorMask[0]), boolConverter(canvas.mColorMask[1]),
                    boolConverter(canvas.mColorMask[2]), boolConverter(canvas.mColorMask[3]))
        glClearDepthf(canvas.mClearDepth)
        glClearStencil(canvas.mClearStencil)
        glStencilMaskSeparate(GLenum(GL_FRONT), canvas.mStencilMask)
        glDepthMask(boolConverter(canvas.mDepthMask))
    }

    @discardableResult func clearIfComposited(_ mask: GLbitfield = 0) -> HowToClear {
        let combinedClear = (mask > 0) && !canvas.mScissorEnabled
        let m = (mask & UInt32(GL_COLOR_BUFFER_BIT))
        glDisable(GLenum(GL_SCISSOR_TEST))
        if (combinedClear && (m == UInt32(GL_COLOR_BUFFER_BIT)) ) {
            let alphaValue = canvas.mColorMask[3] ?  canvas.mClearColor[3] :  0
            glClearColor(canvas.mColorMask[0] ? canvas.mClearColor[0] : 0,
                         canvas.mColorMask[1] ? canvas.mClearColor[1] : 0,
                         canvas.mColorMask[2] ? canvas.mClearColor[2] : 0,
                         canvas.contextAlpha ? alphaValue : 1)
        } else {
            glClearColor(0, 0, 0, canvas.contextAlpha ? 0 : 1)
        }


        glColorMask(GLboolean(GL_TRUE),GLboolean(GL_TRUE),GLboolean(GL_TRUE),GLboolean(GL_TRUE))
        var clearMask = GL_COLOR_BUFFER_BIT
        if (canvas.contextDepth) {
            if (!combinedClear || !canvas.mDepthMask || (mask & UInt32(GL_DEPTH_BUFFER_BIT)) == 0){
                glClearDepthf(1.0);
                clearMask |= GL_DEPTH_BUFFER_BIT
                glDepthMask(GLboolean(GL_TRUE))
            }
        }
        if (canvas.contextStencil) {
            if (combinedClear && (mask & UInt32(GL_STENCIL_BUFFER_BIT)) != 0){
                glClearStencil(GLint(UInt32(canvas.mClearStencil) & canvas.mStencilMask))
            }else{
                glClearStencil(0)
                clearMask |= GL_STENCIL_BUFFER_BIT;
                glStencilMaskSeparate(GLenum(GL_FRONT), 0xFFFFFFFF)}
        }
        //mask
        glClear(GLbitfield(mask))
        restoreStateAfterClear()
        return combinedClear ? .CombinedClear : .JustClear

    }

    public func commit(){
        // NOOP
    }

    public func compileShader(_ shader: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glCompileShader(shader)
    }


    public func compressedTexImage2D(_ target: UInt32,_ level: Int32,_ internalformat: UInt32,_ width: Int32,_ height: Int32,_ border: Int32, _ pixels: Data?){
        canvas.renderer.ensureIsContextIsCurrent()
        var data = pixels
        glCompressedTexImage2D(target, level, internalformat, width, height, border, GLsizei(pixels?.count ?? 0), &data)
    }


    public func compressedTexSubImage2D(_ target: UInt32,_ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ width: Int32, _ height: Int32, _ format: UInt32, _ pixels: Data?){
        canvas.renderer.ensureIsContextIsCurrent()
        var data = pixels
        glCompressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, GLsizei(data?.count ?? 0), &data)
    }



    public func copyTexImage2D(_ target: UInt32,_ level: Int32,_ internalformat: UInt32, _ x: Int32,_ y: Int32,_ width: Int32,_ height: Int32,_ border: Int32) {
        canvas.renderer.ensureIsContextIsCurrent()
        clearIfComposited()
        glCopyTexImage2D(target, level, internalformat, x, y, width, height, border)
    }


    public func copyTexSubImage2D(_ target: UInt32, _ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ x: Int32,_ y: Int32,_ width: Int32, _ height: Int32){
        canvas.renderer.ensureIsContextIsCurrent()
        clearIfComposited()
        glCopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height)
    }

    public func createBuffer() -> UInt32 {
        canvas.renderer.ensureIsContextIsCurrent()
        var bufferId = GLuint()
        glGenBuffers(1, &bufferId)
        return bufferId
    }

    public func createFramebuffer() -> UInt32 {
        canvas.renderer.ensureIsContextIsCurrent()
        var frameBufferId = GLuint()
        glGenFramebuffers(1, &frameBufferId)
        return frameBufferId
    }

    public func createProgram() -> UInt32 {
        canvas.renderer.ensureIsContextIsCurrent()
        return glCreateProgram()
    }

    public func createRenderbuffer() -> UInt32 {
        canvas.renderer.ensureIsContextIsCurrent()
        var renderBufferId = GLuint()
        glGenRenderbuffers(1, &renderBufferId)
        return renderBufferId
    }


    public func createShader(_ type: UInt32) -> UInt32{
        canvas.renderer.ensureIsContextIsCurrent()
        return glCreateShader(type)
    }

    public func createTexture() -> UInt32 {
        canvas.renderer.ensureIsContextIsCurrent()
        var textureId = GLuint()
        glGenTextures(1, &textureId)
        return textureId
    }

    public func cullFace(_ mode: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glCullFace(mode)
    }

    public func deleteBuffer(_ buffer: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        var id = buffer
        glDeleteBuffers(1, &id)
    }

    public func deleteFramebuffer(_ frameBuffer: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        var id = frameBuffer
        glDeleteFramebuffers(1, &id)
    }

    public func deleteProgram(_ program: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glDeleteProgram(program)
    }

    public func deleteRenderbuffer(_ renderbuffer: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        var id = renderbuffer
        glDeleteRenderbuffers(1, &id)
    }
    public func deleteShader(_ shader: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glDeleteShader(shader)
    }
    public func deleteTexture(_ texture: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        var id = texture
        glDeleteTextures(1, &id)
    }
    public func depthFunc(_ fn: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glDepthFunc(fn)
    }
    public func depthMask(_ flag: Bool){
        canvas.renderer.ensureIsContextIsCurrent()
        glDepthMask(boolConverter(flag))
    }
    public func depthRange(_ zNear: Float32,_ zFar: Float32){
        canvas.renderer.ensureIsContextIsCurrent()
        glDepthRangef(zNear, zFar)
    }
    public func detachShader(_ program: UInt32,_ shader: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glDetachShader(program, shader)
    }
    public func disable(_ cap: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glDisable(cap)
    }
    public func disableVertexAttribArray(_ index: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glDisableVertexAttribArray(index)
    }
    public func drawArrays(_ mode: UInt32,_ first: Int32,_ count: Int32){
        canvas.renderer.ensureIsContextIsCurrent()
        clearIfComposited()
        glDrawArrays(mode, first, count)
        canvas.doDraw()
    }

    func BUFFER_OFFSET(n: Int) -> UnsafeRawPointer? {
        return UnsafeRawPointer(bitPattern: n)
    }

    func BUFFER_OFFSET_MUTABLE(n: Int) -> UnsafeMutableRawPointer? {
        return UnsafeMutableRawPointer(bitPattern: n)
    }

    public func drawElements(_ mode: UInt32,_ count: Int32,_ type: UInt32,_ offset: Int) {
        canvas.renderer.ensureIsContextIsCurrent()
        let ptr = BUFFER_OFFSET(n: offset)
        clearIfComposited()
        glDrawElements(mode, count, type, ptr)
        canvas.doDraw()
    }

    public func enable(_ cap: UInt32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glEnable(cap)
    }

    public func enableVertexAttribArray(_ index: UInt32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glEnableVertexAttribArray(index)
    }

    public func finish(){
        canvas.renderer.ensureIsContextIsCurrent()
        glFinish()
    }

    public func flush(){
        canvas.renderer.ensureIsContextIsCurrent()
        glFlush()
    }


    public func framebufferRenderbuffer(_ target: UInt32,_ attachment: UInt32,_ renderbuffertarget: UInt32,_ renderbuffer: UInt32) {
        canvas.renderer.ensureIsContextIsCurrent()
        /*if(attachment == GL_DEPTH_ATTACHMENT){
         if let renderer = canvas.renderer as? GLRenderer {
         glFramebufferRenderbuffer(GLenum(target), GLenum(GL_DEPTH_ATTACHMENT), GLenum(renderbuffertarget),renderer.displayRenderbuffer)

         glFramebufferRenderbuffer(GLenum(target), GLenum(GL_DEPTH_ATTACHMENT), GLenum(renderbuffertarget), renderbuffer)

         glFramebufferRenderbuffer(GLenum(target), GLenum(GL_STENCIL_ATTACHMENT), GLenum(renderbuffertarget), renderer.displayRenderbuffer)
         }
         }else if(attachment == GL_STENCIL_ATTACHMENT){
         if let renderer = canvas.renderer as? GLRenderer {
         glFramebufferRenderbuffer(GLenum(target), GLenum(GL_DEPTH_STENCIL_ATTACHMENT), GLenum(GL_RENDERBUFFER),renderer.displayRenderbuffer)

         glFramebufferRenderbuffer(GLenum(target), GLenum(GL_DEPTH_ATTACHMENT), GLenum(GL_RENDERBUFFER), renderer.displayRenderbuffer)


         glFramebufferRenderbuffer(GLenum(target), GLenum(GL_STENCIL_ATTACHMENT), GLenum(GL_RENDERBUFFER), renderbuffer)

         }
         }else if(attachment == GL_DEPTH_STENCIL_ATTACHMENT){
         if let renderer = canvas.renderer as? GLRenderer {
         glFramebufferRenderbuffer(GLenum(target), GLenum(GL_DEPTH_ATTACHMENT), GLenum(GL_RENDERBUFFER),renderer.displayRenderbuffer)


         glFramebufferRenderbuffer(GLenum(target), GLenum(GL_STENCIL_ATTACHMENT), GLenum(GL_RENDERBUFFER), renderer.displayRenderbuffer)

         glFramebufferRenderbuffer(GLenum(target), GLenum(GL_DEPTH_STENCIL_ATTACHMENT), GLenum(GL_RENDERBUFFER), renderbuffer)


         }
         }else {
         /*
         glFramebufferRenderbuffer(GLenum(target), GLenum(attachment), GLenum(renderbuffertarget), renderbuffer)
         */
         if let renderer = canvas.renderer as? GLRenderer {
         glFramebufferRenderbuffer(GLenum(target), GLenum(GL_DEPTH_STENCIL_ATTACHMENT), GLenum(GL_RENDERBUFFER), renderer.displayRenderbuffer)

         glFramebufferRenderbuffer(GLenum(target), GLenum(GL_DEPTH_ATTACHMENT), GLenum(GL_RENDERBUFFER), renderer.displayRenderbuffer)

         glFramebufferRenderbuffer(GLenum(target), GLenum(GL_STENCIL_ATTACHMENT), GLenum(GL_RENDERBUFFER), renderer.displayRenderbuffer)
         }
         }
         */

        glFramebufferRenderbuffer(target, attachment, renderbuffertarget,renderbuffer)
    }

    public func framebufferTexture2D(_ target: UInt32,_ attachment:UInt32,_ textarget: UInt32,_ texture: UInt32,_ level: Int32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glFramebufferTexture2D(target, attachment, textarget, texture, level)
    }

    public func frontFace(_ mode: UInt32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glFrontFace(mode)
    }

    public func generateMipmap(_ target: UInt32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glGenerateMipmap(target)
    }

    public func getActiveAttrib(_ program: UInt32,_ index: UInt32) -> TNSWebGLActiveInfo{
        canvas.renderer.ensureIsContextIsCurrent()
        var length = GLint()
        var size = GLint()
        var type = GLenum()
        let zero = GLchar()
        var nameLength = GLint()
        glGetProgramiv(program,GLenum(GL_ACTIVE_ATTRIBUTE_MAX_LENGTH), &length)
        var name = Array(repeatElement(zero, count: Int(length)))
        glGetActiveAttrib(program, index, length, &nameLength, &size, &type, &name)
        name.resize(Int(nameLength), fillWith: 0)
        return TNSWebGLActiveInfo(name: String(cString: &name), size: size, type: type)
    }


    public func getActiveUniform(_ program: UInt32,_ index: UInt32) -> TNSWebGLActiveInfo{
        canvas.renderer.ensureIsContextIsCurrent()
        var size = GLint()
        var type = GLenum()
        var length = GLint()
        var nameLength = GLint()
        let zero = GLchar()
        glGetProgramiv(program, GLenum(GL_ACTIVE_UNIFORM_MAX_LENGTH), &length)
        var name = Array(repeatElement(zero, count: Int(length)))
        glGetActiveUniform(program, index, length , &nameLength, &size, &type, &name)
        name.resize(Int(nameLength), fillWith: 0)
        return TNSWebGLActiveInfo(name: String(cString: &name), size: size, type: type)
    }

    public func getAttachedShaders(_ program: UInt32) -> [UInt32]{
        canvas.renderer.ensureIsContextIsCurrent()
        var count = GLint()
        let zero = GLuint()
        glGetProgramiv(program, GLenum(GL_ATTACHED_SHADERS) , &count)
        var shaders = Array(repeating: zero, count: Int(count))
        glGetAttachedShaders(program, count, nil, &shaders)
        return shaders
    }

    public func getAttribLocation(_ program: UInt32,_ name: String) -> Int32 {
        canvas.renderer.ensureIsContextIsCurrent()
        let ptr = (name as NSString).cString(using: String.Encoding.utf8.rawValue)
        return glGetAttribLocation(program, ptr)
    }

    public func getBufferParameter(_ target: UInt32,_ pname: UInt32) -> Int32 {
        canvas.renderer.ensureIsContextIsCurrent()
        var params = GLint()
        glGetBufferParameteriv(target, pname, &params)
        return params
    }


    public func getContextAttributes() -> Any {
        // Return nil if context is lost
        if(isContextLost()){
            return NSNull()
        }
        return [
            "alpha": canvas.contextAlpha,
            "antialias": canvas.contextAntialias,
            "depth": canvas.contextDepth,
            "failIfMajorPerformanceCaveat": canvas.contextFailIfMajorPerformanceCaveat,
            "powerPreference": canvas.contextPowerPreference,
            "premultipliedAlpha": canvas.contextPowerPreference,
            "preserveDrawingBuffer": canvas.contextPreserveDrawingBuffer,
            "stencil": canvas.contextStencil,
            "desynchronized": canvas.contextDesynchronized,
            "xrCompatible": canvas.contextXrCompatible
        ]
    }

    public func getError() -> UInt32 {
        canvas.renderer.ensureIsContextIsCurrent()
        return glGetError()
    }

    private func getRealExtName(name: String) -> String {
        if(name.starts(with: "WEBGL_")){
            return name
        }
        return "GL_" + name
    }

    private func toUpperCase(name: String) -> String {
        return name.uppercased()
    }
    public func getExtension(_ name: String) -> Any? {
        canvas.renderer.ensureIsContextIsCurrent()
        let realName = getRealExtName(name: name)
        if let extPtr = glGetString(GLenum(GL_EXTENSIONS)) {
            let extensions = String(cString: extPtr)
            if(extensions.isEmpty){
                return NSNull()
            }
            if(name.elementsEqual("WEBGL_compressed_texture_etc1") && extensions.contains("GL_IMG_texture_compression_pvrtc")){
                return TNS_WEBGL_compressed_texture_pvrtc()
            }else if(name.elementsEqual("WEBGL_compressed_texture_etc1")){
                return TNS_WEBGL_compressed_texture_etc1()
            }
            if(canvas.renderer.glContext.api == .openGLES3){
                switch name {
                case "EXT_blend_minmax":
                    return TNS_EXT_blend_minmax()
                case "WEBGL_compressed_texture_etc":
                    return TNS_WEBGL_compressed_texture_etc()
                case "WEBGL_depth_texture":
                    return TNS_WEBGL_depth_texture()
                case "WEBGL_color_buffer_float":
                    return TNS_WEBGL_color_buffer_float()
                case "WEBGL_lose_context":
                    return TNS_WEBGL_lose_context(canvas: self.canvas)
                case "OES_texture_half_float":
                    return TNS_OES_texture_half_float()
                case "OES_texture_half_float_linear":
                    return TNS_OES_texture_half_float_linear()
                case "OES_texture_float":
                    //EXT_color_buffer_half_float
                    return TNS_OES_texture_float()
                case "OES_element_index_uint":
                    return TNS_OES_element_index_uint()
                case "OES_fbo_render_mipmap":
                    return TNS_OES_fbo_render_mipmap()
                case "OES_standard_derivatives":
                    return TNS_OES_standard_derivatives()
                case "OES_texture_float_linear":
                    return TNS_OES_texture_float_linear()
                case "OES_depth_texture":
                    return TNS_WEBGL_depth_texture()
                case "WEBGL_draw_buffers":
                    return TNS_WEBGL_draw_buffers()
                default: break
                }
            }
            if(name.elementsEqual("ANGLE_instanced_arrays")){
                return TNS_ANGLE_instanced_arrays(context: self)
            }
            if(extensions.contains(realName)){
                switch (realName){
                case getRealExtName(name: "EXT_blend_minmax"):
                    return TNS_EXT_blend_minmax()
                case getRealExtName(name: "EXT_color_buffer_float"):
                    return TNS_EXT_color_buffer_float()
                case getRealExtName(name: "EXT_color_buffer_half_float"):
                    return TNS_EXT_color_buffer_half_float()
                case getRealExtName(name: "EXT_sRGB"):
                    return TNS_EXT_sRGB()
                case getRealExtName(name: "EXT_shader_texture_lod"):
                    return TNS_EXT_shader_texture_lod()
                case getRealExtName(name: "EXT_texture_filter_anisotropic"):
                    return TNS_EXT_texture_filter_anisotropic()
                case getRealExtName(name: "OES_element_index_uint"):
                    return TNS_OES_element_index_uint()
                case getRealExtName(name: "EXT_texture_filter_anisotropic"):
                    return TNS_EXT_texture_filter_anisotropic()
                case getRealExtName(name: "OES_fbo_render_mipmap"):
                    return TNS_OES_fbo_render_mipmap()
                case getRealExtName(name: "OES_standard_derivatives"):
                    return TNS_OES_standard_derivatives()
                case getRealExtName(name: "OES_texture_float_linear"):
                    return TNS_OES_texture_float_linear()
                case getRealExtName(name: "OES_texture_half_float"):
                    return TNS_OES_texture_half_float()
                case getRealExtName(name: "OES_texture_half_float_linear"):
                    return TNS_OES_texture_half_float_linear()
                case getRealExtName(name: "OES_depth_texture"):
                    return TNS_WEBGL_depth_texture()
                    // N/A
                    //EXT_float_blend
                    //EXT_frag_depth
                    //EXT_texture_compression_bptc
                    //EXT_texture_compression_rgtc
                    //OVR_multiview2
                    //WEBGL_compressed_texture_astc
                    //WEBGL_compressed_texture_atc
                    //WEBGL_compressed_texture_s3tc
                    //WEBGL_compressed_texture_s3tc_srgb
                    //WEBGL_debug_renderer_info
                //EBGL_debug_shaders
                default:
                    return NSNull()
                }
            }
            return NSNull()
        }
        return NSNull()
    }

    public func getFramebufferAttachmentParameter(_ target: UInt32, _ attachment: UInt32,_ pname: UInt32) -> TNSFramebufferAttachmentParameter {
        canvas.renderer.ensureIsContextIsCurrent()
        var params = GLint()
        let result = TNSFramebufferAttachmentParameter()
        if(attachment == FRAMEBUFFER_ATTACHMENT_OBJECT_NAME){
            glGetFramebufferAttachmentParameteriv(target, attachment, pname, &params)
            var name = GLint()
            glGetFramebufferAttachmentParameteriv(target,attachment, GLenum(GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE), &name)
            switch name {
            case GL_RENDERBUFFER:
                result._isRenderbuffer = true
                result._value = params
            case GL_TEXTURE:
                result._isTexture = true
                result._value = params
            default:
                result._value = params
            }
        }else {
            glGetFramebufferAttachmentParameteriv(target, attachment, pname, &params)
            result._value = params
        }
        return result
    }


    func fromGLboolean(value: UInt8) -> Bool{
        return value == GL_TRUE
    }

    func fromGLbooleanArray(value: [UInt8]) -> [Bool]{
        return value.map { val -> Bool in
            return val == GL_TRUE
        }
    }
    func fromGLint(value: [Int32]) -> [Bool]{
        return value.map { val -> Bool in
            return val == GL_TRUE
        }
    }
    public func getParameter(_ pname: UInt32) -> Any?{
        canvas.renderer.ensureIsContextIsCurrent()
        switch pname {
        case ACTIVE_TEXTURE, ALPHA_BITS, ARRAY_BUFFER_BINDING, BLEND_DST_ALPHA, BLEND_DST_RGB, BLEND_EQUATION, BLEND_EQUATION_ALPHA, BLEND_EQUATION_RGB, BLEND_SRC_ALPHA, BLEND_SRC_RGB, BLUE_BITS, CULL_FACE_MODE, CURRENT_PROGRAM, DEPTH_BITS, DEPTH_FUNC, ELEMENT_ARRAY_BUFFER_BINDING, FRAMEBUFFER_BINDING, FRONT_FACE, GENERATE_MIPMAP_HINT, GREEN_BITS, IMPLEMENTATION_COLOR_READ_FORMAT, IMPLEMENTATION_COLOR_READ_TYPE, MAX_COMBINED_TEXTURE_IMAGE_UNITS, MAX_CUBE_MAP_TEXTURE_SIZE, MAX_FRAGMENT_UNIFORM_VECTORS, MAX_RENDERBUFFER_SIZE, MAX_TEXTURE_IMAGE_UNITS, MAX_TEXTURE_SIZE,MAX_VARYING_VECTORS, MAX_VERTEX_ATTRIBS, MAX_VERTEX_TEXTURE_IMAGE_UNITS, MAX_VERTEX_UNIFORM_VECTORS, PACK_ALIGNMENT, RED_BITS, RENDERBUFFER_BINDING, SAMPLE_BUFFERS, SAMPLES, STENCIL_BACK_FAIL, STENCIL_BACK_FUNC, STENCIL_BACK_PASS_DEPTH_FAIL, STENCIL_BACK_PASS_DEPTH_PASS, STENCIL_BACK_REF, STENCIL_BACK_VALUE_MASK, STENCIL_BACK_WRITEMASK, STENCIL_BITS,STENCIL_CLEAR_VALUE,STENCIL_FAIL, STENCIL_FUNC,STENCIL_PASS_DEPTH_FAIL,STENCIL_PASS_DEPTH_PASS,STENCIL_REF,STENCIL_VALUE_MASK, STENCIL_WRITEMASK, SUBPIXEL_BITS, TEXTURE_BINDING_2D, TEXTURE_BINDING_CUBE_MAP, UNPACK_ALIGNMENT:
            var param = GLint()
            glGetIntegerv(GLenum(pname), &param)
            
            if((pname == CURRENT_PROGRAM || pname == ARRAY_BUFFER_BINDING || pname == ELEMENT_ARRAY_BUFFER_BINDING || pname == TEXTURE_BINDING_2D || pname == TEXTURE_BINDING_CUBE_MAP || pname == RENDERBUFFER_BINDING || pname == FRAMEBUFFER_BINDING) && param == 0){
                return nil
            }
            return param
        case ALIASED_LINE_WIDTH_RANGE, ALIASED_POINT_SIZE_RANGE, DEPTH_RANGE:
            var param = Array(repeating: GLfloat(), count: 2)
            glGetFloatv(GLenum(pname), &param)
            return param
        case BLEND_COLOR, COLOR_CLEAR_VALUE:
            var param = Array(repeating: GLfloat(), count: 4)
            glGetFloatv(GLenum(pname), &param)
            return param
        case BLEND, CULL_FACE, DEPTH_TEST, DEPTH_WRITEMASK, DITHER, POLYGON_OFFSET_FILL, SAMPLE_COVERAGE_INVERT, SCISSOR_TEST, STENCIL_TEST:
            var param = GLboolean()
            glGetBooleanv(GLenum(pname), &param)
            return  fromGLboolean(value: param)
        case UNPACK_PREMULTIPLY_ALPHA_WEBGL:
            return premultiplyAlphaWebGL
        case UNPACK_FLIP_Y_WEBGL:
            return flipYWebGL
        case UNPACK_COLORSPACE_CONVERSION_WEBGL:
            var cs = colorSpaceConversionWebGL
            if(cs == 0){
                cs = BROWSER_DEFAULT_WEBGL
            }
            return cs
        case COLOR_WRITEMASK:
            var param = Array(repeating: GLboolean(), count: 4)
            glGetBooleanv(GLenum(pname), &param)
            return fromGLbooleanArray(value: param)
        case COMPRESSED_TEXTURE_FORMATS:
            var count = GLint()
            glGetIntegerv(GLenum(GL_NUM_COMPRESSED_TEXTURE_FORMATS), &count)
            var params = Array(repeating: GLint(), count: Int(count))
            glGetIntegerv(GLenum(GL_COMPRESSED_TEXTURE_FORMATS), &params)
            return params
        case DEPTH_CLEAR_VALUE, LINE_WIDTH, POLYGON_OFFSET_FACTOR, POLYGON_OFFSET_UNITS, SAMPLE_COVERAGE_VALUE:
            var param = GLfloat()
            glGetFloatv(GLenum(pname), &param)
            return param
        case MAX_VIEWPORT_DIMS:
            var params = Array(repeating: GLint(), count: 2)
            glGetIntegerv(GLenum(pname), &params)
            return params
        case SCISSOR_BOX, VIEWPORT:
            var params = Array(repeating: GLint(), count: 4)
            glGetIntegerv(GLenum(pname), &params)
            return params
        case RENDERER, SHADING_LANGUAGE_VERSION, VENDOR, VERSION:
            let params = glGetString(GLenum(pname))
            if(params == nil){
                return nil
            }
            return String(cString: params!)
        default:
            return nil
        }
    }

    public func getProgramInfoLog(_ program: UInt32) -> String {
        canvas.renderer.ensureIsContextIsCurrent()
        var length = GLint()
        glGetProgramiv(program, GLenum(GL_INFO_LOG_LENGTH), &length)
        var info: [GLchar] = Array(repeatElement(0, count: Int(length)))
        glGetProgramInfoLog(program, length, nil, &info)
        return (NSString(cString: &info, encoding: String.Encoding.utf8.rawValue) ?? "") as String
    }


    public func getProgramParameter(_ program: UInt32,_ pname: UInt32) -> Any {
        canvas.renderer.ensureIsContextIsCurrent()
        var param = GLint()
        glGetProgramiv(program, pname, &param)
        switch pname {
        case DELETE_STATUS:
            if(param == GL_TRUE){
                return true
            }
            return false
        case LINK_STATUS:
            if(param == GL_TRUE){
                return true
            }
            return false
        case VALIDATE_STATUS:
            if(param == GL_TRUE){
                return true
            }
            return false
        default:
            return param
        }
    }


    public func getRenderbufferParameter(_ target: UInt32, _ pname: UInt32) -> Int32{
        canvas.renderer.ensureIsContextIsCurrent()
        var params = GLint()
        glGetRenderbufferParameteriv(target, pname, &params)
        return params
    }

    public func getShaderInfoLog(_ shader: UInt32) -> String  {
        canvas.renderer.ensureIsContextIsCurrent()
        var length = GLint()
        glGetShaderiv(shader, GLenum(GL_INFO_LOG_LENGTH), &length)
        var info: [GLchar] = Array(repeatElement(0, count: Int(length)))
        glGetShaderInfoLog(shader, length, nil, &info)
        return (NSString(cString: &info, encoding: String.Encoding.utf8.rawValue) ?? "") as String
    }

    public func getShaderParameter(_ shader: UInt32,_ pname: UInt32) -> Any {
        canvas.renderer.ensureIsContextIsCurrent()
        var params = GLint()
        glGetShaderiv(shader, pname, &params)
        switch pname {
        case DELETE_STATUS:
            if(params == GL_TRUE){
                return true
            }
            return false
        case COMPILE_STATUS:
            if(params == GL_TRUE){
                return true
            }
            return false
        default:
            return params
        }
    }

    public func getShaderPrecisionFormat(_ shaderType: UInt32, _ precisionType: UInt32) -> TNSWebGLShaderPrecisionFormat {
        canvas.renderer.ensureIsContextIsCurrent()
        var range: [GLint] = Array(repeating: 0, count: 2)
        var precision = GLint()
        glGetShaderPrecisionFormat(shaderType, precisionType, &range, &precision)
        return TNSWebGLShaderPrecisionFormat(rangeMin: range[0], rangeMax: range[1], precision: precision)
    }

    public func getShaderSource(_ shader: UInt32) -> String {
        canvas.renderer.ensureIsContextIsCurrent()
        var length = GLint()
        glGetShaderiv(shader, GLenum(GL_SHADER_SOURCE_LENGTH), &length)
        var source:[GLchar] = Array(repeating: 0, count: Int(length))
        glGetShaderSource(shader, length, nil, &source)
        return String(cString: source)
    }

    public func getSupportedExtensions() -> [String]{
        canvas.renderer.ensureIsContextIsCurrent()
        let extensions = String(cString: glGetString(GLenum(GL_EXTENSIONS)))
        var list = extensions.components(separatedBy: .whitespaces)
        if let last = list.last {
            if(last.isEmpty){
                let _ = list.popLast()
            }
        }

        list.append("EXT_blend_minmax")
        list.append("EXT_color_buffer_float")
        list.append("EXT_color_buffer_half_float")
        list.append("EXT_sRGB")
        list.append("EXT_shader_texture_lod")
        list.append("EXT_texture_filter_anisotropic")
        list.append("OES_element_index_uint")
        list.append("OES_fbo_render_mipmap")
        list.append("OES_standard_derivatives")
        list.append("OES_texture_float")
        list.append("OES_texture_float_linear")
        list.append("OES_texture_half_float")
        list.append("OES_texture_half_float_linear")
        list.append("WEBGL_color_buffer_float")
        list.append("WEBGL_compressed_texture_etc")
        list.append("WEBGL_compressed_texture_etc1")
        list.append("WEBGL_compressed_texture_pvrtc")
        list.append("WEBGL_depth_texture")
        list.append("WEBGL_lose_context")

        return list
    }


    public func getTexParameter(_ target: UInt32, _ pname: UInt32) -> Int32{
        canvas.renderer.ensureIsContextIsCurrent()
        var params = GLint()
        glGetTexParameteriv(target, pname, &params)
        return params
    }

    func getFloatSlice(_ count: Int) -> [Float]{
        return Array(repeating: 0, count: count)
    }

    func getIntSlice(_ count: Int) -> [Int32] {
        return Array(repeating: 0, count: count)
    }

    public func getUniform(_ program: UInt32,_ location: Int32) -> Any {
        canvas.renderer.ensureIsContextIsCurrent()
        var type = GLuint()
        glGetActiveUniform(program, GLuint(location), 0, nil, nil, &type, nil)
        switch Int32(type) {
        case GL_FLOAT:
            var single = getFloatSlice(1)
            glGetUniformfv(program, location, &single)
            return single[0]
        case GL_FLOAT_VEC2:
            var vec2 = getFloatSlice(2)
            glGetUniformfv(program, location, &vec2)
            return vec2
        case GL_FLOAT_VEC3:
            var vec3 = getFloatSlice(3)
            glGetUniformfv(program, location, &vec3)
            return vec3
        case GL_FLOAT_VEC4:
            var vec4 = getFloatSlice(4)
            glGetUniformfv(program, location, &vec4)
            return vec4
        case GL_INT, GL_SAMPLER_2D, GL_SAMPLER_CUBE:
            var singleInt = getIntSlice(1)
            glGetUniformiv(program, location, &singleInt)
            return singleInt[0]
        case GL_INT_VEC2:
            var intVec2 = getIntSlice(2)
            glGetUniformiv(program, location, &intVec2)
            return intVec2
        case GL_INT_VEC3:
            var intVec3 = getIntSlice(3)
            glGetUniformiv(program, location, &intVec3)
            return  intVec3
        case GL_INT_VEC4:
            var intVec4 = getIntSlice(4)
            glGetUniformiv(program, location, &intVec4)
            return  intVec4
        case GL_BOOL:
            var singleBool = getIntSlice(1)
            glGetUniformiv(program, location, &singleBool)
            return singleBool[0] == GL_TRUE
        case GL_BOOL_VEC2:
            var boolVec2 = getIntSlice(2)
            var boolVec2Result: [Bool] = []
            glGetUniformiv(program, location, &boolVec2)
            boolVec2Result.append(boolVec2[0] == GL_TRUE)
            boolVec2Result.append(boolVec2[1] == GL_TRUE)
            return boolVec2Result
        case GL_BOOL_VEC3:
            var boolVec3 = getIntSlice(3);
            var boolVec3Result: [Bool] = []
            glGetUniformiv(program, location, &boolVec3)
            boolVec3Result.append(boolVec3[0] == GL_TRUE)
            boolVec3Result.append(boolVec3[1] == GL_TRUE)
            boolVec3Result.append(boolVec3[2] == GL_TRUE)
            return boolVec3Result
        case GL_BOOL_VEC4:
            var boolVec4 = getIntSlice(4)
            var boolVec4Result: [Bool] = []
            glGetUniformiv(program, location, &boolVec4)
            boolVec4Result.append(boolVec4[0] == GL_TRUE)
            boolVec4Result.append(boolVec4[1] == GL_TRUE)
            boolVec4Result.append(boolVec4[2] == GL_TRUE)
            boolVec4Result.append(boolVec4[3] == GL_TRUE)
            return boolVec4Result
        case GL_FLOAT_MAT2:
            var mat2 = getFloatSlice(2)
            glGetUniformfv(program, location, &mat2)
            return mat2
        case GL_FLOAT_MAT3:
            var mat3 = getFloatSlice(3)
            glGetUniformfv(program, location, &mat3)
            return mat3
        case GL_FLOAT_MAT4:
            var mat4 = getFloatSlice(4)
            glGetUniformfv(program, location, &mat4)
            return mat4
        default:
            return NSNull()
        }

    }

    public func getUniformLocation(_ program: UInt32,_ name: String) -> Int32 {
        canvas.renderer.ensureIsContextIsCurrent()
        let namePtr = (name as NSString).cString(using: String.Encoding.utf8.rawValue)
        return glGetUniformLocation(program, namePtr)
    }



    public func getVertexAttrib(_ index: UInt32, _ pname: UInt32) -> Any {
        canvas.renderer.ensureIsContextIsCurrent()
        if(pname == CURRENT_VERTEX_ATTRIB){
            var params: [GLfloat] = Array(repeating: 0, count: 4)
            glGetVertexAttribfv(index, pname, &params)
            return params
        }
        var params = GLint()
        glGetVertexAttribiv(index, GLenum(pname), &params)
        switch pname {
        case VERTEX_ATTRIB_ARRAY_ENABLED:
            if(params == GL_TRUE){
                return true
            }
            return false
        case VERTEX_ATTRIB_ARRAY_NORMALIZED:
            if(params == GL_TRUE){
                return true
            }
            return false
        default:
            return params
        }
    }


    public func getVertexAttribOffset(_ index: UInt32,_ pname: UInt32) -> Int {
        canvas.renderer.ensureIsContextIsCurrent()
       return Int(gl_get_vertex_attrib_offset(index, pname))
    }


    public func hint(_ target: UInt32,_ mode: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glHint(target, mode)
    }


    public func isBuffer(_ buffer: UInt32) -> Bool{
        canvas.renderer.ensureIsContextIsCurrent()
        return glIsBuffer(buffer) == GL_TRUE
    }

    public func isContextLost() -> Bool {
        return canvas.isContextLost
    }


    public func isEnabled(_ cap: UInt32) -> Bool {
        canvas.renderer.ensureIsContextIsCurrent()
        return glIsEnabled(cap) == GL_TRUE
    }


    public func isFramebuffer(_ framebuffer: UInt32) -> Bool {
        canvas.renderer.ensureIsContextIsCurrent()
        return glIsFramebuffer(framebuffer) == GL_TRUE
    }


    public func isProgram(_ program: UInt32) -> Bool {
        canvas.renderer.ensureIsContextIsCurrent()
        return glIsProgram(program) == GL_TRUE
    }

    public func isRenderbuffer(_ renderbuffer: UInt32) -> Bool {
        canvas.renderer.ensureIsContextIsCurrent()
        return glIsRenderbuffer(renderbuffer) == GL_TRUE
    }

    public func isShader(_ shader: UInt32) ->Bool {
        canvas.renderer.ensureIsContextIsCurrent()
        return glIsShader(shader) == GL_TRUE
    }

    public func isTexture(_ texture: UInt32) -> Bool {
        canvas.renderer.ensureIsContextIsCurrent()
        return glIsTexture(texture)  == GL_TRUE
    }

    public func lineWidth(_ width: Float32){
        canvas.renderer.ensureIsContextIsCurrent()
        glLineWidth(width)
    }

    public func linkProgram(_ program: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glLinkProgram(program)
    }



    private func anyToInt(_ value: Any?, _ defaultValue: Int32) -> Int32 {
        if (value != nil) {
            if let intVal = value as? Int32 {
                return intVal
            }
            return defaultValue
        }
        return defaultValue
    }

    private func anyToBoolean(_ value: Any?, _ defaultValue: Bool) -> Bool {
        if (value != nil) {
            if let boolVal = value as? Bool {
                return boolVal
            }
            return defaultValue
        }
        return defaultValue
    }

    private func anyToColorSpace(_ value: Any?, _ defaultValue: UInt32) -> UInt32 {
        if (value != nil) {
            if let intVal = value as? UInt32 {
                if(intVal == BROWSER_DEFAULT_WEBGL || intVal == GL_NONE){
                    return intVal
                }
                return BROWSER_DEFAULT_WEBGL
            }
            return defaultValue
        }
        return defaultValue
    }


    public func pixelStorei(_ pname: UInt32, _ param: UInt32) {
        canvas.renderer.ensureIsContextIsCurrent()
        switch pname {
        case UNPACK_ALIGNMENT, PACK_ALIGNMENT:
            glPixelStorei(pname, anyToInt(param, 4))
            break;
        case UNPACK_FLIP_Y_WEBGL:
            flipYWebGL = anyToBoolean(param == 1, false)
        case UNPACK_PREMULTIPLY_ALPHA_WEBGL:
            premultiplyAlphaWebGL = anyToBoolean(param == 1, false)
        case UNPACK_COLORSPACE_CONVERSION_WEBGL:
            colorSpaceConversionWebGL = anyToColorSpace(param, BROWSER_DEFAULT_WEBGL)
        default:
            break;
        }
    }


    public func polygonOffset(_ factor: Float32, _ units: Float32){
        canvas.renderer.ensureIsContextIsCurrent()
        glPolygonOffset(factor, units)
    }

    public func readPixels(_ x: Int32,_ y: Int32,_ width: Int32,_ height: Int32,_ format: UInt32,_ type: UInt32,_ pixels: UnsafeMutableRawPointer) {
        canvas.renderer.ensureIsContextIsCurrent()
        clearIfComposited()
        glReadPixels(x, y, width, height, format, type, pixels)
        
    }

    public func renderbufferStorage(_ target: UInt32,_ internalFormat: UInt32,_ width: Int32,_ height: Int32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glRenderbufferStorage(target, internalFormat, width, height)
    }


    public func sampleCoverage(_ value:Float32 ,_ invert: Bool){
        canvas.renderer.ensureIsContextIsCurrent()
        glSampleCoverage(value, boolConverter(invert))
    }


    public func scissor(_ x: Int32,_ y: Int32,_ width: Int32,_ height: Int32){
        canvas.renderer.ensureIsContextIsCurrent()
        glScissor(x, y, width, height)
    }

    public func shaderSource(_ shader: UInt32,_ source: String) {
        canvas.renderer.ensureIsContextIsCurrent()
        var ptr = (source as NSString).cString(using: String.Encoding.utf8.rawValue)
        glShaderSource(shader, 1, &ptr, nil)
    }

    public func stencilFunc(_ fn:UInt32, _ ref: Int32,_ mask: UInt32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glStencilFunc(fn, ref, mask)
    }


    public func stencilFuncSeparate(_ face: UInt32,_ fn: UInt32, _ ref:Int32,_ mask: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        switch Int32(face) {
        case GL_FRONT_AND_BACK:
            canvas.mStencilFuncRef = ref
            canvas.mStencilFuncRefBack = ref
            canvas.mStencilFuncMask = mask
            canvas.mStencilFuncMaskBack = mask
            break;
        case GL_FRONT:
            canvas.mStencilFuncRef = ref
            canvas.mStencilFuncMask = mask;
            break;
        case GL_BACK:
            canvas.mStencilFuncRefBack = ref
            canvas.mStencilFuncMaskBack = mask;
            break;
        default:
            // TODO emit error
            break;
        }
        glStencilFuncSeparate(face, fn, ref, mask)
    }

    public func stencilMask(_ mask: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        canvas.mStencilMask = mask
        canvas.mStencilMaskBack = mask
        glStencilMask(mask)
    }

    public func stencilMaskSeparate(_ face: UInt32,_ mask: UInt32) {
        canvas.renderer.ensureIsContextIsCurrent()
        switch Int32(face) {
        case GL_FRONT_AND_BACK:
            canvas.mStencilMask = mask
            canvas.mStencilMaskBack = mask
        case GL_FRONT:
            canvas.mStencilMask = mask
        case GL_BACK:
            canvas.mStencilMaskBack = mask
        default:
            // TODO emit error
            break;
        }
        glStencilMaskSeparate(face, mask)
    }

    public func stencilOp(_ fail: UInt32,_ zfail: UInt32,_ zpass: UInt32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glStencilOp(fail, zfail, zpass)
    }

    public func stencilOpSeparate(_ face: UInt32,_ fail: UInt32,_ zfail: UInt32,_ zpass: UInt32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glStencilOpSeparate(face, fail, zfail, zpass)
    }
    
    
    public func texImage2D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32, data: NSData) {
        canvas.renderer.ensureIsContextIsCurrent()
        var bytes = [UInt8](data)
        if(flipYWebGL){
            GLUtils.flipYInPlace(&bytes,bytes.count, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height))
        }
        
        glTexImage2D(target, level, internalformat, width, height, border, format, type, &bytes)
    }


    public func texImage2D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32, u8 pixels: [UInt8]) {
        canvas.renderer.ensureIsContextIsCurrent()
        var data = pixels
        if(flipYWebGL){
            GLUtils.flipYInPlace(&data,data.count, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height))
        }
        glTexImage2D(target, level, internalformat, width, height, border, format, type, &data)
    }
    
    
    public func texImage2D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32, pixels: UnsafeRawPointer, size: Int) {
        canvas.renderer.ensureIsContextIsCurrent()
        if(flipYWebGL){
            var px = Data(bytes: pixels, count: size)
            px.withUnsafeMutableBytes { ptr in
                let pointer = ptr.baseAddress?.assumingMemoryBound(to: UInt8.self)
                GLUtils.flipYInPlace(pointer,size, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height))
                glTexImage2D(target, level, internalformat, width, height, border, format, type, pointer)
            }
            
        }else {
            glTexImage2D(target, level, internalformat, width, height, border, format, type, pixels)
        }
        
    }


    public func texImage2D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,u16 pixels: [UInt16]) {
        canvas.renderer.ensureIsContextIsCurrent()
        var data = pixels
        if(flipYWebGL){
            GLUtils.flipYInPlace(&data,data.count, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height))
        }
        
        glTexImage2D(target, level, internalformat, width, height, border, format, type, &data)
    }



    public func texImage2D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,u32 pixels: [UInt32]) {
        canvas.renderer.ensureIsContextIsCurrent()
        var data = pixels
        if(flipYWebGL){
            GLUtils.flipYInPlace(&data,data.count, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height))
        }
        glTexImage2D(target, level, internalformat, width, height, border, format, type, &data)
    }


    public func texImage2D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,f32 pixels: [Float32]) {
        canvas.renderer.ensureIsContextIsCurrent()
        var data = pixels
        if(flipYWebGL){
            GLUtils.flipYInPlace(&data,data.count, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height))
        }
        glTexImage2D(target, level, internalformat, width, height, border, format, type, &data)
    }

    public func texImage2D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,_ pixels: NSNull) {
        canvas.renderer.ensureIsContextIsCurrent()
        glTexImage2D(target, level, internalformat, width, height, border, format, type, nil)
    }



    public func texImage2D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ format: UInt32,_ type: UInt32, pixels: UIImage) {
        let (buffer,size) = GLUtils.getBytesFromImage(pixels: pixels)
        canvas.renderer.ensureIsContextIsCurrent()
        let width = Int32(pixels.size.width)
        let height = Int32(pixels.size.height)
        if(flipYWebGL){
            GLUtils.flipYInPlace(buffer?.assumingMemoryBound(to: UInt8.self),size, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height))
        }

        // Use RGBA for now
        glTexImage2D(target, level, internalformat, GLsizei(width), GLsizei(height), 0, format, type, buffer)

        buffer?.deallocate()

    }

    public func texImage2D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ format: UInt32,_ type: UInt32, asset: TNSImageAsset) {
        canvas.renderer.ensureIsContextIsCurrent()
        gl_tex_image_2D_asset(target, level, internalformat, 0, format, type, asset.asset, flipYWebGL)

    }
    
    
    public func texImage2D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ format: UInt32,_ type: UInt32, bitmap: TNSImageBitmap) {
        canvas.renderer.ensureIsContextIsCurrent()
        gl_tex_image_2D_asset(target, level, internalformat, 0, format, type, bitmap.asset, flipYWebGL)

    }

    public func texImage2D(_ target: UInt32,_ level: Int32,_  internalformat: Int32,_  format: UInt32,_ type: UInt32,_ pixels: NSNull) {
        canvas.renderer.ensureIsContextIsCurrent()
        glTexImage2D(target, level, internalformat, 0, 0, 0, format, type, nil)
    }

    public func texImage2D(_ target: UInt32,_  level: Int32,_  internalformat: Int32,_  format: UInt32,_ type: UInt32, canvas: TNSCanvas) {
        var snapshot = canvas.snapshot()
        let width = canvas.renderer.drawingBufferWidth
        let height = canvas.renderer.drawingBufferHeight
        let _ = self.canvas.renderer.ensureIsContextIsCurrent()
        if(flipYWebGL){
            GLUtils.flipYInPlace(&snapshot,snapshot.count, Int(Int32(width) * bytes_per_pixel(pixel_type: type, format: format)), Int(height))
        }
        glTexImage2D(target, level, internalformat, GLsizei(width), GLsizei(height), 0, format, type, &snapshot)
    }


    public func texParameterf(_ target: UInt32,_ pname: UInt32,_ param: Float32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glTexParameterf(target, pname, param)
    }

    public func texParameteri(_ target: UInt32,_ pname: UInt32,_ param: Int32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glTexParameteri(target, pname, param)
    }
    

    
    public func texSubImage2D(_ target: UInt32,_ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ width: Int32,_ height: Int32, _ format: UInt32,_ type: UInt32,data: NSData){
        canvas.renderer.ensureIsContextIsCurrent()
        var bytes = [UInt8](data)
        if(flipYWebGL){
            GLUtils.flipYInPlace(&bytes,bytes.count, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height))
        }
        glTexSubImage2D(GLenum(target), level, xoffset, yoffset, width, height, GLenum(format), GLenum(type), &bytes)

    }
    
    
    public func texSubImage2D(_ target: UInt32,_ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ width: Int32,_ height: Int32, _ format: UInt32,_ type: UInt32,u8 pixels: [UInt8]){
        canvas.renderer.ensureIsContextIsCurrent()
        var data = pixels
        if(flipYWebGL){
            GLUtils.flipYInPlace(&data,data.count, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height))
        }
        glTexSubImage2D(GLenum(target), level, xoffset, yoffset, width, height, GLenum(format), GLenum(type), &data)

    }

    

    public func texSubImage2D(_ target: UInt32,_ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ width: Int32,_ height: Int32, _ format: UInt32,_ type: UInt32,pixels: UnsafeMutableRawPointer, size: Int){
        canvas.renderer.ensureIsContextIsCurrent()
        
        if(flipYWebGL){
            var px = Data(bytes: pixels, count: size)
            px.withUnsafeMutableBytes { ptr in
                let pointer = ptr.baseAddress?.assumingMemoryBound(to: UInt8.self)
                GLUtils.flipYInPlace(pointer,size, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height))
                
                glTexSubImage2D(GLenum(target), level, xoffset, yoffset, width, height, GLenum(format), GLenum(type), pixels)
            }
            
        }else {
            glTexSubImage2D(GLenum(target), level, xoffset, yoffset, width, height, GLenum(format), GLenum(type), pixels)
        }
        
    }

    public func texSubImage2D(_ target: UInt32,_ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ width: Int32,_ height: Int32, _ format: UInt32,_ type: UInt32,u16 pixels: [UInt16]){
        canvas.renderer.ensureIsContextIsCurrent()
        var data = pixels
        if(flipYWebGL){
            GLUtils.flipYInPlace(&data,data.count, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height))
        }
        glTexSubImage2D(GLenum(target), level, xoffset, yoffset, width, height, GLenum(format), GLenum(type), &data)
    }


    public func texSubImage2D(_ target: UInt32,_ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ width: Int32,_ height: Int32, _ format: UInt32,_ type: UInt32,f32 pixels: [Float32]){
        canvas.renderer.ensureIsContextIsCurrent()
        var data = pixels
        if(flipYWebGL){
            GLUtils.flipYInPlace(&data,data.count, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height))
        }
        glTexSubImage2D(GLenum(target), level, xoffset, yoffset, width, height, GLenum(format), GLenum(type), &data)
    }

    public func texSubImage2D(_ target: UInt32,_ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ width: Int32,_ height: Int32, _ format: UInt32,_ type: UInt32,_ pixels: NSNull){
        canvas.renderer.ensureIsContextIsCurrent()
        glTexSubImage2D(GLenum(target), level, xoffset, yoffset, width, height, GLenum(format), GLenum(type), nil)
    }



    public func texSubImage2D(_ target: UInt32,_ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ format: UInt32,_ type: UInt32,pixels: UIImage) {
        let (buffer, size) = GLUtils.getBytesFromImage(pixels: pixels)
        canvas.renderer.ensureIsContextIsCurrent()
        let width = Int32(pixels.size.width)
        let height = Int32(pixels.size.height)
        if(flipYWebGL){
            GLUtils.flipYInPlace(buffer?.assumingMemoryBound(to: UInt8.self),size, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height))
        }

        glTexSubImage2D(target, level, xoffset, yoffset, GLsizei(width), GLsizei(height),format, type, buffer)
        buffer?.deallocate()
    }


    public func texSubImage2D(_ target: UInt32,_ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ format: UInt32,_ type:UInt32, asset: TNSImageAsset) {
        canvas.renderer.ensureIsContextIsCurrent()
        gl_tex_sub_image_2D_asset(target, level, xoffset, yoffset, format, type, asset.asset, flipYWebGL)
    }
    
    public func texSubImage2D(_ target: UInt32,_ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ format: UInt32,_ type:UInt32, bitmap: TNSImageBitmap) {
        canvas.renderer.ensureIsContextIsCurrent()
        gl_tex_sub_image_2D_asset(target, level, xoffset, yoffset, format, type, bitmap.asset, flipYWebGL)
    }


    public func texSubImage2D(_ target: UInt32,_ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ format: UInt32,_ type:UInt32, canvas: TNSCanvas) {
        let width = Int32(canvas.renderer.drawingBufferWidth)
        let height = Int32(canvas.renderer.drawingBufferWidth)
        var snapshot = canvas.snapshot()
        let _ = self.canvas.renderer.ensureIsContextIsCurrent()
        if(flipYWebGL){
            GLUtils.flipYInPlace(&snapshot,snapshot.count, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height))
        }

        glTexSubImage2D(GLenum(target), level, xoffset, yoffset, GLsizei(width), GLsizei(height),format, type, &snapshot)
    }

    public func texSubImage2D(_ target: UInt32,_ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ format: UInt32,_ type:UInt32, _ pixels: NSNull) {
        canvas.renderer.ensureIsContextIsCurrent()
        glTexSubImage2D(GLenum(target), level, xoffset, yoffset, 0, 0, format, type, nil)
    }


    public func uniform1f(_ location: Int32,_ v0: Float32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glUniform1f(location, v0)
    }

    public func uniform1fv(_ location: Int32, _ value: UnsafeMutableRawPointer,_ size: Int) {
        canvas.renderer.ensureIsContextIsCurrent()
        glUniform1fv(location, GLint(size/1), value.assumingMemoryBound(to: GLfloat.self))
    }


    public func uniform1i(_ location: Int32,_ v0: Int32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glUniform1i(location, v0)
    }

    public func uniform1iv(_ location: Int32, _ value: UnsafeMutableRawPointer,_ size: Int) {
        canvas.renderer.ensureIsContextIsCurrent()
        glUniform1iv(location, GLint(size / 1), value.assumingMemoryBound(to: GLint.self))
    }


    public func uniform2f(_ location: Int32,_ v0: Float32,_ v1: Float32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glUniform2f(location, v0, v1)
    }

    public func uniform2fv(_ location: Int32, _ value: UnsafeMutableRawPointer,_ size: Int) {
        canvas.renderer.ensureIsContextIsCurrent()
        glUniform1fv(location, GLint(size / 2), value.assumingMemoryBound(to: GLfloat.self))
    }


    public func uniform2i(_ location: Int32,_ v0: Int32,_ v1: Int32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glUniform2i(location, v0, v1)
    }

    public func uniform2iv(_ location: Int32, _ value: UnsafeMutableRawPointer,_ size: Int) {
        canvas.renderer.ensureIsContextIsCurrent()
        glUniform2iv(location, GLint(size / 2), value.assumingMemoryBound(to: GLint.self))
    }


    public func uniform3f(_ location: Int32,_ v0: Float32,_ v1: Float32,_ v2: Float32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glUniform3f(location, v0, v1, v2)
    }

    public func uniform3fv(_ location: Int32, _ value: UnsafeMutableRawPointer,_ size: Int) {
        canvas.renderer.ensureIsContextIsCurrent()
        glUniform3fv(location, GLint(size / 3), value.assumingMemoryBound(to: GLfloat.self))
    }


    public func uniform3i(_ location: Int32,_ v0: Int32,_ v1: Int32,_ v2: Int32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glUniform3i(location, v0, v1, v2)
    }

    public func uniform3iv(_ location: Int32, _ value: UnsafeMutableRawPointer,_ size: Int) {
        canvas.renderer.ensureIsContextIsCurrent()
        glUniform3iv(location, GLint(size / 3), value.assumingMemoryBound(to: GLint.self))
    }

    public func uniform4f(_ location: Int32,_ v0: Float32,_ v1: Float32,_ v2: Float32,_ v3: Float32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glUniform4f(location, v0, v1, v2, v3)
    }

    public func uniform4fv(_ location: Int32, _ value: UnsafeMutableRawPointer,_ size: Int) {
        canvas.renderer.ensureIsContextIsCurrent()
        glUniform4fv(location, GLint(size / 4), value.assumingMemoryBound(to: GLfloat.self))
    }

    public func uniform4i(_ location: Int32,_ v0: Int32,_ v1: Int32,_ v2: Int32,_ v3: Int32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glUniform4i(location, v0, v1, v2, v3)
    }

    public func uniform4iv(_ location: Int32, _ value: UnsafeMutableRawPointer,_ size: Int) {
        canvas.renderer.ensureIsContextIsCurrent()
        glUniform4iv(location, GLint(size / 4) , value.assumingMemoryBound(to: GLint.self))
    }

    public func uniformMatrix2fv(_ location: Int32,_ transpose: Bool,_ value: UnsafeMutableRawPointer,_ size: Int) {
        canvas.renderer.ensureIsContextIsCurrent()
        glUniformMatrix2fv(location,GLsizei(size / 4), boolConverter(transpose), value.assumingMemoryBound(to: GLfloat.self))
    }

    public func uniformMatrix3fv(_ location: Int32,_ transpose: Bool,_ value: UnsafeMutableRawPointer,_ size: Int) {
        canvas.renderer.ensureIsContextIsCurrent()
        glUniformMatrix3fv(location, GLsizei(size / 9), boolConverter(transpose), value.assumingMemoryBound(to: GLfloat.self))
    }

    public func uniformMatrix4fv(_ location: Int32,_ transpose: Bool, _ value: UnsafeMutableRawPointer,_ size: Int) {
        canvas.renderer.ensureIsContextIsCurrent()
        glUniformMatrix4fv(location, GLsizei(size / 16), boolConverter(transpose), value.assumingMemoryBound(to: GLfloat.self))
    }

    public func useProgram(_ program: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glUseProgram(program)
    }

    public func validateProgram(_ program: UInt32){
        canvas.renderer.ensureIsContextIsCurrent()
        glValidateProgram(program)
    }

    public func vertexAttrib1f(_ index: UInt32,_ v0: Float32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glVertexAttrib1f(index, v0)
    }

    public func vertexAttrib2f(_ index: UInt32, _ v0: Float32,_ v1: Float32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glVertexAttrib2f(GLuint(index), v0, v1)
    }

    public func vertexAttrib3f(_ index: UInt32,_ v0: Float32,_ v1: Float32,_ v2: Float32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glVertexAttrib3f(index, v0, v1, v2)
    }

    public func vertexAttrib4f(_ index: UInt32,_ v0: Float32,_ v1: Float32,_ v2: Float32,_ v3: Float32) {
        canvas.renderer.ensureIsContextIsCurrent()
        glVertexAttrib4f(index, v0, v1, v2, v3)
    }

    public func vertexAttrib1fv(_ index: UInt32,_ value: UnsafeMutableRawPointer) {
        canvas.renderer.ensureIsContextIsCurrent()
        glVertexAttrib1fv(index,value.assumingMemoryBound(to: GLfloat.self))
    }

    public func vertexAttrib2fv(_ index: UInt32,_ value: UnsafeMutableRawPointer) {
        canvas.renderer.ensureIsContextIsCurrent()
        glVertexAttrib2fv(index,value.assumingMemoryBound(to: GLfloat.self))
    }

    public func vertexAttrib3fv(_ index: UInt32, _ value: UnsafeMutableRawPointer) {
        canvas.renderer.ensureIsContextIsCurrent()
        glVertexAttrib3fv(GLuint(index),value.assumingMemoryBound(to: GLfloat.self))
    }

    public func vertexAttrib4fv(_ index: UInt32, _ value: UnsafeMutableRawPointer) {
        canvas.renderer.ensureIsContextIsCurrent()
        glVertexAttrib4fv(index,value.assumingMemoryBound(to: GLfloat.self))
    }

    public func vertexAttribPointer(_ index: UInt32,_ size: Int32,_ type: UInt32,_ normalized: Bool,_ stride: Int32,_ offset: Int) {
        canvas.renderer.ensureIsContextIsCurrent()
        /*let ptr = BUFFER_OFFSET(n: offset)
        glVertexAttribPointer(index, size, type, boolConverter(normalized), stride, ptr)
 */
        gl_vertex_attrib_pointer(index, size, type, normalized, stride, Int64(offset))
    }


    public func viewport(_ x: Int32,_ y: Int32,_ width: Int32,_ height: Int32){
        canvas.renderer.ensureIsContextIsCurrent()
        glViewport(x, y, width, height)
    }



    /* Clearing buffers */

    public var DEPTH_BUFFER_BIT : UInt32 { return UInt32(GL_DEPTH_BUFFER_BIT) }

    public var COLOR_BUFFER_BIT : UInt32 { return UInt32(GL_COLOR_BUFFER_BIT) }

    public var STENCIL_BUFFER_BIT : UInt32 { return UInt32(GL_STENCIL_BUFFER_BIT) }

    /* Clearing buffers */

    /* Rendering primitives */

    public var POINTS : UInt32 { return UInt32(GL_POINTS) }

    public var LINES : UInt32 { return UInt32(GL_LINES) }

    public var LINE_LOOP : UInt32 { return UInt32(GL_LINE_LOOP) }

    public var LINE_STRIP : UInt32 { return UInt32(GL_LINE_STRIP) }

    public var TRIANGLES : UInt32 { return UInt32(GL_TRIANGLES) }

    public var TRIANGLE_STRIP : UInt32 { return UInt32(GL_TRIANGLE_STRIP) }

    public var TRIANGLE_FAN : UInt32 { return UInt32(GL_TRIANGLE_FAN) }

    /* Rendering primitives */

    /* Blending modes */


    public var ONE : UInt32 { return UInt32(GL_ONE) }

    public var ZERO : UInt32 { return UInt32(GL_ZERO) }
    public var SRC_COLOR : UInt32 { return UInt32(GL_SRC_COLOR) }

    public var ONE_MINUS_SRC_COLOR : UInt32 { return UInt32(GL_ONE_MINUS_SRC_COLOR) }

    public var SRC_ALPHA : UInt32 { return UInt32(GL_SRC_ALPHA) }

    public var ONE_MINUS_SRC_ALPHA : UInt32 { return UInt32(GL_ONE_MINUS_SRC_ALPHA) }

    public var DST_ALPHA : UInt32 { return UInt32(GL_DST_ALPHA) }

    public var ONE_MINUS_DST_ALPHA : UInt32 { return UInt32(GL_ONE_MINUS_DST_ALPHA) }

    public var DST_COLOR : UInt32 { return UInt32(GL_DST_COLOR) }

    public var ONE_MINUS_DST_COLOR : UInt32 { return UInt32(GL_ONE_MINUS_DST_COLOR) }

    public var SRC_ALPHA_SATURATE : UInt32 { return UInt32(GL_SRC_ALPHA_SATURATE) }

    public var CONSTANT_COLOR : UInt32 { return UInt32(GL_CONSTANT_COLOR) }
    public var ONE_MINUS_CONSTANT_COLOR : UInt32 { return UInt32(GL_ONE_MINUS_CONSTANT_COLOR) }

    public var CONSTANT_ALPHA : UInt32 { return UInt32(GL_CONSTANT_ALPHA) }
    public var ONE_MINUS_CONSTANT_ALPHA : UInt32 { return UInt32(GL_ONE_MINUS_CONSTANT_ALPHA) }

    /* Blending modes */

    /* Blending equations */
    public var FUNC_ADD : UInt32 { return UInt32(GL_FUNC_ADD) }

    public var FUNC_SUBTRACT : UInt32 { return UInt32(GL_FUNC_SUBTRACT) }

    public var FUNC_REVERSE_SUBTRACT : UInt32 { return UInt32(GL_FUNC_REVERSE_SUBTRACT) }

    /* Blending equations */


    /* Getting GL parameter information */

    public var BLEND_EQUATION : UInt32 { return UInt32(GL_BLEND_EQUATION) }

    public var BLEND_EQUATION_RGB : UInt32 { return UInt32(GL_BLEND_EQUATION_RGB) }

    public var BLEND_EQUATION_ALPHA : UInt32 { return UInt32(GL_BLEND_EQUATION_ALPHA) }

    public var BLEND_DST_RGB : UInt32 { return UInt32(GL_BLEND_DST_RGB) }

    public var BLEND_SRC_RGB : UInt32 { return UInt32(GL_BLEND_SRC_RGB) }

    public var BLEND_DST_ALPHA : UInt32 { return UInt32(GL_BLEND_DST_ALPHA) }

    public var BLEND_SRC_ALPHA : UInt32 { return UInt32(GL_BLEND_SRC_ALPHA) }

    public var BLEND_COLOR : UInt32 { return UInt32(GL_BLEND_COLOR) }

    public var ARRAY_BUFFER_BINDING : UInt32 { return UInt32(GL_ARRAY_BUFFER_BINDING) }

    public var ELEMENT_ARRAY_BUFFER_BINDING : UInt32 { return UInt32(GL_ELEMENT_ARRAY_BUFFER_BINDING) }

    public var LINE_WIDTH : UInt32 { return UInt32(GL_LINE_WIDTH) }

    public var ALIASED_POINT_SIZE_RANGE : UInt32 { return UInt32(GL_ALIASED_POINT_SIZE_RANGE) }

    public var ALIASED_LINE_WIDTH_RANGE : UInt32 { return UInt32(GL_ALIASED_LINE_WIDTH_RANGE) }

    public var CULL_FACE_MODE : UInt32 { return UInt32(GL_CULL_FACE_MODE) }

    public var FRONT_FACE : UInt32 { return UInt32(GL_FRONT_FACE) }

    public var DEPTH_RANGE : UInt32 { return UInt32(GL_DEPTH_RANGE) }

    public var DEPTH_WRITEMASK : UInt32 { return UInt32(GL_DEPTH_WRITEMASK) }

    public var DEPTH_CLEAR_VALUE : UInt32 { return UInt32(GL_DEPTH_CLEAR_VALUE) }

    public var DEPTH_FUNC : UInt32 { return UInt32(GL_DEPTH_FUNC) }

    public var STENCIL_CLEAR_VALUE : UInt32 { return UInt32(GL_STENCIL_CLEAR_VALUE) }

    public var STENCIL_FUNC : UInt32 { return UInt32(GL_STENCIL_FUNC) }

    public var STENCIL_FAIL : UInt32 { return UInt32(GL_STENCIL_FAIL) }

    public var STENCIL_PASS_DEPTH_FAIL : UInt32 { return UInt32(GL_STENCIL_PASS_DEPTH_FAIL) }

    public var STENCIL_PASS_DEPTH_PASS : UInt32 { return UInt32(GL_STENCIL_PASS_DEPTH_PASS) }

    public var STENCIL_REF : UInt32 { return UInt32(GL_STENCIL_REF) }

    public var STENCIL_VALUE_MASK : UInt32 { return UInt32(GL_STENCIL_VALUE_MASK) }

    public var STENCIL_WRITEMASK : UInt32 { return UInt32(GL_STENCIL_WRITEMASK) }

    public var STENCIL_BACK_FUNC : UInt32 { return UInt32(GL_STENCIL_BACK_FUNC) }

    public var STENCIL_BACK_FAIL : UInt32 { return UInt32(GL_STENCIL_BACK_FAIL) }

    public var STENCIL_BACK_PASS_DEPTH_FAIL : UInt32 { return UInt32(GL_STENCIL_BACK_PASS_DEPTH_FAIL) }

    public var STENCIL_BACK_PASS_DEPTH_PASS : UInt32 { return UInt32(GL_STENCIL_BACK_PASS_DEPTH_PASS) }

    public var STENCIL_BACK_REF : UInt32 { return UInt32(GL_STENCIL_BACK_REF) }

    public var STENCIL_BACK_VALUE_MASK : UInt32 { return UInt32(GL_STENCIL_BACK_VALUE_MASK) }

    public var STENCIL_BACK_WRITEMASK : UInt32 { return UInt32(GL_STENCIL_BACK_WRITEMASK) }

    public var VIEWPORT : UInt32 { return UInt32(GL_VIEWPORT) }

    public var SCISSOR_BOX : UInt32 { return UInt32(GL_SCISSOR_BOX) }

    public var COLOR_CLEAR_VALUE : UInt32 { return UInt32(GL_COLOR_CLEAR_VALUE) }

    public var COLOR_WRITEMASK : UInt32 { return UInt32(GL_COLOR_WRITEMASK) }

    public var UNPACK_ALIGNMENT : UInt32 { return UInt32(GL_UNPACK_ALIGNMENT) }

    public var PACK_ALIGNMENT : UInt32 { return UInt32(GL_PACK_ALIGNMENT) }

    public var MAX_TEXTURE_SIZE : UInt32 { return UInt32(GL_MAX_TEXTURE_SIZE) }

    public var MAX_VIEWPORT_DIMS : UInt32 { return UInt32(GL_MAX_VIEWPORT_DIMS) }

    public var SUBPIXEL_BITS : UInt32 { return UInt32(GL_SUBPIXEL_BITS) }

    public var RED_BITS : UInt32 { return UInt32(GL_RED_BITS) }

    public var GREEN_BITS : UInt32 { return UInt32(GL_GREEN_BITS) }

    public var BLUE_BITS : UInt32 { return UInt32(GL_BLUE_BITS) }

    public var ALPHA_BITS : UInt32 { return UInt32(GL_ALPHA_BITS) }

    public var DEPTH_BITS : UInt32 { return UInt32(GL_DEPTH_BITS) }

    public var STENCIL_BITS : UInt32 { return UInt32(GL_STENCIL_BITS) }

    public var POLYGON_OFFSET_UNITS : UInt32 { return UInt32(GL_POLYGON_OFFSET_UNITS) }

    public var POLYGON_OFFSET_FACTOR : UInt32 { return UInt32(GL_POLYGON_OFFSET_FACTOR) }

    public var TEXTURE_BINDING_2D : UInt32 { return UInt32(GL_TEXTURE_BINDING_2D) }

    public var SAMPLE_BUFFERS : UInt32 { return UInt32(GL_SAMPLE_BUFFERS) }

    public var SAMPLES : UInt32 { return UInt32(GL_SAMPLES) }

    public var SAMPLE_COVERAGE_VALUE : UInt32 { return UInt32(GL_SAMPLE_COVERAGE_VALUE) }

    public var SAMPLE_COVERAGE_INVERT : UInt32 { return UInt32(GL_SAMPLE_COVERAGE_INVERT) }

    public var COMPRESSED_TEXTURE_FORMATS : UInt32 { return UInt32(GL_COMPRESSED_TEXTURE_FORMATS) }

    public var VENDOR : UInt32 { return UInt32(GL_VENDOR) }

    public var RENDERER : UInt32 { return UInt32(GL_RENDERER) }

    public var VERSION : UInt32 { return UInt32(GL_VERSION) }

    public var IMPLEMENTATION_COLOR_READ_TYPE : UInt32 { return UInt32(GL_IMPLEMENTATION_COLOR_READ_TYPE) }

    public var IMPLEMENTATION_COLOR_READ_FORMAT : UInt32 { return UInt32(GL_IMPLEMENTATION_COLOR_READ_FORMAT) }

    public var BROWSER_DEFAULT_WEBGL : UInt32 { return  0x9244 }

    /* Getting GL parameter information */

    /* Buffers */

    public var STATIC_DRAW : UInt32 { return UInt32(GL_STATIC_DRAW) }

    public var STREAM_DRAW : UInt32 { return UInt32(GL_STREAM_DRAW) }

    public var DYNAMIC_DRAW : UInt32 { return UInt32(GL_DYNAMIC_DRAW) }

    public var ARRAY_BUFFER : UInt32 { return UInt32(GL_ARRAY_BUFFER) }

    public var ELEMENT_ARRAY_BUFFER : UInt32 { return UInt32(GL_ELEMENT_ARRAY_BUFFER) }

    public var BUFFER_SIZE : UInt32 { return UInt32(GL_BUFFER_SIZE) }

    public var BUFFER_USAGE : UInt32 { return UInt32(GL_BUFFER_USAGE) }

    /* Buffers */

    /* Vertex attributes */

    public var CURRENT_VERTEX_ATTRIB : UInt32 { return UInt32(GL_CURRENT_VERTEX_ATTRIB) }

    public var VERTEX_ATTRIB_ARRAY_ENABLED : UInt32 { return UInt32(GL_VERTEX_ATTRIB_ARRAY_ENABLED) }

    public var VERTEX_ATTRIB_ARRAY_SIZE : UInt32 { return UInt32(GL_VERTEX_ATTRIB_ARRAY_SIZE) }

    public var VERTEX_ATTRIB_ARRAY_STRIDE : UInt32 { return UInt32(GL_VERTEX_ATTRIB_ARRAY_STRIDE) }

    public var VERTEX_ATTRIB_ARRAY_TYPE : UInt32 { return UInt32(GL_VERTEX_ATTRIB_ARRAY_TYPE) }

    public var VERTEX_ATTRIB_ARRAY_NORMALIZED : UInt32 { return UInt32(GL_VERTEX_ATTRIB_ARRAY_NORMALIZED) }

    public var VERTEX_ATTRIB_ARRAY_POINTER : UInt32 { return UInt32(GL_VERTEX_ATTRIB_ARRAY_POINTER) }

    public var VERTEX_ATTRIB_ARRAY_BUFFER_BINDING : UInt32 { return UInt32(GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING) }

    /* Vertex attributes */

    /* Culling */

    public var CULL_FACE : UInt32 { return UInt32(GL_CULL_FACE) }

    public var FRONT : UInt32 { return UInt32(GL_FRONT) }

    public var BACK : UInt32 { return UInt32(GL_BACK) }

    public var FRONT_AND_BACK : UInt32 { return UInt32(GL_FRONT_AND_BACK) }

    /* Culling */

    /* Enabling and disabling */

    public var BLEND : UInt32 { return UInt32(GL_BLEND) }

    public var DEPTH_TEST : UInt32 { return UInt32(GL_DEPTH_TEST) }

    public var DITHER : UInt32 { return UInt32(GL_DITHER) }

    public var POLYGON_OFFSET_FILL : UInt32 { return UInt32(GL_POLYGON_OFFSET_FILL) }

    public var SAMPLE_ALPHA_TO_COVERAGE : UInt32 { return UInt32(GL_SAMPLE_ALPHA_TO_COVERAGE) }

    public var SAMPLE_COVERAGE : UInt32 { return UInt32(GL_SAMPLE_COVERAGE) }

    public var SCISSOR_TEST : UInt32 { return UInt32(GL_SCISSOR_TEST) }

    public var STENCIL_TEST : UInt32 { return UInt32(GL_STENCIL_TEST) }

    /* Enabling and disabling */

    /* Errors */
    public var NO_ERROR : UInt32 { return UInt32(GL_NO_ERROR) }

    public var INVALID_ENUM : UInt32 { return UInt32(GL_INVALID_ENUM) }

    public var INVALID_VALUE : UInt32 { return UInt32(GL_INVALID_VALUE) }

    public var INVALID_OPERATION : UInt32 { return UInt32(GL_INVALID_OPERATION) }

    public var INVALID_FRAMEBUFFER_OPERATION : UInt32 { return UInt32(GL_INVALID_FRAMEBUFFER_OPERATION) }

    public var OUT_OF_MEMORY : UInt32 { return UInt32(GL_OUT_OF_MEMORY) }

    public var CONTEXT_LOST_WEBGL: UInt32 { return 0x9242 }
    /* Errors */

    /* Front face directions */

    public var CW : UInt32 { return UInt32(GL_CW) }

    public var CCW : UInt32 { return UInt32(GL_CCW) }

    /* Front face directions */


    /* Hints */

    public var DONT_CARE : UInt32 { return UInt32(GL_DONT_CARE) }

    public var FASTEST : UInt32 { return UInt32(GL_FASTEST) }

    public var NICEST : UInt32 { return UInt32(GL_NICEST) }

    public var GENERATE_MIPMAP_HINT : UInt32 { return UInt32(GL_GENERATE_MIPMAP_HINT) }

    /* Hints */


    /* Data types */

    public var BYTE : UInt32 { return UInt32(GL_BYTE) }

    public var UNSIGNED_BYTE : UInt32 { return UInt32(GL_UNSIGNED_BYTE) }

    public var UNSIGNED_SHORT : UInt32 { return UInt32(GL_UNSIGNED_SHORT) }

    public var SHORT : UInt32 { return UInt32(GL_SHORT) }

    public var UNSIGNED_INT : UInt32 { return UInt32(GL_UNSIGNED_INT) }

    public var INT : UInt32 { return UInt32(GL_INT) }

    public var FLOAT : UInt32 { return UInt32(GL_FLOAT) }

    /* Data types */


    /* Pixel formats */

    public var DEPTH_COMPONENT : UInt32 { return UInt32(GL_DEPTH_COMPONENT) }

    public var ALPHA : UInt32 { return UInt32(GL_ALPHA) }

    public var RGB : UInt32 { return UInt32(GL_RGB) }

    public var RGBA : UInt32 { return UInt32(GL_RGBA) }

    public var LUMINANCE : UInt32 { return UInt32(GL_LUMINANCE) }

    public var LUMINANCE_ALPHA : UInt32 { return UInt32(GL_LUMINANCE_ALPHA) }

    /* Pixel formats */

    /* Pixel types */

    // public var UNSIGNED_BYTE : UInt32 { return UInt32(GL_UNSIGNED_BYTE) }

    public var UNSIGNED_SHORT_4_4_4_4 : UInt32 { return UInt32(GL_UNSIGNED_SHORT_4_4_4_4) }

    public var UNSIGNED_SHORT_5_5_5_1 : UInt32 { return UInt32(GL_UNSIGNED_SHORT_5_5_5_1) }

    public var UNSIGNED_SHORT_5_6_5 : UInt32 { return UInt32(GL_UNSIGNED_SHORT_5_6_5) }

    /* Pixel types */

    /* Shaders */

    public var FRAGMENT_SHADER : UInt32 { return UInt32(GL_FRAGMENT_SHADER) }

    public var VERTEX_SHADER : UInt32 { return UInt32(GL_VERTEX_SHADER) }

    public var COMPILE_STATUS : UInt32 { return UInt32(GL_COMPILE_STATUS) }

    public var DELETE_STATUS : UInt32 { return UInt32(GL_DELETE_STATUS) }

    public var LINK_STATUS : UInt32 { return UInt32(GL_LINK_STATUS) }

    public var VALIDATE_STATUS : UInt32 { return UInt32(GL_VALIDATE_STATUS) }

    public var ATTACHED_SHADERS : UInt32 { return UInt32(GL_ATTACHED_SHADERS) }

    public var ACTIVE_ATTRIBUTES : UInt32 { return UInt32(GL_ACTIVE_ATTRIBUTES) }

    public var ACTIVE_UNIFORMS : UInt32 { return UInt32(GL_ACTIVE_UNIFORMS) }

    public var MAX_VERTEX_UNIFORM_VECTORS : UInt32 { return UInt32(GL_MAX_VERTEX_UNIFORM_VECTORS) }

    public var MAX_VARYING_VECTORS : UInt32 { return UInt32(GL_MAX_VARYING_VECTORS) }

    public var MAX_COMBINED_TEXTURE_IMAGE_UNITS : UInt32 { return UInt32(GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS) }

    public var MAX_VERTEX_TEXTURE_IMAGE_UNITS : UInt32 { return UInt32(GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS) }

    public var MAX_TEXTURE_IMAGE_UNITS : UInt32 { return UInt32(GL_MAX_TEXTURE_IMAGE_UNITS) }

    public var MAX_VERTEX_ATTRIBS : UInt32 { return UInt32(GL_MAX_VERTEX_ATTRIBS) }

    public var MAX_FRAGMENT_UNIFORM_VECTORS : UInt32 { return UInt32(GL_MAX_FRAGMENT_UNIFORM_VECTORS) }

    public var SHADER_TYPE : UInt32 { return UInt32(GL_SHADER_TYPE) }

    public var SHADING_LANGUAGE_VERSION : UInt32 { return UInt32(GL_SHADING_LANGUAGE_VERSION) }

    public var CURRENT_PROGRAM : UInt32 { return UInt32(GL_CURRENT_PROGRAM) }

    /* Shaders */

    /* Depth or stencil tests */

    public var NEVER : UInt32 { return UInt32(GL_NEVER) }

    public var LESS : UInt32 { return UInt32(GL_LESS) }

    public var EQUAL : UInt32 { return UInt32(GL_EQUAL) }

    public var LEQUAL : UInt32 { return UInt32(GL_LEQUAL) }

    public var GREATER : UInt32 { return UInt32(GL_GREATER) }

    public var NOTEQUAL : UInt32 { return UInt32(GL_NOTEQUAL) }

    public var GEQUAL : UInt32 { return UInt32(GL_GEQUAL) }

    public var ALWAYS : UInt32 { return UInt32(GL_ALWAYS) }

    /* Depth or stencil tests */

    /* Stencil actions */

    public var KEEP : UInt32 { return UInt32(GL_KEEP) }

    public var REPLACE : UInt32 { return UInt32(GL_REPLACE) }

    public var INCR : UInt32 { return UInt32(GL_INCR) }

    public var DECR : UInt32 { return UInt32(GL_DECR) }

    public var INVERT : UInt32 { return UInt32(GL_INVERT) }

    public var INCR_WRAP : UInt32 { return UInt32(GL_INCR_WRAP) }

    public var DECR_WRAP : UInt32 { return UInt32(GL_DECR_WRAP) }

    /* Stencil actions */

    /* Textures */

    public var NEAREST : UInt32 { return UInt32(GL_NEAREST) }

    public var LINEAR : UInt32 { return UInt32(GL_LINEAR) }

    public var NEAREST_MIPMAP_NEAREST : UInt32 { return UInt32(GL_NEAREST_MIPMAP_NEAREST) }

    public var LINEAR_MIPMAP_NEAREST : UInt32 { return UInt32(GL_LINEAR_MIPMAP_NEAREST) }

    public var NEAREST_MIPMAP_LINEAR : UInt32 { return UInt32(GL_NEAREST_MIPMAP_LINEAR) }

    public var LINEAR_MIPMAP_LINEAR : UInt32 { return UInt32(GL_LINEAR_MIPMAP_LINEAR) }

    public var TEXTURE_MAG_FILTER : UInt32 { return UInt32(GL_TEXTURE_MAG_FILTER) }

    public var TEXTURE_MIN_FILTER : UInt32 { return UInt32(GL_TEXTURE_MIN_FILTER) }

    public var TEXTURE_WRAP_S : UInt32 { return UInt32(GL_TEXTURE_WRAP_S) }

    public var TEXTURE_WRAP_T : UInt32 { return UInt32(GL_TEXTURE_WRAP_T) }

    public var TEXTURE_2D : UInt32 { return UInt32(GL_TEXTURE_2D) }

    public var TEXTURE : UInt32 { return UInt32(GL_TEXTURE) }

    public var TEXTURE_CUBE_MAP : UInt32 { return UInt32(GL_TEXTURE_CUBE_MAP) }

    public var TEXTURE_BINDING_CUBE_MAP : UInt32 { return UInt32(GL_TEXTURE_BINDING_CUBE_MAP) }

    public var TEXTURE_CUBE_MAP_POSITIVE_X : UInt32 { return UInt32(GL_TEXTURE_CUBE_MAP_POSITIVE_X) }

    public var TEXTURE_CUBE_MAP_NEGATIVE_X : UInt32 { return UInt32(GL_TEXTURE_CUBE_MAP_NEGATIVE_X) }

    public var TEXTURE_CUBE_MAP_POSITIVE_Y : UInt32 { return UInt32(GL_TEXTURE_CUBE_MAP_POSITIVE_Y) }

    public var TEXTURE_CUBE_MAP_NEGATIVE_Y : UInt32 { return UInt32(GL_TEXTURE_CUBE_MAP_NEGATIVE_Y) }

    public var TEXTURE_CUBE_MAP_POSITIVE_Z : UInt32 { return UInt32(GL_TEXTURE_CUBE_MAP_POSITIVE_Z) }

    public var TEXTURE_CUBE_MAP_NEGATIVE_Z : UInt32 { return UInt32(GL_TEXTURE_CUBE_MAP_NEGATIVE_Z) }

    public var MAX_CUBE_MAP_TEXTURE_SIZE : UInt32 { return UInt32(GL_MAX_CUBE_MAP_TEXTURE_SIZE) }

    public var TEXTURE0 : UInt32 { return UInt32(GL_TEXTURE0) }

    public var TEXTURE1 : UInt32 { return UInt32(GL_TEXTURE1) }

    public var TEXTURE2 : UInt32 { return UInt32(GL_TEXTURE2) }

    public var TEXTURE3 : UInt32 { return UInt32(GL_TEXTURE3) }

    public var TEXTURE4 : UInt32 { return UInt32(GL_TEXTURE4) }

    public var TEXTURE5 : UInt32 { return UInt32(GL_TEXTURE5) }

    public var TEXTURE6 : UInt32 { return UInt32(GL_TEXTURE6) }

    public var TEXTURE7 : UInt32 { return UInt32(GL_TEXTURE7) }

    public var TEXTURE8 : UInt32 { return UInt32(GL_TEXTURE8) }

    public var TEXTURE9 : UInt32 { return UInt32(GL_TEXTURE9) }

    public var TEXTURE10 : UInt32 { return UInt32(GL_TEXTURE10) }

    public var TEXTURE11 : UInt32 { return UInt32(GL_TEXTURE11) }

    public var TEXTURE12 : UInt32 { return UInt32(GL_TEXTURE12) }

    public var TEXTURE13 : UInt32 { return UInt32(GL_TEXTURE13) }

    public var TEXTURE14 : UInt32 { return UInt32(GL_TEXTURE14) }

    public var TEXTURE15 : UInt32 { return UInt32(GL_TEXTURE15) }

    public var TEXTURE16 : UInt32 { return UInt32(GL_TEXTURE16) }

    public var TEXTURE17 : UInt32 { return UInt32(GL_TEXTURE17) }

    public var TEXTURE18 : UInt32 { return UInt32(GL_TEXTURE18) }

    public var TEXTURE19 : UInt32 { return UInt32(GL_TEXTURE19) }

    public var TEXTURE20 : UInt32 { return UInt32(GL_TEXTURE20) }

    public var TEXTURE21 : UInt32 { return UInt32(GL_TEXTURE21) }

    public var TEXTURE22 : UInt32 { return UInt32(GL_TEXTURE22) }

    public var TEXTURE23 : UInt32 { return UInt32(GL_TEXTURE23) }

    public var TEXTURE24 : UInt32 { return UInt32(GL_TEXTURE24) }

    public var TEXTURE25 : UInt32 { return UInt32(GL_TEXTURE25) }

    public var TEXTURE26 : UInt32 { return UInt32(GL_TEXTURE26) }

    public var TEXTURE27 : UInt32 { return UInt32(GL_TEXTURE27) }

    public var TEXTURE28 : UInt32 { return UInt32(GL_TEXTURE28) }

    public var TEXTURE29 : UInt32 { return UInt32(GL_TEXTURE29) }

    public var TEXTURE30 : UInt32 { return UInt32(GL_TEXTURE30) }

    public var TEXTURE31 : UInt32 { return UInt32(GL_TEXTURE31) }

    public var ACTIVE_TEXTURE : UInt32 { return UInt32(GL_ACTIVE_TEXTURE) }

    public var REPEAT : UInt32 { return UInt32(GL_REPEAT) }

    public var CLAMP_TO_EDGE : UInt32 { return UInt32(GL_CLAMP_TO_EDGE) }

    public var MIRRORED_REPEAT : UInt32 { return UInt32(GL_MIRRORED_REPEAT) }

    /* Textures */



    /* Uniform types */

    public var FLOAT_VEC2 : UInt32 { return UInt32(GL_FLOAT_VEC2) }

    public var FLOAT_VEC3 : UInt32 { return UInt32(GL_FLOAT_VEC3) }

    public var FLOAT_VEC4 : UInt32 { return UInt32(GL_FLOAT_VEC4) }

    public var INT_VEC2 : UInt32 { return UInt32(GL_INT_VEC2) }

    public var INT_VEC3 : UInt32 { return UInt32(GL_INT_VEC3) }

    public var INT_VEC4 : UInt32 { return UInt32(GL_INT_VEC4) }


    public var BOOL : UInt32 { return UInt32(GL_BOOL) }


    public var BOOL_VEC2 : UInt32 { return UInt32(GL_BOOL_VEC2) }

    public var BOOL_VEC3 : UInt32 { return UInt32(GL_BOOL_VEC3) }

    public var BOOL_VEC4 : UInt32 { return UInt32(GL_BOOL_VEC4) }


    public var FLOAT_MAT2 : UInt32 { return UInt32(GL_FLOAT_MAT2) }


    public var FLOAT_MAT3 : UInt32 { return UInt32(GL_FLOAT_MAT3) }


    public var FLOAT_MAT4 : UInt32 { return UInt32(GL_FLOAT_MAT4) }

    public var SAMPLER_2D : UInt32 { return UInt32(GL_SAMPLER_2D) }

    public var SAMPLER_CUBE : UInt32 { return UInt32(GL_SAMPLER_CUBE) }

    /* Uniform types */

    /* Shader precision-specified types */

    public var LOW_FLOAT : UInt32 { return UInt32(GL_LOW_FLOAT) }
    public var MEDIUM_FLOAT : UInt32 { return UInt32(GL_MEDIUM_FLOAT) }
    public var HIGH_FLOAT : UInt32 { return UInt32(GL_HIGH_FLOAT) }
    public var LOW_INT : UInt32 { return UInt32(GL_LOW_INT) }
    public var MEDIUM_INT : UInt32 { return UInt32(GL_MEDIUM_INT) }
    public var HIGH_INT : UInt32 { return UInt32(GL_HIGH_INT) }

    /* Shader precision-specified types */


    /* Framebuffers and renderbuffers */

    public var FRAMEBUFFER : UInt32 { return UInt32(GL_FRAMEBUFFER) }

    public var RENDERBUFFER : UInt32 { return UInt32(GL_RENDERBUFFER) }

    public var RGBA4 : UInt32 { return UInt32(GL_RGBA4) }

    public var RGB565 : UInt32 { return UInt32(GL_RGB565) }

    public var RGB5_A1 : UInt32 { return UInt32(GL_RGB5_A1) }

    public var DEPTH_COMPONENT16 : UInt32 { return UInt32(GL_DEPTH_COMPONENT16) }

    public var STENCIL_INDEX8 : UInt32 { return UInt32(GL_STENCIL_INDEX8) }

    public var DEPTH_STENCIL : UInt32 { return  0x84F9 }

    public var RENDERBUFFER_WIDTH : UInt32 { return UInt32(GL_RENDERBUFFER_WIDTH) }

    public var RENDERBUFFER_HEIGHT : UInt32 { return UInt32(GL_RENDERBUFFER_HEIGHT) }

    public var RENDERBUFFER_INTERNAL_FORMAT : UInt32 { return UInt32(GL_RENDERBUFFER_INTERNAL_FORMAT) }

    public var RENDERBUFFER_RED_SIZE : UInt32 { return UInt32(GL_RENDERBUFFER_RED_SIZE) }

    public var RENDERBUFFER_GREEN_SIZE : UInt32 { return UInt32(GL_RENDERBUFFER_GREEN_SIZE) }

    public var RENDERBUFFER_BLUE_SIZE : UInt32 { return UInt32(GL_RENDERBUFFER_BLUE_SIZE) }

    public var RENDERBUFFER_ALPHA_SIZE : UInt32 { return UInt32(GL_RENDERBUFFER_ALPHA_SIZE) }

    public var RENDERBUFFER_DEPTH_SIZE : UInt32 { return UInt32(GL_RENDERBUFFER_DEPTH_SIZE) }

    public var RENDERBUFFER_STENCIL_SIZE : UInt32 { return UInt32(GL_RENDERBUFFER_STENCIL_SIZE) }

    public var FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE : UInt32 { return UInt32(GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE) }

    public var FRAMEBUFFER_ATTACHMENT_OBJECT_NAME : UInt32 { return UInt32(GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME) }

    public var FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL : UInt32 { return UInt32(GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL) }

    public var FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE : UInt32 { return UInt32(GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE) }

    public var COLOR_ATTACHMENT0 : UInt32 { return UInt32(GL_COLOR_ATTACHMENT0) }

    public var DEPTH_ATTACHMENT : UInt32 { return UInt32(GL_DEPTH_ATTACHMENT) }

    public var STENCIL_ATTACHMENT : UInt32 { return UInt32(GL_STENCIL_ATTACHMENT) }

    public var DEPTH_STENCIL_ATTACHMENT: UInt32 { return 0x821A }

    public var NONE : UInt32 { return UInt32(GL_NONE) }

    public var FRAMEBUFFER_COMPLETE : UInt32 { return UInt32(GL_FRAMEBUFFER_COMPLETE) }

    public var FRAMEBUFFER_INCOMPLETE_ATTACHMENT : UInt32 { return UInt32(GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT) }

    public var FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT : UInt32 { return UInt32(GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT) }

    public var FRAMEBUFFER_INCOMPLETE_DIMENSIONS : UInt32 { return UInt32(GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS) }

    public var FRAMEBUFFER_UNSUPPORTED : UInt32 { return UInt32(GL_FRAMEBUFFER_UNSUPPORTED) }

    public var FRAMEBUFFER_BINDING : UInt32 { return UInt32(GL_FRAMEBUFFER_BINDING) }

    public var RENDERBUFFER_BINDING : UInt32 { return UInt32(GL_RENDERBUFFER_BINDING) }

    public var MAX_RENDERBUFFER_SIZE : UInt32 { return UInt32(GL_MAX_RENDERBUFFER_SIZE) }

    //public var INVALID_FRAMEBUFFER_OPERATION : UInt32 { return UInt32(GL_INVALID_FRAMEBUFFER_OPERATION) }

    /* Framebuffers and renderbuffers */

    /* Pixel storage modes */

    public var UNPACK_COLORSPACE_CONVERSION_WEBGL : UInt32 { 0x9243 }

    public var UNPACK_FLIP_Y_WEBGL : UInt32 { return  0x9240 }

    public var UNPACK_PREMULTIPLY_ALPHA_WEBGL : UInt32 { return  0x9241 }

    /* Pixel storage modes */


}
