use std::error::Error;
use std::io::Result;

/// Defines the base methods that must be implemented
/// if input errors will be handled
pub trait InputErrorHandling {
    /// Handles the input error
    fn handle_select_error(&mut self, input: std::io::Result<Option<usize>>) -> Option<usize> {
        match input {
            Err(e) => {
                println!("ERROR: {}", e.description());
                None
            }
            Ok(val) => val,
        }
    }
}
