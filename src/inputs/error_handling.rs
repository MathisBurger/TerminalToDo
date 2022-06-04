use std::io::Result;

pub trait InputErrorHandling {
    fn handle_select_error(&mut self, input: Result<Option<usize>>) -> Option<usize>;
}