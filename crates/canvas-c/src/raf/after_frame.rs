//! Runs a task once the task running now returns to the thread's loop (ALooper on Android,
//! CFRunLoop on Apple). Posted from a page's requestAnimationFrame callback, it runs after the
//! frame's callbacks: where a browser presents a canvas the page drew this frame.

use std::cell::RefCell;

type Task = Box<dyn FnOnce()>;

thread_local! {
    static PENDING: RefCell<Vec<Task>> = const { RefCell::new(Vec::new()) };
}

fn run_pending() {
    // Tasks queued while these run woke the loop again and wait for that turn.
    let tasks = PENDING.with(|pending| std::mem::take(&mut *pending.borrow_mut()));
    for task in tasks {
        task();
    }
}

/// Queues `task` on this thread's loop. Hands it back if the thread has none.
pub(crate) fn post(task: Task) -> Result<(), Task> {
    if !platform::wake() {
        return Err(task);
    }
    PENDING.with(|pending| pending.borrow_mut().push(task));
    Ok(())
}

#[cfg(target_os = "android")]
mod platform {
    use ndk::looper::{FdEvent, ForeignLooper};
    use std::os::fd::RawFd;

    extern "C" {
        fn ALooper_forThread() -> *mut std::ffi::c_void;
        fn ALooper_removeFd(looper: *mut std::ffi::c_void, fd: RawFd) -> i32;
    }

    struct Waker(RawFd);

    impl Drop for Waker {
        fn drop(&mut self) {
            unsafe {
                let looper = ALooper_forThread();
                if !looper.is_null() {
                    ALooper_removeFd(looper, self.0);
                }
                libc::close(self.0);
            }
        }
    }

    thread_local! {
        static WAKER: Option<Waker> = create();
    }

    fn create() -> Option<Waker> {
        let looper = ForeignLooper::for_thread()?;
        let fd = unsafe { libc::eventfd(0, libc::EFD_NONBLOCK | libc::EFD_CLOEXEC) };
        if fd < 0 {
            return None;
        }
        let waker = Waker(fd);
        looper
            .add_fd_with_callback(fd, FdEvent::INPUT, |fd| {
                let mut count = 0u64;
                unsafe { libc::read(fd, &mut count as *mut u64 as *mut _, 8) };
                super::run_pending();
                true
            })
            .ok()?;
        Some(waker)
    }

    pub(super) fn wake() -> bool {
        WAKER.with(|waker| match waker {
            Some(waker) => {
                let one = 1u64;
                unsafe { libc::write(waker.0, &one as *const u64 as *const _, 8) };
                true
            }
            None => false,
        })
    }
}

#[cfg(any(target_os = "ios", target_os = "visionos", target_os = "tvos"))]
mod platform {
    use std::ffi::c_void;

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
        fn CFRelease(value: CFTypeRef);
    }

    struct Waker {
        run_loop: CFTypeRef,
        source: CFTypeRef,
    }

    impl Drop for Waker {
        fn drop(&mut self) {
            unsafe {
                CFRunLoopSourceInvalidate(self.source);
                CFRelease(self.source);
            }
        }
    }

    thread_local! {
        static WAKER: Option<Waker> = create();
    }

    extern "C" fn perform(_info: *mut c_void) {
        super::run_pending();
    }

    fn create() -> Option<Waker> {
        unsafe {
            let run_loop = CFRunLoopGetCurrent();
            if run_loop.is_null() {
                return None;
            }
            let mut context = CFRunLoopSourceContext {
                version: 0,
                info: std::ptr::null_mut(),
                retain: None,
                release: None,
                copy_description: None,
                equal: None,
                hash: None,
                schedule: None,
                cancel: None,
                perform: Some(perform),
            };
            let source = CFRunLoopSourceCreate(std::ptr::null(), 0, &mut context);
            if source.is_null() {
                return None;
            }
            CFRunLoopAddSource(run_loop, source, kCFRunLoopCommonModes);
            // The run loop outlives the thread's locals, so it is not retained here.
            Some(Waker { run_loop, source })
        }
    }

    pub(super) fn wake() -> bool {
        WAKER.with(|waker| match waker {
            Some(waker) => {
                unsafe {
                    CFRunLoopSourceSignal(waker.source);
                    CFRunLoopWakeUp(waker.run_loop);
                }
                true
            }
            None => false,
        })
    }
}
