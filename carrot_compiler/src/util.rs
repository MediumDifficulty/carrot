pub trait ErrStr {
    fn to_err<T>(&self) -> Result<T, String>;
}

impl ErrStr for str {
    fn to_err<T>(&self) -> Result<T, String> {
        Err(self.to_string())
    }
}