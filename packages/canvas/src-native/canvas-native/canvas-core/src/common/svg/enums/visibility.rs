#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Visible
    }
}

impl From<&str> for Visibility {
    fn from(val: &str) -> Self {
        match val {
            "hidden" => Visibility::Hidden,
            "collapse" => Visibility::Collapse,
            _ => Visibility::Visible,
        }
    }
}
