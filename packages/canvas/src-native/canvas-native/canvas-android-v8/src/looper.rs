#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ALooper {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " Returns the looper associated with the calling thread, or NULL if"]
    #[doc = " there is not one."]
    pub fn ALooper_forThread() -> *mut ALooper;
}
#[doc = " This looper will accept calls to ALooper_addFd() that do not"]
#[doc = " have a callback (that is provide NULL for the callback).  In"]
#[doc = " this case the caller of ALooper_pollOnce() or ALooper_pollAll()"]
#[doc = " MUST check the return from these functions to discover when"]
#[doc = " data is available on such fds and process it."]
pub const ALOOPER_PREPARE_ALLOW_NON_CALLBACKS: ::std::os::raw::c_uint = 1;
#[doc = " Option for for ALooper_prepare()."]
pub type _bindgen_ty_2 = ::std::os::raw::c_uint;
extern "C" {
    #[doc = " Prepares a looper associated with the calling thread, and returns it."]
    #[doc = " If the thread already has a looper, it is returned.  Otherwise, a new"]
    #[doc = " one is created, associated with the thread, and returned."]
    #[doc = ""]
    #[doc = " The opts may be ALOOPER_PREPARE_ALLOW_NON_CALLBACKS or 0."]
    pub fn ALooper_prepare(opts: ::std::os::raw::c_int) -> *mut ALooper;
}
#[doc = " The poll was awoken using wake() before the timeout expired"]
#[doc = " and no callbacks were executed and no other file descriptors were ready."]
pub const ALOOPER_POLL_WAKE: ::std::os::raw::c_int = -1;
#[doc = " Result from ALooper_pollOnce() and ALooper_pollAll():"]
#[doc = " One or more callbacks were executed."]
pub const ALOOPER_POLL_CALLBACK: ::std::os::raw::c_int = -2;
#[doc = " Result from ALooper_pollOnce() and ALooper_pollAll():"]
#[doc = " The timeout expired."]
pub const ALOOPER_POLL_TIMEOUT: ::std::os::raw::c_int = -3;
#[doc = " Result from ALooper_pollOnce() and ALooper_pollAll():"]
#[doc = " An error occurred."]
pub const ALOOPER_POLL_ERROR: ::std::os::raw::c_int = -4;
#[doc = " Result from ALooper_pollOnce() and ALooper_pollAll()."]
pub type _bindgen_ty_3 = ::std::os::raw::c_int;
extern "C" {
    #[doc = " Acquire a reference on the given ALooper object.  This prevents the object"]
    #[doc = " from being deleted until the reference is removed.  This is only needed"]
    #[doc = " to safely hand an ALooper from one thread to another."]
    pub fn ALooper_acquire(looper: *mut ALooper);
}
extern "C" {
    #[doc = " Remove a reference that was previously acquired with ALooper_acquire()."]
    pub fn ALooper_release(looper: *mut ALooper);
}
#[doc = " The file descriptor is available for read operations."]
pub const ALOOPER_EVENT_INPUT: ::std::os::raw::c_uint = 1;
#[doc = " The file descriptor is available for write operations."]
pub const ALOOPER_EVENT_OUTPUT: ::std::os::raw::c_uint = 2;
#[doc = " The file descriptor has encountered an error condition."]
#[doc = ""]
#[doc = " The looper always sends notifications about errors; it is not necessary"]
#[doc = " to specify this event flag in the requested event set."]
pub const ALOOPER_EVENT_ERROR: ::std::os::raw::c_uint = 4;
#[doc = " The file descriptor was hung up."]
#[doc = " For example, indicates that the remote end of a pipe or socket was closed."]
#[doc = ""]
#[doc = " The looper always sends notifications about hangups; it is not necessary"]
#[doc = " to specify this event flag in the requested event set."]
pub const ALOOPER_EVENT_HANGUP: ::std::os::raw::c_uint = 8;
#[doc = " The file descriptor is invalid."]
#[doc = " For example, the file descriptor was closed prematurely."]
#[doc = ""]
#[doc = " The looper always sends notifications about invalid file descriptors; it is not necessary"]
#[doc = " to specify this event flag in the requested event set."]
pub const ALOOPER_EVENT_INVALID: ::std::os::raw::c_uint = 16;
#[doc = " Flags for file descriptor events that a looper can monitor."]
#[doc = ""]
#[doc = " These flag bits can be combined to monitor multiple events at once."]
pub type _bindgen_ty_4 = ::std::os::raw::c_uint;
#[doc = " For callback-based event loops, this is the prototype of the function"]
#[doc = " that is called when a file descriptor event occurs."]
#[doc = " It is given the file descriptor it is associated with,"]
#[doc = " a bitmask of the poll events that were triggered (typically ALOOPER_EVENT_INPUT),"]
#[doc = " and the data pointer that was originally supplied."]
#[doc = ""]
#[doc = " Implementations should return 1 to continue receiving callbacks, or 0"]
#[doc = " to have this file descriptor and callback unregistered from the looper."]
pub type ALooper_callbackFunc = ::std::option::Option<
    unsafe extern "C" fn(
        fd: ::std::os::raw::c_int,
        events: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    #[doc = " Waits for events to be available, with optional timeout in milliseconds."]
    #[doc = " Invokes callbacks for all file descriptors on which an event occurred."]
    #[doc = ""]
    #[doc = " If the timeout is zero, returns immediately without blocking."]
    #[doc = " If the timeout is negative, waits indefinitely until an event appears."]
    #[doc = ""]
    #[doc = " Returns ALOOPER_POLL_WAKE if the poll was awoken using wake() before"]
    #[doc = " the timeout expired and no callbacks were invoked and no other file"]
    #[doc = " descriptors were ready."]
    #[doc = ""]
    #[doc = " Returns ALOOPER_POLL_CALLBACK if one or more callbacks were invoked."]
    #[doc = ""]
    #[doc = " Returns ALOOPER_POLL_TIMEOUT if there was no data before the given"]
    #[doc = " timeout expired."]
    #[doc = ""]
    #[doc = " Returns ALOOPER_POLL_ERROR if an error occurred."]
    #[doc = ""]
    #[doc = " Returns a value >= 0 containing an identifier (the same identifier"]
    #[doc = " `ident` passed to ALooper_addFd()) if its file descriptor has data"]
    #[doc = " and it has no callback function (requiring the caller here to"]
    #[doc = " handle it).  In this (and only this) case outFd, outEvents and"]
    #[doc = " outData will contain the poll events and data associated with the"]
    #[doc = " fd, otherwise they will be set to NULL."]
    #[doc = ""]
    #[doc = " This method does not return until it has finished invoking the appropriate callbacks"]
    #[doc = " for all file descriptors that were signalled."]
    pub fn ALooper_pollOnce(
        timeoutMillis: ::std::os::raw::c_int,
        outFd: *mut ::std::os::raw::c_int,
        outEvents: *mut ::std::os::raw::c_int,
        outData: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Like ALooper_pollOnce(), but performs all pending callbacks until all"]
    #[doc = " data has been consumed or a file descriptor is available with no callback."]
    #[doc = " This function will never return ALOOPER_POLL_CALLBACK."]
    pub fn ALooper_pollAll(
        timeoutMillis: ::std::os::raw::c_int,
        outFd: *mut ::std::os::raw::c_int,
        outEvents: *mut ::std::os::raw::c_int,
        outData: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Wakes the poll asynchronously."]
    #[doc = ""]
    #[doc = " This method can be called on any thread."]
    #[doc = " This method returns immediately."]
    pub fn ALooper_wake(looper: *mut ALooper);
}
extern "C" {
    #[doc = " Adds a new file descriptor to be polled by the looper."]
    #[doc = " If the same file descriptor was previously added, it is replaced."]
    #[doc = ""]
    #[doc = " \"fd\" is the file descriptor to be added."]
    #[doc = " \"ident\" is an identifier for this event, which is returned from ALooper_pollOnce()."]
    #[doc = " The identifier must be >= 0, or ALOOPER_POLL_CALLBACK if providing a non-NULL callback."]
    #[doc = " \"events\" are the poll events to wake up on.  Typically this is ALOOPER_EVENT_INPUT."]
    #[doc = " \"callback\" is the function to call when there is an event on the file descriptor."]
    #[doc = " \"data\" is a private data pointer to supply to the callback."]
    #[doc = ""]
    #[doc = " There are two main uses of this function:"]
    #[doc = ""]
    #[doc = " (1) If \"callback\" is non-NULL, then this function will be called when there is"]
    #[doc = " data on the file descriptor.  It should execute any events it has pending,"]
    #[doc = " appropriately reading from the file descriptor.  The 'ident' is ignored in this case."]
    #[doc = ""]
    #[doc = " (2) If \"callback\" is NULL, the 'ident' will be returned by ALooper_pollOnce"]
    #[doc = " when its file descriptor has data available, requiring the caller to take"]
    #[doc = " care of processing it."]
    #[doc = ""]
    #[doc = " Returns 1 if the file descriptor was added or -1 if an error occurred."]
    #[doc = ""]
    #[doc = " This method can be called on any thread."]
    #[doc = " This method may block briefly if it needs to wake the poll."]
    pub fn ALooper_addFd(
        looper: *mut ALooper,
        fd: ::std::os::raw::c_int,
        ident: ::std::os::raw::c_int,
        events: ::std::os::raw::c_int,
        callback: ALooper_callbackFunc,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Removes a previously added file descriptor from the looper."]
    #[doc = ""]
    #[doc = " When this method returns, it is safe to close the file descriptor since the looper"]
    #[doc = " will no longer have a reference to it.  However, it is possible for the callback to"]
    #[doc = " already be running or for it to run one last time if the file descriptor was already"]
    #[doc = " signalled.  Calling code is responsible for ensuring that this case is safely handled."]
    #[doc = " For example, if the callback takes care of removing itself during its own execution either"]
    #[doc = " by returning 0 or by calling this method, then it can be guaranteed to not be invoked"]
    #[doc = " again at any later time unless registered anew."]
    #[doc = ""]
    #[doc = " Returns 1 if the file descriptor was removed, 0 if none was previously registered"]
    #[doc = " or -1 if an error occurred."]
    #[doc = ""]
    #[doc = " This method can be called on any thread."]
    #[doc = " This method may block briefly if it needs to wake the poll."]
    pub fn ALooper_removeFd(
        looper: *mut ALooper,
        fd: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
