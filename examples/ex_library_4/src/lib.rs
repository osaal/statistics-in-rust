#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod v1 {
    use std::error::Error;
    use std::fmt::Display;
    // ANCHOR: v1-1
    pub fn mean(x: Vec<usize>) -> Result<f64, MeanError> {
        let length = x.len();

        if length == 0 {
            return Err(MeanError::DivideByZero);
        }

        let tally = x.into_iter().fold(0, |acc, el| acc + el);

        let result = tally as f64 / length as f64;

        Ok(result)
    }
    // ANCHOR_END: v1-1
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

    // ANCHOR: v1-2
    fn do_something<T>(x: T)
    where
        T: std::fmt::Debug,
    {
        // Do something
    }
    // ANCHOR_END: v1-2

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
    pub fn mean<T>(x: T) -> Result<f64, MeanError>
    where
        T: IntoIterator<Item = usize> + Clone,
    {
        let data = x.clone();
        let length = x.into_iter().count();

        if length == 0 {
            return Err(MeanError::DivideByZero);
        }

        let tally = data.into_iter().fold(0, |acc, el| acc + el);

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

mod v3 {
    use std::error::Error;
    use std::fmt::Display;

    pub fn mean<T>(x: T) -> Result<f64, MeanError>
    where
        T: IntoIterator<Item = usize> + Clone,
    {
        let data = x.clone();
        let length = x.into_iter().count();

        if length == 0 {
            return Err(MeanError::DivideByZero);
        }

        let tally = data.into_iter().fold(0, |acc, el| acc + el);

        let result = tally as f64 / length as f64;

        Ok(result)
    }

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

        // ANCHOR: v3
        #[test]
        fn generic_input() {
            let input_1: Vec<usize> = vec![1, 2, 3];
            let input_2: [usize; 3] = [1, 2, 3];

            let output_1 = mean(input_1);
            let output_2 = mean(input_2);

            assert_eq!(output_1.unwrap(), output_2.unwrap())
        }
        // ANCHOR_END: v3
    }
}

// This module is the final code version, as embedded in the book.
mod v4 {
    // ANCHOR: v4
    use std::error::Error;
    use std::fmt::Display;

    pub fn mean<T>(x: T) -> Result<f64, MeanError>
    where
        T: IntoIterator<Item = usize> + Clone,
    {
        let data = x.clone();
        let length = x.into_iter().count();

        if length == 0 {
            return Err(MeanError::DivideByZero);
        }

        let tally = data.into_iter().fold(0, |acc, el| acc + el);

        let result = tally as f64 / length as f64;

        Ok(result)
    }

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

        #[test]
        fn generic_input() {
            let input_1: Vec<usize> = vec![1, 2, 3];
            let input_2: [usize; 3] = [1, 2, 3];

            let output_1 = mean(input_1);
            let output_2 = mean(input_2);

            assert_eq!(output_1.unwrap(), output_2.unwrap())
        }
    }
    // ANCHOR_END: v4
}
