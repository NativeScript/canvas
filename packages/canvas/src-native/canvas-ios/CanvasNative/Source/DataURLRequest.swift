//
//  DataURLRequest.swift
//  CanvasNative
//
//  Created by Osei Fortune on 3/31/20.
//

import Foundation
class DataURLRequest {
    var type: String
    var format: Float
    var callback: (String) -> Void
    init(type: String, format: Float, callback: @escaping (String) -> Void) {
        self.type = type
        self.format = format
        self.callback = callback
    }
}
