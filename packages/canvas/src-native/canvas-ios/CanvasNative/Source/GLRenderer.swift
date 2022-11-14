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
    var invalidateState = TNSCanvas.INVALIDATE_STATE_NONE  // bitwise flag
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

@objcMembers
@objc(CanvasCPUView)
public class CanvasCPUView: UIView {
    var invalidateState = TNSCanvas.INVALIDATE_STATE_NONE  // bitwise flag
    weak var renderer: GLRenderer?
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
        if !(renderer?.canvasView?.ignorePixelScaling ?? false)  {
            return Float32(UIScreen.main.nativeScale)
        }
        return 1
    }
    
    
    public override func draw(_ rect: CGRect) {
        if let renderer = renderer {
            if(renderer.context > 0){
                let width = Int(Float(frame.size.width) * deviceScale())
                let height = Int(Float(frame.size.height) * deviceScale())
                let size = width * height * 4
                let buffer = UnsafeMutablePointer<UInt8>.allocate(capacity: size)
                let colorSpace = CGColorSpaceCreateDeviceRGB()
                let ctx = CGContext(data: buffer, width: width, height: height, bitsPerComponent: 8, bytesPerRow: width * 4, space: colorSpace, bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue | CGBitmapInfo.byteOrder32Big.rawValue)
                context_custom_with_buffer_flush(renderer.context, buffer, UInt(size), Float(width), Float(height))
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
        invalidateState = TNSCanvas.INVALIDATE_STATE_NONE  // bitwise flag
    }
}


public protocol RenderListener {
    func didDraw()
}


public enum ContextType: Int, RawRepresentable {
    case none
    case webGL
    case twoD
}



public class GLRenderer: NSObject, GLKViewDelegate {
    
    public var attributes = TNSContextAttributes()
    
    public func updateDirection(_ direction: String) {
        cachedDirection = direction
    }

    public var invalidateState: Int {
        set{
            if(useCpu){
                cpuView.invalidateState = newValue
            }else {
                glkView.invalidateState = newValue
            }
        }
        get {
            if(useCpu){
                return cpuView.invalidateState
            }
            return glkView.invalidateState
        }
    }
    
    var listener: RenderListener?
    var displayRenderbuffer: GLuint
    var displayFramebuffer: GLuint
    var depthRenderbuffer: GLuint
    static var sharedGroup: EAGLSharegroup =  EAGLSharegroup()
    public var context: Int64 = 0
    weak var canvasView: TNSCanvas?
    public var view: UIView {
        get {
            if(useCpu){
                return cpuView
            }
            return glkView
        }
    }
    
    var cachedFrame: CGRect = .zero
    public var didMoveOffMain: Bool = false
    public var drawingBufferWidth: Int {
        return glkView.drawableWidth
    }
    public var drawingBufferHeight: Int {
        return glkView.drawableHeight
    }
    public var width: Float {
        get {
            return Float(view.frame.size.width)
        }
    }
    public var height: Float {
        get {
            return Float(view.frame.size.height)
        }
    }
    
    var glkView: CanvasGLKView
    var cpuView: CanvasCPUView
    var didExit: Bool = false
    var glContext: EAGLContext
    var currentOrientation: UIDeviceOrientation
    var cachedDirection: String = "ltr"
    
    func deviceScale() -> Float32 {
        if !(canvasView?.ignorePixelScaling ?? false) {
            return Float32(UIScreen.main.nativeScale)
        }
        return 1
    }
    
    func initAttributes(){
        self.isOpaque = !self.canvasView!.contextAlpha
        if(!useCpu){
            var properties: [String: Any] = [:]
            
            if(self.contextType == ContextType.webGL && self.canvasView!.contextPreserveDrawingBuffer){
                properties[kEAGLDrawablePropertyRetainedBacking] = self.canvasView!.contextPreserveDrawingBuffer
            }
            
            if(self.canvasView!.contextAlpha){
                properties[kEAGLDrawablePropertyColorFormat] = kEAGLColorFormatRGBA8
                self.isOpaque = false
            }else {
                properties[kEAGLDrawablePropertyColorFormat] = kEAGLColorFormatRGB565
                self.isOpaque = true
            }
            
            
            if(self.contextType == ContextType.webGL && self.canvasView!.contextDepth){
                self.glkView.drawableDepthFormat = .format24
            }else {
                self.glkView.drawableDepthFormat = .formatNone
            }
            
            if(self.contextType == ContextType.webGL && self.canvasView!.contextStencil){
                self.glkView.drawableStencilFormat = .format8
            }else if(self.contextType == .twoD) {
                self.glkView.drawableStencilFormat = .format8
            }
            
            if(self.contextType == ContextType.webGL && self.canvasView!.contextAntialias){
                // self.glkView.drawableMultisample = .multisample4X
            }else if(self.contextType == .twoD) {
                // self.glkView.drawableMultisample = .multisample4X
            }
        }
        setup()
    }
    public func setupContext(){
        if(Thread.isMainThread){
            if(canvasView != nil){
                initAttributes()
            }
        }else {
            let q = DispatchQueue(label: "setupContext")
            q.sync {
                if(canvasView != nil){
                    DispatchQueue.main.sync {
                        initAttributes()
                    }
                }
            }
        }
    }
    
    var forceResize = false
    func handlePixelScale(){
        forceResize = true
        if(!useCpu){
            glkView.contentScaleFactor = CGFloat(deviceScale())
        }else {
            cpuView.contentScaleFactor = CGFloat(deviceScale())
        }
        resize()
    }
    
    private var useCpu: Bool
    public init(useCpu: Bool) {
        glkView = CanvasGLKView()
        glkView.enableSetNeedsDisplay = false
        displayFramebuffer = GLuint()
        displayRenderbuffer = GLuint()
        depthRenderbuffer = GLuint()
        cpuView = CanvasCPUView()
        currentOrientation = UIDevice.current.orientation
        var glContext = EAGLContext(api: .openGLES3)
        if glContext == nil {
            glContext = EAGLContext(api: .openGLES2)
        }
        self.glContext = glContext!
        self.useCpu = useCpu
        super.init()
        if(!useCpu){
            //self.context.isMultiThreaded = true
            // glkView.layer.masksToBounds = true
            glkView.context = glContext!
            glkView.layer.isOpaque = false
            glkView.delegate = self
            // glkView.contentMode = .bottomLeft
        }else {
            cpuView.renderer = self
        }
        exitObserver = NotificationCenter.default.addObserver(forName: UIApplication.didEnterBackgroundNotification, object: nil, queue: nil) { _ in
            self.didExit = true
        }
        
        enterObserver = NotificationCenter.default.addObserver(forName: UIApplication.didBecomeActiveNotification, object: nil, queue: nil) { _ in
            self.didExit = false
        }
    }
    
    var exitObserver: Any?
    var enterObserver: Any?
    
    public func setRenderListener(listener: RenderListener?) {
        self.listener = listener
    }
    
    public var isOpaque: Bool {
        get {
            if(useCpu) {return cpuView.layer.isOpaque}
            return (glkView.layer as! CAEAGLLayer).isOpaque
        }
        set {
            if(Thread.isMainThread){
                if(useCpu){
                    self.cpuView.layer.isOpaque = newValue
                    self.cpuView.isOpaque = newValue
                    self.cpuView.superview?.isOpaque = newValue
                }else {
                    (self.glkView.layer as! CAEAGLLayer).isOpaque = newValue
                    self.glkView.superview?.isOpaque = newValue
                }
            }else {
                DispatchQueue.main.sync {
                    if(useCpu){
                        self.cpuView.layer.isOpaque = newValue
                        self.cpuView.isOpaque = newValue
                        self.cpuView.superview?.isOpaque = newValue
                    }else {
                        (self.glkView.layer as! CAEAGLLayer).isOpaque = newValue
                        self.glkView.superview?.isOpaque = newValue
                    }
                }
            }
        }
    }
    
    deinit {
        if(exitObserver != nil){
            NotificationCenter.default.removeObserver(exitObserver!)
        }
        if(enterObserver != nil){
            NotificationCenter.default.removeObserver(enterObserver!)
        }
    }
    
    
    @discardableResult public func ensureIsContextIsCurrent() ->  Bool{
        if(useCpu){
            return true
        }
        return EAGLContext.setCurrent(glContext)
    }
    
    var fboid = GLint()
    public func resize() {
        let _ = ensureIsContextIsCurrent()
        if(contextType == .twoD || contextType == .webGL){
            if(useCpu){
                let width = Float(cpuView.frame.size.width) * deviceScale()
                let height = Float(cpuView.frame.size.height) * deviceScale()
                if((width != 0 && height != 0) || forceResize){
                    context_resize_custom_surface(context,width, height, deviceScale(), true, 0)
                    cpuView.setNeedsDisplay()
                }
            } else {
                if(forceResize || (glkView.drawableWidth != Int(Float(glkView.frame.size.width) * deviceScale()) && glkView.drawableHeight != Int(Float(glkView.frame.size.height) * deviceScale()))){
                    glkView.deleteDrawable()
                    glkView.bindDrawable()
                    var binding = GLint(0)
                    glGetIntegerv(GLenum(GL_FRAMEBUFFER_BINDING), &binding)
                    displayFramebuffer = GLuint(binding)
                    var renderBufferBinding = GLint()
                    glGetIntegerv(GLenum(GL_RENDERBUFFER_BINDING), &renderBufferBinding)
                    displayRenderbuffer = GLuint(renderBufferBinding)
                    if(contextType == .webGL){
                        if let gl = canvasView?.renderingContextWebGL as? TNSWebGLRenderingContext {
                            gl.reset()
                        }
                        
                        if let gl = canvasView?.renderingContextWebGL2 as? TNSWebGL2RenderingContext {
                            gl.reset()
                        }
                    }
                    if(contextType == .twoD){
                        glClearColor(0, 0, 0, 0)
                        glClearStencil(0)
                        glClearDepthf(1)
                        glClear(GLbitfield(GL_COLOR_BUFFER_BIT | GL_STENCIL_BUFFER_BIT | GL_DEPTH_BUFFER_BIT))
                        glViewport(0, 0, GLsizei(drawingBufferWidth), GLsizei(drawingBufferHeight))
                        context_resize_surface(context,Float32(drawingBufferWidth), Float32(drawingBufferHeight),deviceScale(), binding, 4,canvasView?.contextAlpha ?? true, 100.0)
                    }
                    glkView.display()
                    
                    if(contextType == .webGL){
                        if let gl = canvasView?.renderingContextWebGL as? TNSWebGLRenderingContext {
                            gl.restoreStateAfterClear()
                        }
                        
                        if let gl = canvasView?.renderingContextWebGL2 as? TNSWebGL2RenderingContext {
                            gl.restoreStateAfterClear()
                        }
                    }
                }
            }
        }
        if forceResize {
            forceResize = false
        }
    }
    
    public func setup() {
        ensureIsContextIsCurrent()
        if((contextType == .twoD || contextType == .webGL)){
            if(useCpu){
                if(cpuView.frame.size.width == 0 && cpuView.frame.size.height == 0){
                    cpuView.frame = CGRect(x: 0, y: 0, width: CGFloat(1/deviceScale()), height: CGFloat(1/deviceScale()))
                }
                
                if(context == 0 && contextType == .twoD){
                    var direction = TNSTextDirection.Ltr
                    if(Thread.isMainThread){
                        if(UIView.userInterfaceLayoutDirection(for: cpuView.semanticContentAttribute) == .rightToLeft){
                            direction = TNSTextDirection.Rtl
                        }
                    }else {
                        DispatchQueue.main.sync {
                            if(UIView.userInterfaceLayoutDirection(for: cpuView.semanticContentAttribute) == .rightToLeft){
                                direction = TNSTextDirection.Rtl
                            }
                        }
                        
                    }
                    let width = Float(cpuView.frame.size.width) * deviceScale()
                    let height = Float(cpuView.frame.size.height) * deviceScale()
                    context = context_init_context_with_custom_surface(width, height,deviceScale(), canvasView?.contextAlpha ?? true,0,100.0, TextDirection(rawValue: direction.rawValue))
                }
                
            }else {
                if(glkView.frame.size.width == 0 && glkView.frame.size.height == 0){
                    glkView.frame = CGRect(x: 0, y: 0, width: CGFloat(1/deviceScale()), height: CGFloat(1/deviceScale()))
                }
                glkView.bindDrawable()
                var binding = GLint()
                glGetIntegerv(GLenum(GL_FRAMEBUFFER_BINDING), &binding)
                displayFramebuffer = GLuint(binding)
                var renderBufferBinding = GLint()
                glGetIntegerv(GLenum(GL_RENDERBUFFER_BINDING), &renderBufferBinding)
                displayRenderbuffer = GLuint(renderBufferBinding)
                if(contextType == .webGL){
                    if let gl = canvasView?.renderingContextWebGL as? TNSWebGLRenderingContext {
                        gl.reset()
                    }
                    
                    if let gl = canvasView?.renderingContextWebGL2 as? TNSWebGL2RenderingContext {
                        gl.reset()
                    }
                }
                
                if(context == 0 && contextType == .twoD){
                    glClearColor(0, 0, 0, 0)
                    glClear(GLbitfield(GL_COLOR_BUFFER_BIT | GL_STENCIL_BUFFER_BIT))
                    var direction = TNSTextDirection.Ltr
                    if(Thread.isMainThread){
                        if(UIView.userInterfaceLayoutDirection(for: glkView.semanticContentAttribute) == .rightToLeft){
                            direction = TNSTextDirection.Rtl
                        }
                    }else {
                        DispatchQueue.main.sync {
                            if(UIView.userInterfaceLayoutDirection(for: glkView.semanticContentAttribute) == .rightToLeft){
                                direction = TNSTextDirection.Rtl
                            }
                        }
                        
                    }
                    glViewport(0, 0, GLsizei(drawingBufferWidth), GLsizei(drawingBufferHeight))
                    context = context_init_context(Float32(drawingBufferWidth), Float32(drawingBufferHeight),deviceScale(), Int32(displayFramebuffer), 4, canvasView?.contextAlpha ?? true,0,100.0, TextDirection(rawValue: direction.rawValue))
                    
                }
                
                
                if(contextType == .webGL){
                    if let gl = canvasView?.renderingContextWebGL as? TNSWebGLRenderingContext {
                        gl.restoreStateAfterClear()
                    }
                    
                    if let gl = canvasView?.renderingContextWebGL2 as? TNSWebGL2RenderingContext {
                        gl.restoreStateAfterClear()
                    }
                }
            }
            //render()
        }
    }
    
    public var contextType: ContextType  = .none
    public func render() {
        if(didExit){return}
        ensureIsContextIsCurrent()
        if(useCpu){
            cpuView.setNeedsDisplay()
        }else {
            if(drawingBufferWidth > 0 && drawingBufferHeight > 0){
                glkView.display()
            }
        }
        if(contextType == .twoD){
            guard let listener = self.listener else {return}
            DispatchQueue.main.async {
                listener.didDraw()
            }
        }
    }
    
    public func flush(){
        if(didExit){return}
        ensureIsContextIsCurrent()
        if((invalidateState & TNSCanvas.INVALIDATE_STATE_PENDING) == TNSCanvas.INVALIDATE_STATE_PENDING){
            return
        }
        if(useCpu){
            cpuView.invalidateState = cpuView.invalidateState & TNSCanvas.INVALIDATE_STATE_INVALIDATING
        }else {
            glkView.invalidateState = cpuView.invalidateState & TNSCanvas.INVALIDATE_STATE_INVALIDATING
        }
//        if(contextType == .twoD){
//            guard let listener = self.listener else {return}
//            DispatchQueue.main.async {
//                listener.didDraw()
//            }
//        }
    }
    
    public func ensureIsReady(){
        if((invalidateState & TNSCanvas.INVALIDATE_STATE_PENDING) == TNSCanvas.INVALIDATE_STATE_PENDING){
            return
        }
        glkView.invalidateState = cpuView.invalidateState & TNSCanvas.INVALIDATE_STATE_INVALIDATING
    }
    
    
    public func pause(){
        //EAGLContext.setCurrent(nil)
    }
    
    public func resume(){
        // let _ = ensureIsContextIsCurrent()
    }
    
    func internalFlush(){
        if(contextType == .twoD && context > 0){
            context_flush(context)
        }
    }
    
    public func glkView(_ view: GLKView, drawIn rect: CGRect){
        internalFlush()
        invalidateState = TNSCanvas.INVALIDATE_STATE_INVALIDATING
    }
}
