use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub fullscreen: bool,
    pub dev_mode: bool,
    pub font_file: String,
    pub font_size: usize,
    pub screensize_x: usize,
    pub screensize_y: usize,
    pub map_x: usize,
    pub map_y: usize,
    pub world_seed: u64,
}
