#[derive(Clone)]
pub struct Message {
    pub contents: String,
    _turn_sent: usize,
}

impl Message {
    pub fn new(contents: String) -> Self {
        Self {
            contents,
            _turn_sent: 0,
        }
    }
}
