//
//  TNSImageAssetFormat.swift
//  CanvasNative
//
//  Created by Osei Fortune on 5/4/20.
//

import Foundation
@objc(TNSImageAssetFormat)
public enum TNSImageAssetFormat:Int, RawRepresentable {
    public init(rawValue: Int) {
        switch rawValue {
        case 1:
            self = .PNG
        case 2:
            self = .ICO
        case 3:
            self = .BMP
        case 4:
            self = .TIFF
        default:
            self = .JPG
        }
    }
    
    public var rawValue: Int {
        switch self {
        case .PNG:
            return 1
        case .ICO:
            return 2
        case .BMP:
            return 3
        case .TIFF:
            return 4
        default:
            return 0
        }
    }
    
    public typealias RawValue = Int
    case JPG
    case PNG
    case ICO
    case BMP
    case TIFF
}
