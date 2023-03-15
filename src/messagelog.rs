pub struct Message {
    pub contents: String,
    turn_sent: usize,
}

impl Message {
    pub fn new(contents: String) -> Self {
        Self {
            contents,
            turn_sent: 0,
        }
    }
}
