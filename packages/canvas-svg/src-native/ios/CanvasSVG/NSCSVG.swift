//
//  NSCSVG.swift
//  CanvasSVG
//
//  Created by Osei Fortune on 18/03/2024.
//

import Foundation
import UIKit

@objcMembers
@objc(NSCSVGData)
public class NSCSVGData: NSObject {
	var data_size: CGSize = .zero
	var buf_size: UInt = 0
	private(set) public var data: NSMutableData? = nil
	let colorSpace = CGColorSpaceCreateDeviceRGB()
	var image: UIImage? = nil
	
	
	func resize(_ width: CGFloat, _ height: CGFloat){
		if(!width.isZero && !width.isNaN && !height.isZero && !height.isNaN){
			image = nil
			data = NSMutableData(length: Int(width * height * 4))
			buf_size = UInt(width * height * 4)
			data_size = CGSize(width: CGFloat(width), height: CGFloat(height))
		}
	}

	func setPixels(_ bytes: NSData, _ width: CGFloat, _ height: CGFloat){
		image = nil
		data = NSMutableData(data: bytes as Data)
		buf_size = UInt(bytes.length)
		data_size = CGSize(width: width, height: height)
	}
	
	public var width: CGFloat {
		get {
			return data_size.width
		}
	}
	
	public var height: CGFloat {
		get {
			return data_size.height
		}
	}
	
	public var rawData: UnsafeMutableRawPointer? {
		get {
			return data?.mutableBytes
		}
	}
	
	
	public func getImage() -> UIImage? {
		return getImage(1)
	}

	/// `scale` is the device scale the pixels were rendered at. Without it the image is taken
	/// as 1x and draws at pixel size, oversized on every Retina screen.
	public func getImage(_ scale: CGFloat) -> UIImage? {
		if(image != nil){
			return image
		}
		guard let data = data else {return nil}

		let width = Int(self.data_size.width)
		let height = Int(self.data_size.height)
		let ctx = CGContext(data: data.mutableBytes, width: width, height: height, bitsPerComponent: 8, bytesPerRow: width * 4, space: colorSpace, bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue | CGBitmapInfo.byteOrder32Big.rawValue)


		guard let cgImage = ctx?.makeImage() else {return nil}
		self.image = UIImage(cgImage: cgImage, scale: scale, orientation: .up)
		return self.image
	}

	/// Buffer of exactly this pixel size, reused when the size is unchanged. Reallocating it
	/// every frame is what makes an animated svg churn.
	func ensureBuffer(_ width: Int32, _ height: Int32) -> NSMutableData? {
		if width <= 0 || height <= 0 {
			return nil
		}
		if data == nil || Int(data_size.width) != Int(width) || Int(data_size.height) != Int(height) {
			resize(CGFloat(width), CGFloat(height))
		}
		// Pixels come straight from the renderer, so the cached image is stale every frame.
		image = nil
		return data
	}
}


@objcMembers
@objc(NSCSVG)
public class NSCSVG: UIView {
	var didInitDrawing = false
	var forceResize = false
	public var sync = false
	var data: NSCSVGData? = nil
	var customData = false
	private var usingData = false
	public var autoScale = true {
		didSet {
			forceResize = true
		}
	}
	public var src: String? = nil {
		didSet {
			if(usingData){
				return
			}
			if(customData){
				data = NSCSVGData()
				forceResize = true
				customData = false
				update()
				return
			}
			doDraw()
		}
	}
	
	public var srcPath: String? = nil {
		didSet {
			if(usingData){
				return
			}
			if(customData){
				data = NSCSVGData()
				forceResize = true
				customData = false
				update()
				return
			}
			customData = false
			doDraw()
		}
	}
	
	
	fileprivate static func executeInLoop(_ runloop: CFRunLoop?, _ function: @escaping() -> Void){
		if let runloop = runloop {
			CFRunLoopPerformBlock(runloop, CFRunLoopMode.commonModes.rawValue) {
				function()
			}
			CFRunLoopWakeUp(runloop)
		}else {
			function()
		}
	}
	
	
	// MARK: - GPU

	/// Mirrors `canvas_svg_c::gpu::Backend`.
	@objc public enum Backend: Int32 {
		case auto = 0
		case gl = 1
		case vulkan = 2
		case metal = 3
	}

	/// Mirrors `canvas_svg_c::gpu::FrameStatus`.
	private static let statusRecovered: Int32 = 2
	private static let statusLost: Int32 = 3

	private var gpuContext: Int64 = 0
	private var renderThread: Int64 = 0
	/// The retained host handed to the render thread, released once it has joined.
	private var renderThreadView: UnsafeMutableRawPointer?
	/// Held as `UIView` rather than `SVGMetalView`: a stored property may not name a type that
	/// is only available from iOS 13, and the GPU path is gated on that.
	private var metalView: UIView?
	private var pendingDocument: Int64 = 0
	private var pendingScale: Float = 1
	private var pendingWidth: Int32 = 0
	private var pendingHeight: Int32 = 0
	private var surfaceWidth: Int32 = 0
	private var surfaceHeight: Int32 = 0

	/// Told when the GPU context dies and when one comes back. A context survives neither the
	/// GPU being reclaimed nor a driver reset, and the view keeps drawing either way; this
	/// exists so the JS side can say which path it is on.
	public var onContextLost: (() -> Void)?
	public var onContextRestored: (() -> Void)?

	/// Rasterize on a separate thread so a heavy document doesn't block the UI. Off by default.
	public var threaded = false {
		didSet {
			if threaded != oldValue { applySurfaceMode() }
		}
	}

	/// Falls back to the raster path when no GPU surface can be made.
	public var gpu = true {
		didSet {
			if gpu != oldValue { applySurfaceMode() }
		}
	}

	/// Forces a rasterizer. `.auto` is right unless a device's driver is the problem.
	public var backend: Backend = .auto {
		didSet {
			if backend != oldValue { applySurfaceMode() }
		}
	}

	/// Which backend is actually running, once a surface exists.
	public var activeBackend: Backend {
		if gpuContext != 0 {
			return Backend(rawValue: CanvasSVGHelper.gpuBackend(gpuContext)) ?? .auto
		}
		// The threaded renderer keeps its context on its own thread and does not hand the handle
		// out, so what it resolved `auto` to is not observable from here.
		if renderThread != 0 { return backend }
		return .auto
	}

	public var isGpuActive: Bool {
		return gpuContext != 0 || renderThread != 0
	}

	/// Adds or removes the Metal host to match `gpu`.
	private func applySurfaceMode() {
		destroyGpuContext()
		metalView?.removeFromSuperview()
		metalView = nil
		// The next host lays out and reports its own size; keeping the old one would let a
		// rebuild come back at a size nothing is drawing at.
		surfaceWidth = 0
		surfaceHeight = 0

		if !gpu {
			setNeedsDisplay()
			return
		}

		guard #available(iOS 13.0, tvOS 13.0, *) else {
			// No Metal to be had; the raster path always draws.
			gpu = false
			return
		}
		let host = SVGMetalView(frame: bounds)
		host.autoresizingMask = [.flexibleWidth, .flexibleHeight]
		host.onSizeChanged = { [weak self] width, height in
			self?.onSurfaceResized(width, height)
		}
		metalView = host
		addSubview(host)
	}

	@discardableResult
	private func createGpuContext() -> Bool {
		guard gpuContext == 0, renderThread == 0, let host = metalView, surfaceWidth > 0, surfaceHeight > 0 else {
			return false
		}
		if threaded {
			let view = Unmanaged.passRetained(host).toOpaque()
			renderThread = CanvasSVGHelper.renderThreadCreate(view, width: surfaceWidth, height: surfaceHeight, backend: backend.rawValue)
			if renderThread != 0 {
				renderThreadView = view
				return true
			}
			// Could not start one; the single-threaded context still might work.
			Unmanaged<UIView>.fromOpaque(view).release()
		}
		// The surface holds the view for as long as it lives (it rebuilds itself against the
		// same pointer after a device loss), so hand it a retained one. `destroyGpuContext`
		// gives it back.
		let view = Unmanaged.passRetained(host).toOpaque()
		gpuContext = CanvasSVGHelper.gpuCreate(view, width: surfaceWidth, height: surfaceHeight, backend: backend.rawValue)
		if gpuContext == 0 {
			Unmanaged<UIView>.fromOpaque(view).release()
			return false
		}
		return true
	}

	private func onSurfaceResized(_ width: Int32, _ height: Int32) {
		surfaceWidth = width
		surfaceHeight = height
		if gpuContext == 0 && renderThread == 0 {
			if !createGpuContext() {
				// No usable context: drop back to the raster path, which always works.
				gpu = false
				return
			}
		} else if renderThread != 0 {
			CanvasSVGHelper.renderThreadResize(renderThread, width: width, height: height)
		} else {
			CanvasSVGHelper.gpuResize(gpuContext, width: width, height: height)
		}
		replayLastFrame()
	}

	/// Throws the GPU context away so the next frame exercises recovery. For testing.
	public func debugLoseContext() {
		if gpuContext != 0 {
			CanvasSVGHelper.gpuDebugLoseContext(gpuContext)
		}
	}

	private func destroyGpuContext() {
		if renderThread != 0 {
			// Blocks until the thread has joined, so the host is safe to release after.
			CanvasSVGHelper.renderThreadDestroy(renderThread)
			renderThread = 0
			if let view = renderThreadView {
				Unmanaged<UIView>.fromOpaque(view).release()
				renderThreadView = nil
			}
		}
		if gpuContext == 0 { return }
		// Read the view out first: `gpuDestroy` frees the surface that holds it.
		let view = CanvasSVGHelper.gpuView(gpuContext)
		CanvasSVGHelper.gpuDestroy(gpuContext)
		gpuContext = 0
		if let view = view {
			Unmanaged<UIView>.fromOpaque(view).release()
		}
	}

	private func replayLastFrame() {
		if pendingDocument == 0 { return }
		renderFrame(pendingDocument, pendingScale, pendingWidth, pendingHeight)
	}

	/// Renders a live document. On the GPU that draws straight into the layer's drawable; on
	/// the CPU it renders into the backing buffer in place. Neither stages the frame in an
	/// intermediate buffer (the copy an animated svg would otherwise pay every frame).
	public func renderDocument(_ document: Int64, _ width: Int32, _ height: Int32, _ scale: Float) {
		if document == 0 { return }
		pendingDocument = document
		pendingScale = scale
		pendingWidth = width
		pendingHeight = height

		renderFrame(document, scale, width, height)
	}

	/// Draws one frame on whichever path is available, and deals with a context that died doing
	/// it. Native already rebuilds a lost context a few times itself and only reports
	/// `statusLost` once it has given up, but it rebuilds against the same layer, so one more
	/// attempt through a fresh context is worth making before the view gives up on the GPU.
	private func renderFrame(_ document: Int64, _ scale: Float, _ width: Int32, _ height: Int32) {
		if renderThread != 0 {
			// Paint into a display list here and hand it over; the rasterizing happens on the
			// render thread, so this returns without waiting for it.
			CanvasSVGHelper.renderThreadCommit(renderThread, document: document, width: width, height: height, scale: scale)
			// Reported asynchronously, so loss surfaces a frame or two late; it only decides
			// which path JS is told it is on.
			let status = CanvasSVGHelper.renderThreadStatus(renderThread)
			if status == NSCSVG.statusRecovered {
				onContextRestored?()
			} else if status == NSCSVG.statusLost {
				destroyGpuContext()
				onContextLost?()
				if !createGpuContext() {
					gpu = false
				}
			}
			return
		}
		if gpuContext != 0 {
			let status = CanvasSVGHelper.gpuRender(gpuContext, document: document, scale: scale)
			if status == NSCSVG.statusRecovered {
				onContextRestored?()
			}
			if status != NSCSVG.statusLost {
				return
			}

			destroyGpuContext()
			onContextLost?()

			if createGpuContext(),
			   CanvasSVGHelper.gpuRender(gpuContext, document: document, scale: scale) != NSCSVG.statusLost {
				onContextRestored?()
				return
			}
			// Out of options. The raster path always draws, so the view keeps working; turning
			// `gpu` off also drops the now-useless Metal host and is visible from JS.
			gpu = false
		}
		// Keyed off the context, not the `gpu` flag: until a drawable exists there is nothing
		// to draw into, and the view would otherwise sit blank.
		guard let data = self.data?.ensureBuffer(width, height),
		      let buf = data.mutableBytes.assumingMemoryBound(to: UInt8.self) as UnsafeMutablePointer<UInt8>? else {
			return
		}
		CanvasSVGHelper.renderDocument(
			document,
			data: buf,
			size: UInt(data.length),
			width: width,
			height: height,
			rowBytes: UInt(Int(width) * 4),
			scale: scale
		)
		customData = true
		didInitDrawing = true
		setNeedsDisplay()
	}

	public func loadBuffer(_ buffer: NSData, _ width: CGFloat, _ height: CGFloat){
		func apply() {
			self.customData = true
			self.usingData = true
			self.data?.setPixels(buffer, width, height)
			self.usingData = false
			self.didInitDrawing = true
			self.setNeedsDisplay()
		}
		if Thread.isMainThread {
			apply()
			return
		}
		DispatchQueue.main.async {
			apply()
		}
	}

	public func loadData(_ data: NSCSVGData){
		if(Thread.isMainThread){
			self.data = data
			self.customData = true
			usingData = true
			self.src = nil
			self.srcPath = nil
			usingData = false
			self.didInitDrawing = true
			self.setNeedsDisplay()
			return
		}
		DispatchQueue.main.async {
			self.data = data
			self.customData = true
			self.usingData = true
			self.src = nil
			self.srcPath = nil
			self.usingData = false
			self.didInitDrawing = true
			self.setNeedsDisplay()
		}
	}
	
	func deviceScale() -> CGFloat {
		if autoScale  {
			#if os(visionOS)
			// visionOS has no UIScreen; derive the scale from the active scene's trait environment.
			let scale = UITraitCollection.current.displayScale
			return scale > 0 ? scale : 2.0
			#else
			return UIScreen.main.nativeScale
			#endif
		}
		return 1
	}
	
	var workItem: DispatchWorkItem?
	func doDraw(){
		if self.srcPath == nil && self.src == nil {return}
		workItem?.cancel()
		
		if(sync){
			if(self.srcPath != nil){
				guard let srcPath = self.srcPath else{return}
				let source = srcPath as NSString
				
				guard let data = self.data else {return}
				
				guard let buf = data.rawData?.assumingMemoryBound(to: UInt8.self) else {return}
				
				CanvasSVGHelper.draw(fromPath: buf, size: data.buf_size, width: Float(data.data_size.width), height: Float(data.data_size.height), path: source as String)
				
				self.didInitDrawing = true
				self.setNeedsDisplay()
				return
			}
			
			guard let src = self.src else{return}
			let source = src as NSString
			
			guard let data = self.data else {return}
			
			guard let buf = data.rawData?.assumingMemoryBound(to: UInt8.self) else {return}
			
			
			CanvasSVGHelper.draw(fromString: buf, size: data.buf_size, width: Float(data.data_size.width), height: Float(data.data_size.height), svg: source as String)
			
			self.didInitDrawing = true
			self.setNeedsDisplay()
			
			return
		}
		
		
		workItem = DispatchWorkItem {
			[weak self] in
			guard let self =  self else {return}
			
			if(self.srcPath != nil){
				guard let srcPath = self.srcPath else{return}
				let source = srcPath as NSString
				guard let data = self.data else {return}
				guard let buf = data.rawData?.assumingMemoryBound(to: UInt8.self) else {return}
				
				
				CanvasSVGHelper.draw(fromPath: buf, size: data.buf_size, width: Float(data.data_size.width), height: Float(data.data_size.height), path: source as String)
				
				
				DispatchQueue.main.async { [self] in
					if(self.workItem != nil && self.workItem!.isCancelled){
						self.workItem = nil
						return
					}
					self.didInitDrawing = true
					self.setNeedsDisplay()
				}
				return
			}
			
			
			guard let src = self.src else{return}
			let source = src as NSString
			
			guard let data = self.data else {return}
			
			guard let buf = data.rawData?.assumingMemoryBound(to: UInt8.self) else {return}
			
			
			
			CanvasSVGHelper.draw(fromString: buf, size: data.buf_size, width: Float(data.data_size.width), height: Float(data.data_size.height), svg: source as String)
			
			
			DispatchQueue.main.async {
				[self] in
				if(self.workItem != nil && self.workItem!.isCancelled){
					self.workItem = nil
					return
				}
				self.didInitDrawing = true
				self.setNeedsDisplay()
			}
		}
		queue.async(execute: workItem!)
	}
	
	
	public func update(){
		if(customData){
			setNeedsDisplay()
			return
		}
		let size = layer.frame.size
		let scale = deviceScale()
		let width = size.width * scale
		let height = size.height * scale
		guard let data = data else {return}
		if (width != data.data_size.width && height != data.data_size.height) || forceResize {
			data.resize(width, height)
			doDraw()
			
			if forceResize {
				forceResize = false
			}
		}
	}
	
	public override func layoutSubviews() {
		update()
	}
	
	
	let colorSpace = CGColorSpaceCreateDeviceRGB()
	private var queue: DispatchQueue
	public override init(frame: CGRect) {
		queue = DispatchQueue(label: "NSCSVG")
		data = NSCSVGData()
		super.init(frame: frame)
		// SVG content composites over whatever is behind the view; an opaque white background
		// also hides the Metal layer entirely.
		backgroundColor = .clear
		isOpaque = false
		applySurfaceMode()
	}

	required init?(coder: NSCoder) {
		queue = DispatchQueue(label: "NSCSVG")
		data = NSCSVGData()
		super.init(coder: coder)
		backgroundColor = .clear
		isOpaque = false
		applySurfaceMode()
	}

	deinit {
		destroyGpuContext()
	}
	
	private func drawImage(_ rect: CGRect){
		guard let data = self.data else {return}
		// The pixels are at device scale; drawing them as a 1x image would blow them up.
		guard let image = data.getImage(deviceScale()) else {return}
		image.draw(in: rect)
	}

	public override func draw(_ rect: CGRect) {
		if gpuContext != 0 {
			return
		}
		if didInitDrawing {
			drawImage(rect)
		}
	}
	
	public func toImage() -> UIImage? {
		if didInitDrawing {
			return self.data?.getImage()
		}
		return nil
	}
	
	public func toData() -> NSData? {
		return self.data?.data
	}
	
	public static func fromStringSync(_ source: String) -> NSCSVGData? {
		let dim = parseSVGDimensions(source)
		if(dim.width.isZero || dim.height.isZero){
			return nil
		}
		let width = dim.width
		let height = dim.height
		
		let data = NSCSVGData()
		data.resize(CGFloat(width), CGFloat(height))
		
		guard let buf = data.rawData?.assumingMemoryBound(to: UInt8.self) else {return nil}
		CanvasSVGHelper.draw(fromString: buf, size: data.buf_size, width: Float(data.data_size.width), height: Float(data.data_size.height), svg: source as String)
		
		return data
	}
	
	
	public static func fromPathSync(_ path: String) -> NSCSVGData? {
		var data: NSCSVGData? = nil
		if(!FileManager.default.fileExists(atPath: path)){return nil}
		do {
			let text = try String(contentsOfFile: path)
			if (text.isEmpty) {
				return nil
			}
			
			let dim = parseSVGDimensions(text)
			if(dim.width.isZero || dim.height.isZero){
				return nil
			}
			
			let ret = NSCSVGData()
			
			ret.resize(CGFloat(dim.width), CGFloat(dim.height))
			
			guard let buf = ret.rawData?.assumingMemoryBound(to: UInt8.self) else {return nil}
			
			CanvasSVGHelper.draw(fromString: buf, size: ret.buf_size, width: Float(ret.data_size.width), height: Float(ret.data_size.height), svg: text as String)
			
			data = ret
			
		}catch{
			return nil
		}
		return data
	}
	
	
	public static func fromRemoteSync(_ path: String) -> NSCSVGData? {
		var data: NSCSVGData? = nil
		guard let url = URL(string: path) else {
			return nil
		}
		do {
			let contents = try Data(contentsOf: url)
			guard let text = String(data: contents, encoding: .utf8) else {
				return nil
			}
			if (text.isEmpty) {
				return nil
			}
			
			let dim = parseSVGDimensions(text)
			
			if(dim.width.isZero || dim.height.isZero){
				return nil
			}
			
			
			let ret = NSCSVGData()
			
			ret.resize(CGFloat(dim.width), CGFloat(dim.height))
			
			guard let buf = ret.rawData?.assumingMemoryBound(to: UInt8.self) else {return nil}
			
			CanvasSVGHelper.draw(fromString: buf, size: ret.buf_size, width: Float(ret.data_size.width), height: Float(ret.data_size.height), svg: text as String)
			
			data = ret
			
			
		}catch{
			return nil
		}
		return data
	}
	
	
	public static func fromString(_ source: String, _ callback:@escaping ((NSCSVGData?)-> Void)) {
		let current = CFRunLoopGetCurrent()
		DispatchQueue.global(qos: .utility).async {
			
			
			let dim = parseSVGDimensions(source)
			
			
			if(dim.width.isZero || dim.height.isZero){
				executeInLoop(current) {
					callback(nil)
				}
				return
			}
			
			var data = NSCSVGData()
			data.resize(CGFloat(dim.width), CGFloat(dim.height))
			
			
			guard let buf = data.rawData?.assumingMemoryBound(to: UInt8.self) else {
				executeInLoop(current) {
					callback(nil)
				}
				return
			}
			
			CanvasSVGHelper.draw(fromString: buf, size: data.buf_size, width: Float(data.data_size.width), height: Float(data.data_size.height), svg: source as String)
			
			executeInLoop(current) {
				callback(data)
			}
			
		}
	}
	
	
	public static func fromPath(_ path: String, _ callback:@escaping ((NSCSVGData?)-> Void)) {
		let current =  CFRunLoopGetCurrent()
		DispatchQueue.global(qos: .utility).async {
			
			if(!FileManager.default.fileExists(atPath: path)){
				executeInLoop(current) {
					callback(nil)
				}
				return
			}
			do {
				let text = try String(contentsOfFile: path)
				if (text.isEmpty) {
					executeInLoop(current) {
						callback(nil)
					}
					return
				}
				
				var dim = parseSVGDimensions(text)
				if(dim.width.isZero || dim.height.isZero){
					executeInLoop(current) {
						callback(nil)
					}
					return
				}
				
				let data = NSCSVGData()
				data.resize(CGFloat(dim.width), CGFloat(dim.height))
				
				
				guard let buf = data.rawData?.assumingMemoryBound(to: UInt8.self) else {
					executeInLoop(current) {
						callback(nil)
					}
					return
				}
				
				CanvasSVGHelper.draw(fromString: buf, size: data.buf_size, width: Float(data.data_size.width), height: Float(data.data_size.height), svg: text as String)
				
				
				executeInLoop(current) {
					callback(data)
				}
				
			}catch{
				executeInLoop(current) {
					callback(nil)
				}
			}
			
		}
	}
	
	
	public static func fromRemote(_ path: String, _ callback:@escaping ((NSCSVGData?)-> Void)) {
		let current =  CFRunLoopGetCurrent()
		DispatchQueue.global(qos: .utility).async {
			
			guard let url = URL(string: path) else {
				executeInLoop(current) {
					callback(nil)
				}
				return
			}
			do {
				let text = try String(contentsOf: url, encoding: .utf8)
				if (text.isEmpty) {
					executeInLoop(current) {
						callback(nil)
					}
					return
				}
				
				var dim = parseSVGDimensions(text)
				
				let data = NSCSVGData()
				data.resize(CGFloat(dim.width), CGFloat(dim.height))
				
				
				guard let buf = data.rawData?.assumingMemoryBound(to: UInt8.self) else {
					executeInLoop(current) {
						callback(nil)
					}
					return
				}
				CanvasSVGHelper.draw(fromString: buf, size: data.buf_size, width: Float(data.data_size.width), height: Float(data.data_size.height), svg: text as String)
				
				
				executeInLoop(current) {
					callback(data)
				}
				
				
			}catch{
				executeInLoop(current) {
					callback(nil)
				}
			}
		}
	}
	
	struct Dimensions {
		var width: Double
		var height: Double
	}
	
	
	static func parseSVGDimensions(_ svgString: String) -> Dimensions {
		let svgRegex = try! NSRegularExpression(pattern: "<svg\\s*([^>]*)>", options: [.caseInsensitive])
		let matches = svgRegex.matches(in: svgString, options: [], range: NSRange(location: 0, length: svgString.utf16.count))
		
		var dimensions = Dimensions(width: 0, height: 0)
		
		guard let match = matches.first else {
			return dimensions
		}
		
		let svgAttributes = (svgString as NSString).substring(with: match.range(at: 1))
		
		let attributesPattern = try! NSRegularExpression(pattern: "(?:width|height|viewBox)\\s*=\\s*\"([^\"]+)\"", options: [])
		let matchesAttributes = attributesPattern.matches(in: svgAttributes, options: [], range: NSRange(location: 0, length: svgAttributes.utf16.count))
		
		var width: Double = 0.0
		var height: Double = 0.0
		var widthDefined = false
		var heightDefined = false
		var viewBox = [Double](repeating: 0.0, count: 4)
		
		for match in matchesAttributes {
			let attributePair = (svgAttributes as NSString).substring(with: match.range)
			let split = attributePair.components(separatedBy: " ").map { $0.trimmingCharacters(in: .whitespaces) }
			
			for part in split {
				let parts = part.components(separatedBy: "=").map { $0.trimmingCharacters(in: .whitespaces) }
				
				if parts.count == 2 {
					let attributeName = parts[0]
					var attributeValue = parts[1].replacingOccurrences(of: "\"", with: "")
					
					let stringLiteralPattern = try! NSRegularExpression(pattern: "\\\\\"(\\d*\\.?\\d+)\\\\\"")
					let stringLiteralMatches = stringLiteralPattern.matches(in: attributeValue, options: [], range: NSRange(location: 0, length: attributeValue.utf16.count))
					
					if let stringLiteralMatch = stringLiteralMatches.first {
						attributeValue = (attributeValue as NSString).substring(with: stringLiteralMatch.range(at: 1))
					}
					
					if attributeName == "width" {
						width = (attributeValue as NSString).doubleValue
						widthDefined = true
					} else if attributeName == "height" {
						height = (attributeValue as NSString).doubleValue
						heightDefined = true
					} else if attributeName == "viewBox" {
						let viewBoxValues = attributeValue.components(separatedBy: .whitespaces)
						for i in 0..<min(4, viewBoxValues.count) {
							var value = viewBoxValues[i].replacingOccurrences(of: "\"", with: "")
							let stringLiteralMatches = stringLiteralPattern.matches(in: value, options: [], range: NSRange(location: 0, length: value.utf16.count))
							if let stringLiteralMatch = stringLiteralMatches.first {
								value = (value as NSString).substring(with: stringLiteralMatch.range(at: 1))
							}
							viewBox[i] = (value as NSString).doubleValue
						}
					}
				}
			}
			
			if (width == 0.0 || width.isNaN) && viewBox.count == 4 {
				let aspectRatio = viewBox[2] / viewBox[3]
				width = 150 * aspectRatio
			}
			
			if (height == 0.0 || height.isNaN)  && viewBox.count == 4 {
				let aspectRatio = viewBox[2] / viewBox[3]
				height = 300 / aspectRatio
			}
			
			if (width == 0.0 || width.isNaN) && !widthDefined {
				width = 300
			}
			
			if (height == 0.0 || height.isNaN) && !heightDefined {
				height = 150
			}
		}
		
		dimensions.width = width
		dimensions.height = height
		
		return dimensions
	}
}

