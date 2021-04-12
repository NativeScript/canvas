//
//  TNSImageBitmapResizeQuality.swift
//  CanvasNative
//
//  Created by Osei Fortune on 11/04/2021.
//

import Foundation

@objc(TNSImageBitmapResizeQuality)
public enum TNSImageBitmapResizeQuality: Int, RawRepresentable {
    case Low
    case Medium
    case High
    case Pixelated
    public typealias RawValue = Int32
    
    public var rawValue: RawValue {
        switch self {
        case .Low:
            return 0
        case .Medium:
            return 1
        case .High:
            return 2
        case .Pixelated:
            return 3
        }
    }
    
    
    public init?(rawValue: RawValue) {
        switch rawValue {
        case 0:
            self = .Low
        case 1:
            self = .Medium
        case 2:
            self = .High
        case 3:
            self = .Pixelated
        default:
            return nil
        }
    }
    
    
    public init?(string: String) {
        switch string {
        case "low":
            self = .Low
        case "medium":
            self = .Medium
        case "high":
            self = .High
        case "pixelated":
            self = .Pixelated
        default:
            return nil
        }
    }
    
}
