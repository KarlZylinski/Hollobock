pub enum List<T> {
    Cons(T, ~List<T>),
    Nil
}

pub fn prepend<T>(xs: List<T>, value: T) -> List<T> {
    Cons(value, ~xs)
}

pub fn concat<T>(to: List<T>, from: List<T>) -> List<T> {
	match from {        
		Cons(x, ~next) => {
			let new_list = prepend(to, x);
			return concat(new_list, next);
		},
		_ => return to
	}
}