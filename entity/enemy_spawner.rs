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

    pub fn update(&self, dt: f32, _world: &World, _input: &Input) -> EntityUpdateResult {
        let new_entities = if self.time_since_spawn > 1. {
            ~[
                Enemy(EnemyStruct {
                    position: Vector2f::new(800.0f32 * rand(), 600.0f32 * rand()),
                    rotation: 0.,
                    renderer: self.renderer.as_ref().map_or(None, |r| r.clone())
                }),
                EnemySpawner(EnemySpawnerStruct {
                    time_since_spawn: dt,
                    renderer: self.renderer.as_ref().map_or(None, |r| r.clone()),
                })
            ]
        } else {
            ~[
                EnemySpawner(EnemySpawnerStruct {
                    time_since_spawn: self.time_since_spawn + dt,
                    renderer: self.renderer.as_ref().map_or(None, |r| r.clone()),
                })
            ]
        };

        return EntityUpdateResult { new_entities: new_entities };
    }


    pub fn draw(&self, _window: &mut RenderWindow) {        
    }
    
    pub fn position(&self) -> Vector2f
    {
        Vector2f::new(0., 0.)
    }
    
    pub fn clone(&self) -> Entity {
        return EnemySpawner(EnemySpawnerStruct {
            time_since_spawn: self.time_since_spawn,
            renderer: self.renderer.as_ref().map_or(None, |r| r.clone()),
        });
    }
}
