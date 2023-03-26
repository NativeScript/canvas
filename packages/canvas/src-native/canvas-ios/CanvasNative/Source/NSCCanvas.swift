//
//  NSCCanvas.swift
//
//  Created by Osei Fortune on 7/14/19.
//  Copyright © 2019 Osei Fortune. All rights reserved.
//

import Foundation
import UIKit


@objcMembers
@objc(NSCCanvas)
public class NSCCanvas: UIView {
    
    private static var views: NSMapTable<NSString,NSCCanvas> = NSMapTable(keyOptions: .copyIn, valueOptions: .weakMemory)
    
    public static func getViews() -> NSMapTable<NSString,NSCCanvas> {
        return views
    }
    
    var ptr: UnsafeMutableRawPointer? = nil
    
    public func getViewPtr() -> UnsafeMutableRawPointer {
        if(ptr == nil){
            ptr = Unmanaged.passUnretained(glkView).toOpaque()
        }
        return ptr!
    }
    
    public var ignorePixelScaling = false
    
    private(set) public var nativeGL: Int64 = 0
    private(set) public var nativeContext: Int64 = 0
    
    internal var glkView: CanvasGLKView = CanvasGLKView(frame: .zero)
    
    public var drawingBufferWidth: Int {
        return glkView.drawableWidth
    }
    public var drawingBufferHeight: Int {
        return glkView.drawableHeight
    }
    public var width: Float {
        get {
            return Float(glkView.frame.size.width)
        }
    }
    public var height: Float {
        get {
            return Float(glkView.frame.size.height)
        }
    }
    
        /*
    
    @objc public func initContext(
        _ type: String,
        _ alpha: Bool = true,
        _ antialias: Bool = true,
        _ depth: Bool = true,
        _ failIfMajorPerformanceCaveat: Bool = false,
        _ powerPreference: String = "default",
        _ premultipliedAlpha: Bool = true,
        _ preserveDrawingBuffer: Bool = false,
        _ stencil: Bool = false,
        _ desynchronized: Bool = false,
        _ xrCompatible: Bool = false
    ) {
        initContextWithContextAttributes(
            type,
            alpha,
            antialias,
            depth,
            failIfMajorPerformanceCaveat,
            powerPreference,
            premultipliedAlpha,
            preserveDrawingBuffer,
            stencil,
            desynchronized,
            xrCompatible
        )
    }
    
    
    
    func initContextWithContextAttributes(
        _ type: String,
        _ alpha: Bool,
        _ antialias: Bool,
        _ depth: Bool,
        _ failIfMajorPerformanceCaveat: Bool,
        _ powerPreference: String,
        _ premultipliedAlpha: Bool,
        _ preserveDrawingBuffer: Bool,
        _ stencil: Bool,
        _ desynchronized: Bool,
        _ xrCompatible: Bool
    ) {
        if (nativeGL != 0) {
            return
        }
        var version = -1
        var isCanvas = false
        switch (type) {
        case "2d":
            version = 0
            isCanvas = true
            break
        case "experimental-webgl", "webgl":
            version = 1
            break
        case "webgl2":
            version = 2
            break
        default:
            break
        }
        
        if (version == -1) {
            return
        }
        
        
        var properties: [String: Any] = [:]
        var useWebGL = !isCanvas
        if(useWebGL && preserveDrawingBuffer){
            properties[kEAGLDrawablePropertyRetainedBacking] = preserveDrawingBuffer
        }
        
        if(alpha){
            properties[kEAGLDrawablePropertyColorFormat] = kEAGLColorFormatRGBA8
            isOpaque = false
            (glkView.layer as! CAEAGLLayer).isOpaque = false
        }else {
            properties[kEAGLDrawablePropertyColorFormat] = kEAGLColorFormatRGB565
            isOpaque = true
            (glkView.layer as! CAEAGLLayer).isOpaque = true
        }
        
        
        if(!properties.isEmpty){
            let eaglLayer = self.glkView.layer as! CAEAGLLayer
            eaglLayer.drawableProperties = [kEAGLDrawablePropertyRetainedBacking: NSNumber(value:false) , kEAGLDrawablePropertyColorFormat: kEAGLColorFormatRGBA8]
            
        }
        
        if(useWebGL && depth){
            glkView.drawableDepthFormat = .format24
        }else {
            glkView.drawableDepthFormat = .formatNone
        }
        
        if(useWebGL && stencil){
            glkView.drawableStencilFormat = .format8
        }else if(isCanvas) {
            glkView.drawableStencilFormat = .format8
        }
        
        if(useWebGL && antialias){
            // glkView.drawableMultisample = .multisample4X
        }else if(isCanvas) {
            // drawableMultisample = .multisample4X
        }
        
        
        let viewPtr = Int64(Int(bitPattern: getViewPtr()))
        
        nativeGL = CanvasHelpers.initGL(withView: viewPtr, width: Int32(drawingBufferWidth), height: Int32(drawingBufferHeight), alpha: alpha, antialias: antialias, depth: depth, fail_if_major_performance_caveat: failIfMajorPerformanceCaveat, power_preference: type, premultiplied_alpha: premultipliedAlpha, preserve_drawing_buffer: preserveDrawingBuffer, stencil: stencil, desynchronized: desynchronized, xr_compatible: xrCompatible, version: Int32(version), is_canvas: isCanvas)
        
        
        nativeContext = CanvasHelpers.getGLPointer(withContext: nativeGL)
    }
    
         */
    
    
    public func forceLayout(_ width: CGFloat, _ height: CGFloat){
        
        if (width == .zero && height == .zero) {
            return
        }
        let w = UIScreen.main.scale * width;
        let h = UIScreen.main.scale * height
        
        if (w == frame.size.width && h == frame.size.height) {
            return;
        }
        frame = CGRect(x: frame.origin.x, y: frame.origin.y, width: w, height: h)
        setNeedsLayout()
        layoutIfNeeded()
    }
    
    
    //
    //    public func snapshot() -> [UInt8]{
    //        renderer.ensureIsContextIsCurrent()
    //        if(renderer.contextType == ContextType.twoD){
    //            let result = context_snapshot_canvas(self.context)
    //            if(result == nil){
    //                return []
    //            }
    //            let pointer = result!.pointee
    //            let data = [UInt8](Data(bytes: pointer.data, count: Int(pointer.data_len)))
    //            destroy_u8_array(result)
    //            return data
    //        }else if(renderer.contextType == ContextType.webGL){
    //            let pixels = (renderer.view as! CanvasGLKView).snapshot
    //            let data = pixels.pngData() ?? Data()
    //            EAGLContext.setCurrent(nil)
    //            return [UInt8](data)
    //        }
    //        return []
    //    }
    
    

    
    
    required init?(coder: NSCoder) {
        glkView = CanvasGLKView(coder: coder)!
        super.init(coder: coder)
        self.isOpaque = false
    }
    
    
    public override init(frame: CGRect) {
        glkView.frame = frame
        super.init(frame: frame)
        self.isOpaque = false
    }
    
    
    var readyListener: NSCCanvasListener?
    public func setListener(_ listener: NSCCanvasListener?){
        readyListener = listener
    }
    
    
    private var lastSize: CGRect = .null
    private var isLoaded: Bool = false
    
//    public override func layoutSubviews() {
//        if(bounds == .zero || (bounds.size.width < 0 || bounds.size.height < 0)){
//            glkView.frame = CGRect(x: 0, y: 0, width: CGFloat(1/UIScreen.main.nativeScale), height: CGFloat(1/UIScreen.main.nativeScale))
//        }else {
//            glkView.frame = bounds
//            glkView.setNeedsLayout()
//            glkView.layoutIfNeeded()
//        }
//        
//        if(drawingBufferHeight == 0 && drawingBufferWidth == 0){
//            if(!isLoaded){
//                self.isLoaded = true
//                self.readyListener?.contextReady()
//            }
//        }else {
//           // renderer.resize()
//        }
//        
//        lastSize = bounds
//    }
    
    
    
    deinit {
        if(nativeContext != 0){
            CanvasHelpers.releaseGLPointer(withContext: nativeContext)
            nativeContext = 0
        }
        if(nativeGL != 0){
            CanvasHelpers.releaseGL(withContext: nativeGL)
            nativeGL = 0
        }
    }
    
}

extension String {
    subscript(_ range: CountableRange<Int>) -> String {
        let idx1 = index(startIndex, offsetBy: max(0, range.lowerBound))
        let idx2 = index(startIndex, offsetBy: min(self.count, range.upperBound))
        return String(self[idx1..<idx2])
    }
}
