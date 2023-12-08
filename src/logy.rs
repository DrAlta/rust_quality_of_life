#[macro_export]
macro_rules! logy {
    ($lvl:expr, $($arg:tt)*) => {
        #[cfg(feature = $lvl)]
        println!("[{}:{}:{}]{}", $lvl, file!(), line!(), format!($($arg)*));
    };
}
