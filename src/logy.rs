
#[macro_export]
macro_rules! logy {
    ($lvl:expr, $($arg:tt)*) => {
        #[cfg(feature = $lvl)]
        if file!().starts_with(env!("CARGO_MANIFEST_DIR")) {
            println!("[{}:{}:{}:{}]{}",
                $lvl,
                env!("CARGO_CRATE_NAME"), 
                {
                    const x: usize = env!("CARGO_MANIFEST_DIR").len();
                    &(file!()[x..])
                },
                line!(), 
                format!($($arg)*)
            )
        } else {
            println!("[{}:{}:{}]{}",
                $lvl,
                file!(),
                line!(), 
                format!($($arg)*)
            )
        }
    };
}