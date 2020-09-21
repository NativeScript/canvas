//
//  TNSImageSmoothingQuality.swift
//  CanvasNative
//
//  Created by Osei Fortune on 8/12/19.
//

import Foundation

@objc public enum TNSImageSmoothingQuality: Int, RawRepresentable {
    case Low
    case Medium
    case High
    public typealias RawValue = String
    
    public var rawValue: RawValue {
        switch self {
        case .Low:
            return "low"
        case .Medium:
            return "medium"
        case .High:
            return "high"
        }
    }
    
    
    public init?(rawValue: RawValue) {
        switch rawValue {
        case "medium":
            self = .Medium
        case "high":
            self = .High
        default:
            self = .Low
        }
    }
    
}
