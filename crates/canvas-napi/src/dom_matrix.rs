use napi::bindgen_prelude::ObjectFinalize;
use napi::*;

#[napi(custom_finalize)]
pub struct DOMMatrix {
    pub(crate) matrix: *mut canvas_c::Matrix,
}

impl ObjectFinalize for DOMMatrix {
    fn finalize(self, _: Env) -> Result<()> {
        canvas_c::canvas_native_matrix_release(self.matrix);
        Ok(())
    }
}

#[napi]
impl DOMMatrix {
    #[napi(constructor)]
    pub fn new(data: Option<Vec<f64>>) -> DOMMatrix {
        let matrix = canvas_c::canvas_native_matrix_create();
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
        DOMMatrix { matrix }
    }
}
