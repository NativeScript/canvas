//
//  TNSImageBitmapColorSpaceConversion.swift
//  CanvasNative
//
//  Created by Osei Fortune on 11/04/2021.
//

import Foundation

@objc(TNSImageBitmapColorSpaceConversion)
public enum TNSImageBitmapColorSpaceConversion: Int, RawRepresentable {
    case Default
    case None
    public typealias RawValue = Int32
    
    public var rawValue: RawValue {
        switch self {
        case .Default:
            return 0
        case .None:
            return  1
        }
    }
    
    
    public init?(rawValue: RawValue) {
        switch rawValue {
        case 0:
            self = .Default
        case 1:
            self = .None
        default:
            return nil
        }
    }
    
    
    public init?(string: String) {
        switch string {
        case "default":
            self = .Default
        case "none":
            self = .None
        default:
            return nil
        }
    }
    
}
