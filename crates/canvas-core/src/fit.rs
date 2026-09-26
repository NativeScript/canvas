//! How a canvas's drawing buffer is laid out in its view: the "fit" modes the iOS and Android
//! views implement natively, as one neutral computation desktop hosts share.
//!
//! The buffer is `surface` physical pixels; at `density` pixels per DIP (per axis: a host's
//! composition scale can differ between them) that is its natural size in the view. Every mode but `None` centres it in the view and scales it about its centre.

/// Mirrors `CanvasFit` on iOS (and the ints `packages/canvas` passes as `fit`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(i32)]
pub enum CanvasFit {
    /// Natural size, top-left.
    None = 0,
    /// Stretched to the view.
    Fill = 1,
    /// Scaled uniformly to the view's width.
    #[default]
    FitX = 2,
    /// Scaled uniformly to the view's height.
    FitY = 3,
    /// Scaled uniformly to fit the view, never up.
    ScaleDown = 4,
}

impl CanvasFit {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::None),
            1 => Some(Self::Fill),
            2 => Some(Self::FitX),
            3 => Some(Self::FitY),
            4 => Some(Self::ScaleDown),
            _ => None,
        }
    }
}

/// Buffer pixels to view DIPs: `dip = pixel * scale + offset`, per axis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceTransform {
    pub scale_x: f32,
    pub scale_y: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

fn positive(value: f32) -> bool {
    value.is_finite() && value > 0.
}

/// Where a `surface` (pixels) buffer goes in a `view` (DIPs) laid out with `fit`. Until both
/// sizes are known the buffer sits at its natural size, top-left.
pub fn surface_transform(fit: CanvasFit, surface: (f32, f32), density: (f32, f32), view: (f32, f32)) -> SurfaceTransform {
    let density = (
        if positive(density.0) { density.0 } else { 1. },
        if positive(density.1) { density.1 } else { 1. },
    );
    let natural = SurfaceTransform {
        scale_x: 1. / density.0,
        scale_y: 1. / density.1,
        offset_x: 0.,
        offset_y: 0.,
    };
    let content = (surface.0 / density.0, surface.1 / density.1);
    if fit == CanvasFit::None || !positive(content.0) || !positive(content.1) || !positive(view.0) || !positive(view.1) {
        return natural;
    }

    let (sx, sy) = (view.0 / content.0, view.1 / content.1);
    let (fx, fy) = match fit {
        CanvasFit::None | CanvasFit::Fill => (sx, sy),
        CanvasFit::FitX => (sx, sx),
        CanvasFit::FitY => (sy, sy),
        CanvasFit::ScaleDown => {
            let scale = sx.min(sy).min(1.);
            (scale, scale)
        }
    };
    SurfaceTransform {
        scale_x: fx / density.0,
        scale_y: fy / density.1,
        offset_x: (view.0 - content.0 * fx) / 2.,
        offset_y: (view.1 - content.1 * fy) / 2.,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(t: SurfaceTransform, px: (f32, f32)) -> (f32, f32) {
        (px.0 * t.scale_x + t.offset_x, px.1 * t.scale_y + t.offset_y)
    }

    #[test]
    fn natural_size_until_laid_out() {
        let t = surface_transform(CanvasFit::Fill, (300., 150.), (2., 2.), (0., 0.));
        assert_eq!(at(t, (300., 150.)), (150., 75.));
        let t = surface_transform(CanvasFit::None, (300., 150.), (2., 2.), (400., 400.));
        assert_eq!(at(t, (0., 0.)), (0., 0.));
        assert_eq!(at(t, (300., 150.)), (150., 75.));
    }

    #[test]
    fn fill_stretches_to_the_view() {
        let t = surface_transform(CanvasFit::Fill, (300., 150.), (1.5, 1.5), (400., 300.));
        assert_eq!(at(t, (0., 0.)), (0., 0.));
        assert_eq!(at(t, (300., 150.)), (400., 300.));
    }

    #[test]
    fn fit_x_scales_uniformly_and_centres() {
        // 150x75 DIPs of content in a 300x300 view: 2x, 150 DIPs tall, centred vertically.
        let t = surface_transform(CanvasFit::FitX, (300., 150.), (2., 2.), (300., 300.));
        assert_eq!(at(t, (0., 0.)), (0., 75.));
        assert_eq!(at(t, (300., 150.)), (300., 225.));
    }

    #[test]
    fn fit_y_scales_uniformly_and_centres() {
        let t = surface_transform(CanvasFit::FitY, (100., 100.), (1., 1.), (300., 200.));
        assert_eq!(at(t, (0., 0.)), (50., 0.));
        assert_eq!(at(t, (100., 100.)), (250., 200.));
    }

    #[test]
    fn densities_per_axis() {
        let t = surface_transform(CanvasFit::None, (300., 300.), (1., 2.), (0., 0.));
        assert_eq!(at(t, (300., 300.)), (300., 150.));
    }

    #[test]
    fn scale_down_never_scales_up() {
        let t = surface_transform(CanvasFit::ScaleDown, (100., 100.), (1., 1.), (300., 200.));
        assert_eq!(at(t, (0., 0.)), (100., 50.));
        assert_eq!(at(t, (100., 100.)), (200., 150.));
        let t = surface_transform(CanvasFit::ScaleDown, (400., 200.), (1., 1.), (200., 200.));
        assert_eq!(at(t, (0., 0.)), (0., 50.));
        assert_eq!(at(t, (400., 200.)), (200., 150.));
    }
}
