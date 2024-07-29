use crate::image_asset::ImageAsset;
use std::sync::Arc;

#[no_mangle]
pub extern "C" fn canvas_native_image_asset_destroy(asset: *const ImageAsset) {
    if asset.is_null() {
        return;
    }
    let _ = unsafe { Arc::decrement_strong_count(asset) };
}
