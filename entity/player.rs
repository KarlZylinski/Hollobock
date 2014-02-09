use std::{num, f32};

use rsfml::system::{Vector2f, Vector2i};
use rsfml::graphics::{RenderWindow};
use rsfml::window::{keyboard, mouse};

use input::Input;
use math;
use entity::{Entity, EntityUpdateResult};
use entity::Player;
use entity::PlayerBullet;
use entity::world::World;
use entity::player_bullet::PlayerBulletStruct;
use entity::renderer::Renderer;
use entity::EntityTrait;

pub struct PlayerStruct {
    position: Vector2f,
    rotation: f32,
    renderer: Option<~Renderer:>,
    weapon_cooldown: f32
}

struct PlayerInput {
    direction: Vector2f,
    mouse_position: Vector2i,
    mouse_1: bool
}

fn get_input(input: &Input) -> PlayerInput {
    return PlayerInput {
        direction: Vector2f::new(
            if input.key_held(keyboard::A) {
                -1.
            } else if input.key_held(keyboard::S) {
                1.
            } else {
                0.
            },
            if input.key_held(keyboard::W) {
                -1.
            } else if input.key_held(keyboard::R) {
                1.
            } else {
                0.
            }
        ),
        mouse_position: input.mouse_position,
        mouse_1: input.mouse_button_held(mouse::MouseLeft),
    };
}

fn process_weapon_input(old_cooldown: f32, dt: f32, mouse_1_down: bool) -> (f32, bool)
{
    let cooldown = num::max(0., old_cooldown - dt);

    if mouse_1_down && cooldown == 0. {
        return (0.1, true);
    }

    return (cooldown, false);
}

impl PlayerStruct {
    pub fn new(position: Vector2f, renderer: Option<~Renderer:>) -> PlayerStruct {
        PlayerStruct {
            position: position,
            rotation: 0.,
            renderer: renderer,
            weapon_cooldown: 0.
        }
    }
}

impl EntityTrait for PlayerStruct {
    fn update(&self, dt: f32, _world: &World, input: &Input) -> EntityUpdateResult {
        let input = get_input(input);
        let new_position = self.position + input.direction * 200.0f32 * dt;
        let look_direction = Vector2f::new(input.mouse_position.x as f32 - new_position.x, input.mouse_position.y as f32 - new_position.y);
        let new_rotation = f32::atan2(look_direction.y, look_direction.x).to_degrees();

        let (weapon_cooldown, weapon_fired) = process_weapon_input(self.weapon_cooldown, dt, input.mouse_1);

        let new_player = PlayerStruct {
            position: new_position,
            rotation: new_rotation,
            renderer: self.renderer.as_ref().map_or(None, |r| r.update(&new_position, new_rotation, dt)),
            weapon_cooldown: weapon_cooldown,
        };

        let mut new_entities = ~[Player(~new_player)];

        if weapon_fired {
            new_entities.push(
                PlayerBullet(~PlayerBulletStruct::new(
                    &new_position,
                    &math::normalize(look_direction),
                    400.0,
                    1
                ))
            );
        }
        
        return EntityUpdateResult { new_entities: new_entities };
    }

    fn position(&self) -> Vector2f
    {
        self.position
    }

    fn draw(&self, window: &mut RenderWindow) {
        self.renderer.as_ref().map(|r| r.draw(window));
    }

    fn clone(&self) -> Entity {
        return Player(~PlayerStruct {
            position: self.position.clone(),
            rotation: self.rotation,
            renderer: self.renderer.as_ref().map_or(None, |r| r.clone()),
            weapon_cooldown: self.weapon_cooldown
        });
    }
}
