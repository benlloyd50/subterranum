#[derive(Clone)]
pub struct Message {
    pub contents: String,
    pub turn_sent: usize,
}

impl Message {
    pub fn new(contents: String, turn_sent: usize) -> Self {
        Self { contents, turn_sent }
    }
}
