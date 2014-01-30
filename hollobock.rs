extern mod native;
extern mod rsfml;

use rsfml::system::{Clock, Vector2i};
use rsfml::window::{ContextSettings, VideoMode, event, Close };
use rsfml::graphics::{RenderWindow, Color};

use input::Input;
use layer::Layer;
use layer::game_layer::GameLayer;
use layer::gui_layer::GuiLayer;
use std::vec;
use resource_store::ResourceStore;

pub mod input;
pub mod math;
pub mod layer;
pub mod resource_store;
pub mod entity;

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
    	~GameLayer::new(&mut resource_store) as ~Layer:,
    	~GuiLayer::new() as ~Layer
    ];

	while window.is_open() {
		let dt = frame_timer.get_elapsed_time().as_seconds();
		frame_timer.restart();

		input = Input::init_from_previous_frame(&input);

		loop {
			let event = window.poll_event();

			match event {
				event::Closed => window.close(),
				event::KeyPressed { code, .. } => input.set_key_pressed(code),
				event::KeyReleased { code, .. } => input.set_key_released(code),
				event::MouseMoved { x, y } => input.mouse_position = Vector2i::new(x as i32, y as i32),
				event::MouseButtonPressed { button, .. } => input.set_mouse_button_pressed(button),
				event::MouseButtonReleased { button, .. } => input.set_mouse_button_released(button),
				event::NoEvent => break,
				_ => {}
			}
		}

		window.clear(&Color::new_RGB(0, 200, 200));

		let mut new_layers: ~[~Layer:] = ~[];
		
		for layer in layers.iter() {
			let update_result = layer.update(dt, &input);
			new_layers = vec::append(new_layers, update_result.new_layers);
			layer.draw(&mut window);
		}
		
		layers = new_layers;

		window.display()
	}
}