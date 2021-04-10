//
//  Utils.swift
//  CanvasNative
//
//  Created by Osei Fortune on 30/03/2021.
//

import Foundation
import OpenGLES
import CoreVideo
import AVFoundation
@objc(Utils)
public class Utils: NSObject {
    private static let BYTES_PER_TEXEL = 4
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
    
    @objc public static func drawFrame(_ player: AVPlayer, _ output: AVPlayerItemVideoOutput,_ videoSize: CGSize){
        
        let currentTime = player.currentTime()
   
        if(!output.hasNewPixelBuffer(forItemTime: currentTime)) {return}
        
        let buffer = output.copyPixelBuffer(forItemTime: currentTime, itemTimeForDisplay: nil)
        guard let pixel_buffer = buffer else {return}
        CVPixelBufferLockBaseAddress(pixel_buffer,CVPixelBufferLockFlags(rawValue: 0))
        let bytesPerRow = CVPixelBufferGetBytesPerRow(pixel_buffer)
        let line_base = CVPixelBufferGetBaseAddress(pixel_buffer)
        guard let base = line_base else {return}
        if (bytesPerRow / BYTES_PER_TEXEL == Int(videoSize.width)) {
            glTexImage2D(GLenum(GL_TEXTURE_2D), 0, GL_RGBA, GLsizei(videoSize.width), GLsizei(videoSize.height), 0, GLenum(GL_BGRA), GLenum(GL_UNSIGNED_BYTE), base)
        } else {
            glTexImage2D(GLenum(GL_TEXTURE_2D), 0, GL_RGBA, GLsizei(videoSize.width), GLsizei(videoSize.height), 0, GLenum(GL_BGRA), GLenum(GL_UNSIGNED_BYTE), nil)
            for i in 0..<Int(videoSize.height) {
                let data = base + i * bytesPerRow
                glTexSubImage2D(GLenum(GL_TEXTURE_2D), 0, 0, GLint(i), GLsizei(videoSize.width), 1, GLenum(GL_BGRA), GLenum(GL_UNSIGNED_BYTE), data)
            }
        }
        
        CVPixelBufferUnlockBaseAddress(pixel_buffer, CVPixelBufferLockFlags(rawValue: 0))
    }
    
}
