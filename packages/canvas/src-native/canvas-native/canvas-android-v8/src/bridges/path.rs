#[derive(Clone)]
pub struct Path(canvas_core::context::paths::path::Path);

impl Default for Path {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Path {
    pub(crate) fn inner(&self) -> &canvas_core::context::paths::path::Path {
        &self.0
    }

    pub(crate) fn inner_mut(&mut self) -> &mut canvas_core::context::paths::path::Path {
        &mut self.0
    }
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Path;
        fn canvas_native_path_add_path(path: &mut Path, path_to_add: &Path);
        fn canvas_native_path_create() -> Box<Path>;
        fn canvas_native_path_create_with_path(path: &Path) -> Box<Path>;
        fn canvas_native_path_create_with_string(string: String) -> Box<Path>;
        fn canvas_native_path_close_path(path: &mut Path);
        fn canvas_native_path_move_to(path: &mut Path, x: f32, y: f32);
        fn canvas_native_path_line_to(path: &mut Path, x: f32, y: f32);
        fn canvas_native_path_bezier_curve_to(
            path: &mut Path,
            cp1x: f32,
            cp1y: f32,
            cp2x: f32,
            cp2y: f32,
            x: f32,
            y: f32,
        );

        fn canvas_native_path_quadratic_curve_to(
            path: &mut Path,
            cpx: f32,
            cpy: f32,
            x: f32,
            y: f32,
        );

        fn canvas_native_path_arc(
            path: &mut Path,
            x: f32,
            y: f32,
            radius: f32,
            start_angle: f32,
            end_angle: f32,
            anti_clockwise: bool,
        );

        fn canvas_native_path_arc_to(
            path: &mut Path,
            x1: f32,
            y1: f32,
            x2: f32,
            y2: f32,
            radius: f32,
        );
        fn canvas_native_path_ellipse(
            path: &mut Path,
            x: f32,
            y: f32,
            radius_x: f32,
            radius_y: f32,
            rotation: f32,
            start_angle: f32,
            end_angle: f32,
            anticlockwise: bool,
        );

        fn canvas_native_path_rect(path: &mut Path, x: f32, y: f32, width: f32, height: f32);

        fn canvas_native_path_to_string(path: &Path) -> String;
    }
}

pub fn canvas_native_path_add_path(path: &mut Path, path_to_add: &Path) {
    path.0.add_path(&path_to_add.0, None);
}

pub fn canvas_native_path_create() -> Box<Path> {
    Box::new(Path::default())
}

pub fn canvas_native_path_create_with_path(path: &Path) -> Box<Path> {
    Box::new(path.clone())
}

pub fn canvas_native_path_create_with_string(string: String) -> Box<Path> {
    Box::new(Path(canvas_core::context::paths::path::Path::from_str(
        string.as_str(),
    )))
}

pub fn canvas_native_path_close_path(path: &mut Path) {
    path.0.close_path()
}

pub fn canvas_native_path_move_to(path: &mut Path, x: f32, y: f32) {
    path.0.move_to(x, y)
}

pub fn canvas_native_path_line_to(path: &mut Path, x: f32, y: f32) {
    path.0.line_to(x, y)
}

pub fn canvas_native_path_bezier_curve_to(
    path: &mut Path,
    cp1x: f32,
    cp1y: f32,
    cp2x: f32,
    cp2y: f32,
    x: f32,
    y: f32,
) {
    path.0.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y)
}

pub fn canvas_native_path_quadratic_curve_to(path: &mut Path, cpx: f32, cpy: f32, x: f32, y: f32) {
    path.0.quadratic_curve_to(cpx, cpy, x, y)
}

pub fn canvas_native_path_arc(
    path: &mut Path,
    x: f32,
    y: f32,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    anti_clockwise: bool,
) {
    path.0
        .arc(x, y, radius, start_angle, end_angle, anti_clockwise)
}

pub fn canvas_native_path_arc_to(path: &mut Path, x1: f32, y1: f32, x2: f32, y2: f32, radius: f32) {
    path.0.arc_to(x1, y1, x2, y2, radius)
}

pub fn canvas_native_path_ellipse(
    path: &mut Path,
    x: f32,
    y: f32,
    radius_x: f32,
    radius_y: f32,
    rotation: f32,
    start_angle: f32,
    end_angle: f32,
    anticlockwise: bool,
) {
    path.0.ellipse(
        x,
        y,
        radius_x,
        radius_y,
        rotation,
        start_angle,
        end_angle,
        anticlockwise,
    )
}

pub fn canvas_native_path_rect(path: &mut Path, x: f32, y: f32, width: f32, height: f32) {
    path.0.rect(x, y, width, height)
}

pub fn canvas_native_path_to_string(path: &Path) -> String {
    path.0.path().to_svg()
}
