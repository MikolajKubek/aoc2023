pub mod seed_mapping {
    #[derive(Debug)]
    pub struct SeedMap {
        ranges: Vec<Range>,
    }

    impl SeedMap {
        pub fn new() -> Self {
            Self {
                ranges: vec![],
            }
        }

        pub fn add_range(&mut self, destination_start: u64, source_start: u64, range_lenght: u64) {
            self.ranges.push(Range::new(destination_start, source_start, range_lenght));
        }

        pub fn get_destination(&self, value: u64) -> u64 {
            for range in &self.ranges {
                match range.get_destination(value) {
                    Some(value) => {return value;}
                    None => {}
                }
            }
            value
        }
    }

    #[derive(Debug)]
    struct Range {
        destination_start: u64,
        source_start: u64,
        range_lenght: u64,
    }

    impl Range {
        pub fn new(destination_start: u64, source_start: u64, range_lenght: u64) -> Self {
            Self {
                destination_start,
                source_start,
                range_lenght,
            }
        }

        fn contains(&self, value: u64) -> bool {
            value >= self.source_start && value < self.source_start + self.range_lenght
        }

        pub fn get_destination(&self, value: u64) -> Option<u64> {
            if self.contains(value) {
                return Some(value - self.source_start + self.destination_start);
            }

            None
        }
    }

}
