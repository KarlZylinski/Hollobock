use rsfml::graphics::RenderWindow;
use rsfml::system::Vector2f;
use input::Input;
use entity::world::World;

pub mod player;
pub mod player_bullet;
pub mod enemy;
pub mod enemy_spawner;
pub mod world;
pub mod renderer;
pub mod sprite_renderer;
pub mod animated_renderer;

pub trait EntityTrait {
    fn update(&self, dt: f32, world: &World, input: &Input) -> EntityUpdateResult;
    fn draw(&self, window: &mut RenderWindow);
    fn position(&self) -> Vector2f;
    fn clone(&self) -> Entity;
}

pub enum Entity {
    Player(~player::PlayerStruct),
    Enemy(~enemy::EnemyStruct),
    EnemySpawner(~enemy_spawner::EnemySpawnerStruct),
    PlayerBullet(~player_bullet::PlayerBulletStruct)
}

impl Clone for Entity {
    fn clone(&self) -> Entity {
        return self.clone();
    }
}

impl Entity {
    fn as_entity_trait<'r>(&'r self) -> &'r EntityTrait: {
        match self {
            &Player(~ref e) => e as &EntityTrait:,
            &Enemy(~ref e) => e as &EntityTrait:,
            &EnemySpawner(~ref e) => e as &EntityTrait:,
            &PlayerBullet(~ref e) => e as &EntityTrait:
        }
    }

    pub fn update(&self, dt: f32, world: &World, input: &Input) -> EntityUpdateResult {
        self.as_entity_trait().update(dt, world, input)
    }

    pub fn draw(&self, window: &mut RenderWindow) {
        self.as_entity_trait().draw(window);
    }

    pub fn position(&self) -> Vector2f {
        self.as_entity_trait().position()
    }

    pub fn clone(&self) -> Entity {
        self.as_entity_trait().clone()
    }
}

pub struct EntityUpdateResult {
    new_entities: ~[Entity]
}
