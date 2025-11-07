pub trait TryAs {
    fn try_as<O>(self) ->Result<O, <Self as TryInto<O>>::Error>
    where Self: TryInto<O>{
        self.try_into()
    }
}
impl<I> TryAs for I {
}