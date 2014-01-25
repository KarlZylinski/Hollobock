extern mod native;
extern mod rsfml;

use rsfml::system::{Clock, Vector2f, Vector2i};
use rsfml::window::{ContextSettings, VideoMode, event, Close };
use rsfml::graphics::{RenderWindow, Color};

use entity::Entity;
use player::Player;
use input::Input;

pub mod world;
pub mod list;
pub mod entity;
pub mod player;
pub mod input;
pub mod player_bullet;
pub mod vector;

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

    let player = Player {
    	position: Vector2f::new(200., 200.),
    	rotation: 0.,
    	weapon_cooldown: 0.
    };

	let mut frame_timer = Clock::new();
    let mut world = world::World {
    	entities: ~[~player as ~Entity]
    };

    let mut input = Input::init(window.get_mouse_position());

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

		world = world.update(dt, &input);
		world.draw(&mut window);
		
		window.display()
	}
}