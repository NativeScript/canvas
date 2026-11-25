import Foundation
import UIKit
import Metal
import IOSurface
import QuartzCore
#if canImport(WebKit)
import WebKit
#endif

private let metalDevice: MTLDevice? = MTLCreateSystemDefaultDevice()
private let deviceRGB = CGColorSpaceCreateDeviceRGB()
private let bgraInfo = CGImageAlphaInfo.premultipliedFirst.rawValue | CGBitmapInfo.byteOrder32Little.rawValue

/// One captured frame of a drawable view: premultiplied BGRA in an IOSurface Metal reads directly.
@objcMembers
@objc(NSCDrawableSnapshot)
public class NSCDrawableSnapshot: NSObject {
	public let width: Int
	public let height: Int
	let surface: IOSurface
	private var _texture: MTLTexture?

	init?(width: Int, height: Int) {
		let properties: [IOSurfacePropertyKey: Any] = [
			.width: width,
			.height: height,
			.bytesPerElement: 4,
			.pixelFormat: kCVPixelFormatType_32BGRA,
		]
		guard width > 0, height > 0, let surface = IOSurface(properties: properties) else { return nil }
		self.width = width
		self.height = height
		self.surface = surface
	}

	var texture: MTLTexture? {
		if let texture = _texture { return texture }
		guard let device = metalDevice else { return nil }
		let descriptor = MTLTextureDescriptor.texture2DDescriptor(pixelFormat: .bgra8Unorm, width: width, height: height, mipmapped: false)
		descriptor.usage = .shaderRead
		descriptor.storageMode = .shared
		// Mac Catalyst builds against the macOS SDK, where IOSurface and IOSurfaceRef are distinct types.
		_texture = device.makeTexture(descriptor: descriptor, iosurface: unsafeBitCast(surface, to: IOSurfaceRef.self), plane: 0)
		return _texture
	}

	/// `id<MTLTexture>` for WebGPU, or 0. Valid while the snapshot is alive.
	public var texturePointer: Int64 {
		guard let texture = texture else { return 0 }
		return Int64(Int(bitPattern: Unmanaged.passUnretained(texture).toOpaque()))
	}

	/// Draws into a 2D context on the GPU. False when it can't: draw `image()` instead.
	public func draw2D(_ context: Int64, _ sx: Float, _ sy: Float, _ sw: Float, _ sh: Float, _ dx: Float, _ dy: Float, _ dw: Float, _ dh: Float) -> Bool {
		guard context != 0, let texture = texture else { return false }
		let owner = Unmanaged.passRetained(self).toOpaque()
		return canvas_native_ios_context_draw_external_metal_texture(
			context, Unmanaged.passUnretained(texture).toOpaque(), owner,
			Float(width), Float(height),
			sx, sy, sw, sh, dx, dy, dw, dh
		)
	}

	/// A CPU copy, for consumers with no Metal path.
	public func image() -> UIImage? {
		surface.lock(options: .readOnly, seed: nil)
		defer { surface.unlock(options: .readOnly, seed: nil) }
		guard let context = CGContext(data: surface.baseAddress, width: width, height: height, bitsPerComponent: 8, bytesPerRow: surface.bytesPerRow, space: deviceRGB, bitmapInfo: bgraInfo),
			  let image = context.makeImage() else { return nil }
		return UIImage(cgImage: image)
	}

	func render(_ view: UIView, scaleX: CGFloat, scaleY: CGFloat, sourceX: CGFloat, sourceY: CGFloat, hide: [UIView], webImages: [ObjectIdentifier: CGImage]) {
		surface.lock(options: [], seed: nil)
		defer { surface.unlock(options: [], seed: nil) }
		guard let context = CGContext(data: surface.baseAddress, width: width, height: height, bitsPerComponent: 8, bytesPerRow: surface.bytesPerRow, space: deviceRGB, bitmapInfo: bgraInfo) else { return }
		context.clear(CGRect(x: 0, y: 0, width: width, height: height))
		context.translateBy(x: 0, y: CGFloat(height))
		context.scaleBy(x: scaleX, y: -scaleY)
		let origin = view.layer.bounds.origin
		context.translateBy(x: -origin.x - sourceX, y: -origin.y - sourceY)
		let webViews = NSCDrawableSnapshot.webViews(in: view, hide)
		// Rendering WebKit's layers costs tens of ms and draws little: its snapshot goes on top instead.
		let hidden = hide + webViews
		if hidden.isEmpty {
			// The presentation layer catches in-flight animations.
			(view.layer.presentation() ?? view.layer).render(in: context)
		} else {
			CATransaction.begin()
			CATransaction.setDisableActions(true)
			for v in hidden { v.layer.isHidden = true }
			view.layer.render(in: context)
			for v in hidden { v.layer.isHidden = false }
			CATransaction.commit()
		}
		for web in webViews {
			guard let image = webImages[ObjectIdentifier(web)] else { continue }
			let rect = web === view ? view.bounds : web.convert(web.bounds, to: view)
			context.saveGState()
			// The context is flipped to UIKit's y-down; CGImages draw y-up.
			context.translateBy(x: rect.minX, y: rect.maxY)
			context.scaleBy(x: 1, y: -1)
			context.draw(image, in: CGRect(origin: .zero, size: rect.size))
			context.restoreGState()
		}
	}

	/// `image` redrawn in this snapshot's pixel format, so drawing it each frame is a plain copy.
	static func nativeFormat(_ image: UIImage) -> CGImage? {
		guard let cg = image.cgImage,
			  let context = CGContext(data: nil, width: cg.width, height: cg.height, bitsPerComponent: 8, bytesPerRow: 0, space: deviceRGB, bitmapInfo: bgraInfo) else { return nil }
		context.draw(cg, in: CGRect(x: 0, y: 0, width: cg.width, height: cg.height))
		return context.makeImage()
	}

	/// WebKit draws in another process: render(in:) often sees an empty layer, so web views are drawn from WebKit's own snapshots.
	static func webViews(in root: UIView, _ hide: [UIView]) -> [UIView] {
		var out: [UIView] = []
		#if canImport(WebKit)
		func visit(_ node: UIView) {
			if node.isHidden || hide.contains(where: { $0 === node }) { return }
			if node is WKWebView {
				out.append(node)
				return
			}
			node.subviews.forEach(visit)
		}
		visit(root)
		#endif
		return out
	}

	func samePixels(as other: NSCDrawableSnapshot) -> Bool {
		guard other.width == width, other.height == height else { return false }
		surface.lock(options: .readOnly, seed: nil)
		other.surface.lock(options: .readOnly, seed: nil)
		defer {
			other.surface.unlock(options: .readOnly, seed: nil)
			surface.unlock(options: .readOnly, seed: nil)
		}
		let rowBytes = width * 4
		let a = surface.baseAddress, b = other.surface.baseAddress
		let strideA = surface.bytesPerRow, strideB = other.surface.bytesPerRow
		if strideA == strideB { return memcmp(a, b, strideA * height) == 0 }
		for y in 0..<height where memcmp(a + y * strideA, b + y * strideB, rowBytes) != 0 {
			return false
		}
		return true
	}
}

/// Holds a `content="drawable"` canvas's children: laid out and hit-testable, never shown.
@objcMembers
@objc(NSCDrawableHost)
public class NSCDrawableHost: UIView {
	private final class Entry {
		weak var view: UIView?
		var snapshot: NSCDrawableSnapshot?
		var scratch: NSCDrawableSnapshot?
		/// Quarter-size renders: a cheap check before a full one.
		var probe: NSCDrawableSnapshot?
		var probeScratch: NSCDrawableSnapshot?
		var signature = 0
		var webGeneration = 0
		/// Seconds (CACurrentMediaTime): frame counts would scale with the refresh rate.
		var hotUntil: CFTimeInterval = 0
		var interval = NSCDrawableHost.minInterval
		var nextCheck: CFTimeInterval = 0
		/// Border box to host, in DOMMatrix order (m11 ... m44).
		var transform: [CGFloat]?
		var clip: CGRect?
		init(_ view: UIView) { self.view = view }
	}

	private static let hotDuration: CFTimeInterval = 0.5
	private static let minInterval: CFTimeInterval = 1.0 / 6
	private static let maxInterval: CFTimeInterval = 2
	private static let probeScale = 0.25

	private var entries: [Entry] = []
	private var byView: [ObjectIdentifier: Entry] = [:]
	/// Bottom to top.
	private var hitOrder: [Entry] = []
	private var hotAll: CFTimeInterval = 0
	private var updates = 0

	private final class WebSnapshot {
		weak var view: UIView?
		var image: CGImage?
		var generation = 0
		var requested = 0
		var arrived = 0
		var inFlight = 0
		init(_ view: UIView) { self.view = view }
	}
	private var webSnapshots: [ObjectIdentifier: WebSnapshot] = [:]
	/// WebKit's snapshots take several frames: two in flight keeps them coming at about twice the rate.
	private static let maxWebSnapshotsInFlight = 2
	private static let webDecodeQueue = DispatchQueue(label: "NSCDrawableHost.web", qos: .userInteractive)

	private var webImages: [ObjectIdentifier: CGImage] {
		webSnapshots.compactMapValues { $0.view == nil ? nil : $0.image }
	}

	/// Asks WebKit for a new snapshot of each web view; it renders off the main thread.
	private func requestWebSnapshots(_ webs: [UIView]) {
		#if canImport(WebKit)
		for case let web as WKWebView in webs {
			let key = ObjectIdentifier(web)
			let state: WebSnapshot
			if let existing = webSnapshots[key], existing.view === web {
				state = existing
			} else {
				state = WebSnapshot(web)
				webSnapshots[key] = state
			}
			if state.inFlight >= NSCDrawableHost.maxWebSnapshotsInFlight { continue }
			state.inFlight += 1
			state.requested += 1
			let sequence = state.requested
			web.takeSnapshot(with: nil) { [weak state] image, _ in
				guard let state = state else { return }
				guard let image = image else {
					state.inFlight -= 1
					return
				}
				NSCDrawableHost.webDecodeQueue.async {
					let converted = NSCDrawableSnapshot.nativeFormat(image)
					DispatchQueue.main.async {
						state.inFlight -= 1
						// A later request may have landed first.
						guard let converted = converted, sequence > state.arrived else { return }
						state.arrived = sequence
						state.image = converted
						state.generation &+= 1
					}
				}
			}
		}
		#endif
	}

	public override init(frame: CGRect) {
		super.init(frame: frame)
		alpha = 0
		clipsToBounds = false
		autoresizingMask = [.flexibleWidth, .flexibleHeight]
	}

	required init?(coder: NSCoder) {
		fatalError("init(coder:) has not been implemented")
	}

	// MARK: drawables

	/// The views to snapshot, in tree order.
	public func setDrawables(_ views: [UIView]) {
		var next: [Entry] = []
		var map: [ObjectIdentifier: Entry] = [:]
		for view in views {
			let key = ObjectIdentifier(view)
			let entry = byView[key] ?? Entry(view)
			next.append(entry)
			map[key] = entry
		}
		hitOrder.removeAll { entry in entry.view.map { map[ObjectIdentifier($0)] == nil } ?? true }
		entries = next
		byView = map
	}

	/// Something in the children may have changed: look every frame for a while.
	public func markHot() {
		hotAll = CACurrentMediaTime() + NSCDrawableHost.hotDuration
	}

	/// Snapshots what changed and syncs hit-testing transforms with layout; returns the indices (in `setDrawables` order) of new snapshots.
	public func update(_ scaleX: Double, _ scaleY: Double) -> [NSNumber]? {
		let now = CACurrentMediaTime()
		updates += 1
		if updates % 600 == 0 { webSnapshots = webSnapshots.filter { $0.value.view != nil } }
		keepOnTop()
		var changed: [NSNumber]?
		for (index, entry) in entries.enumerated() {
			guard let view = entry.view else { continue }
			applyTransform(entry)
			let size = view.bounds.size
			guard size.width > 0, size.height > 0 else { continue }
			let width = max(1, Int(ceil(Double(size.width) * scaleX - 1e-3)))
			let height = max(1, Int(ceil(Double(size.height) * scaleY - 1e-3)))
			let resized = entry.snapshot.map { $0.width != width || $0.height != height } ?? true
			let signature = layerSignature(view.layer)
			let signatureChanged = signature != entry.signature
			entry.signature = signature
			let hide = nested(of: view)
			let webs = NSCDrawableSnapshot.webViews(in: view, hide)
			var webHasher = Hasher()
			for web in webs { webHasher.combine(webSnapshots[ObjectIdentifier(web)]?.generation ?? 0) }
			let webGeneration = webs.isEmpty ? 0 : webHasher.finalize()
			let webChanged = webGeneration != entry.webGeneration
			entry.webGeneration = webGeneration
			let forced = resized || signatureChanged || webChanged
			if !forced {
				let hot = now <= entry.hotUntil || now <= hotAll
				if !hot && now < entry.nextCheck { continue }
				requestWebSnapshots(webs)
				if !probeChanged(entry, view, width, height, scaleX, scaleY, hide) {
					if !hot { entry.interval = min(entry.interval * 2, NSCDrawableHost.maxInterval) }
					entry.nextCheck = now + entry.interval
					continue
				}
			}

			let target: NSCDrawableSnapshot
			if let scratch = entry.scratch, scratch.width == width, scratch.height == height {
				target = scratch
			} else if let fresh = NSCDrawableSnapshot(width: width, height: height) {
				target = fresh
			} else {
				continue
			}
			// Not for a web snapshot that just landed: a changed one makes the entry hot, which asks again.
			if resized || signatureChanged { requestWebSnapshots(webs) }
			target.render(view, scaleX: CGFloat(scaleX), scaleY: CGFloat(scaleY), sourceX: 0, sourceY: 0, hide: hide, webImages: webImages)
			if forced { _ = probeChanged(entry, view, width, height, scaleX, scaleY, hide) }
			if !resized, let current = entry.snapshot, target.samePixels(as: current) {
				entry.scratch = target
				entry.nextCheck = now + entry.interval
				continue
			}
			// The old snapshot may still be read by a pending draw: never render into it again.
			entry.scratch = nil
			entry.snapshot = target
			entry.interval = NSCDrawableHost.minInterval
			entry.nextCheck = now + entry.interval
			entry.hotUntil = now + NSCDrawableHost.hotDuration
			if changed == nil { changed = [] }
			changed?.append(NSNumber(value: index))
		}
		return changed
	}

	/// Renders the quarter-size probe; true when it differs from the last one.
	private func probeChanged(_ entry: Entry, _ view: UIView, _ width: Int, _ height: Int, _ scaleX: Double, _ scaleY: Double, _ hide: [UIView]) -> Bool {
		let s = NSCDrawableHost.probeScale
		let w = max(1, Int(Double(width) * s)), h = max(1, Int(Double(height) * s))
		let target: NSCDrawableSnapshot
		if let scratch = entry.probeScratch, scratch.width == w, scratch.height == h {
			target = scratch
		} else if let fresh = NSCDrawableSnapshot(width: w, height: h) {
			target = fresh
		} else {
			return true
		}
		target.render(view, scaleX: CGFloat(scaleX * s), scaleY: CGFloat(scaleY * s), sourceX: 0, sourceY: 0, hide: hide, webImages: webImages)
		let same = entry.probe.map { target.samePixels(as: $0) } ?? false
		entry.probeScratch = entry.probe
		entry.probe = target
		return !same
	}

	public func snapshot(_ view: UIView) -> NSCDrawableSnapshot? {
		return byView[ObjectIdentifier(view)]?.snapshot
	}

	/// A snapshot of `view` that is not shared with the per-frame one.
	public func capture(_ view: UIView, _ width: Int, _ height: Int, _ scaleX: Double, _ scaleY: Double, _ sourceX: Double, _ sourceY: Double) -> NSCDrawableSnapshot? {
		guard let snapshot = NSCDrawableSnapshot(width: width, height: height) else { return nil }
		snapshot.render(view, scaleX: CGFloat(scaleX), scaleY: CGFloat(scaleY), sourceX: CGFloat(sourceX), sourceY: CGFloat(sourceY), hide: nested(of: view), webImages: webImages)
		return snapshot
	}

	private func nested(of view: UIView) -> [UIView] {
		var out: [UIView] = []
		func visit(_ node: UIView) {
			for child in node.subviews {
				if byView[ObjectIdentifier(child)] != nil { out.append(child) } else { visit(child) }
			}
		}
		visit(view)
		return out
	}

	/// Cheap, render-free fingerprint of what a layer tree looks like.
	private func layerSignature(_ root: CALayer) -> Int {
		var hasher = Hasher()
		// The root's own position and transform are where it is drawn, not what it looks like.
		func visit(_ model: CALayer, root: Bool = false) {
			let layer = model.presentation() ?? model
			hasher.combine(layer.bounds.origin.x)
			hasher.combine(layer.bounds.origin.y)
			hasher.combine(layer.bounds.size.width)
			hasher.combine(layer.bounds.size.height)
			if !root {
				hasher.combine(layer.position.x)
				hasher.combine(layer.position.y)
			}
			hasher.combine(layer.opacity)
			hasher.combine(layer.isHidden)
			hasher.combine(layer.cornerRadius)
			hasher.combine(layer.borderWidth)
			if let color = layer.backgroundColor?.components { hasher.combine(color) }
			if let color = layer.borderColor?.components { hasher.combine(color) }
			if let contents = layer.contents { hasher.combine(ObjectIdentifier(contents as AnyObject)) }
			if !root && !CATransform3DIsIdentity(layer.transform) {
				let t = layer.transform
				hasher.combine(t.m11); hasher.combine(t.m12); hasher.combine(t.m21); hasher.combine(t.m22)
				hasher.combine(t.m41); hasher.combine(t.m42); hasher.combine(t.m34)
			}
			// Animating: a new signature every update.
			if model.animationKeys()?.isEmpty == false { hasher.combine(updates) }
			// Its look comes from WebKit's snapshots; the scroll position is the one change seen in-process.
			#if canImport(WebKit)
			if let web = model.delegate as? WKWebView {
				let scroll = web.scrollView
				hasher.combine(scroll.contentOffset.x)
				hasher.combine(scroll.contentOffset.y)
				hasher.combine(scroll.contentSize.width)
				hasher.combine(scroll.contentSize.height)
				hasher.combine(scroll.zoomScale)
				return
			}
			#endif
			let sublayers = model.sublayers ?? []
			hasher.combine(sublayers.count)
			for sublayer in sublayers { visit(sublayer) }
		}
		visit(root, root: true)
		return hasher.finalize()
	}

	// MARK: geometry

	/// `matrix` (DOMMatrix order) maps `view`'s border box to this view; the clip is in its border box.
	public func setGeometry(_ view: UIView, _ matrix: [NSNumber], _ hasClip: Bool, _ clipX: Double, _ clipY: Double, _ clipWidth: Double, _ clipHeight: Double, _ preserveOrder: Bool) {
		guard matrix.count == 16 else { return }
		let key = ObjectIdentifier(view)
		let entry = byView[key] ?? {
			let created = Entry(view)
			byView[key] = created
			return created
		}()
		entry.transform = matrix.map { CGFloat($0.doubleValue) }
		entry.clip = hasClip ? CGRect(x: clipX, y: clipY, width: clipWidth, height: clipHeight).standardized : nil
		let known = hitOrder.firstIndex { $0 === entry }
		if !(preserveOrder && known != nil) {
			if let known = known { hitOrder.remove(at: known) }
			hitOrder.append(entry)
		}
		applyTransform(entry)
	}

	public func clearGeometry(_ view: UIView) {
		guard let entry = byView[ObjectIdentifier(view)] else { return }
		entry.transform = nil
		entry.clip = nil
		hitOrder.removeAll { $0 === entry }
		CATransaction.begin()
		CATransaction.setDisableActions(true)
		view.layer.transform = CATransform3DIdentity
		CATransaction.commit()
	}

	/// The layer maps p to position + M * (p - anchor).
	private func applyTransform(_ entry: Entry) {
		guard let view = entry.view, let m = entry.transform, let superview = view.superview else { return }
		var target = CATransform3D(
			m11: m[0], m12: m[1], m13: m[2], m14: m[3],
			m21: m[4], m22: m[5], m23: m[6], m24: m[7],
			m31: m[8], m32: m[9], m33: m[10], m34: m[11],
			m41: m[12], m42: m[13], m43: m[14], m44: m[15]
		)
		if superview !== self {
			// Assumes nothing between is 3D.
			let o = superview.convert(CGPoint.zero, to: self)
			let x = superview.convert(CGPoint(x: 1, y: 0), to: self)
			let y = superview.convert(CGPoint(x: 0, y: 1), to: self)
			let toHost = CGAffineTransform(a: x.x - o.x, b: x.y - o.y, c: y.x - o.x, d: y.y - o.y, tx: o.x, ty: o.y)
			target = CATransform3DConcat(target, CATransform3DMakeAffineTransform(toHost.inverted()))
		}
		let size = view.bounds.size
		let anchor = view.layer.anchorPoint
		let position = view.layer.position
		var transform = CATransform3DConcat(CATransform3DMakeTranslation(anchor.x * size.width, anchor.y * size.height, 0), target)
		transform = CATransform3DConcat(transform, CATransform3DMakeTranslation(-position.x, -position.y, 0))
		if CATransform3DEqualToTransform(transform, view.layer.transform) { return }
		CATransaction.begin()
		CATransaction.setDisableActions(true)
		view.layer.transform = transform
		CATransaction.commit()
	}

	private func keepOnTop() {
		guard let superview = superview else { return }
		if superview.subviews.last !== self { superview.bringSubviewToFront(self) }
		if frame != superview.bounds { frame = superview.bounds }
	}

	// MARK: hit testing

	// UIKit skips views at alpha 0, so hit testing is done here.
	public override func hitTest(_ point: CGPoint, with event: UIEvent?) -> UIView? {
		markHot()
		for entry in hitOrder.reversed() {
			guard entry.transform != nil, let view = entry.view, !view.isHidden, view.isUserInteractionEnabled else { continue }
			let local = convert(point, to: view)
			if let clip = entry.clip {
				let origin = view.bounds.origin
				if !clip.contains(CGPoint(x: local.x - origin.x, y: local.y - origin.y)) { continue }
			}
			if let hit = view.hitTest(local, with: event) { return hit }
		}
		return nil
	}

	public override func point(inside point: CGPoint, with event: UIEvent?) -> Bool {
		return hitTest(point, with: event) != nil
	}

	public func dispose() {
		for entry in entries + hitOrder {
			entry.snapshot = nil
			entry.scratch = nil
			if entry.transform != nil, let view = entry.view {
				view.layer.transform = CATransform3DIdentity
			}
		}
		entries = []
		hitOrder = []
		byView = [:]
		removeFromSuperview()
	}
}
