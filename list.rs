pub enum List<T> {
    Cons(T, ~List<T>),
    Nil
}

fn prepend<T>(xs: List<T>, value: T) -> List<T> {
    Cons(value, ~xs)
}