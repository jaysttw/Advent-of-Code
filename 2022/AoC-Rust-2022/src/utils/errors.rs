use std::fmt;

#[derive(Debug)]
pub struct InputLengthError {
    expected: usize,
    actual: usize,
    items: String,
}

impl fmt::Display for InputLengthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Incorrect vector length, expected: {}, actual: {}. Culprit: {:?}",
            self.expected,
            self.actual,
            self.items
        )
    }
}

impl InputLengthError {
    pub fn new(expected: usize, actual: usize, msg: String) -> InputLengthError {
        InputLengthError {
            expected: expected,
            actual: actual,
            items: msg,
        }
    }
}