
#[macro_export]
macro_rules! logy {
    ($lvl:expr, $($arg:tt)*) => {
        #[cfg(feature = $lvl)]
        if file!().starts_with(env!("CARGO_MANIFEST_DIR")) {
            println!("[{}:{}:{}:{}]{}",
                $lvl,
                env!("CARGO_CRATE_NAME"), 
                {
                    const LEN: usize = env!("CARGO_MANIFEST_DIR").len();
                    &(file!()[LEN..])
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
#[test]
fn test(){
    logy!("errors", "Foo{}", "Bar");
}