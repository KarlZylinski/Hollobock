use rsfml::graphics::RenderWindow;
use input::Input;

use std::cast::transmute;
use std::unstable::intrinsics::TypeId;
use entity::world::World;

pub mod player;
pub mod player_bullet;
pub mod enemy;
pub mod enemy_spawner;
pub mod world;

pub trait Entity: Any {
	fn update(&self, dt: f32, world: &World, input: &Input) -> EntityUpdateResult;
	fn draw(&self, window: &mut RenderWindow);
	fn clone(&self) -> ~Entity:;
}

impl Clone for ~Entity: {
	fn clone(&self) -> ~Entity: {
		return self.clone();
	}
}

impl<'a> AnyRefExt<'a> for &'a Entity {
    #[inline]
    fn is<T: 'static>(self) -> bool {
        // Get TypeId of the type this function is instantiated with
        let t = TypeId::of::<T>();

        // Get TypeId of the type in the trait object
        let boxed = self.get_type_id();

        // Compare both TypeIds on equality
        t == boxed
    }

    #[inline]
    fn as_ref<T: 'static>(self) -> Option<&'a T> {
        if self.is::<T>() {
            Some(unsafe { transmute(self.as_void_ptr()) })
        } else {
            None
        }
    }
}

pub struct EntityUpdateResult {
	new_entities: ~[~Entity:]
	// TODO: Add events here.
}
