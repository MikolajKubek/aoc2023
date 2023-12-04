pub mod card {
    pub struct Card {
        pub id: usize,
        pub score: u32,
        pub num_winning: u32,
    }

    impl Card {
        pub fn new(id: usize, score: u32, num_winning: u32) -> Self {
            Self {
                id,
                score,
                num_winning,
            }
        }
    }
}
