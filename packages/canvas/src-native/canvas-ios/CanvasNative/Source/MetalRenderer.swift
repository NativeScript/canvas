//
//  MetalRenderer.swift
//  CanvasNative
//
//  Created by Osei Fortune on 3/24/20.
//

import Foundation
import UIKit
import MetalKit
public class MetalRenderer: NSObject, Renderer, MTKViewDelegate {
    public func resize() {
        
    }
    
    public var attributes: NSDictionary = NSMutableDictionary()
    weak var canvasView: TNSCanvas?
    public var didMoveOffMain: Bool = false
    var cachedDirection = "ltr"
    var cachedFrame: CGRect = .zero
    public func updateDirection(_ direction: String) {
        cachedDirection = direction
    }
    
    public var isDirty: Bool = false
    
    var listener: RenderListener?
    public func setRenderListener(listener: RenderListener?) {
        self.listener = listener
    }
    
    public func updateSize() {
        cachedFrame = mtlView.frame
        mtlView.drawableSize = CGSize(width: mtlView.frame.size.width * CGFloat(scale), height: mtlView.frame.size.height * CGFloat(scale))
    }
    
    public var canvas: Int64 = 0
    public var canvasState: [Int64] = []
    var scale: Float
    var done: Bool = false
    
    public var isOpaque: Bool {
          get {
              return mtlView.isOpaque
          }
          set {
              mtlView.isOpaque = newValue
          }
      }
    
    public func ensureIsReady() {
        setup()
    }
    
    public func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {
        setup()
    }
    
    public func setupContext() {
          
      }
    
    public func setup() {
        if(!done){
            var direction = "ltr"
            if(didMoveOffMain){
                direction = cachedDirection
            }else {
                if(UIView.userInterfaceLayoutDirection(for: mtlView.semanticContentAttribute) == .rightToLeft){
                               direction = "rtl"
                           }
            }
            canvas = native_init(devicePtr, queuePtr, viewPtr, Float(scale), (direction as NSString).utf8String)
            done = true
        }
    }
    
    
    public var drawingBufferWidth: Int{
        return Int(mtlView.drawableSize.width)
    }
      
    public var drawingBufferHeight: Int {
        return Int(mtlView.drawableSize.height)
    }
    
    
    public var width: Float {
        get {
            return Float(mtlView.frame.size.width)
        }
    }
    
    public var height: Float {
        get {
            return Float(mtlView.frame.size.height)
        }
    }
    
    public var contextType: ContextType = .none
    
    public func draw(in view: MTKView) {
        view.clearColor = MTLClearColor(red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0)
        if(!pending) {return}
        canvas = native_flush(canvas)
        guard let commandBuffer = queue?.makeCommandBuffer() else {return}
        guard let drawable = mtlView.currentDrawable else {return}
        
        // https://stackoverflow.com/questions/60774782/addpresentedhandler-not-being-triggered-in-metal-on-ios
        #if targetEnvironment(simulator)
        commandBuffer.addCompletedHandler { _ in
            self.listener?.didDraw()
        }
        #else
        if #available(iOS 10.3, *) {
            drawable.addPresentedHandler { _ in
                self.listener?.didDraw()
            }
        }else {
            commandBuffer.addCompletedHandler { _ in
                self.listener?.didDraw()
            }
        }
        
        #endif
        
        commandBuffer.present(drawable)
        commandBuffer.commit()
        pending = false
    }
    
    var mtlDevice: MTLDevice?
    var queue: MTLCommandQueue?
    var devicePtr: UnsafeMutableRawPointer?
    var queuePtr: UnsafeMutableRawPointer?
    var viewPtr: UnsafeMutableRawPointer?
    var texturePtr: UnsafeMutableRawPointer?
    var layerPtr: UnsafeMutableRawPointer?
    var drawQueue = DispatchQueue(label: "CanvasMetalQueue", qos: .userInteractive, attributes: [], autoreleaseFrequency: .workItem)
    private var mtlView: MTKView
    
    public func ensureIsContextIsCurrent() -> Bool {
        return true
    }
    
    override init() {
        mtlView = MTKView()
        mtlDevice = MTLCreateSystemDefaultDevice()
        queue = mtlDevice?.makeCommandQueue()
        scale = Float(UIScreen.main.scale)
        super.init()
        mtlView.device = mtlDevice
        mtlView.isOpaque = true
        mtlView.layer.isOpaque = false
        mtlView.colorPixelFormat = .bgra8Unorm
        mtlView.framebufferOnly = true
        mtlView.isPaused = true
        mtlView.enableSetNeedsDisplay = true
        mtlView.presentsWithTransaction = false
        mtlView.sampleCount = 1
        mtlView.delegate = self
        //mtlView.autoResizeDrawable = false
        mtlView.clearColor  = MTLClearColor(red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0)
        devicePtr = Unmanaged.passRetained(mtlDevice!).toOpaque()
        queuePtr = Unmanaged.passRetained(queue!).toOpaque()
        layerPtr = Unmanaged.passRetained(self.mtlView.layer).toOpaque()
        viewPtr = Unmanaged.passRetained(self.mtlView).toOpaque()
    }
    public var view: UIView {
        get {
            return self.mtlView
        }
    }
    private var pending: Bool = false
    public func render() {
        pending = true
        self.mtlView.setNeedsDisplay()
    }
    
    public func pause(){
    
    }
    
    public func resume(){}
    public func flush(){
        render()
    }
}
