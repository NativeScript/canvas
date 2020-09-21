//
//  TNSAnimationFrame.swift
//  CanvasNative
//
//  Created by Osei Fortune on 8/12/19.
//

import Foundation
import UIKit
@objcMembers
@objc(TNSAnimationFrame)
public class TNSAnimationFrame: NSObject {
    static var displayLink: CADisplayLink?
    static var callbacks: [String: (Float) -> Void] = [:]
    static var exitObserver: Any?
    static var enterObserver: Any?
    static var lastTime = 0.0
    @objc static func handleAnimation(displayLink: CADisplayLink){
        var ts: Float = 0
        let currentTime = displayLink.timestamp * 1000
        if(lastTime != 0.0){
            ts = Float(currentTime)
        }
        for (_,callback) in callbacks.enumerated() {
            callback.value(ts)
            callbacks.removeValue(forKey: callback.key)
        }
        lastTime = currentTime
        if(callbacks.count == 0){
            self.displayLink?.invalidate()
            self.displayLink = nil
        }
    }
    
    public static func requestAnimationFrame(toLoop: @escaping (Float) -> Void){
        callbacks[UUID().uuidString] = toLoop
        if(displayLink == nil){
            displayLink = CADisplayLink(target: self, selector: #selector(handleAnimation))
            //displayLink?.preferredFramesPerSecond = 60
            displayLink?.add(to: .main, forMode: .common)
            
            exitObserver = NotificationCenter.default.addObserver(forName: UIApplication.didEnterBackgroundNotification, object: nil, queue: nil) { _ in
                self.displayLink?.invalidate()
                self.displayLink = nil
                self.lastTime = 0.0
            }
            
            enterObserver = NotificationCenter.default.addObserver(forName: UIApplication.didBecomeActiveNotification, object: nil, queue: nil) { _ in
                displayLink = CADisplayLink(target: self, selector: #selector(handleAnimation))
                //displayLink?.preferredFramesPerSecond = 60
                displayLink?.add(to: .current, forMode: .common)
            }
            
        }
    }
    
    public static func cancelAnimationFrame(id: String){
        callbacks.removeValue(forKey: id)
        if(callbacks.count == 0){
            displayLink?.invalidate()
            displayLink = nil
            lastTime = 0.0
            if(exitObserver != nil){
                NotificationCenter.default.removeObserver(exitObserver!)
            }
            if(enterObserver != nil){
                NotificationCenter.default.removeObserver(enterObserver!)
            }
        }
    }
}
