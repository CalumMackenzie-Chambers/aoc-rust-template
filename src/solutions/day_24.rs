use crate::Solution;

pub struct Day24;

impl Solution for Day24 {
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
        let input = load(24, Some(true));
        let result = Day24::part_a(&Day24, &input);

        todo!("Implement test for part A.\n\nResult: {result}")
    }

    #[test]
    fn test_part_b() {
        let input = load(24, Some(true));
        let result = Day24::part_b(&Day24, &input);

        todo!("Implement test for part B.\n\nResult: {result}")
    }
}
