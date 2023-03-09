use skia_safe::Rect;

pub trait ScaleUtils {
    fn scale(&mut self, x: f32, y: f32);
    fn from_scale(rect: Rect, x: f32, y: f32) -> Self;
}

impl ScaleUtils for Rect {
    fn scale(&mut self, x: f32, y: f32) {
        self.left *= x;
        self.right *= x;
        self.top *= y;
        self.bottom *= y;
    }

    fn from_scale(rect: Rect, x: f32, y: f32) -> Self {
        Rect::from_xywh(rect.left * x, rect.top * y, rect.right * x, rect.bottom * y)
    }
}
