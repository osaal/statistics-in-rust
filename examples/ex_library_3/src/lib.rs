#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod v1 {
    use std::error::Error;
    use std::fmt::Display;
    // ANCHOR: v1
    pub fn mean(x: Vec<usize>) -> Result<f64, MeanError> {
        let length = x.len();

        if length == 0 {
            return Err(MeanError::DivideByZero);
        }

        let mut tally = 0;
        #[allow(clippy::needless_range_loop)]
        for i in 0..length {
            tally += x[i];
        }
        let result = tally as f64 / length as f64;

        Ok(result)
    }
    // ANCHOR_END: v1
    #[derive(Debug, PartialEq)]
    pub enum MeanError {
        DivideByZero,
    }
    impl Error for MeanError {}
    impl Display for MeanError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MeanError::DivideByZero => write!(f, "Input vector is length 0"),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_mean() {
            let input: Vec<usize> = vec![1, 2, 3];
            let output = mean(input);

            assert_eq!(output.unwrap(), 2.)
        }

        #[test]
        fn divide_by_zero() {
            let input: Vec<usize> = vec![];
            let output = mean(input);

            assert!(output.is_err());
            assert_eq!(output.unwrap_err(), MeanError::DivideByZero,)
        }
    }
}

mod v2 {
    use std::error::Error;
    use std::fmt::Display;
    // ANCHOR: v2
    pub fn mean(x: Vec<usize>) -> Result<f64, MeanError> {
        let length = x.len();

        if length == 0 {
            return Err(MeanError::DivideByZero);
        }

        let tally = x.into_iter().fold(0, |acc, el| acc + el);

        let result = tally as f64 / length as f64;

        Ok(result)
    }
    // ANCHOR_END: v2
    #[derive(Debug, PartialEq)]
    pub enum MeanError {
        DivideByZero,
    }
    impl Error for MeanError {}
    impl Display for MeanError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MeanError::DivideByZero => write!(f, "Input vector is length 0"),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_mean() {
            let input: Vec<usize> = vec![1, 2, 3];
            let output = mean(input);

            assert_eq!(output.unwrap(), 2.)
        }

        #[test]
        fn divide_by_zero() {
            let input: Vec<usize> = vec![];
            let output = mean(input);

            assert!(output.is_err());
            assert_eq!(output.unwrap_err(), MeanError::DivideByZero,)
        }
    }
}
