// this is just a wrapper around println!() so that I can search for println1!()
// to find all my stuff that put in to help develop
// I should use logy!() for that but thten I have to setup the dependancea and 
// features

#[cfg(not(feature = "pout-debug"))]
#[macro_export]
macro_rules! pout {
    ($($arg:tt)*) => { 
        println!($($arg)*)
     };
}


#[cfg(feature = "pout-debug")]
#[macro_export]
macro_rules! pout {
    ($($arg:tt)*) => {
        if file!().starts_with(env!("CARGO_MANIFEST_DIR")) {
            println!("[{}:{}:{}]{}",
                env!("CARGO_CRATE_NAME"), 
                {
                    const LEN: usize = env!("CARGO_MANIFEST_DIR").len();
                    &(file!()[LEN..])
                },
                line!(), 
                format!($($arg)*)
            )
        } else {
            println!("[{}:{}]{}",
                file!(),
                line!(), 
                format!($($arg)*)
            )
        }
    };
}
