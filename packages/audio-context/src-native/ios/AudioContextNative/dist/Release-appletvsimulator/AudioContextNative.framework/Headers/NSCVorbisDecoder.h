//
//  NSCVorbisDecoder.h
//  AudioContextNative
//

#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

NS_ASSUME_NONNULL_BEGIN

@interface NSCVorbisDecoder : NSObject

+ (nullable AVAudioPCMBuffer *)decodeVorbisData:(NSData *)data;

+ (nullable AVAudioPCMBuffer *)decodeVorbisFile:(NSString *)path;

@end

NS_ASSUME_NONNULL_END
