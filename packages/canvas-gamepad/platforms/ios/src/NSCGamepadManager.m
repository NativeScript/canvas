#import "NSCGamepadManager.h"
#import <GameController/GameController.h>

// Keep in sync with packages/canvas-gamepad/common.ts.
static const NSInteger kMaxGamepads = 4;
static const NSInteger kMaxAxes = 8;
static const NSInteger kSlotConnected = 0;
static const NSInteger kSlotSequence = 1;
static const NSInteger kSlotMapping = 2;
static const NSInteger kSlotAxisCount = 3;
static const NSInteger kSlotButtonCount = 4;
static const NSInteger kSlotAxes = 5;
static const NSInteger kSlotButtons = kSlotAxes + kMaxAxes;
static const NSInteger kSlotStride = 80;
static const int kButtonPressed = 1;
static const int kButtonTouched = 2;
static const float kMappingStandard = 1;
static const NSInteger kStandardAxes = 4;
static const NSInteger kStandardButtons = 17;
// Float32 holds integers exactly up to 2^24.
static const float kSequenceWrap = 16777216.0f;

static inline void NSCWriteButton(float *slot, NSInteger index, GCControllerButtonInput *_Nullable button) {
    float *out = slot + kSlotButtons + index * 2;
    if (button == nil) {
        out[0] = 0;
        out[1] = 0;
        return;
    }
    float value = button.value;
    BOOL pressed = button.isPressed;
    out[0] = value;
    out[1] = (float) ((pressed ? kButtonPressed : 0) | ((pressed || value > 0) ? kButtonTouched : 0));
}

@implementation NSCGamepadManager {
    float *_buffer;
    NSCGamepadConnectionListener _listener;
    NSMutableArray *_slots;
}

+ (NSCGamepadManager *)shared {
    static NSCGamepadManager *shared;
    static dispatch_once_t once;
    dispatch_once(&once, ^{
        shared = [[NSCGamepadManager alloc] init];
    });
    return shared;
}

- (instancetype)init {
    if (self = [super init]) {
        _slots = [NSMutableArray arrayWithCapacity:kMaxGamepads];
        for (NSInteger i = 0; i < kMaxGamepads; i++) {
            [_slots addObject:NSNull.null];
        }
    }
    return self;
}

- (void)startWithBuffer:(void *)buffer length:(NSInteger)length listener:(NSCGamepadConnectionListener)listener {
    [self stop];
    if (buffer == NULL || length < kMaxGamepads * kSlotStride) {
        return;
    }
    _buffer = (float *) buffer;
    _listener = [listener copy];
    memset(_buffer, 0, sizeof(float) * kMaxGamepads * kSlotStride);

    NSNotificationCenter *center = NSNotificationCenter.defaultCenter;
    [center addObserver:self selector:@selector(controllerDidConnect:) name:GCControllerDidConnectNotification object:nil];
    [center addObserver:self selector:@selector(controllerDidDisconnect:) name:GCControllerDidDisconnectNotification object:nil];

    for (GCController *controller in GCController.controllers) {
        [self attach:controller];
    }
}

- (void)stop {
    [NSNotificationCenter.defaultCenter removeObserver:self];
    for (NSInteger i = 0; i < kMaxGamepads; i++) {
        id controller = _slots[i];
        if (controller != NSNull.null) {
            ((GCController *) controller).extendedGamepad.valueChangedHandler = nil;
            _slots[i] = NSNull.null;
        }
    }
    _buffer = NULL;
    _listener = nil;
}

- (void)controllerDidConnect:(NSNotification *)notification {
    [self attach:notification.object];
}

- (void)controllerDidDisconnect:(NSNotification *)notification {
    GCController *controller = notification.object;
    NSUInteger index = [_slots indexOfObjectIdenticalTo:controller];
    if (index == NSNotFound || _buffer == NULL) {
        return;
    }
    controller.extendedGamepad.valueChangedHandler = nil;
    _slots[index] = NSNull.null;
    memset(_buffer + index * kSlotStride, 0, sizeof(float) * kSlotStride);
    if (_listener) {
        _listener(index, NO, @"");
    }
}

- (void)attach:(GCController *)controller {
    GCExtendedGamepad *gamepad = controller.extendedGamepad;
    // The Siri Remote stays on NSCCanvas's keyboard mapping.
    if (gamepad == nil || _buffer == NULL || [_slots indexOfObjectIdenticalTo:controller] != NSNotFound) {
        return;
    }
    NSUInteger index = [_slots indexOfObjectIdenticalTo:NSNull.null];
    if (index == NSNotFound) {
        return;
    }
    _slots[index] = controller;

    float *slot = _buffer + index * kSlotStride;
    slot[kSlotConnected] = 1;
    slot[kSlotMapping] = kMappingStandard;
    slot[kSlotAxisCount] = kStandardAxes;
    slot[kSlotButtonCount] = kStandardButtons;
    [self write:gamepad slot:slot];

    __weak NSCGamepadManager *weakSelf = self;
    gamepad.valueChangedHandler = ^(GCExtendedGamepad *changed, __unused GCControllerElement *element) {
        [weakSelf update:changed];
    };

    if (_listener) {
        NSString *name = controller.vendorName ?: @"Gamepad";
        _listener(index, YES, [NSString stringWithFormat:@"%@ (STANDARD GAMEPAD)", name]);
    }
}

- (void)update:(GCExtendedGamepad *)gamepad {
    if (_buffer == NULL) {
        return;
    }
    NSUInteger index = [_slots indexOfObjectIdenticalTo:gamepad.controller];
    if (index == NSNotFound) {
        return;
    }
    [self write:gamepad slot:_buffer + index * kSlotStride];
}

- (void)write:(GCExtendedGamepad *)gamepad slot:(float *)slot {
    float *axes = slot + kSlotAxes;
    axes[0] = gamepad.leftThumbstick.xAxis.value;
    axes[1] = -gamepad.leftThumbstick.yAxis.value;
    axes[2] = gamepad.rightThumbstick.xAxis.value;
    axes[3] = -gamepad.rightThumbstick.yAxis.value;

    NSCWriteButton(slot, 0, gamepad.buttonA);
    NSCWriteButton(slot, 1, gamepad.buttonB);
    NSCWriteButton(slot, 2, gamepad.buttonX);
    NSCWriteButton(slot, 3, gamepad.buttonY);
    NSCWriteButton(slot, 4, gamepad.leftShoulder);
    NSCWriteButton(slot, 5, gamepad.rightShoulder);
    NSCWriteButton(slot, 6, gamepad.leftTrigger);
    NSCWriteButton(slot, 7, gamepad.rightTrigger);
    GCControllerButtonInput *options = nil, *menu = nil, *home = nil, *leftStick = nil, *rightStick = nil;
    if (@available(iOS 12.1, tvOS 12.1, *)) {
        leftStick = gamepad.leftThumbstickButton;
        rightStick = gamepad.rightThumbstickButton;
    }
    if (@available(iOS 13.0, tvOS 13.0, *)) {
        options = gamepad.buttonOptions;
        menu = gamepad.buttonMenu;
    }
    if (@available(iOS 14.0, tvOS 14.0, *)) {
        home = gamepad.buttonHome;
    }
    NSCWriteButton(slot, 8, options);
    NSCWriteButton(slot, 9, menu);
    NSCWriteButton(slot, 10, leftStick);
    NSCWriteButton(slot, 11, rightStick);
    NSCWriteButton(slot, 12, gamepad.dpad.up);
    NSCWriteButton(slot, 13, gamepad.dpad.down);
    NSCWriteButton(slot, 14, gamepad.dpad.left);
    NSCWriteButton(slot, 15, gamepad.dpad.right);
    NSCWriteButton(slot, 16, home);

    slot[kSlotSequence] = fmodf(slot[kSlotSequence] + 1, kSequenceWrap);
}

@end
