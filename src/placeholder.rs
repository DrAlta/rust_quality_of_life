#[macro_export]
macro_rules! placeholder {
    ($e:expr) => {
        {
            let exp= $e;
            eprintln!("[{}:{}]using placeholder value{:?}!", file!(), line!(), exp);
            exp
        }
    };
}
