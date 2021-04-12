//
//  TNSImageBitmapOptions.swift
//  CanvasNative
//
//  Created by Osei Fortune on 11/04/2021.
//

import Foundation

@objcMembers
@objc(TNSImageBitmapOptions)
public class TNSImageBitmapOptions:NSObject {
    public var flipY: Bool
    public var premultiplyAlpha: TNSImageBitmapPremultiplyAlpha
    public var colorSpaceConversion: TNSImageBitmapColorSpaceConversion
    public var resizeQuality: TNSImageBitmapResizeQuality
    public var resizeWidth: Float32
    public var resizeHeight: Float32
    public override init() {
        flipY = false
        premultiplyAlpha = TNSImageBitmapPremultiplyAlpha.Default
        colorSpaceConversion = TNSImageBitmapColorSpaceConversion.Default
        resizeQuality = TNSImageBitmapResizeQuality.Low
        resizeWidth = 0
        resizeHeight = 0
    }
}

