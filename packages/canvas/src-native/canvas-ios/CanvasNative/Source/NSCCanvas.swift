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
	public static var forceGL = false
	
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
		didSet {
			scaleSurface()
		}
	}
	
	public var weblikeScale = false {
		didSet {
			if(!weblikeScale){
				glkView.layer.transform = CATransform3DIdentity
				mtlView.layer.transform = CATransform3DIdentity
			}
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
	
	private(set) public var willReadFrequently = false
	
	internal var engine = Engine.None
	
	internal var mtlView: NSCMTLView
	
	internal var glkView: CanvasGLKView
	
	internal var cpuView: CanvasCPUView
	
	
	public var drawingBufferWidth: CGFloat {
		if(engine == .GPU){
			// Use drawableSize directly — it's set to the exact pixel dimensions in forceLayout.
			// frame.size * nativeScale suffers float precision loss (e.g. 500/3*3 = 499.999…).
			return mtlView.drawableSize.width
		}
		if(is2D && willReadFrequently){
			return CGFloat(surfaceWidth)
		}
		return CGFloat(surfaceWidth)
	}

	public var drawingBufferHeight: CGFloat {
		if(engine == .GPU){
			return mtlView.drawableSize.height
		}
		if(is2D && willReadFrequently){
			return CGFloat(surfaceHeight)
		}
		return CGFloat(surfaceHeight)
	}
	
	var drawingBufferWidthRaw: CGFloat {
		if(engine == .GPU){
			return mtlView.frame.size.width
		}
		
		if(is2D && willReadFrequently){
			return cpuView.frame.size.width
		}
		
		return glkView.frame.size.width
	}
	
	var drawingBufferHeightRaw: CGFloat {
		if(engine == .GPU){
			return mtlView.frame.size.height
		}
		
		if(is2D && willReadFrequently){
			return cpuView.frame.size.height
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
	
	
	
	@objc public func toDataURL(_ format: String, _ quality: Float) -> String {
		if(engine == .None){
			let rect = CGRect(x: 0, y: 0, width: surfaceWidth, height: surfaceHeight)
			let renderer = UIGraphicsImageRenderer(bounds: rect)
			
			let image = renderer.image { context in
				UIColor.white.setFill()
				context.fill(rect)
			}
			
			let base64ImageString: String
			
			switch format {
			case "image/jpeg", "image/jpg":
				if let data = image.jpegData(compressionQuality: CGFloat(quality)) {
					base64ImageString = data.base64EncodedString()
				} else if let data = image.pngData() {
					base64ImageString = data.base64EncodedString()
				} else {
					return "data:,"
				}
			case "image/heic", "image/heic-sequence":
				if #available(iOS 17.0, *), let data = image.heicData() {
					base64ImageString = data.base64EncodedString()
				} else {
					return "data:,"
				}
			default:
				if let data = image.pngData() {
					base64ImageString = data.base64EncodedString()
				} else {
					return "data:,"
				}
			}
			
			return "data:\(format);base64,\(base64ImageString)"
		}
		return "data:,"
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
		_ xrCompatible: Bool = false,
		_ willReadFrequently: Bool = false,
		_ colorSpace: Int32 = 0
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
			xrCompatible,
			willReadFrequently,
			colorSpace
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
		_ xrCompatible: Bool,
		_ willReadFrequently: Bool,
		_ colorSpace: Int32
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
			self.willReadFrequently = willReadFrequently
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
		
		var isWideGamutSupported: Bool = false
		
		var cs = colorSpace
		if #available(iOS 11.0, *) {
			if (UIScreen.main.traitCollection.displayGamut == .P3) {
				isWideGamutSupported = true
			}
		}
		
		if(!isWideGamutSupported && cs == 1){
			cs = 0
		}
		
		
		if(is2D && willReadFrequently){
			isOpaque = !alpha
			var cs = CanvasColorSpaceSrgb
			if(colorSpace == 1){
				cs = CanvasColorSpaceP3
			}
			
			nativeContext = canvas_native_ios_context_init_context_with_custom_surface(Float(drawingBufferWidth), Float(drawingBufferHeight), density, alpha, -16777216, density * 160, direction, cs)
			
			cpuView.data = NSMutableData(length: Int(drawingBufferWidth * drawingBufferHeight) * 4)
			
			canvas_native_context_set_render_func(nativeContext, Unmanaged.passRetained(cpuView).toOpaque(), CPURender)
			engine = .CPU
			cpuView.isHidden = false
		}else if((is2D && NSCCanvas.forceGL) || version == 1 || version == 2){
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
				// Use surfaceWidth/Height directly — at this point engine is still .None so
				// drawingBufferWidth would return glkView.frame * scale, which has float
				// precision loss from the divide-then-multiply round-trip in forceLayout.
				nativeContext = CanvasHelpers.create2DContext(self, Int32(surfaceWidth), Int32(surfaceHeight), alpha, density, -16777216, density * 160, direction, colorSpace)
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
			
			if(cs == 1){
				if #available(iOS 13.0, *) {
					(mtlView.layer as! CAMetalLayer).colorspace = CGColorSpace(name: CGColorSpace.displayP3)
				}
			}
			
			
			nativeContext = CanvasHelpers.create2DContextMetal(self, alpha, density, -16777216, density * 160, direction, cs)
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
		_ fontColor: Int32,
		_ willReadFrequently: Bool,
		_ colorSpace: Int32
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
			xrCompatible,
			willReadFrequently,
			colorSpace
		)
		
		return nativeContext
	}
	
	
	
	public func snapshot(_ flip: Bool) -> UIImage?{
		if(is2D && !willReadFrequently){
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
		}else if(engine == .CPU){
			snapshot = cpuView.snapshot()
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
		cpuView	= CanvasCPUView(frame: frame)
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
		cpuView	= CanvasCPUView(frame: CGRect(x: 0, y: 0, width: unscaledWidth, height: unscaledHeight))
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
		cpuView.canvas = self
		handler = NSCTouchHandler(canvas: self)
		backgroundColor = .clear
		glkView.enableSetNeedsDisplay = false
		glkView.isHidden = true
		mtlView.isHidden = true
		cpuView.isHidden = true
		addSubview(glkView)
		addSubview(mtlView)
		addSubview(cpuView)
		scaleSurface()
		
		self.isOpaque = false
		mtlView.isOpaque = false
		cpuView.isOpaque = false
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
	
	
	private var batchingSizeChange = false
	
	public var surfaceWidth: Int = 300 {
		didSet {
			if(!batchingSizeChange) {
				forceLayout(CGFloat(surfaceWidth), CGFloat(surfaceHeight))
				resize()
			}
		}
	}
	
	
	public var surfaceHeight: Int = 150 {
		didSet {
			if(!batchingSizeChange) {
				forceLayout(CGFloat(surfaceWidth), CGFloat(surfaceHeight))
				resize()
			}
		}
	}
	
	@objc public func setSurfaceSize(_ width: Int, _ height: Int) {
		if(width == surfaceWidth && height == surfaceHeight) {
			return
		}
		batchingSizeChange = true
		surfaceWidth = width
		surfaceHeight = height
		batchingSizeChange = false
		forceLayout(CGFloat(width), CGFloat(height))
		resize()
	}
	
	private func resize(){
		if(nativeContext == 0){
			scaleSurface()
			return
		}
		if(!is2D && engine == .GPU){
			let width = UInt32(surfaceWidth)
			let height =  UInt32(surfaceHeight)
			CanvasHelpers.resizeWebGPUWithView(nativeContext, self, width, height)
			scaleSurface()
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
		
		if(is2D && willReadFrequently){
			cpuView.setNeedsDisplay()
		}
	}
	
	public func forceLayout(_ width: CGFloat, _ height: CGFloat){
		var unscaledWidth = width.rounded(.down)
		var unscaledHeight = height.rounded(.down)
		let scale = UIScreen.main.nativeScale
		if(unscaledWidth.isZero || unscaledWidth.isLess(than: .zero)){
			unscaledWidth = 1
		}else {
			unscaledWidth = unscaledWidth / scale
		}

		if(unscaledHeight.isZero || unscaledHeight.isLess(than: .zero)){
			unscaledHeight = 1
		}else {
			unscaledHeight = unscaledHeight / scale
		}

		// MUST reset transforms to identity before setting frame.
		// Apple docs: "If the transform property is not the identity transform, the value of
		// this property [frame] is undefined and therefore should be ignored."
		// Setting frame on a view with a non-identity transform (applied by scaleSurface)
		// causes UIKit to compute wrong layer.position, resulting in misplaced child views.
		CATransaction.begin()
		CATransaction.setDisableActions(true)
		glkView.layer.transform = CATransform3DIdentity
		mtlView.layer.transform = CATransform3DIdentity
		cpuView.layer.transform = CATransform3DIdentity
		CATransaction.commit()

		glkView.frame = CGRect(x: 0, y: 0, width: unscaledWidth, height: unscaledHeight)
		mtlView.frame = CGRect(x: 0, y: 0, width: unscaledWidth, height: unscaledHeight)
		mtlView.drawableSize = CGSize(width: width.rounded(.down), height: height.rounded(.down))

		cpuView.frame = CGRect(x: 0, y: 0, width: unscaledWidth, height: unscaledHeight)

		cpuView.data = NSMutableData(length: Int(surfaceWidth * surfaceHeight) * 4)

		glkView.setNeedsLayout()
		mtlView.setNeedsLayout()
		cpuView.setNeedsLayout()
		glkView.layoutIfNeeded()
		mtlView.layoutIfNeeded()
		cpuView.layoutIfNeeded()
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
		if(!weblikeScale == false){
			return
		}
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
			transform = CATransform3DIdentity
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
		
		
		guard let transform = transform else {return}
		CATransaction.begin()
		CATransaction.setDisableActions(true)
		glkView.layer.transform = transform
		mtlView.layer.transform = transform
		cpuView.layer.transform = transform
		CATransaction.commit()
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
