use lazy_static::lazy_static;
use tokio::runtime::{Builder, Runtime};

lazy_static! {
pub(crate) static ref THREAD_POOL: Runtime = Builder::new_multi_thread()
            .enable_all()
            .thread_name("CanvasNativePool")
            .build()
            .unwrap();
}