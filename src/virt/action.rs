use super::kvm::core::Run;
use error::*;

#[derive(Debug)]
pub enum Action {
    Continue,
    Replace(Run),
    Err(Error),
}

impl<T> From<Result<T>> for Action {
    fn from(result: Result<T>) -> Action {
        match result {
            Ok(_) => Action::Continue,
            Err(e) => Action::Err(e)
        }
    }
}
