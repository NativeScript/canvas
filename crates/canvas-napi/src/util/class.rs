//! Class definition for raw Node-API callbacks.
//!
//! Classes are defined once per env with `napi_define_class` (methods and accessors on the
//! prototype, like the V8 bindings' templates). Natively created objects -- a gradient returned by
//! `createLinearGradient`, the `ImageData` from `getImageData` -- are instantiated through the same
//! JS constructor so they get the right prototype: [`new_instance`] parks the value in a
//! thread-local slot that the constructor adopts instead of building a new one.

use std::cell::{Cell, RefCell};
use std::ffi::{c_void, CStr};
use std::ptr;

use napi::sys;

use super::native::{self, Native, NativeType, Wrapped};

pub type Callback = unsafe extern "C" fn(sys::napi_env, sys::napi_callback_info) -> sys::napi_value;

const METHOD: sys::napi_property_attributes =
    sys::PropertyAttributes::writable | sys::PropertyAttributes::configurable;
const ACCESSOR: sys::napi_property_attributes = sys::PropertyAttributes::configurable;
const STATIC: sys::napi_property_attributes = sys::PropertyAttributes::static_;

pub struct ClassDef {
    name: &'static CStr,
    constructor: Callback,
    properties: Vec<sys::napi_property_descriptor>,
}

fn descriptor(name: &'static CStr, attributes: sys::napi_property_attributes) -> sys::napi_property_descriptor {
    sys::napi_property_descriptor {
        utf8name: name.as_ptr(),
        name: ptr::null_mut(),
        method: None,
        getter: None,
        setter: None,
        value: ptr::null_mut(),
        attributes,
        data: ptr::null_mut(),
    }
}

impl ClassDef {
    pub fn new(name: &'static CStr, constructor: Callback) -> Self {
        ClassDef {
            name,
            constructor,
            properties: Vec::new(),
        }
    }

    pub fn method(mut self, name: &'static CStr, callback: Callback) -> Self {
        let mut d = descriptor(name, METHOD);
        d.method = Some(callback);
        self.properties.push(d);
        self
    }

    pub fn getter(mut self, name: &'static CStr, getter: Callback) -> Self {
        let mut d = descriptor(name, ACCESSOR);
        d.getter = Some(getter);
        self.properties.push(d);
        self
    }

    pub fn accessor(mut self, name: &'static CStr, getter: Callback, setter: Callback) -> Self {
        let mut d = descriptor(name, ACCESSOR);
        d.getter = Some(getter);
        d.setter = Some(setter);
        self.properties.push(d);
        self
    }

    pub fn static_method(mut self, name: &'static CStr, callback: Callback) -> Self {
        let mut d = descriptor(name, STATIC | METHOD);
        d.method = Some(callback);
        self.properties.push(d);
        self
    }

    /// Defines the class, sets `exports[name]` and registers the constructor for `kind`.
    pub unsafe fn define(self, env: sys::napi_env, exports: sys::napi_value, kind: NativeType) -> sys::napi_value {
        let mut class = ptr::null_mut();
        let status = sys::napi_define_class(
            env,
            self.name.as_ptr(),
            self.name.to_bytes().len() as isize,
            Some(self.constructor),
            ptr::null_mut(),
            self.properties.len(),
            self.properties.as_ptr(),
            &mut class,
        );
        if status != sys::Status::napi_ok {
            return ptr::null_mut();
        }
        sys::napi_set_named_property(env, exports, self.name.as_ptr(), class);
        register(env, kind, class);
        class
    }
}

thread_local! {
    /// (env, kind) -> constructor. Keyed by env so worker threads/contexts stay separate.
    static CONSTRUCTORS: RefCell<Vec<(usize, NativeType, sys::napi_ref)>> = const { RefCell::new(Vec::new()) };
    static ADOPT: Cell<*mut c_void> = const { Cell::new(ptr::null_mut()) };
}

unsafe fn register(env: sys::napi_env, kind: NativeType, class: sys::napi_value) {
    let mut reference = ptr::null_mut();
    if sys::napi_create_reference(env, class, 1, &mut reference) != sys::Status::napi_ok {
        return;
    }
    CONSTRUCTORS.with(|c| {
        let mut c = c.borrow_mut();
        c.retain(|(e, k, _)| !(*e == env as usize && *k == kind));
        c.push((env as usize, kind, reference));
    });
}

pub unsafe fn constructor(env: sys::napi_env, kind: NativeType) -> Option<sys::napi_value> {
    let reference = CONSTRUCTORS.with(|c| {
        c.borrow()
            .iter()
            .find(|(e, k, _)| *e == env as usize && *k == kind)
            .map(|(_, _, r)| *r)
    })?;
    let mut value = ptr::null_mut();
    (sys::napi_get_reference_value(env, reference, &mut value) == sys::Status::napi_ok && !value.is_null()).then_some(value)
}

/// Creates a JS instance of `T`'s class that owns `value`.
pub unsafe fn new_instance<T: Native>(env: sys::napi_env, value: T) -> sys::napi_value {
    let Some(class) = constructor(env, T::KIND) else { return ptr::null_mut() };
    let boxed = Wrapped::boxed(value);
    ADOPT.with(|a| a.set(boxed as *mut c_void));
    let mut instance = ptr::null_mut();
    let status = sys::napi_new_instance(env, class, 0, ptr::null(), &mut instance);
    let unclaimed = ADOPT.with(|a| a.replace(ptr::null_mut()));
    if !unclaimed.is_null() {
        drop(Box::from_raw(unclaimed as *mut Wrapped<T>));
    }
    if status != sys::Status::napi_ok {
        return ptr::null_mut();
    }
    instance
}

/// Sets `exports[name]` to a plain function (module-level functions like `create2DContext`).
pub unsafe fn export_function(env: sys::napi_env, exports: sys::napi_value, name: &'static CStr, callback: Callback) {
    let mut function = ptr::null_mut();
    if sys::napi_create_function(
        env,
        name.as_ptr(),
        name.to_bytes().len() as isize,
        Some(callback),
        ptr::null_mut(),
        &mut function,
    ) == sys::Status::napi_ok
    {
        sys::napi_set_named_property(env, exports, name.as_ptr(), function);
    }
}

/// Called first by every constructor: the value parked by [`new_instance`], if any.
#[inline]
pub fn take_adopted() -> *mut c_void {
    ADOPT.with(|a| a.replace(ptr::null_mut()))
}

/// Constructor body shared by [`constructor!`](crate::constructor).
#[inline]
pub unsafe fn construct<T: Native>(
    env: sys::napi_env,
    this: sys::napi_value,
    make: impl FnOnce() -> Option<T>,
) -> sys::napi_value {
    let adopted = take_adopted();
    if !adopted.is_null() {
        native::attach::<T>(env, this, adopted as *mut Wrapped<T>);
        return this;
    }
    match super::guard(env, make).flatten() {
        Some(value) => {
            if native::wrap(env, this, value) {
                this
            } else {
                ptr::null_mut()
            }
        }
        None => ptr::null_mut(),
    }
}

/// Defines a constructor callback. The body returns `Option<T>`; `None` leaves the object
/// unwrapped (after throwing, if the body chose to).
#[macro_export]
macro_rules! constructor {
    ($name:ident, $T:ty, $n:literal, |$cx:ident| $body:expr) => {
        unsafe extern "C" fn $name(
            env: napi::sys::napi_env,
            info: napi::sys::napi_callback_info,
        ) -> napi::sys::napi_value {
            let $cx = $crate::util::cx::Cx::<$n>::new(env, info);
            let this = $cx.this;
            $crate::util::class::construct::<$T>(env, this, move || $body)
        }
    };
}

/// A constructor for classes JS cannot instantiate directly (`new TextMetrics()` throws, as in
/// browsers); instances only come from [`new_instance`].
#[macro_export]
macro_rules! illegal_constructor {
    ($name:ident, $T:ty) => {
        unsafe extern "C" fn $name(
            env: napi::sys::napi_env,
            info: napi::sys::napi_callback_info,
        ) -> napi::sys::napi_value {
            let cx = $crate::util::cx::Cx::<0>::new(env, info);
            let adopted = $crate::util::class::take_adopted();
            if adopted.is_null() {
                return $crate::util::ret::throw_type_error(env, c"Illegal constructor");
            }
            $crate::util::native::attach::<$T>(env, cx.this, adopted as *mut _);
            cx.this
        }
    };
}

/// Defines a method (or accessor) callback on `T`. `$cx` gives the arguments, `$this` the
/// receiver; a receiver of the wrong type makes the call a no-op returning `undefined`, as in
/// the V8 bindings. The body evaluates to the `napi_value` to return.
#[macro_export]
macro_rules! method {
    ($name:ident, $T:ty, $n:literal, |$cx:ident, $this:ident| $body:expr) => {
        #[allow(unused_variables)]
        unsafe extern "C" fn $name(
            env: napi::sys::napi_env,
            info: napi::sys::napi_callback_info,
        ) -> napi::sys::napi_value {
            let $cx = $crate::util::cx::Cx::<$n>::new(env, info);
            let Some($this) = $cx.this::<$T>() else {
                return std::ptr::null_mut();
            };
            $crate::util::guard_value(env, move || $body)
        }
    };
}
