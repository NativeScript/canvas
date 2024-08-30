//
//  NSCVideoHelperListener.swift
//  CanvasMedia
//
//  Created by Osei Fortune on 28/08/2024.
//

import Foundation
@objc(NSCVideoHelperListener)
public protocol NSCVideoHelperListener {
    func onStateChange(_ state: NSCPlayerState)
    func onTimeUpdate(_ time: Double)
    func onVideoFrameCallback()
}
