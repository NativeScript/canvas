//
//  HowToClear.swift
//  CanvasNative
//
//  Created by Osei Fortune on 02/09/2020.
//

import Foundation


enum HowToClear {
    // Skip clearing the backbuffer.
    case Skipped
    // Clear the backbuffer.
    case JustClear
    // Combine webgl.clear() API with the backbuffer clear, so webgl.clear()
    // doesn't have to call glClear() again.
    case CombinedClear
}

