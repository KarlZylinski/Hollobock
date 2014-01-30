use rsfml::system::Vector2f;
use std::num;

pub fn normalize(vec: Vector2f) -> Vector2f {
	if vec.x != 0. || vec.y != 0. {
		let length = num::sqrt((vec.x * vec.x) + (vec.y * vec.y));
		return Vector2f::new(vec.x / length, vec.y / length);
	} else {
		return vec;
	}
}