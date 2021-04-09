//
//  TNSWebGL2RenderingContext.swift
//  CanvasNative
//
//  Created by Osei Fortune on 4/29/20.
//

import Foundation
import UIKit
import GLKit
@objcMembers
@objc(TNSWebGL2RenderingContext)
public class TNSWebGL2RenderingContext: TNSWebGLRenderingContext {
    public override init(_ canvas: TNSCanvas) {
        super.init(canvas)
    }
    
    public override init(_ canvas: TNSCanvas,_ attrs: [String: Any]) {
        super.init(canvas, attrs)
    }
    
    public func beginQuery(_ target: UInt32,_ query: UInt32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glBeginQuery(target, query)
    }
    
    public func beginTransformFeedback(_ primitiveMode: UInt32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glBeginTransformFeedback(primitiveMode)
    }
    
    public func bindBufferBase(_ target: UInt32,_ index: UInt32,_ buffer: UInt32) {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glBindBufferBase(target, index, buffer)
    }
    
    
    public func bindBufferRange(_ target: UInt32,_ index: UInt32,_ buffer: UInt32,_ offset: Int,_ size: Int){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glBindBufferRange(target, index, buffer, offset, size)
    }
    
    
    public func bindSampler(_ unit: UInt32,_ sampler: UInt32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glBindSampler(unit, sampler)
    }
    
    public func bindTransformFeedback(_ target: UInt32,_ transformFeedback: UInt32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glBindTransformFeedback(target, transformFeedback)
    }
    
    public func bindVertexArray(_ vertexArray: UInt32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glBindVertexArray(vertexArray)
    }
    
    public func blitFramebuffer(_ srcX0: Int32,_ srcY0: Int32,_ srcX1: Int32,_ srcY1: Int32,
                                _ dstX0:Int32,_ dstY0: Int32,_ dstX1:Int32,_ dstY1:Int32,
                                _ mask: UInt32,_ filter: UInt32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glBlitFramebuffer(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter)
    }
    
    public func clearBufferfv(_ buffer: UInt32,_ drawbuffer: Int32,_ values: [Float32]){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var value =  values
        glClearBufferfv(buffer, drawbuffer, &value)
    }
    
    public func clearBufferiv(_ buffer: UInt32, _ drawbuffer: Int32, _ values: [Int32]){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var value =  values
        glClearBufferiv(buffer, drawbuffer, &value)
    }
    
    public func clearBufferuiv(_ buffer: UInt32, _ drawbuffer: Int32,_ values: [UInt32]){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var value =  values
        glClearBufferuiv(buffer, drawbuffer, &value)
    }
    
    public func clearBufferfi(_ buffer: UInt32,_ drawbuffer: Int32,_ depth: Float32,_ stencil: Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glClearBufferfi(buffer, drawbuffer, depth, stencil)
    }
    
    public func clientWaitSync(_ sync: GLsync,_ flags: UInt32,_ timeout: UInt64) -> Int32{
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        return Int32(glClientWaitSync(sync, flags, timeout))
    }
    
    public func compressedTexSubImage3D(_ target: UInt32,_ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32,_ imageSize: Int32,_ offset: Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glCompressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, BUFFER_OFFSET(n: Int(offset)))
    }
    
    public func compressedTexSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32,i8 srcData: [Int8], _ srcOffset: Int32, _ srcLengthOverride: Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        var size = Int32(buffer.count)
        let offset = Int32(srcOffset * Int32(SIZE_OF_BYTE))
        let overrideLength = Int32(srcLengthOverride * Int32(SIZE_OF_BYTE))
        if(srcLengthOverride == 0){
            size = size - offset
        }else if(overrideLength > size - offset) {
            
        }
        offsetI8By(&buffer, offset)
        glCompressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, overrideLength, &buffer)
    }
    
    
    
    public func compressedTexSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32,u8 srcData: [UInt8], _ srcOffset: Int32, _ srcLengthOverride: Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        var size = Int32(buffer.count)
        let offset = Int32(srcOffset * Int32(SIZE_OF_BYTE))
        let overrideLength = Int32(srcLengthOverride * Int32(SIZE_OF_BYTE))
        if(srcLengthOverride == 0){
            size = size - offset
        }else if(overrideLength > size - offset) {
            
        }
        offsetU8By(&buffer, offset)
        glCompressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, overrideLength, &buffer)
    }
    
    
    public func compressedTexSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32,i16 srcData: [Int16], _ srcOffset: Int32, _ srcLengthOverride: Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        var size = Int32(buffer.count)
        let offset = Int32(srcOffset * Int32(SIZE_OF_SHORT))
        let overrideLength = Int32(srcLengthOverride * Int32(SIZE_OF_SHORT))
        if(srcLengthOverride == 0){
            size = size - offset
        }else if(overrideLength > size - offset) {
            
        }
        offsetI16By(&buffer, offset)
        glCompressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, overrideLength, &buffer)
    }
    
    
    public func compressedTexSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32,u16 srcData: [UInt16], _ srcOffset: Int32, _ srcLengthOverride: Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        var size = Int32(buffer.count)
        let offset = Int32(srcOffset * Int32(SIZE_OF_SHORT))
        let overrideLength = Int32(srcLengthOverride * Int32(SIZE_OF_SHORT))
        if(srcLengthOverride == 0){
            size = size - offset
        }else if(overrideLength > size - offset) {
            
        }
        offsetU16By(&buffer, offset)
        glCompressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, overrideLength, &buffer)
    }
    
    
    public func compressedTexSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32,i32 srcData: [Int32], _ srcOffset: Int32, _ srcLengthOverride: Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        var size = Int32(buffer.count)
        let offset = Int32(srcOffset * Int32(SIZE_OF_INT))
        let overrideLength = Int32(srcLengthOverride * Int32(SIZE_OF_INT))
        if(srcLengthOverride == 0){
            size = size - offset
        }else if(overrideLength > size - offset) {
            
        }
        offsetI32By(&buffer, offset)
        glCompressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, overrideLength, &buffer)
    }
    
    
    public func compressedTexSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32,u32 srcData: [UInt32], _ srcOffset: Int32, _ srcLengthOverride: Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        var size = Int32(buffer.count)
        let offset = Int32(srcOffset * Int32(SIZE_OF_INT))
        let overrideLength = Int32(srcLengthOverride * Int32(SIZE_OF_INT))
        if(srcLengthOverride == 0){
            size = size - offset
        }else if(overrideLength > size - offset) {
            
        }
        offsetU32By(&buffer, offset)
        glCompressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, overrideLength, &buffer)
    }
    
    
    
    public func compressedTexSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32,f32 srcData: [Float32], _ srcOffset: Int32, _ srcLengthOverride: Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        var size = Int32(buffer.count)
        let offset = Int32(srcOffset * Int32(SIZE_OF_FLOAT))
        let overrideLength = Int32(srcLengthOverride * Int32(SIZE_OF_FLOAT))
        if(srcLengthOverride == 0){
            size = size - offset
        }else if(overrideLength > size - offset) {
            
        }
        offsetF32By(&buffer, offset)
        glCompressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, overrideLength, &buffer)
    }
    
    
    public func compressedTexSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32,f64 srcData: [Float64], _ srcOffset: Int32, _ srcLengthOverride: Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var buffer = srcData
        var size = Int32(buffer.count)
        let offset = Int32(srcOffset * Int32(SIZE_OF_DOUBLE))
        let overrideLength = Int32(srcLengthOverride * Int32(SIZE_OF_DOUBLE))
        if(srcLengthOverride == 0){
            size = size - offset
        }else if(overrideLength > size - offset) {
            
        }
        offsetF64By(&buffer, offset)
        glCompressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, overrideLength, &buffer)
    }
    
    
    enum DataType {
        case Byte
        case Int
        case Short
        case UShort
        case Float
        case Int32
        case UInt32
        case Float64
    }
    
    
    
    public func copyBufferSubData(_ readTarget: UInt32,_ writeTarget: UInt32,_ readOffset: Int,_ writeOffset: Int,_ size: Int) {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glCopyBufferSubData(readTarget, writeTarget, readOffset, writeOffset, size)
    }
    
    public func copyTexSubImage3D(_ target: UInt32, _ level: Int32, _ xoffset: Int32, _ yoffset: Int32,_ zoffset: Int32,_ x: Int32,_ y: Int32,_ width: Int32,_ height: Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        clearIfComposited()
        glCopyTexSubImage3D(target, level, xoffset, yoffset, zoffset, x, y, width, height)
    }
    
    public func createQuery() -> UInt32 {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var query = GLuint()
        glGenQueries(1, &query)
        return query
    }
    
    public func createSampler() -> UInt32 {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var sampler = GLuint()
        glGenSamplers(1, &sampler)
        return sampler
    }
    
    public func createVertexArray() -> UInt32{
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var array = GLuint()
        glGenVertexArrays(1, &array)
        return array
    }
    
    public func createTransformFeedback() -> UInt32{
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var id = GLuint()
        glGenTransformFeedbacks(1, &id)
        return id
    }
    
    public func deleteQuery(_ query: UInt32) {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var id = query
        glDeleteQueries(1, &id)
    }
    
    public func deleteSampler(_ sampler: UInt32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var id = sampler
        glDeleteQueries(1, &id)
    }
    
    public func deleteSync(_ sync: GLsync){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glDeleteSync(sync)
    }
    
    public func deleteTransformFeedback(_ transformFeedback: UInt32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var feedback = transformFeedback
        glDeleteTransformFeedbacks(1, &feedback)
    }
    
    public func deleteVertexArray(_ vertexArray: UInt32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var array = vertexArray
        glDeleteVertexArrays(1, &array)
    }
    
    public func drawArraysInstanced(_ mode: UInt32, _ first: Int32, _ count: Int32,_ instanceCount: Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        clearIfComposited()
        glDrawArraysInstanced(mode, first, count, instanceCount)
        canvas.doDraw()
    }
    
    public func drawElementsInstanced(_ mode: UInt32,_ count: Int32, _ type: UInt32,_ offset: Int,_ instanceCount: Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var indices = offset
        clearIfComposited()
        glDrawElementsInstanced(mode, count, type, &indices, instanceCount)
        canvas.doDraw()
    }
    
    public func drawRangeElements(_ mode: UInt32,_ start: UInt32,_ end: UInt32,_ count: Int32,_ type: UInt32,_ offset: Int) {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var indices = offset
        clearIfComposited()
        glDrawRangeElements(mode, start, end, count, type, &indices)
        canvas.doDraw()
    }
    
    public func drawBuffers(_ buffers: [UInt32]){
          let _ = canvas.renderer.ensureIsContextIsCurrent()
          var bufs = buffers
          glDrawBuffers(GLsizei(bufs.count), &bufs)
      }
      
    
    public func endQuery(_ target: UInt32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glEndQuery(target)
    }
    public func endTransformFeedback(){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glEndTransformFeedback()
    }
    public func fenceSync(_ condition: UInt32,_ flags: UInt32) {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glFenceSync(condition, flags)
    }
    
    public func framebufferTextureLayer(_ target: UInt32,_ attachment: UInt32, _ texture: UInt32,_ level: Int32, _ layer: Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glFramebufferTextureLayer(target, attachment, texture, level, layer)
    }
    
    public func getActiveUniformBlockName(_ program: UInt32,_ uniformBlockIndex: UInt32) -> String {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var maxNameLength = GLint()
        glGetProgramiv(program, GLenum(GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH), &maxNameLength)
        var name = Array(repeating: GLchar(), count: Int(maxNameLength))
        var length = GLint()
        glGetActiveUniformBlockName(program, uniformBlockIndex, maxNameLength, &length, &name)
        return String(cString: &name)
    }
    
    public func getActiveUniformBlockParameter(_ program: UInt32,_ uniformBlockIndex: UInt32,_ pname: UInt32) -> Any {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        switch (pname) {
        case UNIFORM_BLOCK_BINDING, UNIFORM_BLOCK_DATA_SIZE, UNIFORM_BLOCK_ACTIVE_UNIFORMS:
            var intValue = GLint()
            glGetActiveUniformBlockiv(program, uniformBlockIndex, pname, &intValue)
            return intValue
        case UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES:
            var uniformCount = GLint()
            glGetActiveUniformBlockiv(program, uniformBlockIndex, GLenum(GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS), &uniformCount)
            var indices = Array(repeating: GLint(), count: Int(uniformCount))
            glGetActiveUniformBlockiv(program, uniformBlockIndex,pname,&indices)
            return indices
        case UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER, UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER:
            var boolValue = GLint()
            glGetActiveUniformBlockiv(program, uniformBlockIndex, pname, &boolValue)
            return toBool(value: boolValue)
        default:
            return NSNull()
        }
        
    }
    
    enum ReturnType {
        case EnumType
        case UnsignedIntType
        case IntType
        case BoolType
    }
    
    public func getActiveUniforms(_ program: UInt32,_ uniformIndices: [UInt32],_ pname: UInt32) -> Any {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var returnType: ReturnType
        switch (pname) {
        case UNIFORM_TYPE:
            returnType = .EnumType
            break
        case UNIFORM_SIZE:
            returnType = .UnsignedIntType
            break;
        case UNIFORM_BLOCK_INDEX, UNIFORM_OFFSET, UNIFORM_ARRAY_STRIDE, UNIFORM_MATRIX_STRIDE:
            returnType = .IntType
            break;
        case UNIFORM_IS_ROW_MAJOR:
            returnType = .BoolType
            break;
        default:
            return NSNull()
        }
        
        var activeUniforms = GLint(-1)
        glGetProgramiv(program, GLenum(GL_ACTIVE_UNIFORMS),
                       &activeUniforms)
        
        
        let activeUniformsUnsigned = activeUniforms
        var indices = uniformIndices
        let size = indices.count
        for i in indices {
            if (i >= activeUniformsUnsigned) {
                return NSNull()
            }
        }
        
        var result = Array(repeating: GLint(), count: size)
        glGetActiveUniformsiv(program, GLsizei(indices.count),
                              &indices, pname, &result)
        
        switch (returnType) {
        case .EnumType, .UnsignedIntType, .IntType:
            return result
        case .BoolType:
            return fromGLint(value: result)
        }
        
    }
    
    
    public func getBufferSubData(_ target: UInt32,_ srcByteOffset: Int,_ dstData: Data,_ dstOffset: Int32,_ length: Int32) {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        if(length == 0){
            
        }
        var data = [UInt8](dstData)
        
        let size = data.count
        let typeSize = SIZE_OF_BYTE
        var byteLength = 0
        if (length > 0) {
            // type size is at most 8, so no overflow.
            byteLength = Int(length) * typeSize
        }
        var byteOffset = 0
        if (dstOffset > 0) {
            // type size is at most 8, so no overflow.
            byteOffset = Int(dstOffset) * typeSize
        }
        var total = byteOffset
        total += byteLength;
        if (total > size) {
            return
        }
        if (byteLength == 0) {
            byteLength = size - byteOffset
        }
        
        
        // var offset = srcByteOffset * SIZE_OF_BYTE
        glBufferSubData(target, byteOffset, byteLength, &data)
    }
    
    
    public func getFragDataLocation(_ program: UInt32,_ name: String)-> Int32{
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        let value = (name as NSString).utf8String
        return glGetFragDataLocation(program, value)
    }
    var m_boundIndexedTransformFeedbackBuffers: [UInt32] = []
    var m_boundIndexedUniformBuffers: [UInt32] = []
    
    public func getIndexedParameter(_ target: UInt32,_ index: UInt32) -> Any{
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        let binding = TNSIndexedParameter()
        switch (target) {
        case TRANSFORM_FEEDBACK_BUFFER_BINDING,UNIFORM_BUFFER_BINDING:
            
            var newTarget = GLint()
            glGetIntegerv(GLenum(target), &newTarget)
            // NO BINDING RETURN
            if (newTarget == 0) {
                return NSNull()
            }
            var buffer = GLint()
            
            glGetIntegeri_v(GLenum(newTarget), index, &buffer)
            binding._bufferValue = UInt32(buffer)
            binding._isBuffer = true
            
            return binding
        case TRANSFORM_FEEDBACK_BUFFER_SIZE, TRANSFORM_FEEDBACK_BUFFER_START, UNIFORM_BUFFER_SIZE,UNIFORM_BUFFER_START:
            var value = GLint64(-1);
            glGetInteger64i_v(target, index, &value)
            binding._isBuffer = false
            binding._value = value
            return binding
        default:
            return NSNull()
        }
    }
    
    public func getInternalformatParameter(_ target: UInt32,_ internalformat: UInt32,_ pname: UInt32)-> Any {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        switch (internalformat) {
            // Renderbuffer doesn't support unsized internal formats,
        // though GL_RGB and GL_RGBA are color-renderable.
        case RGB,
             RGBA,
             // Multisampling is not supported for signed and unsigned integer internal
        // formats.
        R8UI,
        R8I,
        R16UI,
        R16I,
        R32UI,
        R32I,
        RG8UI,
        RG8I,
        RG16UI,
        RG16I,
        RG32UI,
        RG32I,
        RGBA8UI,
        RGBA8I,
        RGB10_A2UI,
        RGBA16UI,RGBA16I,RGBA32UI,RGBA32I:
            return Array(repeating: Int32(), count: 0)
        case R8,RG8,RG8,RGB565,RGBA8,SRGB8_ALPHA8,RGB5_A1,
             RGBA4,RGB10_A2,DEPTH_COMPONENT16,DEPTH_COMPONENT24,
             DEPTH_COMPONENT32F,
             DEPTH24_STENCIL8,
             DEPTH32F_STENCIL8,
             STENCIL_INDEX8:
            break;
        case R16F,RG16F,RG16F,R32F,RG32F,RGBA32F,R11F_G11F_B10F:
            break
        default:
            return NSNull()
        }
        
        
        switch (pname) {
        case SAMPLES:
            var length = GLint(-1)
            glGetInternalformativ(target,internalformat,
                                  GLenum(GL_NUM_SAMPLE_COUNTS), 1, &length)
            if (length <= 0){
                return Array(repeating: GLint(), count: 0)
            }
            var values = Array(repeating: GLint(), count: Int(length))
            glGetInternalformativ(target, internalformat, pname, length, &values)
            return values
        default:
            return NSNull()
        }
    }
    
    public override func getParameter(_ pname: UInt32) -> Any?{
        canvas.renderer.ensureIsContextIsCurrent()
        switch pname {
        case COPY_READ_BUFFER_BINDING, COPY_WRITE_BUFFER_BINDING, DRAW_FRAMEBUFFER_BINDING:
            var param = GLint()
            glGetIntegerv(GLenum(pname), &param)
            
            if((pname == COPY_READ_BUFFER_BINDING || pname == COPY_WRITE_BUFFER_BINDING || pname == DRAW_FRAMEBUFFER_BINDING) && param == 0){
                return nil
            }
            return param
        default:
            return super.getParameter(pname)
        }
    }
    
    public func getQuery(_ target: UInt32, _ pname: UInt32)-> Any {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        if(pname == GL_CURRENT_QUERY){
            var params = GLint()
            glGetQueryiv(target, pname, &params)
            return params
        }
        return NSNull()
    }
    
    public func getQueryParameter(_ query: UInt32,_ pname: UInt32) -> Any {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var params = GLuint()
        glGetQueryObjectuiv(query, pname, &params)
        switch (pname) {
        case QUERY_RESULT:
            return params == GL_TRUE
        case QUERY_RESULT_AVAILABLE:
            return params
        default:
            return NSNull()
        }
    }
    
    public func getSamplerParameter(_ sampler: UInt32,_ pname: UInt32) -> Any {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        switch (pname) {
        case TEXTURE_MAX_LOD, TEXTURE_MIN_LOD:
            var floatValue = GLfloat()
            glGetSamplerParameterfv(sampler, pname, &floatValue)
            return floatValue
        case TEXTURE_COMPARE_FUNC,TEXTURE_COMPARE_MODE,TEXTURE_MAG_FILTER,TEXTURE_MIN_FILTER,TEXTURE_WRAP_R,TEXTURE_WRAP_S,TEXTURE_WRAP_T:
            var intValue = GLint()
            glGetSamplerParameteriv(sampler, pname, &intValue)
            return intValue
        default:
            return NSNull()
        }
    }
    
    public func getSyncParameter(_ sync: GLsync,_ pname: UInt32) -> Any {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        switch(pname) {
        case OBJECT_TYPE, SYNC_STATUS,SYNC_CONDITION,SYNC_FLAGS:
            var values = GLint()
            var length = GLint(-1)
            glGetSynciv(sync, pname, 1, &length, &values)
            return values
        default:
            return NSNull()
        }
    }
    
    public func getTransformFeedbackVarying(_ program: UInt32,_ index: UInt32) -> Any {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        // let info = TNSWebGLActiveInfo()
        var maxIndex = GLint()
        glGetProgramiv(program,GLenum(GL_TRANSFORM_FEEDBACK_VARYINGS), &maxIndex)
        if (index >= maxIndex) {
            return NSNull()
        }
        
        var maxNameLength = GLint(-1)
        glGetProgramiv(program,GLenum(GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH),
                       &maxNameLength)
        if (maxNameLength <= 0) {
            return NSNull()
        }
        var name = Array(repeating: GLchar(), count: Int(maxNameLength))
        var length = GLsizei()
        var size = GLsizei()
        var type = GLenum()
        glGetTransformFeedbackVarying(program, index,
                                      maxNameLength, &length, &size, &type,
                                      &name);
        
        if (length == 0 || size == 0 || type == 0) {
            return NSNull();
        }
        
        return TNSWebGLActiveInfo(name: String(cString: &name), size: size, type: type )
    }
    
    public func getUniformBlockIndex(_ program: UInt32,_ uniformBlockName: String) -> UInt32 {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        let name = (uniformBlockName as NSString).utf8String
        return glGetUniformBlockIndex(program, name)
    }
    
    
    public func getUniformIndices(_ program: UInt32,_ uniformNames: [String]) -> [UInt32]{
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var indices = Array(repeating: UInt32(), count: uniformNames.count)
        var names = uniformNames.map { name -> UnsafePointer<Int8>? in
            return (name as NSString).utf8String
        }
        glGetUniformIndices(program, GLsizei(uniformNames.count), &names, &indices)
        return indices
    }
    
    public func invalidateFramebuffer(_ target: UInt32,_ attachments: [UInt32]) {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var attach = attachments
        glInvalidateFramebuffer(target, GLsizei(attachments.count), &attach)
    }
    
    public func invalidateSubFramebuffer(_ target: UInt32, _ attachments: [UInt32], _ x: Int32,_ y: Int32,_ width: Int32,_ height: Int32) {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var attach = attachments
        glInvalidateSubFramebuffer(target, GLsizei(attachments.count), &attach, x, y, width, height)
    }
    
    public func isQuery(_ query: UInt32) -> Bool {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        return fromGLboolean(value: glIsQuery(query))
    }
    
    public func isSampler(_ sampler: UInt32) -> Bool {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        return fromGLboolean(value: glIsSampler(sampler))
    }
    
    public func isSync(_ sync: GLsync) -> Bool {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        return fromGLboolean(value: glIsSync(sync))
    }
    
    public func isTransformFeedback(_ transformFeedback: UInt32) -> Bool {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        return fromGLboolean(value: glIsTransformFeedback(transformFeedback))
    }
    
    public func isVertexArray(_ vertexArray: UInt32) -> Bool {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        return fromGLboolean(value: glIsVertexArray(vertexArray))
    }
    
    public func pauseTransformFeedback(){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glPauseTransformFeedback()
    }
    public func readBuffer(_ src: UInt32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glReadBuffer(src)
    }
    
    public func renderbufferStorageMultisample(_ target:UInt32, _ samples:Int32, _ internalFormat:UInt32, _ width:Int32, _ height:Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glRenderbufferStorageMultisample(target, samples, internalFormat, width, height)
    }
    
    public func resumeTransformFeedback(){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glResumeTransformFeedback()
    }
    
    public func samplerParameteri(_ sampler: UInt32,_ pname: UInt32,_ param: Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glSamplerParameteri(sampler, pname, param)
    }
    
    public func samplerParameterf(_ sampler: UInt32,_ pname: UInt32,_ param: Float){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glSamplerParameterf(sampler, pname, param)
    }
    
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32, offset: Int) {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var pixels = offset
        glTexImage3D(target, level, internalformat, width, height, depth, border,format, type, &pixels)
    }
    
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,i8 source: [Int8]) {
        texImage3D(target, level, internalformat, width, height, depth, border, format, type, i8: source, 0)
    }
    
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,u8 source: [UInt8]) {
        texImage3D(target, level, internalformat, width, height, depth, border, format, type, u8: source, 0)
    }
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,i16 source: [Int16]) {
        texImage3D(target, level, internalformat, width, height, depth, border, format, type, i16: source, 0)
    }
    
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,u16 source: [UInt16]) {
        texImage3D(target, level, internalformat, width, height, depth, border, format, type, u16: source, 0)
    }
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,i32 source: [Int32]) {
        texImage3D(target, level, internalformat, width, height, depth, border, format, type, i32: source, 0)
    }
    
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,u32 source: [UInt32]) {
        texImage3D(target, level, internalformat, width, height, depth, border, format, type, u32: source, 0)
    }
    
    
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,f32 source: [Float32]) {
        texImage3D(target, level, internalformat, width, height, depth, border, format, type, f32: source, 0)
    }
    
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,f64 source: [Float64]) {
        texImage3D(target, level, internalformat, width, height, depth, border, format, type, f64: source, 0)
    }
    
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,i8 source: [Int8], _ srcOffset:Int32) {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var pixels = source
        var buff = pixels.withUnsafeMutableBytes {$0.load(as: UInt8.self)}
        offsetU8By(&buff, srcOffset)
        if(flipYWebGL){
            GLUtils.flipYInPlace3D(&buff, Int(pixels.count), Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        glTexImage3D(target, level, internalformat, width, height, depth, border,format,type, &buff)
    }
    
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,u8 source: [UInt8], _ srcOffset:Int32) {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var pixels = source
        offsetU8By(&pixels, srcOffset)
        if(flipYWebGL){
            GLUtils.flipYInPlace3D(&pixels, Int(source.count), Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        glTexImage3D(target, level, internalformat, width, height, depth, border,format,type, &pixels)
    }
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,i16 source: [Int16], _ srcOffset:Int32) {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var pixels = source
        var buff = pixels.withUnsafeMutableBytes {$0.load(as: UInt8.self)}
        offsetU8By(&buff, srcOffset)
        if(flipYWebGL){
            GLUtils.flipYInPlace3D(&buff, Int(pixels.count * SIZE_OF_SHORT), Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        glTexImage3D(target, level, internalformat, width, height, depth, border,format,type, &buff)
    }
    
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,u16 source: [UInt16], _ srcOffset:Int32) {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var pixels = source
        var buff = pixels.withUnsafeMutableBytes {$0.load(as: UInt8.self)}
        offsetU8By(&buff, srcOffset)
        if(flipYWebGL){
            GLUtils.flipYInPlace3D(&buff, Int(pixels.count * SIZE_OF_SHORT), Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        glTexImage3D(target, level, internalformat, width, height, depth, border,format,type,&buff)
    }
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,i32 source: [Int32], _ srcOffset:Int32) {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var pixels = source
        var buff = pixels.withUnsafeMutableBytes {$0.load(as: UInt8.self)}
        offsetU8By(&buff, srcOffset)
        if(flipYWebGL){
            GLUtils.flipYInPlace3D(&buff, Int(pixels.count * SIZE_OF_INT), Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        glTexImage3D(target, level, internalformat, width, height, depth, border,format,type, &buff)
    }
    
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,u32 source: [UInt32], _ srcOffset:Int32) {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var pixels = source
        var buff = pixels.withUnsafeMutableBytes {$0.load(as: UInt8.self)}
        offsetU8By(&buff, srcOffset)
        if(flipYWebGL){
            GLUtils.flipYInPlace3D(&buff, Int(pixels.count * SIZE_OF_INT), Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        glTexImage3D(target, level, internalformat, width, height, depth, border,format,type,&buff)
    }
    
    
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,f32 source: [Float32], _ srcOffset:Int32) {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var pixels = source
        var buff = pixels.withUnsafeMutableBytes {$0.load(as: UInt8.self)}
        offsetU8By(&buff, srcOffset)
        if(flipYWebGL){
             GLUtils.flipYInPlace3D(&buff, Int(pixels.count * SIZE_OF_FLOAT), Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        glTexImage3D(target, level, internalformat, width, height, depth, border,format,type, &buff)
    }
    
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32,f64 source: [Float64], _ srcOffset:Int32) {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var pixels = source
        var buff = pixels.withUnsafeMutableBytes {$0.load(as: UInt8.self)}
        offsetU8By(&buff, srcOffset)
        if(flipYWebGL){
           GLUtils.flipYInPlace3D(&buff, Int(pixels.count * SIZE_OF_DOUBLE), Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        glTexImage3D(target, level, internalformat, width, height, depth, border,format,type,&buff)
    }
    
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32, source: UIImage) {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        let (buffer, size) = GLUtils.getBytesFromImage(pixels: source)
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        if(flipYWebGL){
            GLUtils.flipYInPlace3D(buffer?.assumingMemoryBound(to: UInt8.self), size, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        
        glTexImage3D(target, level, internalformat, width, height, depth, border, format, type, buffer)
        buffer?.deallocate()
    }
    
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ border: Int32,_ format: UInt32,_ type: UInt32, asset: TNSImageAsset) {
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        gl_tex_image_3D_asset(target, level, internalformat, width, height, depth, border, format, target, asset.asset, flipYWebGL)
    }
    
    public func texImage3D(_ target: UInt32,_ level: Int32,_ internalformat: Int32,_ width: Int32, _ height: Int32,_ depth: Int32, _ border: Int32,_ format: UInt32,_ type: UInt32, canvas: TNSCanvas) {
        var snapshot = canvas.snapshot()
        let _ = self.canvas.renderer.ensureIsContextIsCurrent()
        if(flipYWebGL){
            GLUtils.flipYInPlace3D(&snapshot, snapshot.count, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        glTexImage3D(target, level, internalformat, width, height, depth, border, format, type, &snapshot)
    }
    
    
    
    public func texStorage2D(_ target: UInt32,_ levels: Int32,_ internalformat: UInt32,_ width: Int32,_ height: Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glTexStorage2D(target, levels, internalformat, width, height)
    }
    
    public func texStorage3D(_ target: UInt32,_ levels: Int32,_ internalformat: UInt32,_ width: Int32,_ height: Int32,_ depth: Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glTexStorage3D(target, levels, internalformat, width, height, depth)
    }
    
    
    
    public func texSubImage3D(_ target: UInt32,_ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ zoffset: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32 ,offset: Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var pixels = offset
        glTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, &pixels)
    }
    
    
    
    public func texSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32, _ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32,i8 srcData: [Int8]){
        texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format,type, i8: srcData, 0)
    }
    
    public func texSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32, _ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32,u8 srcData: [UInt8]){
        texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format,type, u8: srcData, 0)
    }
    
    
    public func texSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32, _ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32,i16 srcData: [Int16]){
        texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format,type, i16: srcData, 0)
    }
    
    public func texSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32, _ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32,u16 srcData: [UInt16]){
        texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format,type, u16: srcData,  0)
    }
    
    public func texSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32, _ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32,i32 srcData: [Int32]){
        texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format,type, i32: srcData,  0)
    }
    
    public func texSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32, _ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32,u32 srcData: [UInt32]){
        texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format,type, u32: srcData, 0)
    }
    
    public func texSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32, _ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32,f32 srcData: [Float32]){
        texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format,type, f32: srcData, 0)
    }
    
    public func texSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32, _ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32,f64 srcData: [Float64]){
        texSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format,type, f64: srcData,0)
    }
    
    
    
    public func texSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32, _ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32,i8 srcData: [Int8],_ srcOffset:Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var pixels = srcData
        var buf = pixels.withUnsafeMutableBytes{$0.load(as: UInt8.self)}
        offsetU8By(&buf, srcOffset)
        if(flipYWebGL){
            GLUtils.flipYInPlace3D(&buf, srcData.count, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        glTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, pixels)
    }
    
    public func texSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32, _ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32,u8 srcData: [UInt8],_ srcOffset:Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var pixels = srcData
        var buf = pixels.withUnsafeMutableBytes{$0.load(as: UInt8.self)}
        offsetU8By(&buf, srcOffset)
        if(flipYWebGL){
            GLUtils.flipYInPlace3D(&buf, srcData.count, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        glTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, pixels)
    }
    
    
    public func texSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32, _ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32,i16 srcData: [Int16],_ srcOffset:Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var pixels = srcData
        var buf = pixels.withUnsafeMutableBytes{$0.load(as: UInt8.self)}
        offsetU8By(&buf, srcOffset)
        if(flipYWebGL){
            GLUtils.flipYInPlace3D(&buf,Int(srcData.count * SIZE_OF_SHORT), Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        glTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, pixels)
    }
    
    public func texSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32, _ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32,u16 srcData: [UInt16],_ srcOffset:Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var pixels = srcData
        var buf = pixels.withUnsafeMutableBytes{$0.load(as: UInt8.self)}
        offsetU8By(&buf, srcOffset)
        if(flipYWebGL){
           GLUtils.flipYInPlace3D(&buf,Int(srcData.count * SIZE_OF_SHORT), Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        glTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, pixels)
    }
    
    public func texSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32, _ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32,i32 srcData: [Int32],_ srcOffset:Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var pixels = srcData
        var buf = pixels.withUnsafeMutableBytes{$0.load(as: UInt8.self)}
        offsetU8By(&buf, srcOffset)
        if(flipYWebGL){
            GLUtils.flipYInPlace3D(&buf,Int(srcData.count * SIZE_OF_INT), Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        glTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, pixels)
    }
    
    public func texSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32, _ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32,u32 srcData: [UInt32],_ srcOffset:Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var pixels = srcData
        var buf = pixels.withUnsafeMutableBytes{$0.load(as: UInt8.self)}
        offsetU8By(&buf, srcOffset)
        if(flipYWebGL){
           GLUtils.flipYInPlace3D(&buf,Int(srcData.count * SIZE_OF_INT), Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        glTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, pixels)
    }
    
    public func texSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32, _ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32,f32 srcData: [Float32],_ srcOffset:Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var pixels = srcData
        var buf = pixels.withUnsafeMutableBytes{$0.load(as: UInt8.self)}
        offsetU8By(&buf, srcOffset)
        if(flipYWebGL){
            GLUtils.flipYInPlace3D(&buf,Int(srcData.count * SIZE_OF_FLOAT), Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        glTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, pixels)
    }
    
    public func texSubImage3D(_ target: UInt32, _ level: Int32,_ xoffset: Int32, _ yoffset: Int32,_ zoffset: Int32,_ width: Int32, _ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32,f64 srcData: [Float64],_ srcOffset:Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var pixels = srcData
        var buf = pixels.withUnsafeMutableBytes{$0.load(as: UInt8.self)}
        offsetU8By(&buf, srcOffset)
        if(flipYWebGL){
            GLUtils.flipYInPlace3D(&buf,Int(srcData.count * SIZE_OF_DOUBLE), Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        glTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, pixels)
    }
    
    public func texSubImage3D(_ target: UInt32,_ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ zoffset: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32, srcData: UIImage){
        let (buffer, size) = GLUtils.getBytesFromImage(pixels: srcData)
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        _ = Int(srcData.size.width)
        _ = Int(srcData.size.height)
        
        if(flipYWebGL){
             GLUtils.flipYInPlace3D(buffer?.assumingMemoryBound(to: UInt8.self),size, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        glTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, buffer)
        buffer?.deallocate()
    }
    
    
    public func texSubImage3D(_ target: UInt32,_ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ zoffset: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32, canvas: TNSCanvas){
        var snapshot = canvas.snapshot()
        let _ = self.canvas.renderer.ensureIsContextIsCurrent()
        if(flipYWebGL){
            GLUtils.flipYInPlace3D(&snapshot,snapshot.count, Int(width * bytes_per_pixel(pixel_type: type, format: format)), Int(height), Int(depth))
        }
        glTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, &snapshot)
    }
    
    
    
    public func texSubImage3D(_ target: UInt32,_ level: Int32,_ xoffset: Int32,_ yoffset: Int32,_ zoffset: Int32,_ width: Int32,_ height: Int32,_ depth: Int32,_ format: UInt32, _ type: UInt32, asset: TNSImageAsset){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        gl_tex_sub_image_3D_asset(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, asset.asset, flipYWebGL)
    }
    
    
    public func transformFeedbackVaryings(_ program: UInt32,_ varyings: [String],_ bufferMode: UInt32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var varys = varyings.map { name -> UnsafePointer<Int8>? in
            return (name as NSString).utf8String
        }
        glTransformFeedbackVaryings(program, GLsizei(varyings.count), &varys, bufferMode)
    }
    
    public func uniform1ui(_ location: Int32,_ v0: UInt32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glUniform1ui(location, v0)
    }
    public func uniform2ui(_ location: Int32, _ v0: UInt32,_ v1: UInt32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glUniform2ui(location, v0, v1)
    }
    public func uniform3ui(_ location: Int32,_ v0: UInt32,_ v1: UInt32,_ v2: UInt32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glUniform3ui(location, v0, v1, v2)
    }
    public func uniform4ui(_ location: Int32,_ v0: UInt32,_ v1: UInt32,_ v2: UInt32,_ v3: UInt32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glUniform4ui(location, v0, v1, v2,v3)
    }
    
    public func uniform1uiv(_ location: Int32,_ data: Data){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var locations = Array(data.withUnsafeBytes{$0.bindMemory(to: UInt32.self)})
        glUniform1uiv(location, GLsizei(data.count), &locations)
    }
    
    public func uniform2uiv(_ location: Int32,_ data: Data){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var locations = Array(data.withUnsafeBytes{$0.bindMemory(to: UInt32.self)})
        glUniform2uiv(location, GLsizei(data.count), &locations)
    }
    public func uniform3uiv(_ location: Int32, _ data: Data){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var locations = Array(data.withUnsafeBytes{$0.bindMemory(to: UInt32.self)})
        glUniform3uiv(location, GLsizei(data.count), &locations)
    }
    
    public func uniform4uiv(_ location: Int32,_ data: Data){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var locations = Array(data.withUnsafeBytes{$0.bindMemory(to: UInt32.self)})
        glUniform4uiv(location, GLsizei(data.count), &locations)
    }
    
    public func uniformBlockBinding(_ program: UInt32,_ uniformBlockIndex: UInt32,_ uniformBlockBinding: UInt32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glUniformBlockBinding(program, uniformBlockIndex, uniformBlockBinding)
    }
    public func uniformMatrix3x2fv(_ location: UInt32,_ transpose: Bool,_ data: Data){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var value = Array(data.withUnsafeBytes{$0.bindMemory(to: Float32.self)})
        glUniformMatrix3x2fv(GLint(location), GLsizei(data.count), boolConverter(transpose), &value)
    }
    public func uniformMatrix4x2fv(_ location: UInt32,_ transpose: Bool,_ data: Data){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var value = Array(data.withUnsafeBytes{$0.bindMemory(to: Float32.self)})
        glUniformMatrix4x2fv(GLint(location), GLsizei(data.count), boolConverter(transpose), &value)
    }
    
    public func uniformMatrix2x3fv(_ location: UInt32,_ transpose: Bool,_ data: Data){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var value = Array(data.withUnsafeBytes{$0.bindMemory(to: Float32.self)})
        glUniformMatrix2x3fv(GLint(location), GLsizei(data.count), boolConverter(transpose), &value)
    }
    public func uniformMatrix4x3fv(_ location: UInt32,_ transpose: Bool,_ data: Data){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var value = Array(data.withUnsafeBytes{$0.bindMemory(to: Float32.self)})
        glUniformMatrix4x3fv(GLint(location), GLsizei(data.count), boolConverter(transpose), &value)
    }
    
    public func uniformMatrix2x4fv(_ location: UInt32,_ transpose: Bool,_ data: Data){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var value = Array(data.withUnsafeBytes{$0.bindMemory(to: Float32.self)})
        glUniformMatrix2x4fv(GLint(location), GLsizei(data.count), boolConverter(transpose), &value)
    }
    
    public func uniformMatrix3x4fv(_ location: UInt32,_ transpose: Bool,_ data: Data){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var value = Array(data.withUnsafeBytes{$0.bindMemory(to: Float32.self)})
        glUniformMatrix3x4fv(GLint(location), GLsizei(data.count), boolConverter(transpose), &value)
    }
    public func vertexAttribDivisor(_ index: UInt32,_ divisor: UInt32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glVertexAttribDivisor(index, divisor)
    }
    public func vertexAttribI4i(_ index: UInt32,_ v0: Int32,_ v1: Int32,_ v2: Int32,_ v3: Int32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glVertexAttribI4i(index, v0, v1, v2, v3)
    }
    
    public func vertexAttribI4ui(_ index: UInt32,_ v0: UInt32,_ v1: UInt32,_ v2: UInt32,_ v3: UInt32){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        glVertexAttribI4ui(index, v0, v1, v2, v3)
    }
    
    public func vertexAttribI4iv(_ index: UInt32,_ value: Data){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var v = Array(value.withUnsafeBytes{$0.bindMemory(to: Int32.self)})
        glVertexAttribI4iv(index, &v)
    }
    
    public func vertexAttribI4uiv(_ index: UInt32,_ value: Data){
        let _ = canvas.renderer.ensureIsContextIsCurrent()
        var v = Array(value.withUnsafeBytes{$0.bindMemory(to: UInt32.self)})
        glVertexAttribI4uiv(index, &v)
    }
    
    /* Getting GL parameter information */
    public var READ_BUFFER :UInt32 { return UInt32(GL_READ_BUFFER) }
    
    public var UNPACK_ROW_LENGTH :UInt32 { return UInt32(GL_UNPACK_ROW_LENGTH) }
    
    public var UNPACK_SKIP_ROWS :UInt32 { return UInt32(GL_UNPACK_SKIP_ROWS) }
    
    public var UNPACK_SKIP_PIXELS :UInt32 { return UInt32(GL_UNPACK_SKIP_PIXELS) }
    
    public var PACK_ROW_LENGTH :UInt32 { return UInt32(GL_PACK_ROW_LENGTH) }
    
    public var PACK_SKIP_ROWS :UInt32 { return UInt32(GL_PACK_SKIP_ROWS) }
    
    public var PACK_SKIP_PIXELS :UInt32 { return UInt32(GL_PACK_SKIP_PIXELS) }
    
    public var TEXTURE_BINDING_3D :UInt32 { return UInt32(GL_TEXTURE_BINDING_3D) }
    
    public var UNPACK_SKIP_IMAGES :UInt32 { return UInt32(GL_UNPACK_SKIP_IMAGES) }
    
    public var UNPACK_IMAGE_HEIGHT :UInt32 { return UInt32(GL_UNPACK_IMAGE_HEIGHT) }
    
    public var MAX_3D_TEXTURE_SIZE :UInt32 { return UInt32(GL_MAX_3D_TEXTURE_SIZE) }
    
    public var MAX_ELEMENTS_VERTICES :UInt32 { return UInt32(GL_MAX_ELEMENTS_VERTICES) }
    
    public var MAX_ELEMENTS_INDICES :UInt32 { return UInt32(GL_MAX_ELEMENTS_INDICES) }
    
    public var MAX_TEXTURE_LOD_BIAS :UInt32 { return UInt32(GL_MAX_TEXTURE_LOD_BIAS) }
    
    public var MAX_FRAGMENT_UNIFORM_COMPONENTS :UInt32 { return UInt32(GL_MAX_FRAGMENT_UNIFORM_COMPONENTS) }
    
    public var MAX_VERTEX_UNIFORM_COMPONENTS :UInt32 { return UInt32(GL_MAX_VERTEX_UNIFORM_COMPONENTS) }
    
    public var MAX_ARRAY_TEXTURE_LAYERS :UInt32 { return UInt32(GL_MAX_ARRAY_TEXTURE_LAYERS) }
    
    public var MIN_PROGRAM_TEXEL_OFFSET :UInt32 { return UInt32(GL_MIN_PROGRAM_TEXEL_OFFSET) }
    
    public var MAX_PROGRAM_TEXEL_OFFSET :UInt32 { return UInt32(GL_MAX_PROGRAM_TEXEL_OFFSET) }
    
    public var MAX_VARYING_COMPONENTS :UInt32 { return UInt32(GL_MAX_VARYING_COMPONENTS) }
    
    public var FRAGMENT_SHADER_DERIVATIVE_HINT :UInt32 { return UInt32(GL_FRAGMENT_SHADER_DERIVATIVE_HINT) }
    
    public var RASTERIZER_DISCARD :UInt32 { return UInt32(GL_RASTERIZER_DISCARD) }
    
    public var VERTEX_ARRAY_BINDING :UInt32 { return UInt32(GL_VERTEX_ARRAY_BINDING) }
    
    public var MAX_VERTEX_OUTPUT_COMPONENTS :UInt32 { return UInt32(GL_MAX_VERTEX_OUTPUT_COMPONENTS) }
    
    public var MAX_FRAGMENT_INPUT_COMPONENTS :UInt32 { return UInt32(GL_MAX_FRAGMENT_INPUT_COMPONENTS) }
    
    public var MAX_SERVER_WAIT_TIMEOUT :UInt32 { return UInt32(GL_MAX_SERVER_WAIT_TIMEOUT) }
    
    public var MAX_ELEMENT_INDEX :UInt32 { return UInt32(GL_MAX_ELEMENT_INDEX) }
    /* Getting GL parameter information */
    
    
    /* Textures */
    
    public var RED :UInt32 { return UInt32(GL_RED) }
    
    public var RGB8 :UInt32 { return UInt32(GL_RGB8) }
    
    public var RGBA8 :UInt32 { return UInt32(GL_RGBA8) }
    
    public var RGB10_A2 :UInt32 { return UInt32(GL_RGB10_A2) }
    
    public var TEXTURE_3D :UInt32 { return UInt32(GL_TEXTURE_3D) }
    
    public var TEXTURE_WRAP_R :UInt32 { return UInt32(GL_TEXTURE_WRAP_R) }
    
    public var TEXTURE_MIN_LOD :UInt32 { return UInt32(GL_TEXTURE_MIN_LOD) }
    
    public var TEXTURE_MAX_LOD :UInt32 { return UInt32(GL_TEXTURE_MAX_LOD) }
    
    public var TEXTURE_BASE_LEVEL :UInt32 { return UInt32(GL_TEXTURE_BASE_LEVEL) }
    
    public var TEXTURE_MAX_LEVEL :UInt32 { return UInt32(GL_TEXTURE_MAX_LEVEL) }
    
    public var TEXTURE_COMPARE_MODE :UInt32 { return UInt32(GL_TEXTURE_COMPARE_MODE) }
    
    public var TEXTURE_COMPARE_FUNC :UInt32 { return UInt32(GL_TEXTURE_COMPARE_FUNC) }
    
    public var SRGB :UInt32 { return UInt32(GL_SRGB) }
    
    public var SRGB8 :UInt32 { return UInt32(GL_SRGB8) }
    
    public var SRGB8_ALPHA8 :UInt32 { return UInt32(GL_SRGB8_ALPHA8) }
    
    public var COMPARE_REF_TO_TEXTURE :UInt32 { return UInt32(GL_COMPARE_REF_TO_TEXTURE) }
    
    public var RGBA32F :UInt32 { return UInt32(GL_RGBA32F) }
    
    public var RGB32F :UInt32 { return UInt32(GL_RGB32F) }
    
    public var RGBA16F :UInt32 { return UInt32(GL_RGBA16F) }
    
    public var RGB16F :UInt32 { return UInt32(GL_RGB16F) }
    
    public var TEXTURE_2D_ARRAY :UInt32 { return UInt32(GL_TEXTURE_2D_ARRAY) }
    
    public var TEXTURE_BINDING_2D_ARRAY :UInt32 { return UInt32(GL_TEXTURE_BINDING_2D_ARRAY) }
    
    public var R11F_G11F_B10F :UInt32 { return UInt32(GL_R11F_G11F_B10F) }
    
    public var RGB9_E5 :UInt32 { return UInt32(GL_RGB9_E5) }
    
    public var RGBA32UI :UInt32 { return UInt32(GL_RGBA32UI) }
    
    public var RGB32UI :UInt32 { return UInt32(GL_RGB32UI) }
    
    public var RGBA16UI :UInt32 { return UInt32(GL_RGBA16UI) }
    
    public var RGB16UI :UInt32 { return UInt32(GL_RGB16UI) }
    
    public var RGBA8UI :UInt32 { return UInt32(GL_RGBA8UI) }
    
    public var RGB8UI :UInt32 { return UInt32(GL_RGB8UI) }
    
    public var RGBA32I :UInt32 { return UInt32(GL_RGBA32I) }
    
    public var RGB32I :UInt32 { return UInt32(GL_RGB32I) }
    
    public var RGBA16I :UInt32 { return UInt32(GL_RGBA16I) }
    
    public var RGB16I :UInt32 { return UInt32(GL_RGB16I) }
    
    public var RGBA8I :UInt32 { return UInt32(GL_RGBA8I) }
    
    public var RGB8I :UInt32 { return UInt32(GL_RGB8I) }
    
    public var RED_INTEGER :UInt32 { return UInt32(GL_RED_INTEGER) }
    
    public var RGB_INTEGER :UInt32 { return UInt32(GL_RGB_INTEGER) }
    
    public var RGBA_INTEGER :UInt32 { return UInt32(GL_RGBA_INTEGER) }
    
    public var R8 :UInt32 { return UInt32(GL_R8) }
    
    public var RG8 :UInt32 { return UInt32(GL_RG8) }
    
    public var R16F :UInt32 { return UInt32(GL_R16F) }
    
    public var R32F :UInt32 { return UInt32(GL_R32F) }
    
    public var RG16F :UInt32 { return UInt32(GL_RG16F) }
    
    public var RG32F :UInt32 { return UInt32(GL_RG32F) }
    
    public var R8I :UInt32 { return UInt32(GL_R8I) }
    
    public var R8UI :UInt32 { return UInt32(GL_R8UI) }
    
    public var R16I :UInt32 { return UInt32(GL_R16I) }
    
    public var R16UI :UInt32 { return UInt32(GL_R16UI) }
    
    public var R32I :UInt32 { return UInt32(GL_R32I) }
    
    public var R32UI :UInt32 { return UInt32(GL_R32UI) }
    
    public var RG8I :UInt32 { return UInt32(GL_RG8I) }
    
    public var RG8UI :UInt32 { return UInt32(GL_RG8UI) }
    
    public var RG16I :UInt32 { return UInt32(GL_RG16I) }
    
    public var RG16UI :UInt32 { return UInt32(GL_RG16UI) }
    
    public var RG32I :UInt32 { return UInt32(GL_RG32I) }
    
    public var RG32UI :UInt32 { return UInt32(GL_RG32UI) }
    
    public var R8_SNORM :UInt32 { return UInt32(GL_R8_SNORM) }
    
    public var RG8_SNORM :UInt32 { return UInt32(GL_RG8_SNORM) }
    
    public var RGB8_SNORM :UInt32 { return UInt32(GL_RGB8_SNORM) }
    
    public var RGBA8_SNORM :UInt32 { return UInt32(GL_RGBA8_SNORM) }
    
    public var RGB10_A2UI :UInt32 { return UInt32(GL_RGB10_A2UI) }
    
    public var TEXTURE_IMMUTABLE_FORMAT :UInt32 { return UInt32(GL_TEXTURE_IMMUTABLE_FORMAT) }
    
    public var TEXTURE_IMMUTABLE_LEVELS :UInt32 { return UInt32(GL_TEXTURE_IMMUTABLE_LEVELS) }
    
    /* Textures */
    
    
    /* Pixel types */
    
    public var UNSIGNED_INT_2_10_10_10_REV :UInt32 { return UInt32(GL_UNSIGNED_INT_2_10_10_10_REV) }
    
    public var UNSIGNED_INT_10F_11F_11F_REV :UInt32 { return UInt32(GL_UNSIGNED_INT_10F_11F_11F_REV) }
    
    public var UNSIGNED_INT_5_9_9_9_REV :UInt32 { return UInt32(GL_UNSIGNED_INT_5_9_9_9_REV) }
    
    public var FLOAT_32_UNSIGNED_INT_24_8_REV :UInt32 { return UInt32(GL_FLOAT_32_UNSIGNED_INT_24_8_REV) }
    
    public var UNSIGNED_INT_24_8 :UInt32 { return UInt32(GL_UNSIGNED_INT_24_8) }
    
    public var HALF_FLOAT :UInt32 { return UInt32(GL_HALF_FLOAT) }
    
    public var RG :UInt32 { return UInt32(GL_RG) }
    
    public var RG_INTEGER :UInt32 { return UInt32(GL_RG_INTEGER) }
    
    public var INT_2_10_10_10_REV :UInt32 { return UInt32(GL_INT_2_10_10_10_REV) }
    
    /* Pixel types */
    
    
    
    /* Queries */
    
    public var QUERY_RESULT_AVAILABLE :UInt32 { return UInt32(GL_QUERY_RESULT_AVAILABLE) }
    
    public var QUERY_RESULT :UInt32 { return UInt32(GL_QUERY_RESULT) }
    
    public var CURRENT_QUERY :UInt32 { return UInt32(GL_CURRENT_QUERY) }
    
    public var ANY_SAMPLES_PASSED :UInt32 { return UInt32(GL_ANY_SAMPLES_PASSED) }
    
    public var ANY_SAMPLES_PASSED_CONSERVATIVE :UInt32 { return UInt32(GL_ANY_SAMPLES_PASSED_CONSERVATIVE) }
    
    /* Queries */
    
    
    /* Draw buffers */
    
    public var MAX_DRAW_BUFFERS :UInt32 { return UInt32(GL_MAX_DRAW_BUFFERS) }
    public var DRAW_BUFFER0 :UInt32 { return UInt32(GL_DRAW_BUFFER0) }
    public var DRAW_BUFFER1 :UInt32 { return UInt32(GL_DRAW_BUFFER1) }
    public var DRAW_BUFFER2 :UInt32 { return UInt32(GL_DRAW_BUFFER2) }
    public var DRAW_BUFFER3 :UInt32 { return UInt32(GL_DRAW_BUFFER3) }
    public var DRAW_BUFFER4 :UInt32 { return UInt32(GL_DRAW_BUFFER4) }
    public var DRAW_BUFFER5 :UInt32 { return UInt32(GL_DRAW_BUFFER5) }
    public var DRAW_BUFFER6 :UInt32 { return UInt32(GL_DRAW_BUFFER6) }
    public var DRAW_BUFFER7 :UInt32 { return UInt32(GL_DRAW_BUFFER7) }
    public var DRAW_BUFFER8 :UInt32 { return UInt32(GL_DRAW_BUFFER8) }
    public var DRAW_BUFFER9 :UInt32 { return UInt32(GL_DRAW_BUFFER9) }
    public var DRAW_BUFFER10 :UInt32 { return UInt32(GL_DRAW_BUFFER10) }
    public var DRAW_BUFFER11 :UInt32 { return UInt32(GL_DRAW_BUFFER11) }
    public var DRAW_BUFFER12 :UInt32 { return UInt32(GL_DRAW_BUFFER12) }
    public var DRAW_BUFFER13 :UInt32 { return UInt32(GL_DRAW_BUFFER13) }
    public var DRAW_BUFFER14 :UInt32 { return UInt32(GL_DRAW_BUFFER14) }
    public var DRAW_BUFFER15 :UInt32 { return UInt32(GL_DRAW_BUFFER15) }
    public var MAX_COLOR_ATTACHMENTS :UInt32 { return UInt32(GL_MAX_COLOR_ATTACHMENTS) }
    
    public var COLOR_ATTACHMENT1 :UInt32 { return UInt32(GL_COLOR_ATTACHMENT1) }
    
    public var COLOR_ATTACHMENT2 :UInt32 { return UInt32(GL_COLOR_ATTACHMENT2) }
    
    public var COLOR_ATTACHMENT3 :UInt32 { return UInt32(GL_COLOR_ATTACHMENT3) }
    
    public var COLOR_ATTACHMENT4 :UInt32 { return UInt32(GL_COLOR_ATTACHMENT4) }
    
    public var COLOR_ATTACHMENT5 :UInt32 { return UInt32(GL_COLOR_ATTACHMENT5) }
    
    public var COLOR_ATTACHMENT6 :UInt32 { return UInt32(GL_COLOR_ATTACHMENT6) }
    
    public var COLOR_ATTACHMENT7 :UInt32 { return UInt32(GL_COLOR_ATTACHMENT7) }
    
    public var COLOR_ATTACHMENT8 :UInt32 { return UInt32(GL_COLOR_ATTACHMENT8) }
    
    public var COLOR_ATTACHMENT9 :UInt32 { return UInt32(GL_COLOR_ATTACHMENT9) }
    
    public var COLOR_ATTACHMENT10 :UInt32 { return UInt32(GL_COLOR_ATTACHMENT10) }
    
    public var COLOR_ATTACHMENT11 :UInt32 { return UInt32(GL_COLOR_ATTACHMENT11) }
    
    public var COLOR_ATTACHMENT12 :UInt32 { return UInt32(GL_COLOR_ATTACHMENT12) }
    
    public var COLOR_ATTACHMENT13 :UInt32 { return UInt32(GL_COLOR_ATTACHMENT13) }
    
    public var COLOR_ATTACHMENT14 :UInt32 { return UInt32(GL_COLOR_ATTACHMENT14) }
    
    public var COLOR_ATTACHMENT15 :UInt32 { return UInt32(GL_COLOR_ATTACHMENT15) }
    
    /* Draw buffers */
    
    /* Samplers */
    
    public var SAMPLER_3D :UInt32 { return UInt32(GL_SAMPLER_3D) }
    
    public var SAMPLER_2D_SHADOW :UInt32 { return UInt32(GL_SAMPLER_2D_SHADOW) }
    
    public var SAMPLER_2D_ARRAY :UInt32 { return UInt32(GL_SAMPLER_2D_ARRAY) }
    
    public var SAMPLER_2D_ARRAY_SHADOW :UInt32 { return UInt32(GL_SAMPLER_2D_ARRAY_SHADOW) }
    
    public var SAMPLER_CUBE_SHADOW :UInt32 { return UInt32(GL_SAMPLER_CUBE_SHADOW) }
    
    public var INT_SAMPLER_2D :UInt32 { return UInt32(GL_INT_SAMPLER_2D) }
    
    public var INT_SAMPLER_3D :UInt32 { return UInt32(GL_INT_SAMPLER_3D) }
    
    public var INT_SAMPLER_CUBE :UInt32 { return UInt32(GL_INT_SAMPLER_CUBE) }
    
    public var INT_SAMPLER_2D_ARRAY :UInt32 { return UInt32(GL_INT_SAMPLER_2D_ARRAY) }
    
    public var UNSIGNED_INT_SAMPLER_2D :UInt32 { return UInt32(GL_UNSIGNED_INT_SAMPLER_2D) }
    
    public var UNSIGNED_INT_SAMPLER_3D :UInt32 { return UInt32(GL_UNSIGNED_INT_SAMPLER_3D) }
    
    public var UNSIGNED_INT_SAMPLER_CUBE :UInt32 { return UInt32(GL_UNSIGNED_INT_SAMPLER_CUBE) }
    
    public var UNSIGNED_INT_SAMPLER_2D_ARRAY :UInt32 { return UInt32(GL_UNSIGNED_INT_SAMPLER_2D_ARRAY) }
    
    public var MAX_SAMPLES :UInt32 { return UInt32(GL_MAX_SAMPLES) }
    
    public var SAMPLER_BINDING :UInt32 { return UInt32(GL_SAMPLER_BINDING) }
    
    /* Samplers */
    
    
    /* Buffers */
    
    public var PIXEL_PACK_BUFFER :UInt32 { return UInt32(GL_PIXEL_PACK_BUFFER) }
    
    public var PIXEL_UNPACK_BUFFER :UInt32 { return UInt32(GL_PIXEL_UNPACK_BUFFER) }
    
    public var PIXEL_PACK_BUFFER_BINDING :UInt32 { return UInt32(GL_PIXEL_PACK_BUFFER_BINDING) }
    
    public var PIXEL_UNPACK_BUFFER_BINDING :UInt32 { return UInt32(GL_PIXEL_UNPACK_BUFFER_BINDING) }
    
    public var COPY_READ_BUFFER :UInt32 { return UInt32(GL_COPY_READ_BUFFER) }
    
    public var COPY_WRITE_BUFFER :UInt32 { return UInt32(GL_COPY_WRITE_BUFFER) }
    
    public var COPY_READ_BUFFER_BINDING :UInt32 { return UInt32(GL_COPY_READ_BUFFER_BINDING) }
    
    public var COPY_WRITE_BUFFER_BINDING :UInt32 { return UInt32(GL_COPY_WRITE_BUFFER_BINDING) }
    
    /* Buffers */
    
    
    /* Data types */
    
    public var FLOAT_MAT2x3 :UInt32 { return UInt32(GL_FLOAT_MAT2x3) }
    
    public var FLOAT_MAT2x4 :UInt32 { return UInt32(GL_FLOAT_MAT2x4) }
    
    public var FLOAT_MAT3x2 :UInt32 { return UInt32(GL_FLOAT_MAT3x2) }
    
    public var FLOAT_MAT3x4 :UInt32 { return UInt32(GL_FLOAT_MAT3x4) }
    
    public var FLOAT_MAT4x2 :UInt32 { return UInt32(GL_FLOAT_MAT4x2) }
    
    public var FLOAT_MAT4x3 :UInt32 { return UInt32(GL_FLOAT_MAT4x3) }
    
    public var UNSIGNED_INT_VEC2 :UInt32 { return UInt32(GL_UNSIGNED_INT_VEC2) }
    
    public var UNSIGNED_INT_VEC3 :UInt32 { return UInt32(GL_UNSIGNED_INT_VEC3) }
    public var UNSIGNED_INT_VEC4 :UInt32 { return UInt32(GL_UNSIGNED_INT_VEC4) }
    public var UNSIGNED_NORMALIZED :UInt32 { return UInt32(GL_UNSIGNED_NORMALIZED) }
    public var SIGNED_NORMALIZED :UInt32 { return UInt32(GL_SIGNED_NORMALIZED) }
    
    
    /* Data types */
    
    /* Vertex attributes */
    public var VERTEX_ATTRIB_ARRAY_INTEGER :UInt32 { return UInt32(GL_VERTEX_ATTRIB_ARRAY_INTEGER) }
    
    public var VERTEX_ATTRIB_ARRAY_DIVISOR :UInt32 { return UInt32(GL_VERTEX_ATTRIB_ARRAY_DIVISOR) }
    
    /* Vertex attributes */
    
    
    /* Transform feedback */
    
    public var TRANSFORM_FEEDBACK_BUFFER_MODE :UInt32 { return UInt32(GL_TRANSFORM_FEEDBACK_BUFFER_MODE) }
    public var MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS :UInt32 { return UInt32(GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS) }
    public var TRANSFORM_FEEDBACK_VARYINGS :UInt32 { return UInt32(GL_TRANSFORM_FEEDBACK_VARYINGS) }
    
    public var TRANSFORM_FEEDBACK_BUFFER_START :UInt32 { return UInt32(GL_TRANSFORM_FEEDBACK_BUFFER_START) }
    
    public var TRANSFORM_FEEDBACK_BUFFER_SIZE :UInt32 { return UInt32(GL_TRANSFORM_FEEDBACK_BUFFER_SIZE) }
    
    public var TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN :UInt32 { return UInt32(GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN) }
    
    public var MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS :UInt32 { return UInt32(GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS) }
    
    public var MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS :UInt32 { return UInt32(GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS) }
    
    public var INTERLEAVED_ATTRIBS :UInt32 { return UInt32(GL_INTERLEAVED_ATTRIBS) }
    
    public var SEPARATE_ATTRIBS :UInt32 { return UInt32(GL_SEPARATE_ATTRIBS) }
    
    public var TRANSFORM_FEEDBACK_BUFFER :UInt32 { return UInt32(GL_TRANSFORM_FEEDBACK_BUFFER) }
    
    public var TRANSFORM_FEEDBACK_BUFFER_BINDING :UInt32 { return UInt32(GL_TRANSFORM_FEEDBACK_BUFFER_BINDING) }
    
    public var TRANSFORM_FEEDBACK :UInt32 { return UInt32(GL_TRANSFORM_FEEDBACK) }
    
    public var TRANSFORM_FEEDBACK_PAUSED :UInt32 { return UInt32(GL_TRANSFORM_FEEDBACK_PAUSED) }
    
    public var TRANSFORM_FEEDBACK_ACTIVE :UInt32 { return UInt32(GL_TRANSFORM_FEEDBACK_ACTIVE) }
    
    public var TRANSFORM_FEEDBACK_BINDING :UInt32 { return UInt32(GL_TRANSFORM_FEEDBACK_BINDING) }
    
    /* Transform feedback */
    
    /* Framebuffers and renderbuffers */
    
    public var FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING :UInt32 { return UInt32(GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING) }
    public var FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE :UInt32 { return UInt32(GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE) }
    
    public var FRAMEBUFFER_ATTACHMENT_RED_SIZE :UInt32 { return UInt32(GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE) }
    public var FRAMEBUFFER_ATTACHMENT_GREEN_SIZE :UInt32 { return UInt32(GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE) }
    public var FRAMEBUFFER_ATTACHMENT_BLUE_SIZE :UInt32 { return UInt32(GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE) }
    public var FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE :UInt32 { return UInt32(GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE) }
    
    public var FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE :UInt32 { return UInt32(GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE) }
    public var FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE :UInt32 { return UInt32(GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE) }
    
    public var FRAMEBUFFER_DEFAULT :UInt32 { return UInt32(GL_FRAMEBUFFER_DEFAULT) }
    public override var DEPTH_STENCIL_ATTACHMENT :UInt32 { return UInt32(GL_DEPTH_STENCIL_ATTACHMENT) }
    public override var DEPTH_STENCIL :UInt32 { return UInt32(GL_DEPTH_STENCIL) }
    public var DEPTH24_STENCIL8 :UInt32 { return UInt32(GL_DEPTH24_STENCIL8) }
    
    public var DRAW_FRAMEBUFFER_BINDING :UInt32 { return UInt32(GL_DRAW_FRAMEBUFFER_BINDING) }
    
    public var READ_FRAMEBUFFER :UInt32 { return UInt32(GL_READ_FRAMEBUFFER) }
    
    public var DRAW_FRAMEBUFFER :UInt32 { return UInt32(GL_DRAW_FRAMEBUFFER) }
    
    public var READ_FRAMEBUFFER_BINDING :UInt32 { return UInt32(GL_READ_FRAMEBUFFER_BINDING) }
    
    public var RENDERBUFFER_SAMPLES :UInt32 { return UInt32(GL_RENDERBUFFER_SAMPLES) }
    
    public var FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER :UInt32 { return UInt32(GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER) }
    
    public  var FRAMEBUFFER_INCOMPLETE_MULTISAMPLE :UInt32 { return UInt32(GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE) }
    
    /* Framebuffers and renderbuffers */
    
    
    /* Uniforms */
    
    public var UNIFORM_BUFFER :UInt32 { return UInt32(GL_UNIFORM_BUFFER) }
    public var UNIFORM_BUFFER_BINDING :UInt32 { return UInt32(GL_UNIFORM_BUFFER_BINDING) }
    
    public var UNIFORM_BUFFER_START :UInt32 { return UInt32(GL_UNIFORM_BUFFER_START) }
    
    public var UNIFORM_BUFFER_SIZE :UInt32 { return UInt32(GL_UNIFORM_BUFFER_SIZE) }
    
    public var MAX_VERTEX_UNIFORM_BLOCKS :UInt32 { return UInt32(GL_MAX_VERTEX_UNIFORM_BLOCKS) }
    
    public var MAX_FRAGMENT_UNIFORM_BLOCKS :UInt32 { return UInt32(GL_MAX_FRAGMENT_UNIFORM_BLOCKS) }
    
    public var MAX_COMBINED_UNIFORM_BLOCKS :UInt32 { return UInt32(GL_MAX_COMBINED_UNIFORM_BLOCKS) }
    
    public var MAX_UNIFORM_BUFFER_BINDINGS :UInt32 { return UInt32(GL_MAX_UNIFORM_BUFFER_BINDINGS) }
    
    public var MAX_UNIFORM_BLOCK_SIZE :UInt32 { return UInt32(GL_MAX_UNIFORM_BLOCK_SIZE) }
    
    public var MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS :UInt32 { return UInt32(GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS) }
    
    public var MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS :UInt32 { return UInt32(GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS) }
    
    public var UNIFORM_BUFFER_OFFSET_ALIGNMENT :UInt32 { return UInt32(GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT) }
    
    public var ACTIVE_UNIFORM_BLOCKS :UInt32 { return UInt32(GL_ACTIVE_UNIFORM_BLOCKS) }
    
    public var UNIFORM_TYPE :UInt32 { return UInt32(GL_UNIFORM_TYPE) }
    
    public var UNIFORM_SIZE :UInt32 { return UInt32(GL_UNIFORM_SIZE) }
    
    public var UNIFORM_BLOCK_INDEX :UInt32 { return UInt32(GL_UNIFORM_BLOCK_INDEX) }
    
    public var UNIFORM_OFFSET :UInt32 { return UInt32(GL_UNIFORM_OFFSET) }
    
    public var UNIFORM_ARRAY_STRIDE :UInt32 { return UInt32(GL_UNIFORM_ARRAY_STRIDE) }
    
    public var UNIFORM_MATRIX_STRIDE :UInt32 { return UInt32(GL_UNIFORM_MATRIX_STRIDE) }
    
    public var UNIFORM_IS_ROW_MAJOR :UInt32 { return UInt32(GL_UNIFORM_IS_ROW_MAJOR) }
    
    public var UNIFORM_BLOCK_BINDING :UInt32 { return UInt32(GL_UNIFORM_BLOCK_BINDING) }
    
    public var UNIFORM_BLOCK_DATA_SIZE :UInt32 { return UInt32(GL_UNIFORM_BLOCK_DATA_SIZE) }
    
    public var UNIFORM_BLOCK_ACTIVE_UNIFORMS :UInt32 { return UInt32(GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS) }
    
    public var UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES :UInt32 { return UInt32(GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES) }
    
    public var UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER :UInt32 { return UInt32(GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER) }
    
    public var UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER :UInt32 { return UInt32(GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER) }
    
    
    /* Uniforms */
    
    /* Sync objects */
    
    public var OBJECT_TYPE :UInt32 { return UInt32(GL_OBJECT_TYPE) }
    
    public var SYNC_CONDITION :UInt32 { return UInt32(GL_SYNC_CONDITION) }
    
    public var SYNC_STATUS :UInt32 { return UInt32(GL_SYNC_STATUS) }
    
    public var SYNC_FLAGS :UInt32 { return UInt32(GL_SYNC_FLAGS) }
    
    public var SYNC_FENCE :UInt32 { return UInt32(GL_SYNC_FENCE) }
    
    public var SYNC_GPU_COMMANDS_COMPLETE :UInt32 { return UInt32(GL_SYNC_GPU_COMMANDS_COMPLETE) }
    
    public var UNSIGNALED :UInt32 { return UInt32(GL_UNSIGNALED) }
    
    public var SIGNALED :UInt32 { return UInt32(GL_SIGNALED) }
    
    public var ALREADY_SIGNALED :UInt32 { return UInt32(GL_ALREADY_SIGNALED) }
    
    public var TIMEOUT_EXPIRED :UInt32 { return UInt32(GL_TIMEOUT_EXPIRED) }
    
    
    public var CONDITION_SATISFIED :UInt32 { return UInt32(GL_CONDITION_SATISFIED) }
    
    public var WAIT_FAILED :UInt32 { return UInt32(GL_WAIT_FAILED) }
    
    public var SYNC_FLUSH_COMMANDS_BIT :UInt32 { return UInt32(GL_SYNC_FLUSH_COMMANDS_BIT) }
    
    /* Sync objects */
    
    /* Miscellaneous constants */
    
    public var COLOR :UInt32 { return UInt32(GL_COLOR) }
    
    public var DEPTH :UInt32 { return UInt32(GL_DEPTH) }
    
    public var STENCIL :UInt32 { return UInt32(GL_STENCIL) }
    
    public var MIN :UInt32 { return UInt32(GL_MIN) }
    
    public var MAX :UInt32 { return UInt32(GL_MAX) }
    
    public var DEPTH_COMPONENT24 :UInt32 { return UInt32(GL_DEPTH_COMPONENT24) }
    
    public var STREAM_READ :UInt32 { return UInt32(GL_STREAM_READ) }
    
    public var STREAM_COPY :UInt32 { return UInt32(GL_STREAM_COPY) }
    
    public var STATIC_READ :UInt32 { return UInt32(GL_STATIC_READ) }
    
    public var STATIC_COPY :UInt32 { return UInt32(GL_STATIC_COPY) }
    
    public var DYNAMIC_READ :UInt32 { return UInt32(GL_DYNAMIC_READ) }
    
    public var DYNAMIC_COPY :UInt32 { return UInt32(GL_DYNAMIC_COPY) }
    
    public var DEPTH_COMPONENT32F :UInt32 { return UInt32(GL_DEPTH_COMPONENT32F) }
    
    public var DEPTH32F_STENCIL8 :UInt32 { return UInt32(GL_DEPTH32F_STENCIL8) }
    
    public var INVALID_INDEX : Int { return Int(GL_INVALID_INDEX) }
    
    public var TIMEOUT_IGNORED : UInt64 { return UInt64(GL_TIMEOUT_IGNORED) }
    
    public var MAX_CLIENT_WAIT_TIMEOUT_WEBGL:UInt32 { return 0x9247 }
    
    /* Miscellaneous constants */
}

