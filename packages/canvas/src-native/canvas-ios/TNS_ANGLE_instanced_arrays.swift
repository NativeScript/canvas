//
//  TNS_ANGLE_instanced_arrays.swift
//  CanvasNative
//
//  Created by Osei Fortune on 5/8/20.
//

import Foundation
import OpenGLES
@objcMembers
@objc(TNS_ANGLE_instanced_arrays)
public class TNS_ANGLE_instanced_arrays: NSObject {

    public var VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE: Int32 {
        // GL_VERTEX_ATTRIB_ARRAY_DIVISOR
        return 0x88FE
    }
    private var context: TNSWebGLRenderingContext
    public init(context: TNSWebGLRenderingContext) {
        self.context = context
    }

    func BUFFER_OFFSET(n: Int) -> UnsafeRawPointer? {
        return UnsafeRawPointer(bitPattern: n)
    }

    public func drawArraysInstancedANGLE(mode: Int32, first: Int32, count: Int32, primcount: Int32){
        context.clearIfComposited()
        if context.canvas.renderer.glContext.api == .openGLES3 {
            glDrawArraysInstanced(GLenum(mode), first, count, primcount)
        }else {
            glDrawArraysInstancedEXT(GLenum(mode), first, count, primcount)
        }
        context.canvas.doDraw()
    }

    public func drawElementsInstancedANGLE(mode: Int32, count: Int32, type: Int32, offset: Int32, primcount: Int32){
        let ptr = BUFFER_OFFSET(n: Int(offset))
        context.clearIfComposited()
        if context.canvas.renderer.glContext.api == .openGLES3 {
            glDrawElementsInstanced(GLenum(mode), count, GLenum(type), ptr, primcount)
        }else {
            glDrawElementsInstancedEXT(GLenum(mode), count, GLenum(type), ptr, primcount)
        }
        context.canvas.doDraw()
    }

    public func vertexAttribDivisorANGLE(index: Int32, divisor: Int32) {
        if context.canvas.renderer.glContext.api == .openGLES3 {
            glVertexAttribDivisor(GLuint(index), GLuint(divisor))
        }else {
            glVertexAttribDivisorEXT(GLuint(index), GLuint(divisor))
        }
    }
}
