mod logy;


mod add_or_insert;
pub use add_or_insert::AddOrInsert;

mod get_many_mut;
pub use get_many_mut::GetManyMut;

mod ok_or;
pub use ok_or::OkOr;

mod placeholder;

mod pout;

mod pow;
pub use pow::Pow;

mod push_or_insert;
pub use push_or_insert::PushOrInsert;

pub mod unwrap_or_return_mod;

mod unwrap_array;
pub use unwrap_array::UnwrapArray;

mod vecna;
pub use vecna::Vecna;

mod vec;
pub use vec::VecStuff;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn logy() {
        logy!("test", "Foo{}", "Bar");
    }
    #[test]
    fn ok_or() {
        let a: Result<i32, i32> = Err(1);
        let b: Result<i32, &str> =  Err("bob");
        assert_eq!(b, a.ok_or("bob"));
    }

    #[test]
    fn pow_f64_to_f64() {    
        assert_eq!( 16.0, 4_f64.pow(2_f64));
    }
    #[test]
    fn pow_f64_to_i32() {
        assert_eq!( 16.0, 4_f64.pow(2_i32));
    }
    
    #[test]
    fn pow_f32_to_f32() {
    
        assert_eq!( 16.0, 4_f32.pow(2_f32));
    }
    #[test]
    fn pow_f32_to_i32() {
    
        assert_eq!( 16.0, 4_f32.pow(2_i32));
    }
    use crate as qol;
    #[test]
    fn unwrap_or_return_option() {
        let test = "test";
         assert_eq!(test, unwrap_or_return!(Some(test)));
    }
    #[test]
    fn unwrap_or_return_result() {
        let test = "test";
         assert_eq!(test, unwrap_or_return!(Ok::<_,()>(test)));
    }
    #[test]
    fn unwrap_or_return_two() {
        let test = "test";
         assert_eq!(test, 
            unwrap_or_return!(
                Ok::<_,()>(test),
                ()
            )
        );
    }
    #[test]
    fn unwrap_or_return_five() {
        let test = "test";
         assert_eq!(test, 
            unwrap_or_return!(
                Some(test),
                let mut stuff = 1;
                stuff += 1;
                println!("{stuff}"),
                ()
            )
        );
    }
}
