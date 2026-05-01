//
// NSCAudioFileStreamDecoder.h
//

#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

NS_ASSUME_NONNULL_BEGIN

@interface NSCAudioFileStreamDecoder : NSObject

+ (nullable AVAudioPCMBuffer *)decodeDataToPCMBuffer:(NSData *)data error:(NSError * _Nullable * _Nullable)error;

- (instancetype)init;
- (BOOL)appendData:(NSData *)data error:(NSError * _Nullable * _Nullable)error;
- (nullable AVAudioPCMBuffer *)finalizeAndDecodeWithError:(NSError * _Nullable * _Nullable)error;
- (void)reset;

- (nullable NSArray<AVAudioPCMBuffer *> *)decodeAvailableWithError:(NSError * _Nullable * _Nullable)error;

@end

NS_ASSUME_NONNULL_END
