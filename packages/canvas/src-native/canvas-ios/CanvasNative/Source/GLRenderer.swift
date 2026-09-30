//
//  GLRenderer.swift
//  CanvasNative
//
//  Created by Osei Fortune on 3/24/20.
//

import Foundation
#if !os(visionOS)
import OpenGLES
#endif
import UIKit

#if !os(visionOS)
/// Shows a GL context's drawing buffer. The context stores its color buffer in this view's layer
/// and presents it there itself, from whichever thread it runs on.
@objcMembers
@objc(CanvasGLView)
public class CanvasGLView: UIView {
    internal(set) public weak var canvas: NSCCanvas? = nil

    public override class var layerClass: AnyClass {
        return CAEAGLLayer.self
    }

    public var eaglLayer: CAEAGLLayer {
        return layer as! CAEAGLLayer
    }
}
#else
// visionOS has no OpenGL ES. CanvasGLView is a no-op UIView stand-in so NSCCanvas's
// layout/lifecycle code compiles; the GL engine is never selected on visionOS.
@objcMembers
@objc(CanvasGLView)
public class CanvasGLView: UIView {
    internal(set) public weak var canvas: NSCCanvas? = nil
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
