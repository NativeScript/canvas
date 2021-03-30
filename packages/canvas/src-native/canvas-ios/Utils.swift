//
//  Utils.swift
//  CanvasNative
//
//  Created by Osei Fortune on 30/03/2021.
//

import Foundation
import OpenGLES
import CoreVideo

@objc(Utils)
public class Utils: NSObject {
    
    @objc public static func createTextureCache(_ context: TNSWebGLRenderingContext) -> CVOpenGLESTextureCache? {
        var out: CVOpenGLESTextureCache?
        CVOpenGLESTextureCacheCreate(kCFAllocatorDefault, nil, context.canvas.renderer.glContext, nil, &out)
        return out
    }
   
    @objc public static func createImage(_ texturecache: CVOpenGLESTextureCache,_ buffer: CVImageBuffer,_ textureAttributes: CFDictionary?, _ target: GLenum,_  internalFormat: GLint,_ width: GLsizei,_ height: GLsizei,_ format: GLenum,_ type: GLenum,_ planeIndex: Int) -> CVOpenGLESTexture?{
        var textureOut: CVOpenGLESTexture?
        CVOpenGLESTextureCacheCreateTextureFromImage(kCFAllocatorDefault, texturecache, buffer, textureAttributes, target, internalFormat, width, height, format, type, planeIndex,&textureOut)
        return textureOut
    }
    
}
