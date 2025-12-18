#[macro_export]
macro_rules! assert_specimen {
    ($specimen:expr, $expected:expr) => {
        {
            let specimen = $specimen;
            let expected = $expected;
            if specimen != expected {
                panic!("Got:\n{specimen:?}\nExpected:\n{expected:?}");
            }
        }
    };
}
