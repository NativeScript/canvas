//
//  NSCCanvasUtils.swift
//  CanvasNative
//
//  Created by Osei Fortune on 30/03/2021.
//

import Foundation
import OpenGLES
import CoreVideo
import AVFoundation
@objc(NSCCanvasUtils)
public class NSCCanvasUtils: NSObject {
    private static let BYTES_PER_TEXEL = 4
    @objc public static func createTextureCache() -> CVOpenGLESTextureCache? {
        var out: CVOpenGLESTextureCache?
        guard let currentContext = EAGLContext.current() else {return nil}
        CVOpenGLESTextureCacheCreate(kCFAllocatorDefault, nil, currentContext, nil, &out)
        return out
    }
    
    @objc public static func createImage(_ texturecache: CVOpenGLESTextureCache,_ buffer: CVImageBuffer,_ textureAttributes: CFDictionary?, _ target: GLenum,_  internalFormat: GLint,_ width: GLsizei,_ height: GLsizei,_ format: GLenum,_ type: GLenum,_ planeIndex: Int) -> CVOpenGLESTexture?{
        var textureOut: CVOpenGLESTexture?
        CVOpenGLESTextureCacheCreateTextureFromImage(kCFAllocatorDefault, texturecache, buffer, textureAttributes, target, internalFormat, width, height, format, type, planeIndex,&textureOut)
        return textureOut
    }
    
    @objc public static func setupRender() -> NSCRender{
        return NSCRender()
    }
    
    @objc public static func drawFrame(_ player: AVPlayer, _ output: AVPlayerItemVideoOutput,_ videoSize: CGSize, _ render: NSCRender,_ internalFormat: Int32,_ format: Int32,_ flipYWebGL: Bool){

        let currentTime = player.currentTime()
        
        if(!output.hasNewPixelBuffer(forItemTime: currentTime)) {return}
        
        let buffer = output.copyPixelBuffer(forItemTime: currentTime, itemTimeForDisplay: nil)
        
        
        guard let pixel_buffer = buffer else {return}
        render.drawFrame(buffer: pixel_buffer, width: Int(videoSize.width), height: Int(videoSize.height), internalFormat: internalFormat, format: format, flipYWebGL: flipYWebGL)
    }
    
    
    
    private static func getParent(_ path: String) -> String {
      //  let fileManager = FileManager.default
        let nsString = NSString(string: path)
        let parentPath = nsString.deletingLastPathComponent
        // let name = fileManager.displayName(atPath: parentPath)
        return parentPath
    }
    
    public enum NSCError: Error {
        case customError(String)
    }
    
    private static func getFile(_ path: String) throws -> String {
        let fileManager = FileManager.default
        let exists = fileManager.fileExists(atPath: path)
        if (!exists) {
            let parentPath = getParent(path)
            var failed = false
            
            do {
                try fileManager.createDirectory(atPath: parentPath, withIntermediateDirectories: true)
            }catch {
                failed = true
            }
            
            
            if(failed && !fileManager.createFile(atPath: path, contents: nil, attributes: nil)){
                throw NSCError.customError("Failed to create file at path '" + path + "'")
            }
        }
        
        return path;
    }
    
    
    @objc public static func writeToFile(_ data: NSData, _ path: String) throws {
        let newPath = try getFile(path)
        data.write(to: URL(fileURLWithPath: newPath), atomically: true)
    }
    
}
