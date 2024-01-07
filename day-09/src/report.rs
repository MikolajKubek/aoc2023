pub struct ValueHistory {
    values: Vec<i32>,
    pub extrapolation: Option<i32>,
}

struct IncreaseResult {
    values: Vec<i32>,
    is_all_zeros: bool
}

impl ValueHistory {
    pub fn new(values: &Vec<i32>) -> Self {
        ValueHistory {
            values: values.clone(),
            extrapolation: None
        }
    }

    pub fn calculate_right_extrapolation(&self) -> i32 {
        let mut final_values_sum = *self.values.last().unwrap();
        let mut increase_result = ValueHistory::calculate_increase(&self.values);
        final_values_sum += increase_result.values.last().unwrap();

        while !increase_result.is_all_zeros {
            let values = increase_result.values;
            increase_result = ValueHistory::calculate_increase(&values);
            final_values_sum += increase_result.values.last().unwrap();
        }

        final_values_sum
    }

    pub fn calculate_left_extrapolation(&self) -> i32 {
        let mut first_values_queue: Vec<i32> = vec![];
        first_values_queue.push(*self.values.first().unwrap());
        let mut increase_result = ValueHistory::calculate_increase(&self.values);
        first_values_queue.push(*increase_result.values.first().unwrap());

        while !increase_result.is_all_zeros {
            let values = increase_result.values;
            increase_result = ValueHistory::calculate_increase(&values);
            first_values_queue.push(*increase_result.values.first().unwrap());
        }

        let mut extrapolation = 0;
        while let Some(value) = first_values_queue.pop() {
            extrapolation = value - extrapolation;
        }

        extrapolation
    }

    fn calculate_increase(values: &Vec<i32>) -> IncreaseResult {
        let mut is_all_zeros = true;
        let mut increases: Vec<i32> = vec![];

        for i in 1..values.len() {
            let increase = values[i] - values[i - 1];
            increases.push(increase);
            if is_all_zeros && increase != 0 {
                is_all_zeros = false;
            }
        }
        
        IncreaseResult { values: increases, is_all_zeros }
    }
}
