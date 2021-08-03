use tokio::time::Duration;

pub struct Map {
    pub rows: Vec<[bool; 4]>,
    pub bpm: f32,
    pub base_gap: usize,
    pub length: Duration,
}
