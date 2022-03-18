//
//  TNSTextEncoder.swift
//  CanvasNative
//
//  Created by Osei Fortune on 5/14/20.
//

import Foundation
@objcMembers
@objc(TNSTextEncoder)
public class TNSTextEncoder: NSObject {
    private var encoder: Int64 = 0
    public override init() {
        super.init()
        create(encoding: "utf-8")
    }
    
    public init(encoding: String){
        super.init()
        create(encoding: encoding)
    }
    
    private func create(encoding: String){
        let type = (encoding as NSString).utf8String
        encoder = text_encoder_create(type)
    }
    
    public var encoding: String {
        let raw = text_encoder_get_encoding(encoder)
        if(raw == nil){
            // Return default utf8 ?
            return String()
        }
        let encoding = String(cString: raw!)
        destroy_string(raw)
        return encoding
    }
    
    public func encode(text: String) -> NSData {
        let txt = (text as NSString).utf8String
        let result = text_encoder_encode(encoder, txt)
        if(result == nil){
            return NSData()
        }
        return TNSCanvasData(data: result!)
    }
    
    public func encode(pointer text: UnsafePointer<Int8>?) -> NSData {
        let result = text_encoder_encode(encoder, text)
        if(result == nil){
            return NSData()
        }
        return TNSCanvasData(data: result!)
    }
    
    deinit {
        destroy_text_encoder(encoder)
        encoder = 0
    }
}
