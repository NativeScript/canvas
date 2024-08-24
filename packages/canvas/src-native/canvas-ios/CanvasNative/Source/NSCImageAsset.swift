//
//  NSCImageAsset.swift
//  CanvasNative
//
//  Created by Osei Fortune on 5/4/20.
//

import Foundation
import UIKit
@objcMembers
@objc(NSCImageAsset)
public class NSCImageAsset: NSObject {
    private static let queue = DispatchQueue(label: "NSCImageAssetQueue", qos: .background, attributes:.concurrent, autoreleaseFrequency: .never, target: nil)
        
    public static func loadImageFromImageSync(_ context: Int64, _ image: UIImage) -> Bool {
        return CanvasHelpers.loadImageAssetWithContext(context, image)
    }
    
    public static func loadImageFromImage(_ context: Int64,_ image: UIImage, _ callback: @escaping (Bool)-> ()){
        NSCImageAsset.queue.async {
            let success = NSCImageAsset.loadImageFromImageSync(context,image)
            if(success){
                DispatchQueue.main.async {
                    callback(true)
                }
            }else {
                DispatchQueue.main.async {
                     callback(false)
                }
            }
        }
    }
    
    public static func loadImageFromPathSync(_ asset: Int64, _ path: String) -> Bool {
        return CanvasHelpers.loadImageAssetWithPath(asset, path)
    }
    
    public static func loadImageFromPath(_ asset: Int64,_ path: String, _ callback: @escaping (Bool)-> ()){
        NSCImageAsset.queue.async {
            let success = NSCImageAsset.loadImageFromPathSync(asset,path)
            if(success){
                DispatchQueue.main.async {
                    callback(true)
                }
            }else {
                DispatchQueue.main.async {
                     callback(false)
                }
            }
        }
    }
}

