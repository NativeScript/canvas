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
    
    internal static let INVALIDATE_STATE_NONE = 0
    internal static let INVALIDATE_STATE_PENDING = 1
    internal static let INVALIDATE_STATE_INVALIDATING = 2
    
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
    
    public var ignorePixelScaling = false {
        didSet {
            renderer.handlePixelScale()
        }
    }
    
    
    public var scaling = false {
        didSet {
            renderer.handleScaling()
        }
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
    
    
    
    public static func createSVGMatrix() -> TNSDOMMatrix {
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
                let result = context_data_url(self.context, request.type, request.format)
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
        let result = context_data_url(self.context, type, format)
        let data = String(cString: result!)
        destroy_string(result)
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
        renderer.ensureIsContextIsCurrent()
        if(renderer.contextType == ContextType.twoD){
            let result = context_snapshot_canvas(self.context)
            if(result == nil){
                return []
            }
            let pointer = result!.pointee
            let data = [UInt8](Data(bytes: pointer.data, count: Int(pointer.data_len)))
            destroy_u8_array(result)
            return data
        }else if(renderer.contextType == ContextType.webGL){
            let pixels = (renderer.view as! CanvasGLKView).snapshot
            
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
                var buffer: [UInt8] = Array(repeating: 0, count: height * row)
                let colorSpace = CGColorSpaceCreateDeviceRGB()
                let imageCtx = CGContext(data: &buffer, width: width, height: height, bitsPerComponent: 8, bytesPerRow: row, space: colorSpace, bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue | CGBitmapInfo.byteOrder32Big.rawValue)
                imageCtx!.draw(image, in: CGRect(x: 0, y: 0, width: width, height: height))
                return buffer
            }
        }
        return []
    }
    
    
    public func getImage() -> UIImage?{
        renderer.ensureIsContextIsCurrent()
        let snapshot = (renderer.view as! CanvasGLKView).snapshot
        return snapshot.withHorizontallyFlippedOrientation()
    }
    
    
    public func snapshotEncoded() -> [UInt8]{
        renderer.ensureIsContextIsCurrent()
        if(renderer.contextType == ContextType.twoD){
            let result = context_snapshot_canvas_encoded(self.context)
            if(result == nil){
                return []
            }
            let pointer = result!.pointee
            let data = [UInt8](Data(bytes: pointer.data, count: Int(pointer.data_len)))
            destroy_u8_array(result)
            return data
        }else if(renderer.contextType == ContextType.webGL){
            let pixels = (renderer.view as! CanvasGLKView).snapshot
            let data = pixels.pngData() ?? Data()
            EAGLContext.setCurrent(nil)
            return [UInt8](data)
        }
        return []
    }
    
    var _isGL: Bool = false
    var invalidateState = TNSCanvas.INVALIDATE_STATE_NONE  // bitwise flag
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
    var renderer: GLRenderer
    public var context: Int64 {
        get {
            return renderer.context
        }
        set {
            renderer.context = newValue
        }
    }
    
    var renderingContext2d: TNSCanvasRenderingContext?
    var renderingContextWebGL: TNSCanvasRenderingContext?
    var renderingContextWebGL2: TNSCanvasRenderingContext?
    public func doDraw() {
        if(handleInvalidationManually){return}
        renderer.invalidateState = renderer.invalidateState | TNSCanvas.INVALIDATE_STATE_INVALIDATING
    }
    
    public func flush(){
        renderer.render()
    }
    
    var useGL: Bool = false
    func setup(){
        renderer.canvasView = self
        renderer.setRenderListener(listener: self)
        addSubview(renderer.view)
    }
    
    required init?(coder: NSCoder) {
        renderer = GLRenderer(useCpu: false)
        useCpu = false
        super.init(coder: coder)
        realInit()
    }
    
    func realInit() {
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
    
    private var useCpu: Bool
    
    public init(frame: CGRect, useCpu: Bool) {
        renderer = GLRenderer(useCpu: useCpu)
        self.useCpu = useCpu
        super.init(frame: frame)
        realInit()
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
        
        if((renderer.invalidateState & TNSCanvas.INVALIDATE_STATE_INVALIDATING) == TNSCanvas.INVALIDATE_STATE_INVALIDATING){
            renderer.invalidateState = renderer.invalidateState & TNSCanvas.INVALIDATE_STATE_PENDING
            renderer.render()
        }
    }
    
    private var lastSize: CGRect = .null
    private var isLoaded: Bool = false
    
    public static func layoutView(_ view: UIView, _ width: CGFloat, _ height: CGFloat){
        let frameSize = view.frame.size
        
        if (width == frameSize.width && height == frameSize.height) {
            return
        }
        
        let frame_origin = view.frame.origin
        let frame = CGRectMake(frame_origin.x, frame_origin.y, width, height)
        view.frame = frame
        view.setNeedsLayout()
        view.layoutIfNeeded()
    }
    
    var drawingBufferWidth: Int {
        get {
            return renderer.drawingBufferWidth
        }
    }
    
    
    var drawingBufferHeight: Int {
        get {
            return renderer.drawingBufferHeight
        }
    }
    
    var drawingBufferWidthDip: Int {
        get {
            return Int(renderer.width)
        }
    }
    
    
    var drawingBufferHeightDip: Int {
        get {
            return Int(renderer.width)
        }
    }
    
    
    public override func layoutSubviews() {
        super.layoutSubviews()
        if(bounds == .zero || (bounds.size.width < 0 || bounds.size.height < 0)){
            renderer.view.frame = CGRect(x: 0, y: 0, width: CGFloat(1/UIScreen.main.nativeScale), height: CGFloat(1/UIScreen.main.nativeScale))
        }else {
            renderer.view.frame = bounds
            renderer.view.setNeedsLayout()
            renderer.view.layoutIfNeeded()
        }
        
        if(useCpu){
            if(bounds != .zero || (bounds.size.width > 0 || bounds.size.height > 0)) {
                if(!isLoaded){
                    DispatchQueue.main.async {
                        self.isLoaded = true
                        self.readyListener?.contextReady()
                    }
                }else {
                    renderer.resize()
                }
            }
        }else {
            if(renderer.drawingBufferHeight == 0 && renderer.drawingBufferWidth == 0){
                if(!isLoaded){
                    DispatchQueue.main.async {
                        self.isLoaded = true
                        self.readyListener?.contextReady()
                    }
                }
            }else {
                renderer.resize()
            }
        }
        lastSize = bounds
    }
    
    
    
    deinit {
        if(context > 0){
            destroy_context(context)
            context = 0
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
        return getContext(type: type, contextAttributes: TNSContextAttributes.defaultInstance)
    }
    
    public func getContext(type: String, attributes: String) -> TNSCanvasRenderingContext? {
        return getContext(type: type, contextAttributes: TNSContextAttributes.fromJSONString(attributes))
    }
    
    
    public func getContext(_ type: String, contextAttributes: Dictionary<String, Any>) -> TNSCanvasRenderingContext? {
        return getContext(type: type, contextAttributes: TNSContextAttributes.fromMap(contextAttributes))
    }
    
    
    private func updateContextAttributes(attributes: TNSContextAttributes){
        self.contextAlpha = attributes.alpha
        self.contextDepth = attributes.depth
        self.contextFailIfMajorPerformanceCaveat = attributes.failIfMajorPerformanceCaveat
        self.contextPowerPreference = attributes.powerPreference
        self.contextPremultipliedAlpha = attributes.premultipliedAlpha
        self.contextPreserveDrawingBuffer = attributes.preserveDrawingBuffer
        self.contextStencil = attributes.stencil
        self.contextDesynchronized = attributes.desynchronized
        self.contextXrCompatible = attributes.xrCompatible
    }
    
    public func getContext(type: String, contextAttributes: TNSContextAttributes) -> TNSCanvasRenderingContext? {
        if useCpu && type != "2d" {
            return nil
        }
        if type.elementsEqual("2d"){
            if(renderingContext2d == nil){
                renderer.contextType = .twoD
                updateContextAttributes(attributes: contextAttributes)
                renderer.attributes = contextAttributes
                renderingContext2d = TNSCanvasRenderingContext2D(self)
                renderer.setupContext()
            }else {
                renderer.contextType = .twoD
                renderer.ensureIsContextIsCurrent()
            }
            return renderingContext2d!
        }else if(type.elementsEqual("webgl")){
            if(renderingContextWebGL == nil){
                renderer.contextType = .webGL
                updateContextAttributes(attributes: contextAttributes)
                renderer.attributes = contextAttributes
                renderingContextWebGL = TNSWebGLRenderingContext(self)
                renderer.setupContext()
            }else {
                renderer.contextType = .webGL
                renderer.ensureIsContextIsCurrent()
            }
            
            return renderingContextWebGL!
        }else if(type.elementsEqual("webgl2")){
            if(renderer.glContext.api != .openGLES3){
                return emptyCanvas
            }
            
            if(renderingContextWebGL2 == nil){
                renderer.contextType = .webGL
                updateContextAttributes(attributes: contextAttributes)
                renderer.attributes = contextAttributes
                renderingContextWebGL2 = TNSWebGL2RenderingContext(self)
                renderer.setupContext()
            }else {
                renderer.contextType = .webGL
                renderer.ensureIsContextIsCurrent()
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
