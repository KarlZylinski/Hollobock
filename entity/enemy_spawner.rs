use rsfml::system::Vector2f;
use rsfml::graphics::RenderWindow;

use input::Input;
use std::rand;
use std::rand::Rng;
use entity::world::World;
use entity::Enemy;
use entity::enemy::EnemyStruct;
use entity::EnemySpawner;
use entity::{Entity, EntityUpdateResult};
use entity::renderer::Renderer;
use entity::EntityTrait;

pub struct EnemySpawnerStruct {
    time_since_spawn: f32,
    renderer: Option<~Renderer:>
}

fn rand() -> f32 {
    let mut rng = rand::rng();
    
    if rng.gen() { 
        return rng.gen::<f32>();
    }

    return 0.;
}

impl EnemySpawnerStruct {
    pub fn new(renderer: Option<~Renderer:>) -> EnemySpawnerStruct {
        EnemySpawnerStruct {
            time_since_spawn: 0.,
            renderer: renderer
        }
    }
}

impl EntityTrait for EnemySpawnerStruct {
    fn update(&self, dt: f32, _world: &World, _input: &Input) -> EntityUpdateResult {
        let new_entities = if self.time_since_spawn > 1. {
            ~[
                Enemy(~EnemyStruct::new(
                    &Vector2f::new(800.0f32 * rand(), 600.0f32 * rand()),
                    0.,
                    self.renderer.as_ref().map_or(None, |r| r.clone()),
                    3
                )),
                EnemySpawner(~EnemySpawnerStruct {
                    time_since_spawn: dt,
                    renderer: self.renderer.as_ref().map_or(None, |r| r.clone()),
                })
            ]
        } else {
            ~[
                EnemySpawner(~EnemySpawnerStruct {
                    time_since_spawn: self.time_since_spawn + dt,
                    renderer: self.renderer.as_ref().map_or(None, |r| r.clone()),
                })
            ]
        };

        return EntityUpdateResult { new_entities: new_entities };
    }


    fn draw(&self, _window: &mut RenderWindow) {        
    }
    
    fn position(&self) -> Vector2f
    {
        Vector2f::new(0., 0.)
    }

    fn clone(&self) -> Entity {
        return EnemySpawner(~EnemySpawnerStruct {
            time_since_spawn: self.time_since_spawn,
            renderer: self.renderer.as_ref().map_or(None, |r| r.clone()),
        });
    }
}
