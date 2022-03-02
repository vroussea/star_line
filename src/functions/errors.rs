use std::io::Error;

#[derive(Debug, Clone)]
pub struct CustomError {
    message: String,
}


// impl From<ParsingIntError> for CustomError{

// }

impl From<Error> for CustomError {
    fn from(e: Error) -> CustomError {
        CustomError{message: format!("Generic IO error: {}", e.to_string())}
    }
}

impl From<std::num::ParseIntError> for CustomError {
    fn from(e: std::num::ParseIntError) -> CustomError {
        CustomError{message: format!("Invalid data type: {}", e.to_string())}
    }
}

impl From<&str> for CustomError {
    fn from(e: &str) -> CustomError {
        CustomError{message: e.to_string()}
    }
}