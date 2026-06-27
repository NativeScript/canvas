//
//  GLRenderer.swift
//  CanvasNative
//
//  Created by Osei Fortune on 3/24/20.
//

import Foundation
#if !os(visionOS)
import GLKit
#endif
import UIKit

#if !os(visionOS)
@objcMembers
@objc(CanvasGLKView)
public class CanvasGLKView: GLKView, GLKViewDelegate {
    var isDirty: Bool = false
    internal(set) public weak var canvas: NSCCanvas? = nil
    
    private(set) var fbo: UInt32 = 0
    
    public init() {
        super.init(frame: .zero)
    }
    public override init(frame: CGRect) {
        super.init(frame: frame)
        delegate = self
    }
    
    required init?(coder: NSCoder) {
        super.init(coder: coder)
        delegate = self
    }
    
    public override func bindDrawable() {
        super.bindDrawable()
        var fbo: UInt32 = 0
        glGetIntegerv(GLenum(GL_FRAMEBUFFER_BINDING), &fbo)
        self.fbo = fbo
    }
    
    public override func deleteDrawable() {
        super.deleteDrawable()
        self.fbo = 0
    }
    
    public func glkView(_ view: GLKView, drawIn rect: CGRect) {
//        guard let canvas = canvas else {return}
//        if(canvas.is2D){
//            CanvasHelpers.flush2DContext(canvas.nativeContext)
//        }
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
#else
// visionOS has no OpenGL ES. CanvasGLKView is a no-op UIView stand-in so NSCCanvas's
// layout/lifecycle code compiles; the GL engine is never selected on visionOS.
@objcMembers
@objc(CanvasGLKView)
public class CanvasGLKView: UIView {
    var isDirty: Bool = false
    internal(set) public weak var canvas: NSCCanvas? = nil
    private(set) var fbo: UInt32 = 0
}
#endif


internal func CPURender(_ ptr: UnsafeRawPointer?) {
    guard let ptr = ptr else { return }
    let view: CanvasCPUView = Unmanaged.fromOpaque(ptr).takeUnretainedValue()

    if let renderer = view.canvas, let data = view.data {
        let width = renderer.surfaceWidth
        let height = renderer.surfaceHeight
        canvas_native_ios_context_custom_with_buffer_flush(renderer.nativeContext, data.mutableBytes, UInt(data.length), Float(width), Float(height), !renderer.isOpaque)
    }

    let cgImage = view.makeCGImage()
    if Thread.isMainThread {
        view.layer.contents = cgImage
    } else {
        DispatchQueue.main.async {
            view.layer.contents = cgImage
        }
    }
}

@objcMembers
@objc(CanvasCPUView)
public class CanvasCPUView: UIView {
    var isDirty: Bool = false
    weak var canvas: NSCCanvas?
    public var ignorePixelScaling = false
	internal(set) public var data: NSMutableData? = nil
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
            return Float32(nscNativeScale())
        }
        return 1
    }
	
	
	internal func makeCGImage() -> CGImage? {
		guard let canvas = canvas, let data = data else {return nil}
		let width = canvas.surfaceWidth
		let height = canvas.surfaceHeight
			let bytesPerPixel = 4
			let bytesPerRow = bytesPerPixel * width
		guard let provider = CGDataProvider(dataInfo: nil, data: data.bytes, size: bytesPerRow * height, releaseData: { _,_,_ in }) else {
					return nil
			}

			return CGImage(
					width: width,
					height: height,
					bitsPerComponent: 8,
					bitsPerPixel: 32,
					bytesPerRow: bytesPerRow,
					space: CGColorSpaceCreateDeviceRGB(),
					bitmapInfo: CGBitmapInfo(rawValue: CGImageAlphaInfo.premultipliedLast.rawValue | CGBitmapInfo.byteOrder32Big.rawValue),
					provider: provider,
					decode: nil,
					shouldInterpolate: true,
					intent: .defaultIntent
			)
	}
    
	internal func snapshot()-> UIImage? {
		if let renderer = canvas, let data = data {
			let width = renderer.surfaceWidth
			let height = renderer.surfaceHeight
			canvas_native_ios_context_custom_with_buffer_flush(renderer.nativeContext, data.mutableBytes, UInt(data.length), Float(width), Float(height), !renderer.isOpaque)
			guard let cgImage = makeCGImage() else {return nil}
			return UIImage(cgImage: cgImage)
		}
		
		return nil
	}
}
