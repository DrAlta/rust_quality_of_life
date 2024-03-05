
#[macro_export]
macro_rules! pout {
    ($($arg:tt)*) => { 
        println!($($arg)*)
     };
}
