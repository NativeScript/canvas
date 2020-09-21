//
//  TNSCanvas.swift
//
//  Created by Osei Fortune on 7/14/19.
//  Copyright © 2019 Osei Fortune. All rights reserved.
//

import Foundation
import UIKit
import MetalKit


@objcMembers
@objc(TNSCanvas)
public class TNSCanvas: UIView, RenderListener {
    private static var views: NSMapTable<NSString,TNSCanvas> = NSMapTable(keyOptions: .copyIn, valueOptions: .weakMemory)
    
    public static func getViews() -> NSMapTable<NSString,TNSCanvas> {
        return views
    }
    
    var displayLink: CADisplayLink?
    var ptr: UnsafeMutableRawPointer?
    
    public func getViewPtr() -> UnsafeMutableRawPointer? {
        if(ptr == nil && !isGL){
            ptr = Unmanaged.passRetained(renderer.view).toOpaque()
        }
        return ptr
    }
    
    
    var contextAlpha = true;
    var contextAntialias = true;
    var contextDepth = true;
    var contextFailIfMajorPerformanceCaveat = false;
    var contextPowerPreference = "default";
    var contextPremultipliedAlpha = true;
    var contextPreserveDrawingBuffer = false;
    var contextStencil = false;
    var contextDesynchronized = false;
    var contextXrCompatible = false;
    
    var mClearStencil: Int32 = 0
    var mClearColor: [Float] = [0,0,0,0]
    var mScissorEnabled = false
    var mClearDepth: Float = 1
    var mColorMask: [Bool] = [true, true, true, true]
    var mStencilMask: UInt32 = 0xFFFFFFFF
    var mStencilMaskBack: UInt32 = 0xFFFFFFFF;
    var mStencilFuncRef: Int32 = 0;
    var mStencilFuncRefBack: Int32 = 0;
    var mStencilFuncMask: UInt32 = 0xFFFFFFFF;
    var mStencilFuncMaskBack: UInt32 = 0xFFFFFFFF;
    var mDepthMask: Bool = true

    
    
    public static func createSVGMatrix() -> TNSDOMMatrix{
        TNSDOMMatrix()
    }
    var isContextLost: Bool = false
    var _handleInvalidationManually: Bool = false
    public var handleInvalidationManually: Bool {
        get {
            return _handleInvalidationManually
        }
        set {
            _handleInvalidationManually = newValue
            if(newValue){
                displayLink?.invalidate()
                displayLink = nil
                _fps = 0
            }else {
                displayLink = CADisplayLink(target: self, selector: #selector(handleAnimation))
                displayLink?.add(to: .main, forMode: .common)
            }
        }
    }
    public func didDraw() {
        if(dataURLCallbacks.count == 0) {return}
        DispatchQueue.main.async {
            for request in self.dataURLCallbacks {
                let result = native_to_data_url(self.canvas, request.type, request.format)
                let data = String(cString: result!)
                request.callback(data)
            }
        }
    }
    
    public func toDataURL() -> String {
        return toDataURL("image/png")
    }
    
    public func toDataURL(_ type: String) -> String {
        return toDataURL(type, 0.92)
    }
    
    public func toDataURL(_ type: String,_ format: Float) -> String {
        if(renderer.contextType == ContextType.webGL){
            var ss = snapshot()
            let data = Data(bytes: &ss, count: ss.count)
            return data.base64EncodedString(options: Data.Base64EncodingOptions(rawValue: 0))
        }
        let result = native_to_data_url(canvas, type, format)
        let data = String(cString: result!)
        native_free_char(result)
        return data
    }
    
    
    public func toDataURLAsync(_ callback: @escaping (String) -> Void) {
        toDataURLAsync("image/png", callback)
    }
    
    public func toDataURLAsync(_ type: String,_ callback: @escaping (String) -> Void) {
        toDataURLAsync(type, 0.92, callback)
    }
    
    private var dataURLCallbacks: [DataURLRequest] = []
    
    public func toDataURLAsync(_ type: String,_ format: Float,_ callback: @escaping (String) -> Void) {
        dataURLCallbacks.append(DataURLRequest(type: type, format: format, callback: callback))
    }
    
    public func snapshot() -> [UInt8]{
        let _ = renderer.ensureIsContextIsCurrent()
        if(renderer.contextType == ContextType.twoD){
            let result = native_snapshot_canvas(canvas)
            if(result == nil){
                return []
            }
            let pointer = result!.pointee
            let data = [UInt8](Data(bytes: pointer.array, count: pointer.length))
            native_free_byte_array(result)
            return data
        }else if(renderer.contextType == ContextType.webGL){
            if let gl = renderer as? GLRenderer {
                let pixels = (gl.view as! CanvasGLKView).snapshot
                let data = pixels.pngData() ?? Data()
                EAGLContext.setCurrent(nil)
                return [UInt8](data)
            }else if let metal = renderer as? MetalRenderer {
                let metalView = (metal.view as! MTKView)
                UIGraphicsBeginImageContextWithOptions(frame.size, true, 0.0)
                let context = UIGraphicsGetCurrentContext()!
                context.fill(frame)
                metalView.layer.render(in: context)
                let image = UIGraphicsGetImageFromCurrentImageContext()
                UIGraphicsEndImageContext()
                let data = image?.pngData() ?? Data()
                EAGLContext.setCurrent(nil)
                return [UInt8](data)
            }
        }
        return []
    }
    
    var _isGL: Bool = false
    var isDirty: Bool = false
    public var isGL: Bool {
        get {
            return _isGL
        }
    }
    
    public func getId() -> GLint {
        GLint()
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
    
    public func updateDirection(_ direction: String){
        renderer.updateDirection(direction)
    }
    
    var didMoveOffMain: Bool = false
    var didWait: Bool = false
    var renderer: Renderer
    public var canvas: Int64 {
        get {
            return renderer.canvas
        }
        set {
            renderer.canvas = newValue
        }
    }
    public var canvasState: [Int64]  {
        get {
            return renderer.canvasState
        }
        set {
            renderer.canvasState = newValue
        }
    }
    var renderingContext2d: TNSCanvasRenderingContext?
    var renderingContextWebGL: TNSCanvasRenderingContext?
    var renderingContextWebGL2: TNSCanvasRenderingContext?
    public func doDraw() {
        if(handleInvalidationManually){return}
        renderer.isDirty = true
    }
    
    public func flush(){
        renderer.render()
    }
    
    var useGL: Bool = false
    func setup(){
        if(useGL){
            if let glRenderer = renderer as? GLRenderer {
                glRenderer.canvasView = self
            }
        }else {
            if let mtlRenderer = renderer as? MetalRenderer {
                mtlRenderer.canvasView = self
            }
        }
        renderer.setRenderListener(listener: self)
        addSubview(renderer.view)
    }
    
    public init(frame: CGRect, useGL: Bool) {
        self.useGL = useGL
        if(useGL){
            _isGL = true
            renderer = GLRenderer()
        }else {
            renderer = MetalRenderer()
        }
        super.init(frame: frame)
        setup()
        
        self.isOpaque = false
        self.displayLink = CADisplayLink(target: self, selector: #selector(handleAnimation))
        self.displayLink?.preferredFramesPerSecond = 60
        self.displayLink?.add(to: .main, forMode: .common)
        
        NotificationCenter.default.addObserver(forName: UIApplication.didEnterBackgroundNotification, object: nil, queue: nil) { _ in
            self.displayLink?.invalidate()
            self.displayLink = nil
            self._fps = 0
        }
        
        NotificationCenter.default.addObserver(forName: UIApplication.didBecomeActiveNotification, object: nil, queue: nil) { _ in
            if(self.displayLink == nil){
                self.displayLink = CADisplayLink(target: self, selector: #selector(self.handleAnimation))
                self.displayLink?.add(to: .main, forMode: .common)
            }
        }
    }
    
    var _fps: Float = 0
    var readyListener: TNSCanvasListener?
    public func setListener(_ listener: TNSCanvasListener?){
        readyListener = listener
    }
    public var fps: Float {
        get {
            return _fps
        }
    }
    
    @objc func handleAnimation(displayLink: CADisplayLink){
        self._fps = Float(1 / (displayLink.targetTimestamp - displayLink.timestamp))
        if(renderer.isDirty){
            renderer.render()
            renderer.isDirty = false;
        }
    }
    
    private var lastSize: CGRect = .null
    private var isLoaded: Bool = false
    public override func layoutSubviews() {
        super.layoutSubviews()
        if(bounds == .zero || (bounds.size.width < 0 || bounds.size.height < 0)){
            renderer.view.frame = CGRect(x: 0, y: 0, width: CGFloat(1/UIScreen.main.nativeScale), height: CGFloat(1/UIScreen.main.nativeScale))
        }else {
            renderer.view.frame = bounds
            renderer.view.setNeedsLayout()
            renderer.view.layoutIfNeeded()
        }
        
        if(renderer.drawingBufferHeight == 0 && renderer.drawingBufferWidth == 0){
            if(!isLoaded){
               self.readyListener?.contextReady()
               self.isLoaded = false
            }
        }else {
            renderer.resize()
        }
        lastSize = bounds
    }
    
    
    required init(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    
    deinit {
        if(canvas > 0){
            renderer.resume()
            native_destroy(canvas)
            canvas = 0
        }
    }
    
    public func resume(){
        self.displayLink = CADisplayLink(target: self, selector: #selector(self.handleAnimation))
        self.displayLink?.add(to: .main, forMode: .common)
        renderer.resume()
    }
    public func pause(){
        displayLink?.invalidate()
        displayLink = nil
        _fps = 0
        renderer.pause()
    }
    
    public func moveToMain(){
        self.pause()
        didMoveOffMain = false
        renderer.didMoveOffMain = false
    }
    
    public func moveOffMain(){
        self.pause()
        didMoveOffMain = true
        renderer.didMoveOffMain = true
    }
    
    public func handleMoveOffMain(){
        self.displayLink = CADisplayLink(target: self, selector: #selector(self.handleAnimation))
        self.displayLink?.add(to: .main, forMode: .common)
        renderer.resume()
    }
    
    public func handleMoveToMain(){
        self.displayLink = CADisplayLink(target: self, selector: #selector(self.handleAnimation))
        self.displayLink?.add(to: .main, forMode: .common)
        renderer.resume()
    }
    
    private var emptyCanvas = TNSCanvasRenderingContext()
    public func getContext(_ type: String) -> TNSCanvasRenderingContext? {
        let attributes = NSMutableDictionary()
        if type.elementsEqual("2d"){
            attributes.setValue(contextAlpha, forKey: "alpha")
        }else if(type.contains("webgl")){
            attributes["alpha"] = contextAlpha
            attributes.setValuesForKeys([
                "alpha": contextAlpha,
                "depth": contextDepth,
                "failIfMajorPerformanceCaveat": contextFailIfMajorPerformanceCaveat,
                "powerPreference": "default",
                "premultipliedAlpha": contextPremultipliedAlpha,
                "preserveDrawingBuffer": contextPreserveDrawingBuffer,
                "stencil": contextStencil,
                "desynchronized": contextDesynchronized,
                "xrCompatible": contextXrCompatible
            ])
        }
        return getContext(type, contextAttributes: attributes)
    }
    
    private func updateContextAttributes(attributes: [AnyHashable: Any]){
        self.contextAlpha = attributes["alpha"] as? Bool ?? true
        self.contextDepth = attributes["depth"] as? Bool ?? true
        self.contextFailIfMajorPerformanceCaveat = attributes["failIfMajorPerformanceCaveat"] as? Bool ?? false
        self.contextPowerPreference = attributes["powerPreference"] as? String ?? "default"
        self.contextPremultipliedAlpha = attributes["premultipliedAlpha"] as? Bool ?? true
        self.contextPreserveDrawingBuffer = attributes["preserveDrawingBuffer"] as? Bool ?? false
        self.contextStencil = attributes["stencil"] as? Bool ?? false
        self.contextDesynchronized = attributes["desynchronized"] as? Bool ?? false
        self.contextXrCompatible = attributes["xrCompatible"] as? Bool ?? false
    }
    
    public func getContext(_ type: String, contextAttributes: NSDictionary) -> TNSCanvasRenderingContext? {
        if type.elementsEqual("2d"){
            if(renderingContext2d == nil){
                renderer.contextType = .twoD
                updateContextAttributes(attributes: contextAttributes as! [AnyHashable : Any])
                renderer.attributes = contextAttributes
                renderingContext2d = TNSCanvasRenderingContext2D(self)
                renderer.setupContext()
            }else {
                 renderer.contextType = .twoD
                if let gl = renderer as? GLRenderer{
                    let _ = gl.ensureIsContextIsCurrent()
                }
            }
            return renderingContext2d!
        }else if(type.elementsEqual("webgl")){
            if(renderingContextWebGL == nil){
                renderer.contextType = .webGL
                updateContextAttributes(attributes: contextAttributes as! [AnyHashable : Any])
                renderer.attributes = contextAttributes
                renderingContextWebGL = TNSWebGLRenderingContext(self)
                renderer.setupContext()
            }else {
                renderer.contextType = .webGL
                if let gl = renderer as? GLRenderer{
                    let _ = gl.ensureIsContextIsCurrent()
                }
            }
            
            return renderingContextWebGL!
        }else if(type.elementsEqual("webgl2")){
            if let render = renderer as? GLRenderer {
                if(render.context.api != .openGLES3){
                    return emptyCanvas
                }
            }
            
            if(renderingContextWebGL2 == nil){
                renderer.contextType = .webGL
                updateContextAttributes(attributes: contextAttributes as! [AnyHashable : Any])
                renderer.attributes = contextAttributes
                renderingContextWebGL2 = TNSWebGL2RenderingContext(self)
                renderer.setupContext()
            }else {
                renderer.contextType = .webGL
                if let gl = renderer as? GLRenderer{
                    let _ = gl.ensureIsContextIsCurrent()
                }
            }
            return renderingContextWebGL2!
        }
        return emptyCanvas
    }
}

extension String {
    subscript(_ range: CountableRange<Int>) -> String {
        let idx1 = index(startIndex, offsetBy: max(0, range.lowerBound))
        let idx2 = index(startIndex, offsetBy: min(self.count, range.upperBound))
        return String(self[idx1..<idx2])
    }
}
