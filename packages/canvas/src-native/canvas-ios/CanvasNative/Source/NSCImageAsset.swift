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

