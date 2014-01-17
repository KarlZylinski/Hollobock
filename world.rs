pub mod entity;
pub mod list;

pub struct World
{
	entities: ~list::List<entity::Entity>
}