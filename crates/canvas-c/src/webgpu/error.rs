use std::error::Error;
use std::ffi::CString;
use std::fmt::Display;
use std::os::raw::c_char;
use std::sync::Arc;

fn format_error(context: &Arc<wgpu_core::global::Global>, err: &(impl Error + 'static)) -> String {
    let mut output = String::new();
    let mut level = 1;

    fn print_tree(output: &mut String, level: &mut usize, e: &(dyn Error + 'static)) {
        let mut print = |e: &(dyn Error + 'static)| {
            use std::fmt::Write;
            writeln!(output, "{}{}", " ".repeat(*level * 2), e).unwrap();

            if let Some(e) = e.source() {
                *level += 1;
                print_tree(output, level, e);
                *level -= 1;
            }
        };
        if let Some(multi) = e.downcast_ref::<wgpu_core::error::MultiError>() {
            for e in multi.errors() {
                print(e);
            }
        } else {
            print(e);
        }
    }

    print_tree(&mut output, &mut level, err);

    format!("Validation Error\n\nCaused by:\n{}", output)
}

pub(crate) fn handle_error(
    context: &Arc<wgpu_core::global::Global>,
    sink_mutex: &parking_lot::Mutex<crate::webgpu::gpu_device::ErrorSinkRaw>,
    cause: impl Error + Send + Sync + 'static,
    label_key: &'static str,
    label: wgpu_core::Label,
    string: &'static str,
) {
    let cause_error = cause.to_string();
    let error = wgpu_core::error::ContextError {
        fn_ident: string,
        label: label.unwrap_or_default().to_string(),
        source: Box::new(cause),
    };

    let mut sink = sink_mutex.lock();
    let mut source_opt: Option<&(dyn Error + 'static)> = Some(&error);
    while let Some(source) = source_opt {
        match source.downcast_ref::<wgpu_core::device::DeviceError>() {
            Some(wgpu_core::device::DeviceError::Lost) => {
                return sink.handle_error(CanvasGPUError::Lost);
            }
            Some(wgpu_core::device::DeviceError::OutOfMemory) => {
                return sink.handle_error(CanvasGPUError::OutOfMemory);
            }
            _ => (),
        }
        source_opt = source.source();
    }

    // Otherwise, it is a validation error
    sink.handle_error(CanvasGPUError::Validation(cause_error));
}

pub(crate) fn handle_error_fatal(
    global: &Arc<wgpu_core::global::Global>,
    cause: impl Error + Send + Sync + 'static,
    operation: &'static str,
) {
    // panic!(
    //     "Error in {operation}: {f}",
    //     f = format_error(context, &cause)
    // );
    let error = cause.to_string();

    // log::error!("Error in {operation}: {f}",
    //     f = error);

    log::error!("Error in {operation}: {f}",
        f = format_error(global, &cause))
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CanvasGPUErrorType {
    None,
    Lost,
    OutOfMemory,
    Validation,
    Internal,
}


pub enum CanvasGPUError {
    None,
    Lost,
    OutOfMemory,
    Validation(String),
    Internal,
}

impl Display for CanvasGPUError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CanvasGPUError::Lost { .. } => f.write_str("Device lost"),
            CanvasGPUError::OutOfMemory { .. } => f.write_str("Out of Memory"),
            CanvasGPUError::Validation(value) => f.write_str(value),
            CanvasGPUError::None => f.write_str(""),
            CanvasGPUError::Internal => f.write_str("Internal")
        }
    }
}


fn fmt_err(err: &(dyn Error + 'static)) -> String {
    let mut output = err.to_string();
    let mut level = 0;

    fn print_tree(output: &mut String, level: &mut usize, e: &(dyn Error + 'static)) {
        use std::fmt::Write;

        let mut print = |e: &(dyn Error + 'static)| {
            writeln!(output, "{}{}", " ".repeat(*level * 2), e).unwrap();

            if let Some(e) = e.source() {
                *level += 1;
                print_tree(output, level, e);
                *level -= 1;
            }
        };
        /*if let Some(multi) = e.downcast_ref::<wgpu_core::error::MultiError>() {
            for e in multi.errors() {
                print(e);
            }
        } else {
            print(e);
        }
        */
        print(e);
    }

    print_tree(&mut output, &mut level, err);

    output
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_error_get_type(error: *const CanvasGPUError) -> CanvasGPUErrorType {
    let error = error.as_ref().expect("invalid error");
    match error {
        CanvasGPUError::Lost => CanvasGPUErrorType::Lost,
        CanvasGPUError::OutOfMemory => CanvasGPUErrorType::OutOfMemory,
        CanvasGPUError::Validation(_) => CanvasGPUErrorType::Validation,
        CanvasGPUError::Internal => CanvasGPUErrorType::Internal,
        CanvasGPUError::None => CanvasGPUErrorType::None
    }
}


#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_error_get_message(error: *const CanvasGPUError) -> *mut c_char {
    let error = error.as_ref().expect("invalid error");
    match error {
        CanvasGPUError::Validation(value) => {
            CString::new(value.to_string()).unwrap().into_raw()
        }
        _ => std::ptr::null_mut()
    }
}


impl From<wgpu_core::device::DeviceError> for CanvasGPUError {
    fn from(err: wgpu_core::device::DeviceError) -> Self {
        match err {
            wgpu_core::device::DeviceError::Lost => CanvasGPUError::Lost,
            wgpu_core::device::DeviceError::OutOfMemory => CanvasGPUError::OutOfMemory,
            _ => CanvasGPUError::Validation(fmt_err(&err)),
        }
    }
}

impl From<wgpu_core::pipeline::CreateRenderPipelineError> for CanvasGPUError {
    fn from(value: wgpu_core::pipeline::CreateRenderPipelineError) -> Self {
        match value {
            wgpu_core::pipeline::CreateRenderPipelineError::Device(err) => err.into(),
            err => CanvasGPUError::Validation(fmt_err(&err)),
        }
    }
}

impl From<wgpu_core::resource::CreateBufferError> for CanvasGPUError {
    fn from(err: wgpu_core::resource::CreateBufferError) -> Self {
        match err {
            wgpu_core::resource::CreateBufferError::Device(err) => err.into(),
            wgpu_core::resource::CreateBufferError::AccessError(err) => err.into(),
            err => CanvasGPUError::Validation(fmt_err(&err)),
        }
    }
}

impl From<wgpu_core::resource::BufferAccessError> for CanvasGPUError {
    fn from(err: wgpu_core::resource::BufferAccessError) -> Self {
        match err {
            wgpu_core::resource::BufferAccessError::Device(err) => err.into(),
            err => CanvasGPUError::Validation(fmt_err(&err)),
        }
    }
}

impl From<wgpu_core::binding_model::CreateBindGroupLayoutError> for CanvasGPUError {
    fn from(err: wgpu_core::binding_model::CreateBindGroupLayoutError) -> Self {
        match err {
            wgpu_core::binding_model::CreateBindGroupLayoutError::Device(err) => err.into(),
            err => CanvasGPUError::Validation(fmt_err(&err)),
        }
    }
}

impl From<wgpu_core::binding_model::CreateBindGroupError> for CanvasGPUError {
    fn from(err: wgpu_core::binding_model::CreateBindGroupError) -> Self {
        match err {
            wgpu_core::binding_model::CreateBindGroupError::Device(err) => err.into(),
            err => CanvasGPUError::Validation(fmt_err(&err)),
        }
    }
}

impl From<wgpu_core::command::RenderBundleError> for CanvasGPUError {
    fn from(err: wgpu_core::command::RenderBundleError) -> Self {
        CanvasGPUError::Validation(fmt_err(&err))
    }
}

impl From<wgpu_core::command::CreateRenderBundleError> for CanvasGPUError {
    fn from(err: wgpu_core::command::CreateRenderBundleError) -> Self {
        CanvasGPUError::Validation(fmt_err(&err))
    }
}

impl From<wgpu_core::command::CopyError> for CanvasGPUError {
    fn from(err: wgpu_core::command::CopyError) -> Self {
        CanvasGPUError::Validation(fmt_err(&err))
    }
}

impl From<wgpu_core::command::CommandEncoderError> for CanvasGPUError {
    fn from(err: wgpu_core::command::CommandEncoderError) -> Self {
        CanvasGPUError::Validation(fmt_err(&err))
    }
}

impl From<wgpu_core::command::QueryError> for CanvasGPUError {
    fn from(err: wgpu_core::command::QueryError) -> Self {
        CanvasGPUError::Validation(fmt_err(&err))
    }
}

impl From<wgpu_core::command::ComputePassError> for CanvasGPUError {
    fn from(err: wgpu_core::command::ComputePassError) -> Self {
        CanvasGPUError::Validation(fmt_err(&err))
    }
}

impl From<wgpu_core::pipeline::CreateComputePipelineError> for CanvasGPUError {
    fn from(err: wgpu_core::pipeline::CreateComputePipelineError) -> Self {
        match err {
            wgpu_core::pipeline::CreateComputePipelineError::Device(err) => err.into(),
            err => CanvasGPUError::Validation(fmt_err(&err)),
        }
    }
}

impl From<wgpu_core::binding_model::GetBindGroupLayoutError> for CanvasGPUError {
    fn from(err: wgpu_core::binding_model::GetBindGroupLayoutError) -> Self {
        CanvasGPUError::Validation(fmt_err(&err))
    }
}

impl From<wgpu_core::command::RenderPassError> for CanvasGPUError {
    fn from(err: wgpu_core::command::RenderPassError) -> Self {
        CanvasGPUError::Validation(fmt_err(&err))
    }
}

impl From<wgpu_core::resource::CreateSamplerError> for CanvasGPUError {
    fn from(err: wgpu_core::resource::CreateSamplerError) -> Self {
        match err {
            wgpu_core::resource::CreateSamplerError::Device(err) => err.into(),
            err => CanvasGPUError::Validation(fmt_err(&err)),
        }
    }
}

impl From<wgpu_core::pipeline::CreateShaderModuleError> for CanvasGPUError {
    fn from(err: wgpu_core::pipeline::CreateShaderModuleError) -> Self {
        match err {
            wgpu_core::pipeline::CreateShaderModuleError::Device(err) => err.into(),
            err => CanvasGPUError::Validation(fmt_err(&err)),
        }
    }
}

impl From<wgpu_core::resource::CreateTextureError> for CanvasGPUError {
    fn from(err: wgpu_core::resource::CreateTextureError) -> Self {
        match err {
            wgpu_core::resource::CreateTextureError::Device(err) => err.into(),
            err => CanvasGPUError::Validation(fmt_err(&err)),
        }
    }
}

impl From<wgpu_core::resource::CreateTextureViewError> for CanvasGPUError {
    fn from(err: wgpu_core::resource::CreateTextureViewError) -> Self {
        CanvasGPUError::Validation(fmt_err(&err))
    }
}

impl From<wgpu_core::resource::CreateQuerySetError> for CanvasGPUError {
    fn from(err: wgpu_core::resource::CreateQuerySetError) -> Self {
        match err {
            wgpu_core::resource::CreateQuerySetError::Device(err) => err.into(),
            err => CanvasGPUError::Validation(fmt_err(&err)),
        }
    }
}

impl From<wgpu_core::device::queue::QueueSubmitError> for CanvasGPUError {
    fn from(err: wgpu_core::device::queue::QueueSubmitError) -> Self {
        match err {
            wgpu_core::device::queue::QueueSubmitError::Queue(err) => err.into(),
            err => CanvasGPUError::Validation(fmt_err(&err)),
        }
    }
}

impl From<wgpu_core::device::queue::QueueWriteError> for CanvasGPUError {
    fn from(err: wgpu_core::device::queue::QueueWriteError) -> Self {
        match err {
            wgpu_core::device::queue::QueueWriteError::Queue(err) => err.into(),
            err => CanvasGPUError::Validation(fmt_err(&err)),
        }
    }
}

impl From<wgpu_core::command::ClearError> for CanvasGPUError {
    fn from(err: wgpu_core::command::ClearError) -> Self {
        CanvasGPUError::Validation(fmt_err(&err))
    }
}

impl From<wgpu_core::present::ConfigureSurfaceError> for CanvasGPUError {
    fn from(err: wgpu_core::present::ConfigureSurfaceError) -> Self {
        CanvasGPUError::Validation(fmt_err(&err))
    }
}