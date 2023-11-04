//
//  GLRenderer.swift
//  CanvasNative
//
//  Created by Osei Fortune on 3/24/20.
//

import Foundation
import GLKit
import UIKit

@objcMembers
@objc(CanvasGLKView)
public class CanvasGLKView: GLKView {
    var isDirty: Bool = false
    public init() {
        super.init(frame: .zero)
    }
    public override init(frame: CGRect) {
        super.init(frame: frame)
    }
    
    required init?(coder: NSCoder) {
        super.init(coder: coder)
    }
    
    public override func setNeedsDisplay() {
        super.setNeedsDisplay()
        //isDirty = true
    }
    
    public override func setNeedsDisplay(_ rect: CGRect) {
        super.setNeedsDisplay(rect)
        // isDirty = true
    }
}


extension GLKView {
    @objc public func snapshotWithData(_ data: Data){
        let pixels = self.snapshot
                 
                 var cgImage: CGImage?
                 
                 if let image = pixels.cgImage {
                     cgImage = image
                 } else if let image = pixels.ciImage {
                     let ctx = CIContext()
                     cgImage = ctx.createCGImage(image, from: image.extent)
                 }
                 
                 if let image = cgImage {
                     let width = Int(pixels.size.width)
                     let height = Int(pixels.size.height)
                     let row = width * 4
                     var buffer = [UInt8](data)
                     let colorSpace = CGColorSpaceCreateDeviceRGB()
                     let imageCtx = CGContext(data: &buffer, width: width, height: height, bitsPerComponent: 8, bytesPerRow: row, space: colorSpace, bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue | CGBitmapInfo.byteOrder32Big.rawValue)
                     imageCtx!.draw(image, in: CGRect(x: 0, y: 0, width: width, height: height))
                 }
    }
}

@objcMembers
@objc(CanvasCPUView)
public class CanvasCPUView: UIView {
    var isDirty: Bool = false
    weak var renderer: NSCCanvas?
    public var ignorePixelScaling = false
    public init() {
        super.init(frame: .zero)
    }
    public override init(frame: CGRect) {
        super.init(frame: frame)
        contentMode = .redraw
    }
    
    required init?(coder: NSCoder) {
        super.init(coder: coder)
        contentMode = .redraw
    }
    
    func deviceScale() -> Float32 {
        if (ignorePixelScaling)  {
            return Float32(UIScreen.main.nativeScale)
        }
        return 1
    }
    
    
    public override func draw(_ rect: CGRect) {
        if let renderer = renderer {
            if(renderer.nativeContext != 0){
                let width = Int(Float(frame.size.width) * deviceScale())
                let height = Int(Float(frame.size.height) * deviceScale())
                let size = width * height * 4
                let buffer = UnsafeMutablePointer<UInt8>.allocate(capacity: size)
                let colorSpace = CGColorSpaceCreateDeviceRGB()
                let ctx = CGContext(data: buffer, width: width, height: height, bitsPerComponent: 8, bytesPerRow: width * 4, space: colorSpace, bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue | CGBitmapInfo.byteOrder32Big.rawValue)
//                context_custom_with_buffer_flush(renderer.context, buffer, UInt(size), Float(width), Float(height))
                if let image = ctx?.makeImage() {
                    let currentContext = UIGraphicsGetCurrentContext()
                    currentContext?.clear(bounds)
                    currentContext?.translateBy(x: 0, y: bounds.size.height)
                    currentContext?.scaleBy(x: 1, y: -1)
                    currentContext?.draw(image, in: bounds)
                }
                buffer.deallocate()
            }
        }
        isDirty = false
    }
}
