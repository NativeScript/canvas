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
    
    private var decoder: Int64 = 0
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
        decoder = text_decoder_create(type)
    }
    
    public var encoding: String {
        let raw = text_decoder_get_encoding(decoder)
        if(raw == nil){
            return ""
        }
        let encoding = String(cString: raw!)
        destroy_string(raw)
        return encoding
    }
    
    private func handleRaw(_ raw: UnsafeMutablePointer<U8Array>?) -> String {
        if(raw == nil){
            return ""
        }
        let data = Data(bytes: raw!.pointee.data, count: Int(raw!.pointee.data_len))
        let result = String(bytes: data, encoding: .utf8)
        destroy_u8_array(raw)
        return (result ?? "") as String
    }
    
    
    
    
    private func handleRaw(_ raw: UnsafePointer<CChar>?) -> String {
        if(raw == nil){
            return ""
        }
        let result = NSString(utf8String: raw!)
        destroy_string(raw)
        return (result ?? "") as String
    }
    
    
    
    public func decode(buffer: Data) -> String {
        return decode(buffer: buffer, offset: 0)
    }
    
    public func decode(buffer: Data, offset: Int) -> String {
        return decode(buffer: buffer, offset: offset, length: buffer.count)
    }
    
    public func decode(buffer: Data, offset: Int, length: Int) -> String {
        var data = buffer
        let ptr = data.withUnsafeMutableBytes { ptr in
            return ptr.baseAddress?.assumingMemoryBound(to: UInt8.self).advanced(by: offset)
        }
        return handleRaw(text_decoder_decode_bytes(decoder, ptr, UInt(length)))
    }
    
    public func decode(u8 buffer: UnsafeMutableRawPointer, _ size: Int) -> String {
        return decode(u8: buffer, size, offset: 0)
    }
    
    public func decode(u8 buffer: UnsafeMutableRawPointer, _ size: Int, offset: Int) -> String {
        return handleRaw(text_decoder_decode_bytes(decoder, buffer.assumingMemoryBound(to: UInt8.self).advanced(by: offset), UInt(size)))
    }
    
    
    public func decode(i8 buffer: UnsafeMutableRawPointer, _ size: Int) -> String {
        return decode(i8: buffer, size, offset: 0)
    }
    
    
    public func decode(i8 buffer: UnsafeMutableRawPointer, _ size: Int, offset: Int) -> String {
        return handleRaw(text_decoder_decode_bytes(decoder, buffer.assumingMemoryBound(to: UInt8.self).advanced(by: offset), UInt(size)))
    }
    
    
    public func decode(u16 buffer: UnsafeMutableRawPointer, _ size: Int) -> String {
        return decode(u16: buffer, size, offset: 0)
    }
    
    public func decode(u16 buffer: UnsafeMutableRawPointer, _ size: Int, offset: Int) -> String {
        return handleRaw(text_decoder_decode_u16_bytes(decoder, buffer.assumingMemoryBound(to: UInt16.self), UInt(size)))
    }
    
    
    public func decode(i16 buffer: UnsafeMutableRawPointer, _ size: Int) -> String {
        return decode(i16: buffer, size, offset: 0)
    }
    
    
    public func decode(i16 buffer: UnsafeMutableRawPointer, _ size: Int, offset: Int) -> String {
        return handleRaw(text_decoder_decode_i16_bytes(decoder, buffer.assumingMemoryBound(to: Int16.self), UInt(size)))
    }
    
    public func decode(i32 buffer: UnsafeMutableRawPointer, _ size: Int) -> String {
        return decode(i32: buffer, size, offset: 0)
    }
    
    
    public func decode(i32 buffer: UnsafeMutableRawPointer, _ size: Int, offset: Int) -> String {
        return handleRaw(text_decoder_decode_i32_bytes(decoder, buffer.assumingMemoryBound(to: Int32.self), UInt(size)))
    }
    
    
    public func decode(u32 buffer: UnsafeMutableRawPointer, _ size: Int) -> String {
        return decode(u32: buffer, size, offset: 0)
    }
    
    public func decode(u32 buffer: UnsafeMutableRawPointer, _ size: Int, offset: Int) -> String {
        return handleRaw(text_decoder_decode_u32_bytes(decoder, buffer.assumingMemoryBound(to: UInt32.self), UInt(size)))
    }
    
    
    
    public func decode(bytes: [UInt8]) -> String{
        var data = bytes
        return handleRaw(text_decoder_decode_bytes(decoder, &data, UInt(bytes.count)))
    }
    
    
    public func decode(i8 bytes: [Int8]) -> String{
        let data = bytes.withUnsafeBytes { (buf) -> UnsafePointer<UInt8>? in
            return buf.baseAddress?.assumingMemoryBound(to: UInt8.self)
        }
        return handleRaw(text_decoder_decode_bytes(decoder, data, UInt(bytes.count)))
    }
    
    
    
    public func decode(u16 bytes: [UInt16]) -> String {
        var data = bytes
        return handleRaw(text_decoder_decode_u16_bytes(decoder, &data, UInt(bytes.count)))
    }
    
    
    public func decode(i16 bytes: [Int16]) -> String{
        var data = bytes
        return handleRaw(text_decoder_decode_i16_bytes(decoder, &data, UInt(bytes.count)))
    }
    
    
    
    public func decode(i32 bytes: [Int32]) -> String{
        return handleRaw(text_decoder_decode_i32_bytes(decoder, bytes, UInt(bytes.count)))
    }
    
    
    public func decode(u32 bytes: [UInt32]) -> String{
        return handleRaw(text_decoder_decode_u32_bytes(decoder, bytes, UInt(bytes.count)))
    }
    
    
    deinit {
        destroy_text_decoder(decoder)
    }
    
}
