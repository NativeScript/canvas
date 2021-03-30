use crate::common::svg::elements::filter::filter_in::FilterIn;

#[derive(Copy, Clone, Debug)]
pub enum FilterElement {
    FeGaussianBlur {
        std_deviation: f32,
        in_val: FilterIn,
    },
}
