use crate::uncertain_float::UncertainFloat;

struct TestCase {
    input1: UncertainFloat,
    input2: UncertainFloat,
    expected_output: UncertainFloat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let epsilon = 1e-9;

        let test_cases = vec![
            TestCase {
                input1: UncertainFloat::new(1.0, 0.1),
                input2: UncertainFloat::new(2.0, 0.2),
                expected_output: UncertainFloat::new(3.0, 0.223606797749979),
            },
            TestCase {
                input1: UncertainFloat::new(0.001, 0.00001),
                input2: UncertainFloat::new(1092.883, 0.3882983),
                expected_output: UncertainFloat::new(1092.884, 0.388298300128767),
            },
            TestCase {
                input1: UncertainFloat::new(9983838884.838, 0.00000000192),
                input2: UncertainFloat::new(0.0, 0.0),
                expected_output: UncertainFloat::new(9983838884.838, 0.00000000192), 
            },
        ];

        for case in test_cases {
            let result = case.input1 + case.input2;
            assert!((result.value - case.expected_output.value).abs() < epsilon);
            assert!((result.uncertainty - case.expected_output.uncertainty).abs() < epsilon);
        }
    }
}
