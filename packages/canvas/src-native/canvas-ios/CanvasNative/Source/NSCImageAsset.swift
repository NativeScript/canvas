//
//  NSCImageAsset.swift
//  CanvasNative
//
//  Created by Osei Fortune on 5/4/20.
//

import Foundation
import UIKit
import AVFoundation
@objcMembers
@objc(NSCImageAsset)
public class NSCImageAsset: NSObject {
	private static let queue = DispatchQueue(label: "NSCImageAssetQueue", qos: .background, attributes:.concurrent, autoreleaseFrequency: .never, target: nil)
	
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
	
	
	public static func getImageFromPlayer(_ context: Int64, _ player: AVPlayer, _ output: AVPlayerItemVideoOutput, _ flipX: Bool, _ flipY: Bool, _ callback:@escaping (Bool)	-> Void){
		let runloop = CFRunLoopGetCurrent()
		NSCImageAsset.queue.async {
			let currentTime = player.currentTime()
			if(!output.hasNewPixelBuffer(forItemTime: currentTime)) {
				executeInLoop(runloop) {
					callback(false)
				}
				return
			}
			let buffer = output.copyPixelBuffer(forItemTime: currentTime, itemTimeForDisplay: nil)
			guard let buffer = buffer else {
				executeInLoop(runloop) {
					callback(false)
				}
				return
			}
			var image = CIImage(cvPixelBuffer: buffer)
			if(flipX){
				let transform = CGAffineTransform(scaleX: -1, y: 1)
					.translatedBy(x: -image.extent.width, y: 0)
				
				image = image.transformed(by: transform)
			}
			
			if(flipY){
				let transform = CGAffineTransform(scaleX: 1, y: -1)
					.translatedBy(x: 0, y: -image.extent.height)
				
				image = image.transformed(by: transform)
			}
			let ctx = CIContext()
			let cgImage = ctx.createCGImage(image, from: image.extent)
			guard let cgImage = cgImage else {
				executeInLoop(runloop) {
					callback(false)
				}
				return
			}
			var uiimage = UIImage(cgImage: cgImage)
			
			
			let success = CanvasHelpers.loadImageAssetWithContext(context, uiimage)
			
			executeInLoop(runloop) {
				callback(success)
			}
		}
	}
	
	public static func loadImageFromImageSync(_ context: Int64, _ image: UIImage) -> Bool {
		return CanvasHelpers.loadImageAssetWithContext(context, image)
	}
	
	public static func loadImageFromImage(_ context: Int64,_ image: UIImage, _ callback: @escaping (Bool)-> ()){
		let runloop = CFRunLoopGetCurrent()
		NSCImageAsset.queue.async {
			let success = NSCImageAsset.loadImageFromImageSync(context,image)
			executeInLoop(runloop) {
				callback(success)
			}
		}
	}
	
	public static func toUIImage(_ asset: Int64) -> UIImage? {
		let ptr = OpaquePointer(bitPattern: UInt(asset))
		let width = Int(canvas_native_image_asset_width(ptr))
		let height = Int(canvas_native_image_asset_height(ptr))
		if width <= 0 || height <= 0 {
			return nil
		}
		let rowBytes = width * 4
		var data = Data(count: rowBytes * height)
		let written = data.withUnsafeMutableBytes { (buffer: UnsafeMutableRawBufferPointer) -> Bool in
			canvas_native_image_asset_read_pixels_8888(ptr, false, true, buffer.baseAddress?.assumingMemoryBound(to: UInt8.self), UInt(buffer.count), UInt(rowBytes))
		}
		guard written, let provider = CGDataProvider(data: data as CFData),
			let space = CGColorSpace(name: CGColorSpace.sRGB),
			let image = CGImage(
				width: width, height: height, bitsPerComponent: 8, bitsPerPixel: 32, bytesPerRow: rowBytes,
				space: space, bitmapInfo: CGBitmapInfo(rawValue: CGImageAlphaInfo.premultipliedLast.rawValue),
				provider: provider, decode: nil, shouldInterpolate: true, intent: .defaultIntent)
		else {
			return nil
		}
		return UIImage(cgImage: image)
	}

	public static func toUIImageAsync(_ asset: Int64, _ callback: @escaping (UIImage?) -> ()) {
		let runloop = CFRunLoopGetCurrent()
		NSCImageAsset.queue.async {
			let image = NSCImageAsset.toUIImage(asset)
			executeInLoop(runloop) {
				callback(image)
			}
		}
	}

	public static func loadImageFromPathSync(_ asset: Int64, _ path: String) -> Bool {
		return CanvasHelpers.loadImageAssetWithPath(asset, path)
	}
	
	public static func loadImageFromPath(_ asset: Int64,_ path: String, _ callback: @escaping (Bool)-> ()){
		let runloop = CFRunLoopGetCurrent()
		NSCImageAsset.queue.async {
			let success = NSCImageAsset.loadImageFromPathSync(asset,path)
			executeInLoop(runloop) {
				callback(success)
			}
		}
	}
}

