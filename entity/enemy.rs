use std::f32;

use rsfml::system::Vector2f;
use rsfml::graphics::{RenderWindow, FloatRect};

use input::Input;
use math;
use std::iter::Iterator;
use entity::{Entity, Player, EntityUpdateResult};
use entity::world::World;
use entity::PlayerBullet;
use entity::Enemy;
use entity::renderer::Renderer;

pub struct EnemyStruct {
    position: Vector2f,
    rotation: f32,
    renderer: Option<~Renderer:>
}

fn intersecting_with_bullet(enemy: &EnemyStruct, world: &World) -> bool {
    let mut bullets = world.entities.iter().filter_map(|e|
        match *e {
            PlayerBullet(ref pb) => Some(pb),
            _ => None
        }
    );

    for bullet in bullets {
        let empty_rect = FloatRect::new(0.0,0.0,0.0,0.0);

        if FloatRect::intersects(
            &bullet.rect().get_global_bounds(),
            &enemy.renderer.as_ref().map_or(empty_rect, |r| r.bounds()),
            &empty_rect){
            
            return true;
        }
    }

    return false;
}

impl EnemyStruct {
    pub fn new(position: Vector2f, renderer: Option<~Renderer:>) -> EnemyStruct {
        EnemyStruct {
            position: position,
            rotation: 0.,
            renderer: renderer
        }
    }

    pub fn update(&self, dt: f32, world: &World, _input: &Input) -> EntityUpdateResult {
        let player_option = world.entities.iter().find(|e| {
            match *e {
                &Player(_) => true,
                _ => false
            }
        });

        let player = match player_option {
            Some(player) => player,
            None => fail!("No player found in world.")
        };

        let direction = math::normalize(player.position() - self.position);
        let new_position = self.position + direction * 100.0f32 * dt;
        let new_rotation = f32::atan2(direction.y, direction.x).to_degrees();

        let new_entities = if intersecting_with_bullet(self, world) {
            ~[]
        } else {
            ~[Enemy(EnemyStruct {
                position: new_position,
                rotation: new_rotation,
                renderer: self.renderer.as_ref().map_or(None, |r| r.update(&new_position, new_rotation)),
            })]
        };

        EntityUpdateResult {
            new_entities: new_entities
        }
    }
    
    pub fn position(&self) -> Vector2f
    {
        self.position
    }
    
    pub fn draw(&self, window: &mut RenderWindow) {
        self.renderer.as_ref().map(|r| r.draw(window));
    }

    pub fn clone(&self) -> Entity {
        Enemy(EnemyStruct {
            position: self.position.clone(),
            rotation: self.rotation,
            renderer: self.renderer.as_ref().map_or(None, |r| r.clone()),
        })
    }
}
