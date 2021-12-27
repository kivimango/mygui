#[derive(Copy, Clone, Debug)]
pub struct Constraints {
    pub min_width: u32,
    pub max_width: u32,
    pub min_height: u32,
    pub max_height: u32
}

impl Default for Constraints {
    fn default() -> Self {
        Constraints {
            min_width: 0,
            max_width: 65535,
            min_height: 0,
            max_height: 65535
        }
    }
}