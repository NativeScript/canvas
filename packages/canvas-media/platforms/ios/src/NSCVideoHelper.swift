//
//  NSCVideoHelper.swift
//  CanvasMedia
//  Created by Osei Fortune on 27/08/24.
//

import Foundation
import UIKit
import AVKit
import AVFoundation

// #if canImport(Combine)
// import Combine
// #endif

@objc(NSCPlayerState)
public enum NSCPlayerState: Int, RawRepresentable {
    public typealias RawValue = UInt32
    case Idle
    case Playing
    case Paused
    case Stopped
    
    public var rawValue: RawValue {
        switch self {
        case .Idle:
            return 0
        case .Playing:
            return 1
        case .Paused:
            return 2
        case .Stopped:
            return 3
        }
    }
    
    public init?(rawValue: RawValue) {
        switch rawValue {
        case 0:
            self = .Idle
        case 1:
            self = .Playing
        case 2:
            self = .Paused
        case 3:
            self = .Stopped
        default:
            return nil
        }
    }
    
}

@objc(NSCPlayerReadyState)
public enum NSCPlayerReadyState:Int, RawRepresentable {
    public typealias RawValue = UInt32
    case HAVE_NOTHING
    case HAVE_METADATA
    case HAVE_CURRENT_DATA
    case HAVE_FUTURE_DATA
    case HAVE_ENOUGH_DATA
    
    
    public var rawValue: RawValue {
        switch self {
        case .HAVE_NOTHING:
            return 0
        case .HAVE_METADATA:
            return 1
        case .HAVE_CURRENT_DATA:
            return 2
        case .HAVE_FUTURE_DATA:
            return 3
        case .HAVE_ENOUGH_DATA:
            return 4
        }
    }
    
    public init?(rawValue: RawValue) {
        switch rawValue {
        case 0:
            self = .HAVE_NOTHING
        case 1:
            self = .HAVE_METADATA
        case 2:
            self = .HAVE_CURRENT_DATA
        case 3:
            self = .HAVE_FUTURE_DATA
        case 4:
            self = .HAVE_ENOUGH_DATA
        default:
            return nil
        }
    }
}



@objcMembers
@objc(NSCVideoHelper)
public class NSCVideoHelper: NSObject {
    public let controller = AVPlayerViewController()
    public let player = AVPlayer()
    private (set) public var currentItem: AVPlayerItem? = nil
    private var currentSrc: String? = nil
    private(set) public var state: NSCPlayerState = .Idle
    private var asset: AVURLAsset? = nil
    private(set) var videoSize: CGSize = .zero
    private(set) var assetOutput: AVPlayerItemVideoOutput? = nil
    private(set) var readyState: NSCPlayerReadyState = .HAVE_NOTHING
    var listener: NSCVideoHelperListener? = nil
    private(set) var isInForground = true
    public override init(){
        super.init()
        suspendListenerId = NotificationCenter.default.addObserver(forName:UIApplication.didEnterBackgroundNotification , object: nil, queue: nil) { noti in
            self.isInForground = false
        }
        
        resumeListenerId = NotificationCenter.default.addObserver(forName:UIApplication.didBecomeActiveNotification , object: nil, queue: nil) { noti in
            self.isInForground = true
        }
        
        controller.player = player
    }
    
    public override func observeValue(forKeyPath keyPath: String?, of object: Any?, change: [NSKeyValueChangeKey : Any]?, context: UnsafeMutableRawPointer?) {
        if (keyPath == "status") {
            if(player.currentItem?.status == .failed){
                // let baseError = player.currentItem. .userInfo.objectForKey(NSUnderlyingErrorKey);
            }else if(player.currentItem?.status == .readyToPlay){
                videoSize = asset?.tracks(withMediaType: .video).first?.naturalSize ?? .zero
                player.currentItem?.add(assetOutput!)
            }
        } else if (keyPath == "loadedTimeRanges") {
            
           // let playerItem = object as? AVPlayerItem
           // guard let playerItem = playerItem else {return}
          //  let ranges = playerItem.loadedTimeRanges
          //  guard let first = ranges.first else {return}
            
            //            let loaded = first.CMTimeRangeValue.duration.value / first.CMTimeRangeValue.duration.timescale
            //            let total = playerItem.duration.value / playerItem.duration.timescale
            //    console.log('loaded', loaded / total);
        }
    }
    
    public var controls: Bool {
        get {
            return controller.showsPlaybackControls
        }
        set {
            controller.showsPlaybackControls = newValue
        }
    }
    
    public var muted: Bool {
        get {
            return player.isMuted
        }
        
        set {
            player.isMuted = newValue
        }
    }
    
    public var duration: Double {
        get {
            guard let currentItem = currentItem else {return .nan}
            
            return CMTimeGetSeconds(currentItem.asset.duration)
        }
    }
    
    public var currentTime: Double  {
        get {
            return CMTimeGetSeconds(player.currentTime())
        }
        
        set {
            let time = CMTime(seconds: newValue, preferredTimescale: player.currentTime().timescale)
            player.seek(to: time)
        }
    }
    
    public var isLoop: Bool = false
    
    public var playsinline: Bool {
        get {
            return !controller.entersFullScreenWhenPlaybackBegins
        }
        set {
            controller.entersFullScreenWhenPlaybackBegins = !newValue
        }
    }
    
    private var playEndNotificationId: Any?
    private var resumeListenerId: Any?
    private var suspendListenerId: Any?
    //    private var cancellables = Set<AnyCancellable>()
    public var src: String? {
        get {
            return currentSrc
        }
        
        set {
            guard let src = newValue else {
                readyState = .HAVE_NOTHING
                return
            }
            var url: URL?
            if(src.starts(with: "/")){
                url = URL(fileURLWithPath: src)
            }else {
                url = URL(string: src)
            }
            guard let url = url else {
                readyState = .HAVE_NOTHING
                return
            }
            
            if(playEndNotificationId != nil){
                NotificationCenter.default.removeObserver(playEndNotificationId!)
                playEndNotificationId = nil
            }
            asset = AVURLAsset(url: url)
            
            //            var cancellables = Set<AnyCancellable>()
            
            let keys = ["duration", "tracks", "commonMetadata"]
            
            
            
            //            let publishers = keys.map { key in
            //                asset.publisher(for: \.statusOfValue(forKey: key, error: nil))
            //            }
            //
            //            Publishers.Zip3(publishers[0], publishers[1], publishers[2])
            //                .sink { completion in
            //                    // Handle completion
            //                } receiveValue: { status1, status2, status3 in
            //                    // Handle loaded values for all three keys
            //                }
            //                .store(in: &cancellables)
            //
            //
            
            asset?.loadValuesAsynchronously(forKeys: keys, completionHandler: { [self] in
                videoSize = asset?.tracks(withMediaType: .video).first?.naturalSize ?? .zero
                
                let fps = asset?.tracks.first?.nominalFrameRate ?? 30
                let interval = CMTimeMake(value: 1, timescale: Int32(fps))
                
                playbackFramesObserver = player.addPeriodicTimeObserver(forInterval: interval, queue: nil, using: { currentTime in
                    if(self.state == .Playing){
                        self.listener?.onVideoFrameCallback()
                    }
                })
                
                currentItem = AVPlayerItem(asset: asset!)
                
                guard let currentItem = currentItem else {
                    readyState = .HAVE_NOTHING
                    currentSrc = nil
                    return
                }
                
                let attributes: [String: Any] = [
                    kCVPixelBufferPixelFormatTypeKey as String: NSNumber(value: kCVPixelFormatType_32BGRA),
                    kCVPixelBufferOpenGLESCompatibilityKey as String : NSNumber(value: true),
                    kCVPixelBufferOpenGLCompatibilityKey as String : NSNumber(value: true)
                ]
                
                assetOutput = AVPlayerItemVideoOutput(outputSettings: attributes)
        
                currentItem.addObserver(self, forKeyPath: #keyPath(AVPlayerItem.status),options: .new,context: nil)
                
                currentItem.addObserver(self, forKeyPath: #keyPath(AVPlayerItem.loadedTimeRanges),options: [.initial, .new] ,context: nil)
                
                playEndNotificationId = NotificationCenter.default.addObserver(forName: .AVPlayerItemDidPlayToEndTime, object: currentItem, queue: nil, using: { notfi in
                    if (self.isLoop) {
                        self.player.seek(to: .zero)
                        self.player.play()
                        self.state = .Playing
                        self.listener?.onStateChange(.Playing)
                    }
                })
            
                
                player.replaceCurrentItem(with: currentItem)
                
                readyState = .HAVE_METADATA
                currentSrc = src
                if (isLoop) {
                    play()
                }
            })
            
        }
    }
    
    private var playbackFramesObserver: Any? = nil
    private var playbackTimeObserver: Any? = nil
    private func addTimeObserver() {
        let interval = CMTimeMake(value: 1, timescale: 1)
        player.addPeriodicTimeObserver(forInterval: interval, queue: nil) { currentTime in
            if(self.state == .Playing){
                let seconds = CMTimeGetSeconds(currentTime)
                self.listener?.onTimeUpdate(seconds)
            }
        }
    }
    
    
    public func play(){
        if(state == .Playing){
            return
        }
        addTimeObserver()
        player.play()
        state = .Playing
        listener?.onStateChange(.Playing)
    }
    
    
    public func pause() {
        if (state != .Playing) {
            return
        }
        guard let playbackTimeObserver =  playbackTimeObserver else {return}
        player.removeTimeObserver(playbackTimeObserver)
        self.playbackTimeObserver = nil
        player.pause()
        state = .Paused
        listener?.onStateChange(.Paused)
    }
    
    deinit {
        if(state == .Playing){
            player.pause()
            state = .Stopped
            listener?.onStateChange(.Stopped)
        }
        
        if(playEndNotificationId != nil){
            NotificationCenter.default.removeObserver(playEndNotificationId!)
            playEndNotificationId = nil
        }
        
        if(resumeListenerId != nil){
            NotificationCenter.default.removeObserver(resumeListenerId!)
            resumeListenerId = nil
        }
        
        if(suspendListenerId != nil){
            NotificationCenter.default.removeObserver(suspendListenerId!)
            suspendListenerId = nil
        }
    }
}
