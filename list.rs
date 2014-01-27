pub enum List<T> {
    Cons(T, ~List<T>),
    Nil
}

pub fn prepend<T>(xs: List<T>, value: T) -> List<T> {
    Cons(value, ~xs)
}

pub fn add_range<T>(dest: List<T>, range: List<T>) -> List<T> {
	let mut current = range;
	let mut new_list = dest;

	loop {
		match current {
			Cons(x, ~next) => {
				new_list = prepend(new_list, x);
				current = next;
			},
			_ => break
		}
	}

	new_list
}