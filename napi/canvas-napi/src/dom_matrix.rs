use napi::*;

#[napi(js_name = "DOMMatrix")]
pub struct JSDOMMatrix {
    pub(crate) matrix: *mut canvas_c::Matrix,
}

#[napi]
impl JSDOMMatrix {
    #[napi(constructor)]
    pub fn new(data: Option<Vec<f64>>) -> JSDOMMatrix {
        let mut matrix = canvas_c::canvas_native_matrix_create();
        if let Some(init) = data {
            let init = init.into_iter().map(|v| v as f32).collect::<Vec<f32>>();
            match init.len() {
                6 => {
                    canvas_c::canvas_native_matrix_update(matrix, init.as_ptr(), init.len())
                }
                16 => {
                    canvas_c::canvas_native_matrix_update_3d(matrix, init.as_ptr(), init.len())
                }
                _ => {}
            }
        }
        JSDOMMatrix { matrix }
    }
}
