
#[macro_export]
macro_rules! logy {
    ($lvl:expr, $($arg:tt)*) => {
        #[cfg(feature = $lvl)]
        println!("[{}:{}{}:{}]{}", 
        $lvl, 
        env!("CARGO_CRATE_NAME"), 
        {
            if file!().starts_with(env!("CARGO_MANIFEST_DIR")) {
                let x = env!("CARGO_MANIFEST_DIR").len();
                &(file!()[x..])
            } else {
                concat!("/", file!())
            }
        }, 
        line!(), 
        format!($($arg)*)
    )
    };
}
