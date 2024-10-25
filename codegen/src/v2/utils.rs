pub fn o<T: ToOwned + ?Sized>(x: &T) -> T::Owned {
    x.to_owned()
}
