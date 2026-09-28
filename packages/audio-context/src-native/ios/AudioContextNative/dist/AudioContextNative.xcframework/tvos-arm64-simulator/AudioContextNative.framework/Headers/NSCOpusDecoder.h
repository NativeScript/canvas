//
//  NSCOpusDecoder.h
//  AudioContextNative
//

#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

NS_ASSUME_NONNULL_BEGIN

@interface NSCOpusDecoder : NSObject

+ (nullable AVAudioPCMBuffer *)decodeOpusData:(NSData *)data;

+ (nullable AVAudioPCMBuffer *)decodeOpusFile:(NSString *)path;

@end

NS_ASSUME_NONNULL_END
