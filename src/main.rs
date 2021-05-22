mod app;
mod rules;
use app::{App, AppSettings};
use itertools::Itertools;
use rand::random;
use rules::{GameArea, Player};
use winit;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;

fn main() {
    let mut area = GameArea::default();
    example_play(&mut area);
    start_gui(&mut area);

    println!("\n\nGame has ended!");
}

fn example_play(area: &mut GameArea) {
    let plays_one = [(0i128, 0i128), (1, 0), (4, 0), (3, 0), (2, 0)];
    let plays_two = [(2i128, 1), (3, 2), (6, 5), (4, 3), (5, 4)];
    let first_to_play = if random::<bool>() {
        Player::Naught
    } else {
        Player::Cross
    };

    let plays = plays_one.iter().interleave(&plays_two);
    for (i, (x, y)) in plays.enumerate() {
        let player = if i % 2 == 0 {
            !first_to_play
        } else {
            first_to_play
        };
        area.mark(player, *x, *y).expect("Nobody should've won yet");
        if area.winner().is_some() {
            println!(
                "Longest line: {:?}",
                area.longest_consecutive_line(*x, *y).unwrap()
            );
            break;
        }
    }
}

fn start_gui(area: &mut GameArea) {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let event_loop = winit::event_loop::EventLoop::new();
    let temporary_window = winit::window::Window::new(&event_loop).unwrap();
    let (resolution, scale_factor) = match temporary_window.primary_monitor() {
        Some(monitor) => (monitor.size(), monitor.scale_factor()),
        None => {
            panic!("Could not get monitor size")
        }
    };
    drop(temporary_window);

    println!(
        "Resolution: {:?}, scale_factor: {}",
        resolution, scale_factor
    );

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "first-to-five",
        [
            (resolution.width as f64 / scale_factor),
            (resolution.height as f64 / scale_factor),
        ],
    )
    .graphics_api(opengl)
    .fullscreen(false)
    .resizable(true)
    .exit_on_esc(true)
    .build()
    .unwrap();

    // Create a new game and run it.
    let app_settings = AppSettings { scale_factor };
    let mut app = App::new(GlGraphics::new(opengl), area, app_settings);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        app.event(&e);
    }
}
