#![allow(dead_code)]

mod v1 {
    // ANCHOR: v1
    pub fn mean(x: Vec<usize>) {}
    // ANCHOR_END: v1
}

mod v2 {
    // ANCHOR: v2
    pub fn mean(x: Vec<usize>) -> f64 {
        0.
    }
    // ANCHOR_END: v2
}

mod v3 {
    // ANCHOR: v3
    use std::error::Error;
    use std::fmt::Display;

    pub fn mean(x: Vec<usize>) -> Result<f64, MeanError> {
        Ok(0.)
    }

    #[derive(Debug)]
    pub enum MeanError {}
    impl Error for MeanError {}
    impl Display for MeanError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
    // ANCHOR_END: v3
}

mod v4 {
    use std::error::Error;
    use std::fmt::Display;

    // ANCHOR: v4
    pub fn mean(x: Vec<usize>) -> Result<f64, MeanError> {
        let length = x.len();
        Ok(0.)
    }
    // ANCHOR_END: v4

    #[derive(Debug)]
    pub enum MeanError {}
    impl Error for MeanError {}
    impl Display for MeanError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
}

mod v5 {
    use std::error::Error;
    use std::fmt::Display;

    // ANCHOR: v5
    pub fn mean(x: Vec<usize>) -> Result<f64, MeanError> {
        let length = x.len();
        for i in 0..length - 1 {
            // Do something with x[i]?
        }
        Ok(0.)
    }
    // ANCHOR_END: v5

    #[derive(Debug)]
    pub enum MeanError {}
    impl Error for MeanError {}
    impl Display for MeanError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
}

mod v6 {
    use std::error::Error;
    use std::fmt::Display;

    // ANCHOR: v6
    pub fn mean(x: Vec<usize>) -> Result<f64, MeanError> {
        let length = x.len();
        let mut tally = 0;
        for i in 0..length - 1 {
            tally += x[i];
        }
        Ok(0.)
    }
    // ANCHOR_END: v6

    #[derive(Debug)]
    pub enum MeanError {}
    impl Error for MeanError {}
    impl Display for MeanError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
}

mod v7 {
    use std::error::Error;
    use std::fmt::Display;

    // ANCHOR: v7
    pub fn mean(x: Vec<usize>) -> Result<f64, MeanError> {
        let length = x.len();
        let mut tally = 0;
        for i in 0..length - 1 {
            tally += x[i];
        }
        let result = tally as f64 / length as f64;
        Ok(result)
    }
    // ANCHOR_END: v7

    #[derive(Debug)]
    pub enum MeanError {}
    impl Error for MeanError {}
    impl Display for MeanError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
}
