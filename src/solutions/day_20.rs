use crate::Solution;

pub struct Day20;

impl Solution for Day20 {
    fn part_a(&self, input: &str) -> String {
        todo!("Implement part A.\n\nInput file: \n{input}\n\n")
    }

    fn part_b(&self, input: &str) -> String {
        todo!("Implement part B.\n\nInput file: \n{input}\n\n")
    }
}

// -------------------------------------------------- //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load;

    #[test]
    fn test_part_a() {
        let input = load(20, Some(true));
        let result = Day20::part_a(&Day20, &input);

        todo!("Implement test for part A.\n\nResult: {result}")
    }

    #[test]
    fn test_part_b() {
        let input = load(20, Some(true));
        let result = Day20::part_b(&Day20, &input);

        todo!("Implement test for part B.\n\nResult: {result}")
    }
}
