use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub fullscreen: bool,
    pub dev_mode: bool,
    pub font_file: String,
    pub font_size: usize,
    pub screensize_x: usize,
    pub screensize_y: usize,
    pub world_seed: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fullscreen: false,
            dev_mode: false,
            font_file: "Yayo.png".to_string(),
            font_size: 8,
            screensize_x: 120,
            screensize_y: 80,
            world_seed: 1,
        }
    }
}
