#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod v1 {
    use std::error::Error;
    use std::fmt::Display;

    pub fn mean(x: Vec<usize>) -> Result<f64, MeanError> {
        let length = x.len();
        let mut tally = 0;
        for i in 0..length {
            tally += x[i];
        }
        let result = tally as f64 / length as f64;
        Ok(result)
    }

    #[derive(Debug)]
    pub enum MeanError {}
    impl Error for MeanError {}
    impl Display for MeanError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    // ANCHOR: v1
    #[cfg(test)]
    mod tests {
        use super::*;

        // Test functions go here.
    }
    // ANCHOR_END: v1
}

mod v2 {
    use std::error::Error;
    use std::fmt::Display;

    pub fn mean(x: Vec<usize>) -> Result<f64, MeanError> {
        let length = x.len();
        let mut tally = 0;
        for i in 0..length {
            tally += x[i];
        }
        let result = tally as f64 / length as f64;
        Ok(result)
    }

    #[derive(Debug)]
    pub enum MeanError {}
    impl Error for MeanError {}
    impl Display for MeanError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    // ANCHOR: v2
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_mean() {
            let input: Vec<usize> = vec![1, 2, 3];
            let output = mean(input);

            assert_eq!(output.unwrap(), 2.)
        }
    }
    // ANCHOR_END: v2
}

mod v3 {
    use std::error::Error;
    use std::fmt::Display;

    // Unfortunately, this is one of the few spots where
    // we cannot reproduce the test failure in the book
    // without having test failures in subsequent sections.
    // We thus have to retroactively fix the function...
    pub fn mean(x: Vec<usize>) -> Result<f64, MeanError> {
        let length = x.len();
        if length == 0 {
            return Err(MeanError::DivideByZero);
        }
        let mut tally = 0;
        for i in 0..length {
            tally += x[i];
        }
        let result = tally as f64 / length as f64;
        Ok(result)
    }

    // ANCHOR: v3-2
    // ...
    // `./src/lib.rs`
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
    // ...
    // ANCHOR_END: v3-2

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_mean() {
            let input: Vec<usize> = vec![1, 2, 3];
            let output = mean(input);

            assert_eq!(output.unwrap(), 2.)
        }
        // ANCHOR: v3-1
        // ...
        // `./src/lib.rs`
        // > Inside `mod tests`, under `fn test_mean()`
        #[test]
        fn divide_by_zero() {
            let input: Vec<usize> = vec![];
            let output = mean(input);

            assert!(output.is_err());
            assert_eq!(output.unwrap_err(), MeanError::DivideByZero,)
        }
        // ...
        // ANCHOR_END: v3-1
    }
}

mod v4 {
    use std::error::Error;
    use std::fmt::Display;

    // ANCHOR: v4
    pub fn mean(x: Vec<usize>) -> Result<f64, MeanError> {
        let length = x.len();

        if length == 0 {
            return Err(MeanError::DivideByZero);
        }

        let mut tally = 0;
        for i in 0..length {
            tally += x[i];
        }
        let result = tally as f64 / length as f64;

        Ok(result)
    }
    // ANCHOR_END: v4
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
