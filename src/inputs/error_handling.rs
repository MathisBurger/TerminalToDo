use std::io::Result;

/// Defines the base methods that must be implemented
/// if input errors will be handled
pub trait InputErrorHandling {
    fn handle_select_error(&mut self, input: Result<Option<usize>>) -> Option<usize>;
}
