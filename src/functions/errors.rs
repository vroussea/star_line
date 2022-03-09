use std::io::Error;

#[derive(Debug, Clone)]
pub struct CustomError {
    message: String,
}

impl From<Error> for CustomError {
    fn from(e: Error) -> CustomError {
        CustomError {
            message: format!("Generic IO error: {}", e.to_string()),
        }
    }
}

impl From<&str> for CustomError {
    fn from(e: &str) -> CustomError {
        CustomError {
            message: e.to_string(),
        }
    }
}
