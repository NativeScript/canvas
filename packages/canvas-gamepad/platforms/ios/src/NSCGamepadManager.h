#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

typedef void (^NSCGamepadConnectionListener)(NSInteger index, BOOL connected, NSString *gamepadId);

@interface NSCGamepadManager : NSObject

@property (class, nonatomic, readonly, strong) NSCGamepadManager *shared;

- (void)startWithBuffer:(void *)buffer length:(NSInteger)length listener:(NSCGamepadConnectionListener)listener;

- (void)stop;

@end

NS_ASSUME_NONNULL_END
