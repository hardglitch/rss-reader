pub(crate) fn unbox<T>(value: Box<T>) -> T {
    Box::into_inner(value)
}
