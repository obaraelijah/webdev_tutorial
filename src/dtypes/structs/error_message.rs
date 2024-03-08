use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct ErrorMessage {
    pub message: String,
}

impl ErrorMessage {
    // Constructor for ErrorMessage
    pub fn new(msg: &str) -> Self {
        ErrorMessage {
            message: msg.to_string(),
        }
    }

    // A method to retrieve the message if you need it
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::fmt::Debug for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ErrorMessage: {}", self.message)
    }
}

impl Error for ErrorMessage {}
