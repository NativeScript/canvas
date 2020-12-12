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


public protocol RenderListener {
    func didDraw()
}


public enum ContextType: Int, RawRepresentable {
       case none
       case webGL
       case twoD
}



public class GLRenderer: NSObject, GLKViewDelegate {
    
    public var attributes: NSDictionary = NSMutableDictionary()
    
    public func updateDirection(_ direction: String) {
        cachedDirection = direction
    }
    
    public var isDirty: Bool {
        set{
            glkView.isDirty = newValue
        }
        get {
            return glkView.isDirty
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
    var scale: Float
    var didExit: Bool = false
    var glContext: EAGLContext
    var currentOrientation: UIDeviceOrientation
    var cachedDirection: String = "ltr"
    func initAttributes(){
        self.isOpaque = !self.canvasView!.contextAlpha
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
    

    
    public override init() {
        glkView = CanvasGLKView()
        glkView.enableSetNeedsDisplay = false
        displayFramebuffer = GLuint()
        displayRenderbuffer = GLuint()
        depthRenderbuffer = GLuint()
        var glContext = EAGLContext(api: .openGLES3)
        if glContext == nil {
            glContext = EAGLContext(api: .openGLES2)
        }
        self.glContext = glContext!
        //self.context.isMultiThreaded = true
        scale = Float(UIScreen.main.nativeScale)
        currentOrientation = UIDevice.current.orientation
        super.init()
       // glkView.layer.masksToBounds = true
        glkView.context = glContext!
        glkView.contentScaleFactor = CGFloat(scale)
        glkView.layer.isOpaque = false
        glkView.delegate = self
       // glkView.contentMode = .bottomLeft
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
            return (glkView.layer as! CAEAGLLayer).isOpaque
        }
        set {
            if(Thread.isMainThread){
                (self.glkView.layer as! CAEAGLLayer).isOpaque = newValue
                self.glkView.superview?.isOpaque = newValue
            }else {
                DispatchQueue.main.sync {
                    (self.glkView.layer as! CAEAGLLayer).isOpaque = newValue
                    self.glkView.superview?.isOpaque = newValue
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
        return EAGLContext.setCurrent(glContext)
    }
    
    var fboid = GLint()
    public func resize() {
        let _ = ensureIsContextIsCurrent()
        if(contextType == .twoD || contextType == .webGL){
            if(glkView.drawableWidth != Int(Float(glkView.frame.size.width) * scale) && glkView.drawableHeight != Int(Float(glkView.frame.size.height) * scale)){
                glkView.deleteDrawable()
                glkView.bindDrawable()
                var binding = GLint(0)
                glGetIntegerv(GLenum(GL_FRAMEBUFFER_BINDING), &binding)
                displayFramebuffer = GLuint(binding)
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
                   context_resize_surface(context,Float32(drawingBufferWidth), Float32(drawingBufferHeight),scale, binding, 4,canvasView?.contextAlpha ?? true, 100.0)
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
    
    public func setup() {
        ensureIsContextIsCurrent()
        if((contextType == .twoD || contextType == .webGL)){
            if(glkView.frame.size.width == 0 && glkView.frame.size.height == 0){
                glkView.frame = CGRect(x: 0, y: 0, width: CGFloat(1/scale), height: CGFloat(1/scale))
            }
            glkView.bindDrawable()
            var binding = GLint(0)
            glGetIntegerv(GLenum(GL_FRAMEBUFFER_BINDING), &binding)
            displayFramebuffer = GLuint(binding)
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
                context = context_init_context(Float32(drawingBufferWidth), Float32(drawingBufferHeight),scale, Int32(displayFramebuffer), 4, canvasView?.contextAlpha ?? true,0,100.0, TextDirection(rawValue: direction.rawValue))
                
            }
            
            
            if(contextType == .webGL){
                if let gl = canvasView?.renderingContextWebGL as? TNSWebGLRenderingContext {
                    gl.restoreStateAfterClear()
                }
                
                if let gl = canvasView?.renderingContextWebGL2 as? TNSWebGL2RenderingContext {
                    gl.restoreStateAfterClear()
                }
            }
            //render()
        }
    }
    
    public var contextType: ContextType  = .none
    public func render() {
        if(didExit){return}
        ensureIsContextIsCurrent()
        if(drawingBufferWidth > 0 && drawingBufferHeight > 0){
            glkView.display()
        }
        if(contextType == .twoD){
            DispatchQueue.main.async {
                self.listener?.didDraw()
            }
        }
    }
    
    public func flush(){
        if(didExit){return}
        ensureIsContextIsCurrent()
        glkView.isDirty = true
        if(contextType == .twoD){
            DispatchQueue.main.async {
                self.listener?.didDraw()
            }
        }
    }
    
    public func ensureIsReady(){
        glkView.isDirty = true
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
    }
}
