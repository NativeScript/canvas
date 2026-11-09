//! Runs tasks on the thread that created the poster, posted from any thread: the
//! thread's ALooper (Android) or run loop (Apple). Used to call back into JS from
//! the WebGPU queue thread.

use std::sync::Arc;

type Task = Box<dyn FnOnce() + Send>;

pub(crate) struct ThreadPoster {
    tasks: parking_lot::Mutex<Vec<Task>>,
    waker: platform::Waker,
    thread: std::thread::ThreadId,
}

impl ThreadPoster {
    /// A poster for the current thread, or `None` if it has no loop.
    pub(crate) fn for_current_thread() -> Option<Arc<Self>> {
        platform::Waker::create().map(|waker| {
            let poster = Arc::new(ThreadPoster {
                tasks: parking_lot::Mutex::new(Vec::new()),
                waker,
                thread: std::thread::current().id(),
            });
            poster.waker.attach(Arc::downgrade(&poster));
            poster
        })
    }

    pub(crate) fn is_current(&self) -> bool {
        std::thread::current().id() == self.thread
    }

    /// Runs `task` on the poster's thread: now if called there, else on its loop.
    pub(crate) fn post(&self, task: Task) {
        if self.is_current() {
            task();
            return;
        }
        self.tasks.lock().push(task);
        self.waker.wake();
    }

    fn run(&self) {
        loop {
            let tasks = std::mem::take(&mut *self.tasks.lock());
            if tasks.is_empty() {
                return;
            }
            for task in tasks {
                task();
            }
        }
    }
}

#[cfg(target_os = "android")]
mod platform {
    use super::ThreadPoster;
    use std::os::fd::RawFd;
    use std::sync::Weak;

    extern "C" {
        fn ALooper_forThread() -> *mut std::ffi::c_void;
        fn ALooper_addFd(
            looper: *mut std::ffi::c_void,
            fd: RawFd,
            ident: i32,
            events: i32,
            callback: Option<extern "C" fn(RawFd, i32, *mut std::ffi::c_void) -> i32>,
            data: *mut std::ffi::c_void,
        ) -> i32;
        fn ALooper_removeFd(looper: *mut std::ffi::c_void, fd: RawFd) -> i32;
        fn ALooper_acquire(looper: *mut std::ffi::c_void);
        fn ALooper_release(looper: *mut std::ffi::c_void);
    }

    const ALOOPER_POLL_CALLBACK: i32 = -2;
    const ALOOPER_EVENT_INPUT: i32 = 1;

    pub(super) struct Waker {
        looper: *mut std::ffi::c_void,
        fd: RawFd,
        poster: parking_lot::Mutex<*mut Weak<ThreadPoster>>,
    }

    // SAFETY: ALooper and eventfd writes are thread-safe; `poster` is only set once.
    unsafe impl Send for Waker {}
    unsafe impl Sync for Waker {}

    extern "C" fn on_wake(fd: RawFd, _events: i32, data: *mut std::ffi::c_void) -> i32 {
        let mut count = 0u64;
        unsafe { libc::read(fd, &mut count as *mut u64 as *mut _, 8) };
        let poster = unsafe { &*(data as *const Weak<ThreadPoster>) };
        if let Some(poster) = poster.upgrade() {
            poster.run();
        }
        1
    }

    impl Waker {
        pub(super) fn create() -> Option<Self> {
            let looper = unsafe { ALooper_forThread() };
            if looper.is_null() {
                return None;
            }
            let fd = unsafe { libc::eventfd(0, libc::EFD_NONBLOCK | libc::EFD_CLOEXEC) };
            if fd < 0 {
                return None;
            }
            unsafe { ALooper_acquire(looper) };
            Some(Waker {
                looper,
                fd,
                poster: parking_lot::Mutex::new(std::ptr::null_mut()),
            })
        }

        pub(super) fn attach(&self, poster: Weak<ThreadPoster>) {
            let data = Box::into_raw(Box::new(poster));
            *self.poster.lock() = data;
            unsafe {
                ALooper_addFd(
                    self.looper,
                    self.fd,
                    ALOOPER_POLL_CALLBACK,
                    ALOOPER_EVENT_INPUT,
                    Some(on_wake),
                    data as *mut _,
                )
            };
        }

        pub(super) fn wake(&self) {
            let one = 1u64;
            unsafe { libc::write(self.fd, &one as *const u64 as *const _, 8) };
        }
    }

    impl Drop for Waker {
        fn drop(&mut self) {
            unsafe {
                ALooper_removeFd(self.looper, self.fd);
                libc::close(self.fd);
                ALooper_release(self.looper);
                let data = *self.poster.lock();
                if !data.is_null() {
                    drop(Box::from_raw(data));
                }
            }
        }
    }
}

#[cfg(any(target_os = "ios", target_os = "visionos", target_os = "tvos"))]
mod platform {
    use super::ThreadPoster;
    use std::ffi::c_void;
    use std::sync::Weak;

    type CFTypeRef = *const c_void;

    #[repr(C)]
    struct CFRunLoopSourceContext {
        version: isize,
        info: *mut c_void,
        retain: Option<extern "C" fn(*const c_void) -> *const c_void>,
        release: Option<extern "C" fn(*const c_void)>,
        copy_description: Option<extern "C" fn(*const c_void) -> CFTypeRef>,
        equal: Option<extern "C" fn(*const c_void, *const c_void) -> u8>,
        hash: Option<extern "C" fn(*const c_void) -> usize>,
        schedule: Option<extern "C" fn(*mut c_void, CFTypeRef, CFTypeRef)>,
        cancel: Option<extern "C" fn(*mut c_void, CFTypeRef, CFTypeRef)>,
        perform: Option<extern "C" fn(*mut c_void)>,
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        static kCFRunLoopCommonModes: CFTypeRef;
        fn CFRunLoopGetCurrent() -> CFTypeRef;
        fn CFRunLoopSourceCreate(
            allocator: CFTypeRef,
            order: isize,
            context: *mut CFRunLoopSourceContext,
        ) -> CFTypeRef;
        fn CFRunLoopAddSource(run_loop: CFTypeRef, source: CFTypeRef, mode: CFTypeRef);
        fn CFRunLoopSourceInvalidate(source: CFTypeRef);
        fn CFRunLoopSourceSignal(source: CFTypeRef);
        fn CFRunLoopWakeUp(run_loop: CFTypeRef);
        fn CFRetain(value: CFTypeRef) -> CFTypeRef;
        fn CFRelease(value: CFTypeRef);
    }

    pub(super) struct Waker {
        run_loop: CFTypeRef,
        source: parking_lot::Mutex<CFTypeRef>,
        poster: parking_lot::Mutex<*mut Weak<ThreadPoster>>,
    }

    // SAFETY: CFRunLoopSourceSignal and CFRunLoopWakeUp are thread-safe.
    unsafe impl Send for Waker {}
    unsafe impl Sync for Waker {}

    extern "C" fn perform(info: *mut c_void) {
        let poster = unsafe { &*(info as *const Weak<ThreadPoster>) };
        if let Some(poster) = poster.upgrade() {
            poster.run();
        }
    }

    impl Waker {
        pub(super) fn create() -> Option<Self> {
            let run_loop = unsafe { CFRunLoopGetCurrent() };
            if run_loop.is_null() {
                return None;
            }
            unsafe { CFRetain(run_loop) };
            Some(Waker {
                run_loop,
                source: parking_lot::Mutex::new(std::ptr::null()),
                poster: parking_lot::Mutex::new(std::ptr::null_mut()),
            })
        }

        pub(super) fn attach(&self, poster: Weak<ThreadPoster>) {
            let data = Box::into_raw(Box::new(poster));
            *self.poster.lock() = data;
            let mut context = CFRunLoopSourceContext {
                version: 0,
                info: data as *mut c_void,
                retain: None,
                release: None,
                copy_description: None,
                equal: None,
                hash: None,
                schedule: None,
                cancel: None,
                perform: Some(perform),
            };
            unsafe {
                let source = CFRunLoopSourceCreate(std::ptr::null(), 0, &mut context);
                if !source.is_null() {
                    CFRunLoopAddSource(self.run_loop, source, kCFRunLoopCommonModes);
                    *self.source.lock() = source;
                }
            }
        }

        pub(super) fn wake(&self) {
            let source = *self.source.lock();
            if source.is_null() {
                return;
            }
            unsafe {
                CFRunLoopSourceSignal(source);
                CFRunLoopWakeUp(self.run_loop);
            }
        }
    }

    impl Drop for Waker {
        fn drop(&mut self) {
            unsafe {
                let source = *self.source.lock();
                if !source.is_null() {
                    CFRunLoopSourceInvalidate(source);
                    CFRelease(source);
                }
                CFRelease(self.run_loop);
                let data = *self.poster.lock();
                if !data.is_null() {
                    drop(Box::from_raw(data));
                }
            }
        }
    }
}

#[cfg(not(any(
    target_os = "android",
    target_os = "ios",
    target_os = "visionos",
    target_os = "tvos"
)))]
mod platform {
    use super::ThreadPoster;
    use std::sync::Weak;

    pub(super) struct Waker;

    impl Waker {
        pub(super) fn create() -> Option<Self> {
            None
        }
        pub(super) fn attach(&self, _poster: Weak<ThreadPoster>) {}
        pub(super) fn wake(&self) {}
    }
}
