pub trait OkOr<T, E> {
    fn ok_or(self, error: E) -> Result<T, E>;
}

impl<T, E, Q> OkOr<T, E> for Result<T, Q> {
    fn ok_or(self, error: E) -> Result<T, E> {
        match self {
            Ok(ok) => Ok(ok),
            Err(_) => Err(error)
        }
    }
}
