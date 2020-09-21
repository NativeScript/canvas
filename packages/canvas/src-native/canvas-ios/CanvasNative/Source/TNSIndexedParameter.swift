//
//  TNSIndexedParameter.swift
//  CanvasNative
//
//  Created by Osei Fortune on 4/30/20.
//

import Foundation
@objcMembers
@objc(TNSIndexedParameter)
public class TNSIndexedParameter: NSObject {
    var _isBuffer: Bool = false
    var _bufferValue: UInt32
    var _value: Int64
    override init() {
        _isBuffer = false
        _bufferValue = 0
        _value = -1
    }
    init(isBuffer: Bool, bufferValue: UInt32, value: Int64) {
        self._isBuffer = isBuffer
        self._bufferValue = bufferValue
        self._value = value
    }
    
    var isBuffer: Bool {
        return _isBuffer
    }
    var bufferValue: UInt32 {
        return _bufferValue
    }
    var value: Int64 {
        return _value
    }
}
