pub trait AsA {
    fn as_a<O>(self) ->O
    where Self: Into<O>{
        self.into()
    }
}
impl<I> AsA for I {
}