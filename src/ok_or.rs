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


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn ok_or() {
        let a: Result<i32, i32> = Err(1);
        let b: Result<i32, &str> =  Err("bob");
        assert_eq!(b, a.ok_or("bob"));
    }
}