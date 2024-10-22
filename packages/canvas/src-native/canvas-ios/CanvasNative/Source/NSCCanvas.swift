//
//  NSCCanvas.swift
//
//  Created by Osei Fortune on 7/14/19.
//  Copyright © 2019 Osei Fortune. All rights reserved.
//

import Foundation
import UIKit
import GLKit
import WebKit
import MetalKit
@objcMembers
@objc(NSCCanvas)
public class NSCCanvas: UIView {
    public static var forceGL = true
    
    enum Engine {
        case None
        case CPU
        case GL
        case GPU
    }
    
    @objc(CanvasFit)
    public enum CanvasFit: Int, RawRepresentable {
        public typealias RawValue = UInt32
        
        case None
        case Fill
        case FitX
        case FitY
        case ScaleDown
        
        public var rawValue: RawValue {
            switch self {
            case .None:
                return 0
            case .Fill:
                return 1
            case .FitX:
                return 2
            case .FitY:
                return 3
            case .ScaleDown:
                return 4
            }
        }
        
        public init?(rawValue: RawValue) {
            switch rawValue {
            case 0:
                self = .None
            case 1:
                self = .Fill
            case 2:
                self = .FitX
            case 3:
                self = .FitY
            case 4:
                self = .ScaleDown
            default:
                return nil
            }
        }
    }
    
    public var fit = CanvasFit.FitX {
        willSet {
            scaleSurface()
        }
    }
        
    public static let store = NSMutableDictionary()
    
    private static var views: NSMapTable<NSString,NSCCanvas> = NSMapTable(keyOptions: .copyIn, valueOptions: .weakMemory)
    
    public static func getViews() -> NSMapTable<NSString ,NSCCanvas> {
        return views
    }
    
    var glPtr: UnsafeMutableRawPointer? = nil
    var mtlPtr: UnsafeMutableRawPointer? = nil
    var mtlLayerPtr: UnsafeMutableRawPointer? = nil
    
    public func getGlViewPtr() -> UnsafeMutableRawPointer {
        if(glPtr == nil){
            glPtr = Unmanaged.passRetained(glkView).toOpaque()
        }
        return glPtr!
    }
    
    public func getMtlViewPtr() -> UnsafeMutableRawPointer {
        if(mtlPtr == nil){
            mtlPtr = Unmanaged.passRetained(mtlView).toOpaque()
        }
        return mtlPtr!
    }
    
    
    public func getMtlLayerPtr() -> UnsafeMutableRawPointer {
        if(mtlLayerPtr == nil){
            mtlLayerPtr = Unmanaged.passRetained(mtlView.layer).toOpaque()
        }
        return mtlLayerPtr!
    }
    
    public var autoScale: Bool = true {
        didSet {
            if(!autoScale){
                glkView.contentScaleFactor = 1
            }else {
                glkView.contentScaleFactor = UIScreen.main.nativeScale
            }
        }
    }
    
    private(set) public var nativeContext: Int64 = 0
    
    private(set) public var is2D = false
    
    internal var engine = Engine.None
    
    internal var mtlView: NSCMTLView
    
    internal var glkView: CanvasGLKView
    
    
    public var drawingBufferWidth: CGFloat {
        if(engine == .GPU){
            return mtlView.frame.size.width * UIScreen.main.nativeScale
        }
        return glkView.frame.size.width * UIScreen.main.nativeScale
    }
    
    public var drawingBufferHeight: CGFloat {
        if(engine == .GPU){
            return mtlView.frame.size.height * UIScreen.main.nativeScale
        }
        return glkView.frame.size.height * UIScreen.main.nativeScale
    }
    
    var drawingBufferWidthRaw: CGFloat {
        if(engine == .GPU){
            return mtlView.frame.size.width
        }
        return glkView.frame.size.width
    }
    
    var drawingBufferHeightRaw: CGFloat {
        if(engine == .GPU){
            return mtlView.frame.size.height
        }
        return glkView.frame.size.height
    }
    
    public var width: Float {
        get {
            return Float(frame.size.width)
        }
    }
    
    public var height: Float {
        get {
            return Float(frame.size.height)
        }
    }
    
    @objc public func initContext(
        _ type: String,
        _ alpha: Bool = true,
        _ antialias: Bool = true,
        _ depth: Bool = true,
        _ failIfMajorPerformanceCaveat: Bool = false,
        _ powerPreference: Int32 = 0,
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
    @objc public func initWebGPUContext(_ instance: Int64){
        if (nativeContext != 0) {
            return
        }
        nativeContext = CanvasHelpers.initWebGPUWithView(instance, self, UInt32(surfaceWidth) as UInt32, UInt32(surfaceHeight) as UInt32)
        mtlView.isHidden = false
        engine = .GPU
    }
    
    
    func initContextWithContextAttributes(
        _ type: String,
        _ alpha: Bool,
        _ antialias: Bool,
        _ depth: Bool,
        _ failIfMajorPerformanceCaveat: Bool,
        _ powerPreference: Int32,
        _ premultipliedAlpha: Bool,
        _ preserveDrawingBuffer: Bool,
        _ stencil: Bool,
        _ desynchronized: Bool,
        _ xrCompatible: Bool
    ) {
        if (nativeContext != 0) {
            return
        }
        var version: Int32 = -1
        var isCanvas = false
        switch (type) {
        case "2d":
            version = 0
            isCanvas = true
            is2D = isCanvas
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
        
        var density = Float(UIScreen.main.nativeScale)
        
        if (!autoScale) {
            density = 1
        }
        
        var direction: Int32 = 0
        if(UIView.userInterfaceLayoutDirection(for: semanticContentAttribute) == .rightToLeft){
            direction = 1
        }
        
        if((is2D && NSCCanvas.forceGL) || version == 1 || version == 2){
            var properties: [String: Any] = [:]
            let useWebGL = !isCanvas
            if(useWebGL && preserveDrawingBuffer){
                properties[kEAGLDrawablePropertyRetainedBacking] = NSNumber(value: preserveDrawingBuffer)
            }
            
            if(alpha){
                properties[kEAGLDrawablePropertyColorFormat] = kEAGLColorFormatRGBA8
                isOpaque = false
                glkView.isOpaque = false
                (glkView.layer as! CAEAGLLayer).isOpaque = false
            }else {
                properties[kEAGLDrawablePropertyColorFormat] = kEAGLColorFormatRGBA8
                isOpaque = true
                (glkView.layer as! CAEAGLLayer).isOpaque = true
                glkView.isOpaque = true
            }
            
            
            if(!properties.isEmpty){
                let eaglLayer = self.glkView.layer as! CAEAGLLayer
                eaglLayer.drawableProperties = properties
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
            
            // antialias fails in 2D
            if(useWebGL && antialias){
                glkView.drawableMultisample = .multisample4X
            }
            
            
            if(is2D){
            nativeContext = CanvasHelpers.create2DContext(self, Int32(drawingBufferWidth), Int32(drawingBufferHeight), alpha, density, -16777216, density * 160, direction)
            }else {
                nativeContext = CanvasHelpers.initWebGLWithView(self, alpha, antialias, depth, failIfMajorPerformanceCaveat, powerPreference, premultipliedAlpha, preserveDrawingBuffer, stencil, desynchronized, xrCompatible, version)
            }

            engine = .GL
            
            if(glkView.drawableWidth == 0 && glkView.drawableHeight == 0){
                glkView.bindDrawable()
            }
            
            glkView.isHidden = false
        }else if(is2D) {
            isOpaque = !alpha
            mtlView.isOpaque = !alpha
            nativeContext = CanvasHelpers.create2DContextMetal(self, alpha, density, -16777216, density * 160, direction)
            engine = .GPU
            mtlView.isHidden = false
        }
    
        
    }
    
    @objc public func create2DContext(
        _ alpha: Bool,
        _ antialias: Bool,
        _ depth: Bool,
        _ failIfMajorPerformanceCaveat: Bool,
        _ powerPreference: Int32,
        _ premultipliedAlpha: Bool,
        _ preserveDrawingBuffer: Bool,
        _ stencil: Bool,
        _ desynchronized: Bool,
        _ xrCompatible: Bool,
        _ fontColor: Int32
    ) -> Int64 {
        
        if(nativeContext != 0){
            return nativeContext
        }
        
        initContext(
            "2d",
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
        
        return nativeContext
    }
    
    
    
    public func snapshot(_ flip: Bool) -> UIImage?{
        if(is2D){
            CanvasHelpers.flush2DContext(nativeContext)
        }
        var snapshot: UIImage? = nil
        if(engine == .GL){
            if(nativeContext != 0){
                glkView.display()
            }
            
            snapshot = glkView.snapshot
         
        }else if(engine == .GPU){
            // todo
//            let drawable = mtlView.currentDrawable?.texture
//            snapshot =  drawable?.toImage()
        }
        
        if(flip){
            return snapshot?.withHorizontallyFlippedOrientation()
        }
        return snapshot
    }
    
    
    var renderer: NSCRender? = nil
    @discardableResult public func render() -> Bool{
        if(engine == .GL){
            return CanvasHelpers.flushWebGL(nativeContext)
        }
        
        if(is2D && engine == .GPU){
           mtlView.present()
        }
        
        return false
    }
    
    private var enterBackgroundNotification: Any?
    private var becomeActiveNotification: Any?
    var enterBackgroundListener: (()-> Void)?
    var becomeActiveListener: (()-> Void)?
    private func setup(){
        self.enterBackgroundNotification = NotificationCenter.default.addObserver(forName: UIApplication.didEnterBackgroundNotification, object: nil, queue: nil){ _ in
            self.enterBackgroundListener?()
        }
        
        self.becomeActiveNotification = NotificationCenter.default.addObserver(forName: UIApplication.didBecomeActiveNotification, object: nil, queue: nil) { _ in
            self.becomeActiveListener?()
        }
    }
    
    private var handler: NSCTouchHandler?
    
    public var touchEventListener: ((String, UIGestureRecognizer) -> Void)?
    
    required init?(coder: NSCoder) {
        let scale = UIScreen.main.nativeScale
        // passing 300 px * 150 px
        // by default it will use dpi but we want px
        let unscaledWidth = (300 / scale).rounded(.down)
        let unscaledHeight = (150 / scale).rounded(.down)
        
        let frame = CGRect(x: 0, y: 0, width: unscaledWidth, height: unscaledHeight)
        mtlView = NSCMTLView(frame: frame)
        glkView = CanvasGLKView(frame: frame)
        mtlView.drawableSize = CGSize(width: 300, height: 150)
        super.init(coder: coder)
        initializeView()
    }
    
    
    public override init(frame: CGRect) {
        let scale = UIScreen.main.nativeScale
        // passing 300 px * 150 px
        // by default it will use dpi but we want px
        let unscaledWidth = (300 / scale).rounded(.down)
        let unscaledHeight = (150 / scale).rounded(.down)
        
        mtlView = NSCMTLView(frame: CGRect(x: 0, y: 0, width: unscaledWidth, height: unscaledHeight))
        glkView = CanvasGLKView(frame: CGRect(x: 0, y: 0, width: unscaledWidth, height: unscaledHeight))
        mtlView.drawableSize = CGSize(width: 300, height: 150)
        super.init(frame: frame)
        initializeView()
    }
    
    
    
    private func initializeView(){
        setup()
        if(UIScreen.instancesRespond(to: #selector(getter: UIScreen.main.nativeScale))){
            let scale = UIScreen.main.nativeScale
            glkView.contentScaleFactor = scale
            mtlView.contentScaleFactor = scale
        }
        
        glkView.canvas = self
        mtlView.canvas = self
        handler = NSCTouchHandler(canvas: self)
        backgroundColor = .clear
        glkView.enableSetNeedsDisplay = false
        glkView.isHidden = true
        mtlView.isHidden = true
        addSubview(glkView)
        addSubview(mtlView)
        scaleSurface()
        
        self.isOpaque = false
        mtlView.isOpaque = false
        addGestureRecognizer(handler!.gestureRecognizer!)
    }
    
    
    public var ignoreTouchEvents = false {
        didSet {
            if(ignoreTouchEvents){
                removeGestureRecognizer(handler!.gestureRecognizer!)
            }else {
                addGestureRecognizer(handler!.gestureRecognizer!)
            }
        }
    }
    
    
    var readyListener: NSCCanvasListener?
    public func setListener(_ listener: NSCCanvasListener?){
        readyListener = listener
    }
    
    private var isLoaded: Bool = false
    
    
    public var surfaceWidth: Int = 300 {
        willSet {
            forceLayout(CGFloat(newValue), CGFloat(surfaceHeight))
            resize()
        }
    }
    
    
    public var surfaceHeight: Int = 150 {
        willSet {
            forceLayout(CGFloat(surfaceWidth), CGFloat(newValue))
            resize()
        }
    }
    
    private func resize(){
        if(nativeContext == 0){return}
        if(!is2D && engine == .GPU){
            let width = UInt32(surfaceWidth)
            let height =  UInt32(surfaceHeight)
            CanvasHelpers.resizeWebGPUWithView(nativeContext, self, width, height)
            return
        }
        if(engine == .GL){
            EAGLContext.setCurrent(glkView.context)
        }
        
        if(engine == .GL){
            glkView.deleteDrawable()
            glkView.bindDrawable()
        }
        if(is2D){
            CanvasHelpers.resize2DContext(nativeContext, Float(surfaceWidth), Float(surfaceHeight))
        }
        
        scaleSurface()
    }
    
    public func forceLayout(_ width: CGFloat, _ height: CGFloat){
        var unscaledWidth = width.rounded(.down)
        var unscaledHeight = height.rounded(.down)
        let scale = UIScreen.main.nativeScale
        if(unscaledWidth.isZero || unscaledWidth.isLess(than: .zero)){
            unscaledWidth = 1 / scale
        }else {
            unscaledWidth = unscaledWidth / scale
        }
        
        if(unscaledHeight.isZero || unscaledHeight.isLess(than: .zero)){
            unscaledHeight = 1 / scale
        }else {
            unscaledHeight = unscaledHeight / scale
        }
        
        glkView.frame = CGRect(x: 0, y: 0, width: unscaledWidth, height: unscaledHeight)
        mtlView.frame = CGRect(x: 0, y: 0, width: unscaledWidth, height: unscaledHeight)
        mtlView.drawableSize = CGSize(width: width.rounded(.down), height: height.rounded(.down))
        glkView.setNeedsLayout()
        mtlView.setNeedsLayout()
        glkView.layoutIfNeeded()
        mtlView.layoutIfNeeded()
        
    }
    
    public func context2DTest(_ context: Int64){
        canvas_native_context_2d_test(context)
    }
    
    public func context2DPathTest(_ context: Int64){
        canvas_native_context_2d_path_test(context)
    }
    
    public func context2DConic(_ context: Int64){
        canvas_native_context_2d_conic_test(context)
    }
    
    private func scaleSurface(){
        if(surfaceWidth == 0 || surfaceHeight == 0){
            return
        }
        
         var density = UIScreen.main.nativeScale
         
         if(!autoScale){
             density = 1
         }
         
         let scaledInternalWidth = CGFloat(surfaceWidth) / density
         let scaledInternalHeight = CGFloat(surfaceHeight) / density
         
         if(scaledInternalWidth.isZero || scaledInternalHeight.isZero){
             return
         }
         
         if(frame.size.width.isZero || frame.size.width.isNaN || frame.size.height.isZero || frame.size.height.isNaN){return}
         
         let scaleX = frame.size.width / scaledInternalWidth
         let scaleY = frame.size.height / scaledInternalHeight
         
         
         if(scaleX.isZero || scaleX.isNaN ||  scaleY.isZero || scaleY.isNaN ){
         return
         }
         
        var transform: CATransform3D? = nil
         
        switch(fit){
        case .None:
            // noop
            break
        case .Fill:
            transform = CATransform3DMakeScale(scaleX, scaleY, 1)
        case .FitX:
            let dx = (frame.size.width - scaledInternalWidth) / 2
            let dy = ((scaledInternalHeight * scaleX ) - scaledInternalHeight) / 2
            
            transform = CATransform3DMakeScale(scaleX, scaleX, 1)
            
            transform = CATransform3DConcat(transform!, CATransform3DMakeTranslation(dx, dy, 0))
            break
        case .FitY:
            
            
            let dx = ((scaledInternalWidth * scaleY) - scaledInternalWidth) / 2
            let dy = (frame.size.height - scaledInternalHeight) / 2
            
            transform = CATransform3DMakeScale(scaleY, scaleY, 1)
            
            transform = CATransform3DConcat(transform!, CATransform3DMakeTranslation(dx, dy, 0))
            break
        case .ScaleDown:
            let scale =  min(min(scaleX, scaleY), 1)
            
            transform = CATransform3DMakeScale(scale, scale, 1)
            break
        }
         
         
         // disable animation
        guard let transform = transform else {return}
        glkView.layer.transform = transform
        mtlView.layer.transform = transform
    }
    
    
    public override func layoutSubviews() {
        if(!isLoaded && surfaceWidth > 0 && surfaceHeight > 0){
            self.isLoaded = true
            scaleSurface()
            let event = Timer(timeInterval: 0, repeats: false) { _ in
                self.readyListener?.contextReady()
            }
            RunLoop.current.add(event, forMode: .common)
        }else {
            resize()
        }
    }
    
    deinit {
        if(engine == .GL){
            if(nativeContext != 0){
                CanvasHelpers.releaseWebGL(nativeContext)
                nativeContext = 0
            }
        }
        
        if(glPtr != nil){
            let _ = Unmanaged<AnyObject>.fromOpaque(glPtr!).takeRetainedValue()
        }
        
        if(mtlPtr != nil){
            let _ = Unmanaged<AnyObject>.fromOpaque(mtlPtr!).takeRetainedValue()
        }
        
        if(mtlLayerPtr != nil){
            let _ = Unmanaged<AnyObject>.fromOpaque(mtlLayerPtr!).takeRetainedValue()
        }
        
        
        if(enterBackgroundNotification != nil){
            NotificationCenter.default.removeObserver(enterBackgroundNotification!)
        }
        
        if(becomeActiveNotification != nil){
            NotificationCenter.default.removeObserver(becomeActiveNotification!)
        }
    }
    
    @objc public static func getBoundingClientRect(_ view: UIView, _ buffer: UnsafeMutableRawPointer) {
        view.getBoundingClientRect(buffer)
    }
    
}

extension UIView {
    @objc public func getBoundingClientRect(_ buffer: UnsafeMutableRawPointer){
        let bytes = buffer.assumingMemoryBound(to: Float.self)
        let x = Float(self.frame.origin.x)
        let y = Float(self.frame.origin.y)
        let width = Float(self.frame.size.width)
        let height = Float(self.frame.size.height)
        bytes.pointee = y
        (bytes + 1).pointee = x + width
        (bytes + 2).pointee = y + height
        (bytes + 3).pointee = x
        (bytes + 4).pointee = width
        (bytes + 5).pointee = height
        (bytes + 6).pointee = x
        (bytes + 7).pointee = y
    }
}

extension String {
    subscript(_ range: CountableRange<Int>) -> String {
        let idx1 = index(startIndex, offsetBy: max(0, range.lowerBound))
        let idx2 = index(startIndex, offsetBy: min(self.count, range.upperBound))
        return String(self[idx1..<idx2])
    }
}


extension MTLTexture {

    func toImage() -> UIImage? {
               let ciImage = CIImage(mtlTexture: self, options: nil)?.oriented(.downMirrored)
            
               let ciContext = CIContext(mtlDevice: self.device)
            
               guard let cgImage = ciContext.createCGImage(ciImage!, from: ciImage!.extent) else {
                   return nil
               }
               
                return UIImage(cgImage: cgImage)
    }
}
