use rsfml::system::{Vector2f};
use rsfml::graphics::{RenderWindow, FloatRect};

pub trait Renderer {
    fn update(&self, position: &Vector2f, rotation: f32) -> Option<~Renderer:>;
    fn draw(&self, window: &mut RenderWindow);
    fn bounds(&self) -> FloatRect;
    fn clone(&self) -> Option<~Renderer:>;
}
