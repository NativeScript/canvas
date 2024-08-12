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
    
    enum Engine {
        case None
        case CPU
        case GL
        case Metal
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
    
    public var fit = CanvasFit.Fill {
        didSet {
            scaleSurface()
        }
    }
    
    private static let shared_context_view = GLKView(frame: .init(x: 0, y: 0, width: 1, height: 1))
    
    private static var _shared_context: Int64 = 0
    private static var shared_context: Int64 {
        get {
            
            if(_shared_context != 0){
                return _shared_context
            }
            
            var properties: [String: Any] = [:]
            
            properties[kEAGLDrawablePropertyColorFormat] = kEAGLColorFormatRGBA8
            shared_context_view.isOpaque = false
            (shared_context_view.layer as! CAEAGLLayer).isOpaque = false
            
            let eaglLayer = shared_context_view.layer as! CAEAGLLayer
            eaglLayer.drawableProperties = properties
            
            shared_context_view.drawableDepthFormat = .format24
            
            
            let ptr = Unmanaged.passRetained(shared_context_view).toOpaque()
            let viewPtr = Int64(Int(bitPattern: ptr))
            
            
            _shared_context = CanvasHelpers.initGLWithView(viewPtr, true, true, true, false, 0, true, false, false, false, false, 2, false)
            
            
            return _shared_context
            
        }
    }
    
    public static let store = NSMutableDictionary()
    
    private static var views: NSMapTable<NSString,NSCCanvas> = NSMapTable(keyOptions: .copyIn, valueOptions: .weakMemory)
    
    public static func getViews() -> NSMapTable<NSString,NSCCanvas> {
        return views
    }
    
    var ptr: UnsafeMutableRawPointer? = nil
    var mtlPtr: UnsafeMutableRawPointer? = nil
    
    public func getViewPtr() -> UnsafeMutableRawPointer {
        if(ptr == nil){
            ptr = Unmanaged.passRetained(glkView).toOpaque()
        }
        return ptr!
    }
    
    public func getMtlViewPtr() -> UnsafeMutableRawPointer {
        if(mtlPtr == nil){
            mtlPtr = Unmanaged.passRetained(mtlView).toOpaque()
        }
        return mtlPtr!
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
    
    
    private(set) public var nativeGL: Int64 = 0
    
    private(set) public var nativeContext: Int64 = 0
    
    private(set) var native2DContext: Int64 = 0
    
    internal var is2D = false
    
    private var engine = Engine.None
    
    internal var mtlView: MTKView
    
    internal var glkView: CanvasGLKView
    
    
    public var drawingBufferWidth: CGFloat {
        if(engine == .Metal){
            return mtlView.frame.size.width * UIScreen.main.nativeScale
        }
        return glkView.frame.size.width * UIScreen.main.nativeScale
    }
    
    public var drawingBufferHeight: CGFloat {
        if(engine == .Metal){
            return mtlView.frame.size.height * UIScreen.main.nativeScale
        }
        return glkView.frame.size.height * UIScreen.main.nativeScale
    }
    
    var drawingBufferWidthRaw: CGFloat {
        if(engine == .Metal){
            return mtlView.frame.size.width
        }
        return glkView.frame.size.width
    }
    
    var drawingBufferHeightRaw: CGFloat {
        if(engine == .Metal){
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
        let viewPtr = Int64(Int(bitPattern: getMtlViewPtr()))
        nativeContext = CanvasHelpers.initWebGPUWithView(instance, viewPtr, UInt32(surfaceWidth) as UInt32, UInt32(surfaceHeight) as UInt32)
        mtlView.isHidden = false
        engine = .Metal
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
        if (nativeGL != 0 || nativeContext != 0) {
            return
        }
        var version = -1
        var isCanvas = false
        switch (type) {
        case "2d":
            version = 1
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
            version = 0
            break
        }
        
        if (version == -1) {
            return
        }
        
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
        
        let viewPtr = Int64(Int(bitPattern: getViewPtr()))
        
        
        let shared_context = NSCCanvas.shared_context
        
        nativeGL = CanvasHelpers.initSharedGLWithView(viewPtr,alpha, antialias, depth, failIfMajorPerformanceCaveat, powerPreference, premultipliedAlpha, preserveDrawingBuffer, stencil, desynchronized, xrCompatible, Int32(version), isCanvas, shared_context)
        
        engine = .GL
        
        if(glkView.drawableWidth == 0 && glkView.drawableHeight == 0){
            glkView.bindDrawable()
        }
        
        // get new fbo
        
        nativeContext = CanvasHelpers.getGLPointer(nativeGL)
        
        glkView.isHidden = false
        
        
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
        
        if(native2DContext != 0){
            return native2DContext
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
        
        // disable for now
        let samples: Int32 = antialias ? 0 : 0
        
        
        glViewport(0, 0, GLsizei(drawingBufferWidth), GLsizei(drawingBufferHeight))
        
        var density = Float(UIScreen.main.nativeScale)
        
        if (!autoScale) {
            density = 1
        }
        
        
        var direction: Int32 = 0
        if(UIView.userInterfaceLayoutDirection(for: semanticContentAttribute) == .rightToLeft){
            direction = 1
        }
        
        native2DContext = CanvasHelpers.create2DContext(
            nativeGL, Int32(drawingBufferWidth), Int32(drawingBufferHeight),
            alpha, density, samples, fontColor, density * 160, direction
        )
        
        return native2DContext
    }
    
    
    
    public func snapshot(_ flip: Bool) -> UIImage?{
        if(is2D){
            CanvasHelpers.flush2DContext(native2DContext)
        }else {
            if(nativeGL != 0){
                glkView.display()
            }
        }
        let snapshot = glkView.snapshot
        if(flip){
            return snapshot.withHorizontallyFlippedOrientation()
        }
        return snapshot
    }
    
    
    var renderer: NSCRender? = nil
    @discardableResult public func render() -> Bool{
        return CanvasHelpers.flushGL(nativeGL)
    }
    
    
    public func context2DTest(_ context: Int64){
        CanvasHelpers.test2D(context)
        render()
    }
    
    public func context2DTestToDataURL(_ context: Int64) -> String{
        return CanvasHelpers.testToDataURL(context)
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
        let unscaledWidth = 300 / scale
        let unscaledHeight = 150 / scale
        
        let frame = CGRect(x: 0, y: 0, width: unscaledWidth, height: unscaledHeight)
        mtlView = MTKView(frame: frame)
        glkView = CanvasGLKView(frame: frame)
        super.init(coder: coder)
        initializeView()
    }
    
    
    public override init(frame: CGRect) {
        let scale = UIScreen.main.nativeScale
        // passing 300 px * 150 px
        // by default it will use dpi but we want px
        let unscaledWidth = 300 / scale
        let unscaledHeight = 150 / scale
        
        mtlView = MTKView(frame: CGRect(x: 0, y: 0, width: unscaledWidth, height: unscaledHeight))
        glkView = CanvasGLKView(frame: CGRect(x: 0, y: 0, width: unscaledWidth, height: unscaledHeight))
        super.init(frame: frame)
        initializeView()
    }
    
    
    
    private func initializeView(){
        if #available(iOS 13.0, *) {
            if((mtlView.layer as? CAMetalLayer) != nil){
               // let layer = mtlView.layer as! CAMetalLayer
                // https://developer.apple.com/documentation/quartzcore/cametallayer/1478157-presentswithtransaction/
                //  layer.presentsWithTransaction = false
                // layer.framebufferOnly = true
            }
        }
        let scale = UIScreen.main.nativeScale
        if(UIScreen.instancesRespond(to: #selector(getter: UIScreen.main.nativeScale))){
            glkView.contentScaleFactor = scale
            mtlView.contentScaleFactor = scale
        }
        
        glkView.canvas = self
        handler = NSCTouchHandler(canvas: self)
        backgroundColor = .clear
        glkView.enableSetNeedsDisplay = false
        glkView.isHidden = true
        mtlView.isHidden = true
        addSubview(glkView)
        addSubview(mtlView)
        scaleSurface()
        
        self.isOpaque = false
        addGestureRecognizer(handler!.gestureRecognizer!)
    }
    
    
    var ignoreTouchEvents = false {
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
        if(nativeGL == 0){return}
        if(engine == .GL){
            EAGLContext.setCurrent(glkView.context)
        }
        scaleSurface()
        if(engine == .GL){
            glkView.deleteDrawable()
            glkView.bindDrawable()
        }
        if(nativeContext != 0 && is2D){
            CanvasHelpers.resize2DContext(native2DContext, Float(drawingBufferWidth), Float(drawingBufferHeight))
        }
    }
    
    public func forceLayout(_ width: CGFloat, _ height: CGFloat){
        var unscaledWidth = width
        var unscaledHeight = height
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
        glkView.setNeedsLayout()
        mtlView.setNeedsLayout()
        glkView.layoutIfNeeded()
        mtlView.layoutIfNeeded()
        
    }
    
    private func scaleSurface(){
        if(surfaceWidth == 0 || surfaceHeight == 0){
            return
        }
     /*
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
        
        var transform = CGAffineTransform.identity

        switch(fit){
        case .None:
            // noop
            break
        case .Fill:
            transform = CGAffineTransform.identity.scaledBy(x: scaleX , y: scaleY)
            
        case .FitX:
            let dx = (frame.size.width - scaledInternalWidth) / 2
            let  dy = ((scaledInternalHeight * scaleX ) - scaledInternalHeight) / 2
            
            
            transform = CGAffineTransform.identity.scaledBy(x: scaleX, y: scaleX).translatedBy(x: dx, y: dy)
            break
        case .FitY:
            
           
            let dx = ((scaledInternalWidth * scaleY) - scaledInternalWidth) / 2
            let dy = (frame.size.height - scaledInternalHeight) / 2
            
            
            transform = CGAffineTransform.identity.scaledBy(x: scaleY, y: scaleY).translatedBy(x: dx, y: dy)
            break
        case .ScaleDown:
            let scale =  min(min(scaleX, scaleY), 1)
            
            transform = CGAffineTransform.identity.scaledBy(x: scale, y: scale)
            break
        }
        
        
        // disable animation
       
        glkView.transform = transform
        mtlView.transform = transform
        
        */
        
        
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
                if(nativeGL != 0){
                    CanvasHelpers.releaseGLPointer(nativeContext)
                }
                
                nativeContext = 0
            }
            if(nativeGL != 0){
                CanvasHelpers.releaseGL(nativeGL)
                nativeGL = 0
            }
        }
        
        if(ptr != nil){
            let _ = Unmanaged<AnyObject>.fromOpaque(ptr!).takeRetainedValue()
        }
        
        if(mtlPtr != nil){
            let _ = Unmanaged<AnyObject>.fromOpaque(mtlPtr!).takeRetainedValue()
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
