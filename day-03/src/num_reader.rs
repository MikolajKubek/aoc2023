pub mod num_reader {
    #[derive(Debug)]
    pub struct Number {
        pub value: u32,
        pub indices: Vec<[usize; 2]>
    }

    impl Number {
        pub fn new(value: u32, indices: &Vec<[usize; 2]>) -> Self {
            let mut new_indices = vec![];
            indices.into_iter().for_each(|index| new_indices.push(*index));
            Self {
                value,
                indices: new_indices,
            }
        }
    }

    pub struct NumReader {
        is_reading: bool,
        buffer: String,
        indices: Vec<[usize; 2]>,
        numbers: Vec<Number>,
    }

    impl NumReader {
        pub fn new() -> Self {
            Self {
                is_reading: false,
                buffer: "".to_owned(),
                indices: vec![],
                numbers: vec![],
            }
        }

        pub fn read_char(&mut self, value: char, x: usize, y: usize) {
            if !self.is_reading {
                self.is_reading = true;
                self.buffer = "".to_owned();
                self.indices = vec![];
            }
            self.buffer.push(value);
            self.indices.push([x, y]);
        }

        // finish_reading doesn't clean the buffer and list of indices.
        // It's done in read_char when new reading process starts to make potential
        // investigations easier.
        pub fn finish_reading(&mut self) {
            if self.is_reading {
                self.is_reading = false;
                let buffer_u32 = self.buffer.parse::<u32>().expect("unable to parse buffer to u32");
                self.numbers.push(Number::new(buffer_u32, &self.indices));
            }
        }

        pub fn get_numbers(&self) -> &Vec<Number> {
            &self.numbers
        }
    }
}
