pub mod matrix {
    use std::collections::HashSet;

    pub struct Matrix {
        rows: usize,
        cols: usize,
        data: Vec<char>
    }

    pub trait MatrixTrait<T> {
        fn get(&self, x: usize, y: usize) -> T;
        fn set(&mut self, x: usize, y: usize, value: T);
        fn get_neighbour_set(&self, x: usize, y: usize) -> HashSet<T>;
    }

    impl Matrix {
        pub fn new(rows: usize, cols: usize) -> Self {
            Self {
                rows,
                cols,
                data: vec!['.'; rows*cols]
            }
        }

        fn get_symbol_neighbour(&self, x: i32, y: i32) -> Option<char> {
            if x > 0 && x < self.cols as i32 && y > 0 && y < self.rows as i32 {
                let value = self.get(x as usize, y as usize);
                if !value.is_alphanumeric() && value != '.' {
                    return Some(value);
                }
            }

            None
        }

    }

    impl MatrixTrait<char> for Matrix {
        fn set(&mut self, x: usize, y: usize, value: char) {
            let index = y * self.cols + x;
            self.data[index] = value;
        }

        fn get(&self, x: usize, y: usize) -> char {
            let index = y * self.cols + x;
            self.data[index]
        }

        fn get_neighbour_set(&self, x: usize, y: usize) -> HashSet<char> {
            let x = x as i32;
            let y = y as i32;
            let mut neighbours_set: HashSet<char> = HashSet::new();


            if let Some(neigbour) = self.get_symbol_neighbour(x - 1, y) {
                neighbours_set.insert(neigbour);
            }
            if let Some(neighbour) = self.get_symbol_neighbour(x - 1, y) {
                neighbours_set.insert(neighbour);
            }
            if let Some(neighbour) = self.get_symbol_neighbour(x - 1, y - 1) {
                neighbours_set.insert(neighbour);
            }
            if let Some(neighbour) = self.get_symbol_neighbour(x - 1, y + 1) {
                neighbours_set.insert(neighbour);
            }
            if let Some(neighbour) = self.get_symbol_neighbour(x, y - 1) {
                neighbours_set.insert(neighbour);
            }
            if let Some(neighbour) = self.get_symbol_neighbour(x, y + 1) {
                neighbours_set.insert(neighbour);
            }
            if let Some(neighbour) = self.get_symbol_neighbour(x + 1, y) {
                neighbours_set.insert(neighbour);
            }
            if let Some(neighbour) = self.get_symbol_neighbour(x + 1, y - 1) {
                neighbours_set.insert(neighbour);
            }
            if let Some(neighbour) = self.get_symbol_neighbour(x + 1, y + 1) {
                neighbours_set.insert(neighbour);
            }

            neighbours_set
        }
    }

}
