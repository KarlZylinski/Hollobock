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

pub enum Entity {
    Player(player::PlayerStruct),
    Enemy(enemy::EnemyStruct),
    EnemySpawner(enemy_spawner::EnemySpawnerStruct),
    PlayerBullet(player_bullet::PlayerBulletStruct)
}

impl Clone for Entity {
    fn clone(&self) -> Entity {
        return self.clone();
    }
}

impl Entity {
    fn update(&self, dt: f32, world: &World, input: &Input) -> EntityUpdateResult {
        match self {
            &Player(ref e) => e.update(dt, world, input),
            &Enemy(ref e) => e.update(dt, world, input),
            &EnemySpawner(ref e) => e.update(dt, world, input),
            &PlayerBullet(ref e) => e.update(dt, world, input)
        }
    }

    fn draw(&self, window: &mut RenderWindow) {
        match self {
            &Player(ref e) => e.draw(window),
            &Enemy(ref e) => e.draw(window),
            &EnemySpawner(ref e) => e.draw(window),
            &PlayerBullet(ref e) => e.draw(window)
        }
    }

    fn position(&self) -> Vector2f {
        match self {
            &Player(ref e) => e.position(),
            &Enemy(ref e) => e.position(),
            &EnemySpawner(ref e) => e.position(),
            &PlayerBullet(ref e) => e.position()
        }
    }

    fn clone(&self) -> Entity {
        match self {
            &Player(ref e) => e.clone(),
            &Enemy(ref e) => e.clone(),
            &EnemySpawner(ref e) => e.clone(),
            &PlayerBullet(ref e) => e.clone()
        }
    }
}

pub struct EntityUpdateResult {
    new_entities: ~[Entity]
}
