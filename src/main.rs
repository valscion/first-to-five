mod app;
mod rules;
use app::App;
use itertools::Itertools;
use rand::random;
use rules::{GameArea, Player};

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
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
        print_area(&area);
        if area.winner().is_some() {
            println!(
                "Longest line: {:?}",
                area.longest_consecutive_line(*x, *y).unwrap()
            );
            break;
        }
    }
}

fn print_area(area: &GameArea) {
    println!("{}", area);
    if let Some(player) = area.winner() {
        println!("{:?} has won!", player);
    }
}

fn start_gui(area: &mut GameArea) {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("first-to-five", [400, 400])
        .graphics_api(opengl)
        .decorated(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(GlGraphics::new(opengl), area);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
