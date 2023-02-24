#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AChoreographer {
    _unused: [u8; 0],
}

#[doc = " Prototype of the function that is called when a new frame is being rendered."]
#[doc = " It's passed the time that the frame is being rendered as nanoseconds in the"]
#[doc = " CLOCK_MONOTONIC time base, as well as the data pointer provided by the"]
#[doc = " application that registered a callback. All callbacks that run as part of"]
#[doc = " rendering a frame will observe the same frame time, so it should be used"]
#[doc = " whenever events need to be synchronized (e.g. animations)."]
pub type AChoreographer_frameCallback = ::std::option::Option<
    unsafe extern "C" fn(frameTimeNanos: ::std::os::raw::c_long, data: *mut ::std::os::raw::c_void),
>;
#[doc = " Prototype of the function that is called when a new frame is being rendered."]
#[doc = " It's passed the time that the frame is being rendered as nanoseconds in the"]
#[doc = " CLOCK_MONOTONIC time base, as well as the data pointer provided by the"]
#[doc = " application that registered a callback. All callbacks that run as part of"]
#[doc = " rendering a frame will observe the same frame time, so it should be used"]
#[doc = " whenever events need to be synchronized (e.g. animations)."]
pub type AChoreographer_frameCallback64 = ::std::option::Option<
    unsafe extern "C" fn(frameTimeNanos: i64, data: *mut ::std::os::raw::c_void),
>;

extern "C" {
    #[doc = " Get the AChoreographer instance for the current thread. This must be called"]
    #[doc = " on an ALooper thread."]
    pub fn AChoreographer_getInstance() -> *mut AChoreographer;
}

extern "C" {
    #[doc = " Deprecated: Use AChoreographer_postFrameCallback64 instead."]
    pub fn AChoreographer_postFrameCallback(
        choreographer: *mut AChoreographer,
        callback: AChoreographer_frameCallback,
        data: *mut ::std::os::raw::c_void,
    );
}

extern "C" {
    #[doc = " Deprecated: Use AChoreographer_postFrameCallbackDelayed64 instead."]
    pub fn AChoreographer_postFrameCallbackDelayed(
        choreographer: *mut AChoreographer,
        callback: AChoreographer_frameCallback,
        data: *mut ::std::os::raw::c_void,
        delayMillis: ::std::os::raw::c_long,
    );
}

extern "C" {
    #[doc = " Power a callback to be run on the next frame.  The data pointer provided will"]
    #[doc = " be passed to the callback function when it's called."]
    pub fn AChoreographer_postFrameCallback64(
        chroreographer: *mut AChoreographer,
        callback: AChoreographer_frameCallback64,
        data: *mut ::std::os::raw::c_void,
    );
}

extern "C" {
    #[doc = " Post a callback to be run on the frame following the specified delay.  The"]
    #[doc = " data pointer provided will be passed to the callback function when it's"]
    #[doc = " called."]
    pub fn AChoreographer_postFrameCallbackDelayed64(
        choreographer: *mut AChoreographer,
        callback: AChoreographer_frameCallback64,
        data: *mut ::std::os::raw::c_void,
        delayMillis: u32,
    );
}
