//
//  TNSTextMetrics.swift
//
//  Created by Osei Fortune on 7/15/19.
//  Copyright © 2019 Osei Fortune. All rights reserved.
//

import Foundation
@objcMembers
@objc(TNSTextMetrics)
public class TNSTextMetrics: NSObject {
    private var _width: Float = 0
    
    init(width: Float) {
        _width = width
    }
    
    public var width: Float {
        get {
            return _width
        }
    }
}
