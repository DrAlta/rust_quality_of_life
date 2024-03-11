// this is just a wrapper around println!() so that I can search for println1!()
// to find all my stuff that put in to help develop
// I should use logy!() for that but thten I have to setup the dependancea and 
// features

#[macro_export]
macro_rules! pout {
    ($($arg:tt)*) => { 
        println!($($arg)*)
     };
}
