extern crate native;
extern crate rsfml;
extern crate extra;

use std::rc::Rc;
use std::cell::RefCell;
use std::vec;

use rsfml::system::{Clock, Vector2i};
use rsfml::window::{ContextSettings, VideoMode, Close };
use rsfml::graphics::{RenderWindow, Color};

use input::Input;
use layer::{Layer, LayerTrait, GameLayer, GuiLayer};
use layer::game_layer::{GameLayerStruct};
use layer::gui_layer::{GuiLayerStruct, GuiEventHandler};
use resource_store::ResourceStore;
use rsfml::graphics::rc::Sprite;
use event::{Event, EventHandler};

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

    let mut event_handlers: ~[~EventHandler:] = ~[];

    let gui_layer = Rc::new(RefCell::new(GuiLayer(~GuiLayerStruct::new(&mut resource_store))));
    event_handlers = vec::append_one(event_handlers, ~GuiEventHandler { gui: gui_layer.clone() } as ~EventHandler:);

    let mut layers = ~[
        Rc::new(RefCell::new(GameLayer(~GameLayerStruct::new(&mut resource_store)))),
        gui_layer
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

        let mut events: ~[Event] = ~[];
        
        for layer in layers.mut_iter() {
            events = vec::append(events, layer.borrow().borrow_mut().get().update(dt, &input));
            layer.borrow().borrow().get().draw(&mut window);
        }

        for event in events.iter() {
            for event_handler in event_handlers.mut_iter() {
                event_handler.handle(event.clone());
            }
        }

        window.display()
    }
}