use std::os::raw::c_void;

pub struct CPUContext {
    #[cfg(not(target_os = "android"))]
    data: *mut c_void,
    #[cfg(not(target_os = "android"))]
    render: Option<extern "C" fn(*const c_void)>,

    #[cfg(target_os = "android")]
    jvm: jni::JavaVM,
    #[cfg(target_os = "android")]
    render: jni::objects::GlobalRef,
}

impl CPUContext {
    #[cfg(not(target_os = "android"))]
    pub fn new(data: *mut c_void, render: Option<extern "C" fn(*const c_void)>) -> Self {
        Self { data, render }
    }

    #[cfg(target_os = "android")]
    pub fn new(jvm: jni::JavaVM, render: jni::objects::GlobalRef) -> Self {
        Self { jvm, render }
    }

    #[cfg(not(target_os = "android"))]
    pub fn render(&self) {
        if let Some(render) = self.render {
            render(self.data);
        }
    }
    #[cfg(target_os = "android")]
    pub fn render(&self) {
        let vm = self.jvm.attach_current_thread();
        let mut env = vm.unwrap();
        env.call_method(self.render.as_obj(), "render", "()V", &[]);
    }
}
