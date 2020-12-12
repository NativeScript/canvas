//
//  TNSImageSmoothingQuality.swift
//  CanvasNative
//
//  Created by Osei Fortune on 8/12/19.
//

import Foundation

@objc(TNSImageSmoothingQuality)
public enum TNSImageSmoothingQuality: UInt32, RawRepresentable {
    case Low
    case Medium
    case High
    public typealias RawValue = UInt32
    
    public var rawValue: RawValue {
        switch self {
        case .Low:
            return 0
        case .Medium:
            return 1
        case .High:
            return 2
        }
    }
    
    
    public init?(rawValue: RawValue) {
        switch rawValue {
        case 1:
            self = .Medium
        case 2:
            self = .High
        default:
            self = .Low
        }
    }
    
    
    public init?(string: String) {
        switch string {
        case "medium":
            self = .Medium
        case "high":
            self = .High
        default:
            self = .Low
        }
    }
    
}
