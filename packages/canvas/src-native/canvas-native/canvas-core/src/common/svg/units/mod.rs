pub mod length;
pub mod stream;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Units {
    ObjectBoundingBox,
    UserSpaceOnUse,
}

impl Units {
    pub fn from_str(val: &str) -> Option<Self> {
        match val {
            "objectBoundingBox" => Some(Self::ObjectBoundingBox),
            "userSpaceOnUse" => Some(Self::UserSpaceOnUse),
            _ => None,
        }
    }
}
