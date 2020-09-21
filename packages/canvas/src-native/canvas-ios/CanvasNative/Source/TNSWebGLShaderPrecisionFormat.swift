//
//  TNSWebGLShaderPrecisionFormat.swift
//  CanvasNative
//
//  Created by Osei Fortune on 4/17/20.
//

import Foundation
@objcMembers
@objc(TNSWebGLShaderPrecisionFormat)
public class TNSWebGLShaderPrecisionFormat: NSObject {
    var _rangeMin: Int32
    var _rangeMax: Int32
    var _precision: Int32
    public var rangeMin: Int32 {
        get {
            return _rangeMin
        }
    }
    public var rangeMax: Int32 {
        get {
            return _rangeMax
        }
    }
    public var precision: Int32 {
        get {
            return _precision
        }
    }
    
    public init(rangeMin: Int32, rangeMax: Int32, precision: Int32) {
        _rangeMin = rangeMin
        _rangeMax = rangeMax
        _precision = precision
    }
}
