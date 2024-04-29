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

@objcMembers
@objc(NSCCanvas)
public class NSCCanvas: UIView {
    
    
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
    
    public func getViewPtr() -> UnsafeMutableRawPointer {
        if(ptr == nil){
            ptr = Unmanaged.passRetained(glkView).toOpaque()
        }
        return ptr!
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
    
    internal var glkView: CanvasGLKView
    internal var is2D = false
    
    
    public var drawingBufferWidth: Int {
        return Int(glkView.frame.size.width)
    }
    public var drawingBufferHeight: Int {
        return Int(glkView.frame.size.height)
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
        if (nativeGL != 0) {
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
        
        if(frame.size.width.isZero || frame.size.height.isZero){
            let width = frame.size.width.isZero ? 1 : frame.size.width
            let height = frame.size.height.isZero ? 1 : frame.size.height
            frame = CGRect(x: frame.origin.x, y: frame.origin.y, width: CGFloat(width), height: CGFloat(height))
            zeroSize = true
            setNeedsLayout()
            layoutIfNeeded()
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
        
        
        if(glkView.drawableWidth == 0 && glkView.drawableHeight == 0){
            glkView.bindDrawable()
        }
                
        // get new fbo
        
        nativeContext = CanvasHelpers.getGLPointer(nativeGL)
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
    
    
    
    
    public func forceLayout(_ width: CGFloat, _ height: CGFloat){
        if(frame.size.equalTo(CGSize(width: width, height: height))){
            return
        }
        frame = CGRect(x: frame.origin.x, y: frame.origin.y, width: width, height: height)
        setNeedsLayout()
        layoutIfNeeded()
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
        glkView = CanvasGLKView(coder: coder)!
        glkView.translatesAutoresizingMaskIntoConstraints = false
        
        if(UIScreen.instancesRespond(to: #selector(getter: UIScreen.main.nativeScale))){
            glkView.contentScaleFactor = UIScreen.main.nativeScale
        }
        
        super.init(coder: coder)
        
        glkView.canvas = self
        
        
        handler = NSCTouchHandler(canvas: self)
        backgroundColor = .clear
        glkView.enableSetNeedsDisplay = false
        addSubview(glkView)
        
        NSLayoutConstraint.activate([
            glkView.leadingAnchor.constraint(equalTo: leadingAnchor),
            glkView.trailingAnchor.constraint(equalTo: trailingAnchor),
            glkView.topAnchor.constraint(equalTo: topAnchor),
            glkView.bottomAnchor.constraint(equalTo: bottomAnchor)
        ])
        
        
        self.isOpaque = false
        addGestureRecognizer(handler!.gestureRecognizer!)
    }
    
    public override init(frame: CGRect) {
        glkView = CanvasGLKView(frame: frame)
        glkView.translatesAutoresizingMaskIntoConstraints = false
        if(UIScreen.instancesRespond(to: #selector(getter: UIScreen.main.nativeScale))){
            glkView.contentScaleFactor = UIScreen.main.nativeScale
        }
        super.init(frame: frame)
        glkView.canvas = self
        
        handler = NSCTouchHandler(canvas: self)
        backgroundColor = .clear
        glkView.enableSetNeedsDisplay = false
        addSubview(glkView)
        
        NSLayoutConstraint.activate([
            glkView.leadingAnchor.constraint(equalTo: leadingAnchor),
            glkView.trailingAnchor.constraint(equalTo: trailingAnchor),
            glkView.topAnchor.constraint(equalTo: topAnchor),
            glkView.bottomAnchor.constraint(equalTo: bottomAnchor)
        ])
        
        
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
    
    
    private var lastSize: CGRect = .null
    private var isLoaded: Bool = false
    
    private func resize(){
        if(nativeGL == 0){return}
        EAGLContext.setCurrent(glkView.context)
       // glkView.deleteDrawable()
       // glkView.bindDrawable()
        if(is2D){
            glViewport(0, 0, GLsizei(drawingBufferWidth), GLsizei(drawingBufferHeight))
            CanvasHelpers.resize2DContext(native2DContext, Float(drawingBufferWidth), Float(drawingBufferHeight))
        }
    }
    
    
    private var viewSize = CGSize.zero
    private var realViewSize = CGSize.zero
    private var zeroSize = false
    private var isReady = false
    public override func layoutSubviews() {
        if(viewSize.equalTo(frame.size)){
            return
        }
        
        if(!zeroSize){
            realViewSize.width = frame.size.width
            realViewSize.height = frame.size.height
        }
        
        if(!isReady && nativeGL == 0 && !realViewSize.width.isZero && !realViewSize.height.isZero){
            viewSize.width = frame.size.width
            viewSize.height = frame.size.height
            isReady = true
            readyListener?.contextReady()
        }
        
        /*
        if((!viewSize.width.isZero && !viewSize.height.isZero) && frame.width.isZero || frame.height.isZero){
            let width = width.isZero ? 1 : width
            let height = height.isZero ? 1 : height
            frame = CGRect(x: frame.origin.x, y: frame.origin.y, width: CGFloat(width), height: CGFloat(height))
        }
        */
        if(!frame.width.isZero && !frame.height.isZero){
            viewSize = CGSize(width: frame.size.width, height: frame.size.height)
        }
        if(nativeGL != 0) {
            resize()
        }
        
        zeroSize = false
    }
    
    
    
    deinit {
        if(nativeContext != 0){
            CanvasHelpers.releaseGLPointer(nativeContext)
            nativeContext = 0
        }
        if(nativeGL != 0){
            CanvasHelpers.releaseGL(nativeGL)
            nativeGL = 0
        }
        
        if(ptr != nil){
            let _ = Unmanaged<AnyObject>.fromOpaque(ptr!).takeRetainedValue()
        }
        
        if(enterBackgroundNotification != nil){
            NotificationCenter.default.removeObserver(enterBackgroundNotification!)
        }
        
        if(becomeActiveNotification != nil){
            NotificationCenter.default.removeObserver(becomeActiveNotification!)
        }
    }
    
    @objc public static func getBoundingClientRect(_ view: UIView, _ buffer: UnsafeMutableRawPointer) {
        let bytes = buffer.assumingMemoryBound(to: Float.self)
        let x = Float(view.frame.origin.x)
        let y = Float(view.frame.origin.y)
        let width = Float(view.frame.size.width)
        let height = Float(view.frame.size.height)
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
