mod rules;
use itertools::Itertools;
use rand::random;
use rules::{GameArea, Player};

extern crate piston_window;

use piston_window::*;

fn main() {
    let mut area = GameArea::default();
    example_play(&mut area);
    start_gui(&area);

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

fn start_gui(_area: &GameArea) {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle(
                [1.0, 0.0, 0.0, 1.0], // red
                [0.0, 0.0, 100.0, 100.0],
                context.transform,
                graphics,
            );
        });
    }
}
