//
//  NSCCanvasVideoFrame.swift
//  CanvasNative
//
//  Created by Osei Fortune on 25/08/2024.
//

import Foundation
import CoreVideo
import AVFoundation

@objc(NSCVideoFrame)
public class NSCVideoFrame: NSObject {
    @objc(NSCVideoFrameFormat)
    public enum NSCVideoFrameFormat: Int, RawRepresentable {
           public typealias RawValue = UInt32
           
           case I420
           case I420A
           case I422
           case I444
           case NV12
           case RGBA
           case RGBX
           case BGRA
           case BGRX
           
           public var rawValue: RawValue {
               switch self {
               case .I420:
                   return 0
               case .I420A:
                   return 1
               case .I422:
                   return 2
               case .I444:
                   return 3
               case .NV12:
                   return 4
               case .RGBA:
                   return 5
               case .RGBX:
                   return 6
               case .BGRA:
                   return 7
               case .BGRX:
                   return 8
               }
           }
           
           public init?(rawValue: RawValue) {
               switch rawValue {
               case 0:
                   self = .I420
               case 1:
                   self = .I420A
               case 2:
                   self = .I422
               case 3:
                   self = .I444
               case 4:
                   self = .NV12
               case 5:
                   self = .RGBA
               case 6:
                   self = .RGBX
               case 7:
                   self = .BGRA
               case 8:
                   self = .BGRX
               default:
                   return nil
               }
           }
       }
    
    private(set) public var pixelBuffer: CVPixelBuffer
    private(set) public var timestamp: CMTimeValue
    private(set) public var format: NSCVideoFrameFormat
    
    private static func initValues(ts: CMTimeValue, pixelBuffer: CVPixelBuffer) throws -> NSCVideoFrameFormat{
 
        let type = CVPixelBufferGetPixelFormatType(pixelBuffer)
    
        //  case BGRX -> kCVPixelFormatType_32BGRA
        // BGRX -> kCVPixelFormatType_32ARGB
        switch(type){
        case kCVPixelFormatType_32ARGB:
            // unsupported on the web
            return .BGRX
        case kCVPixelFormatType_420YpCbCr8Planar:
            return .I420
        case kCVPixelFormatType_420YpCbCr8BiPlanarFullRange ,kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange:
            return .I420A
        case kCVPixelFormatType_32BGRA:
            return .BGRA
        default:
            throw NSCCanvasUtils.NSCError.customError("Invalid format \(type)")
            // unsupported?
            // todo throw invalid format ?
           // break
        }
    }
    
    public static func getCurrentFrame(_ player: AVPlayer, _ output: AVPlayerItemVideoOutput) throws -> NSCVideoFrame? {
        let currentTime = player.currentTime()
        if(!output.hasNewPixelBuffer(forItemTime: currentTime)) {
            return nil
        }
        let buffer = output.copyPixelBuffer(forItemTime: currentTime, itemTimeForDisplay: nil)
        
        guard let buffer = buffer else {
            return nil
        }
        
        return try NSCVideoFrame(currentTime: currentTime, frame: buffer)
    }
    
    public init(currentTime: CMTime, frame: CVPixelBuffer) throws {
        timestamp = CMTimeValue(currentTime.seconds * 1000)
        pixelBuffer = frame
        format = try NSCVideoFrame.initValues(ts: timestamp, pixelBuffer: frame)
        super.init()
    }
    
//    public init(data: Data, options) {
//        let currentTime = player.currentTime()
//        if(!output.hasNewPixelBuffer(forItemTime: currentTime)) {
//            timestamp = 0
//        }else {
//            timestamp = CMTimeValue(currentTime.seconds * 1000)
//            pixelBuffer = output.copyPixelBuffer(forItemTime: currentTime, itemTimeForDisplay: nil)
//        }
//        super.init()
//    }
//    
    
    public init(buffer: CVPixelBuffer, ts: CMTimeValue) throws {
        pixelBuffer = buffer
        timestamp = ts
        format = try NSCVideoFrame.initValues(ts: ts, pixelBuffer: buffer)
        super.init()
    }
    
    public func clone() throws -> NSCVideoFrame {
        return try NSCVideoFrame(buffer: pixelBuffer, ts: timestamp)
    }
    
    public var codedWidth: Int {
        get {
          return CVPixelBufferGetWidthOfPlane(pixelBuffer, 0)
        }
    }
    
    public var codedHeight: Int {
        get {
          return CVPixelBufferGetHeightOfPlane(pixelBuffer, 0)
        }
    }
    
    public var codedRect: CGRect {
        let width = CVPixelBufferGetWidthOfPlane(pixelBuffer, 0)
        let height = CVPixelBufferGetHeightOfPlane(pixelBuffer, 0)
        return CGRect(x: 0, y: 0, width: width, height: height)
    }

    public var visibleRect: CGRect {
        let pixelFormat = CVPixelBufferGetPixelFormatType(pixelBuffer)
    
        
        let width = CVPixelBufferGetWidth(pixelBuffer)
        let height = CVPixelBufferGetHeight(pixelBuffer)
        
        let bytesPerRowPlane0 = CVPixelBufferGetBytesPerRowOfPlane(pixelBuffer, 0)
        
        var visibleWidth: Int
        switch (pixelFormat) {
        case kCVPixelFormatType_420YpCbCr8Planar:
            visibleWidth = bytesPerRowPlane0 / 4
            break;
        case kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange:
            visibleWidth = bytesPerRowPlane0 / 4
            break;
        case kCVPixelFormatType_32BGRA, kCVPixelFormatType_32ARGB:
            visibleWidth = width
            break;
        default:
            // todo handle other pixel formats
            visibleWidth = width;
            break;
        }
        
        return CGRect(x: 0, y: 0,width :visibleWidth, height:height)
    }
}
