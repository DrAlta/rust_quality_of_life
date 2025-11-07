pub trait MaybeAs {
    fn maybe_as<O>(self) ->Result<O, <Self as TryInto<O>>::Error>
    where Self: TryInto<O>{
        self.try_into()
    }
}
impl<I> MaybeAs for I {
}