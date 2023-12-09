pub struct Race {
    pub time: u64,
    pub record_distance: u64,
}

impl Race {
    pub fn new(time: u64, record_distance: u64) -> Self {
        Self {
            time,
            record_distance,
        }
    }
}
