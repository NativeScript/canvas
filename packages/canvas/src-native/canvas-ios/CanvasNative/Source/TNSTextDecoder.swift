//
//  TNSTextDecoder.swift
//  CanvasNative
//
//  Created by Osei Fortune on 5/14/20.
//

import Foundation
@objcMembers
@objc(TNSTextDecoder)
public class TNSTextDecoder: NSObject {

    private var nativeDecoder: Int64 = 0
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
        nativeDecoder = native_create_text_decoder(type)
    }

    public var encoding: String {
        let raw = native_text_encoder_get_encoding(nativeDecoder)
        if(raw == nil){
            // Return default utf8 ?
            return String()
        }
        let encoding = String(cString: raw!)
        native_free_char(raw)
        return encoding
    }

    public func decode(buffer: Data) -> String{
        var data = [UInt8](buffer)
        let raw = native_text_decoder_decode(nativeDecoder, &data, buffer.count)
        if(raw == nil){
            return String()
        }
        let result = String(cString: raw!)
        native_free_char(raw)
        return result
    }

    public func decode(bytes: [UInt8]) -> String{
        var data = bytes
        let raw = native_text_decoder_decode(nativeDecoder, &data, bytes.count)
        if(raw == nil){
            return String()
        }
        let result = String(cString: raw!)
        native_free_char(raw)
        return result
    }


    public func decode(i8 bytes: [Int8]) -> String{
        let data = bytes.withUnsafeBytes { (buf) -> UnsafePointer<UInt8>? in
            return buf.baseAddress?.assumingMemoryBound(to: UInt8.self)
        }
        let raw = native_text_decoder_decode(nativeDecoder, data, bytes.count)
        if(raw == nil){
            return String()
        }
        let result = String(cString: raw!)
        native_free_char(raw)
        return result
    }



    public func decode(u16 bytes: [UInt16]) -> String {
        var data = bytes
        let raw = native_text_decoder_decode_u16(nativeDecoder, &data, bytes.count)
        if(raw == nil){
            return String()
        }
        let result = String(cString: raw!)
        native_free_char(raw)
        return result
    }


    public func decode(i16 bytes: [Int16]) -> String{
        var data = bytes
        let raw = native_text_decoder_decode_i16(nativeDecoder, &data, bytes.count)
        if(raw == nil){
            return String()
        }
        let result = String(cString: raw!)
        native_free_char(raw)
        return result
    }



    public func decode(i32 bytes: [Int32]) -> String{
        let data = bytes
        let raw = native_text_decoder_decode_i32(nativeDecoder, data, bytes.count)
        if(raw == nil){
            return String()
        }
        let result = String(cString: raw!)
        native_free_char(raw)
        return result
    }

    deinit {
        native_text_decoder_free(nativeDecoder)
    }

}
