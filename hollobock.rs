extern crate native;
extern crate rsfml;
extern crate extra;

use std::rc::Rc;

use rsfml::system::{Clock, Vector2i};
use rsfml::window::{ContextSettings, VideoMode, Close };
use rsfml::graphics::{RenderWindow, Color};

use input::Input;
use layer::Layer;
use layer::game_layer::GameLayer;
use layer::gui_layer::GuiLayer;
use std::vec;
use resource_store::ResourceStore;
use rsfml::graphics::rc::Sprite;

pub mod input;
pub mod math;
pub mod layer;
pub mod resource_store;
pub mod entity;
pub mod gui;
pub mod event;

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    let setting = ContextSettings::default();
    let mut window = match RenderWindow::new(VideoMode::new_init(800, 600, 32), "Hollobock", Close, &setting) {
        Some(window) => window,
        None => fail!("Cannot create a new Render Window.")
    };

    let mut frame_timer = Clock::new();
    let mut input = Input::init(window.get_mouse_position());
    let mut resource_store = ResourceStore::new();

    let mut layers = ~[
        Rc::new(~GameLayer::new(&mut resource_store) as ~Layer:),
        Rc::new(~GuiLayer::new(&mut resource_store) as ~Layer:)
    ];

    let background_texture = resource_store.load_texture(~"background.png");

    while window.is_open() {
        let dt = frame_timer.get_elapsed_time().as_seconds();
        frame_timer.restart();

        input = Input::init_from_previous_frame(&input);

        loop {
            let event = window.poll_event();

            match event {
                rsfml::window::event::Closed => window.close(),
                rsfml::window::event::KeyPressed { code, .. } => input.set_key_pressed(code),
                rsfml::window::event::KeyReleased { code, .. } => input.set_key_released(code),
                rsfml::window::event::MouseMoved { x, y } => input.mouse_position = Vector2i::new(x as i32, y as i32),
                rsfml::window::event::MouseButtonPressed { button, .. } => input.set_mouse_button_pressed(button),
                rsfml::window::event::MouseButtonReleased { button, .. } => input.set_mouse_button_released(button),
                rsfml::window::event::NoEvent => break,
                _ => {}
            }
        }

        window.clear(&Color::new_RGB(0, 200, 200));

        background_texture.clone().map(|t| {
            Sprite::new_with_texture(t).map(|s| {
                window.draw(&s);
            });
        });

        let mut new_layers: ~[~Layer:] = ~[];
        
        for layer in layers.iter() {
            let update_result = layer.borrow().update(dt, &input);
            new_layers = vec::append(new_layers, update_result.new_layers);
            layer.borrow().draw(&mut window);
        }

        // TODO: this is shit. Immutable layers have gone to shit.
        // Can't delete stuff and recreate it allt the time since
        // it'll break event handlers. What the shit.
        
        layers = ~[];

        for layer in new_layers.move_iter() {
            layers = vec::append_one(layers, Rc::new(layer));
        }
        
        window.display()
    }
}