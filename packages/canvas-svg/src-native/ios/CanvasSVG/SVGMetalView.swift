//
//  SVGMetalView.swift
//  CanvasSVG
//
//  Host for the GPU path: a view whose backing layer is a CAMetalLayer, which is what
//  Skia's Metal backend draws into. Kept separate from NSCSVG so the CPU path stays a plain
//  UIView, since the surface type is not knowable when the view is constructed.
//

import Foundation
import UIKit
import QuartzCore

/// Metal in the *simulator* only exists from iOS 13 / tvOS 13, so `CAMetalLayer` carries that
/// availability even though it goes back to iOS 8 on device. The deployment target is 12.0, so
/// the whole host is gated and `NSCSVG` keeps the raster path for anything older.
@available(iOS 13.0, tvOS 13.0, *)
@objcMembers
@objc(SVGMetalView)
public class SVGMetalView: UIView {
	public override class var layerClass: AnyClass {
		return CAMetalLayer.self
	}

	public var metalLayer: CAMetalLayer {
		return layer as! CAMetalLayer
	}

	/// Notifies NSCSVG that the drawable size changed, so it can resize the Skia surface.
	var onSizeChanged: ((Int32, Int32) -> Void)?

	public override init(frame: CGRect) {
		super.init(frame: frame)
		configure()
	}

	required init?(coder: NSCoder) {
		super.init(coder: coder)
		configure()
	}

	private func configure() {
		isOpaque = false
		backgroundColor = .clear
		let layer = metalLayer
		layer.pixelFormat = .bgra8Unorm
		layer.isOpaque = false
		// Skia snapshots and reads back from the drawable; framebufferOnly forbids that.
		layer.framebufferOnly = false
		layer.presentsWithTransaction = false
	}

	/// The view's own scale, not the screen's. `UIScreen` does not exist on visionOS and
	/// `UITraitCollection.current` is iOS 13+, but the real reason is agreement: the JS side
	/// renders at `Screen.mainScreen.scale`, which is `displayScale`. Taking `nativeScale`
	/// here would size the drawable differently from the frame drawn into it on every device
	/// where the two differ.
	private var displayScale: CGFloat {
		let scale = traitCollection.displayScale
		// Zero until the view has a trait environment.
		return scale > 0 ? scale : 1
	}

	public override func layoutSubviews() {
		super.layoutSubviews()
		let scale = displayScale
		let size = CGSize(width: bounds.width * scale, height: bounds.height * scale)
		if size.width < 1 || size.height < 1 {
			return
		}
		if metalLayer.drawableSize != size {
			metalLayer.drawableSize = size
			onSizeChanged?(Int32(size.width), Int32(size.height))
		}
	}
}
