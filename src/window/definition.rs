#[derive(Debug)]
pub struct Definition {
    pub title: String,
    pub width: u32,
    pub height: u32
}

impl Definition {
    pub(crate) fn build(title: String, width: u32, height: u32) -> Self {
        Definition { title, width, height }
    }
}
