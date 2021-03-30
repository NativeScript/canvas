#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Display {
    None,
    Inline,
    Inherit,
}

impl Default for Display {
    fn default() -> Self {
        Self::Inline
    }
}

impl From<&str> for Display {
    fn from(val: &str) -> Self {
        match val {
            "none" => Display::None,
            "inherit" => Display::Inherit,
            _ => Display::Inline,
        }
    }
}
