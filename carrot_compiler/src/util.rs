#[macro_export]
macro_rules! err_str {
    ($value:literal) => {
        Err(($value).to_string())
    };
}

#[macro_export]
macro_rules! return_err {
    ($e:expr) => {
        $e.or_else(|s| return Err(s)).unwrap()
    };
}