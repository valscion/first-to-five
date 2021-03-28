mod rules;
use itertools::Itertools;
use rules::{GameArea, Player};

const FIRST_TO_PLAY: rules::Player = Player::Cross;

extern crate piston_window;

use piston_window::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut area = GameArea::default();
    let plays_one = [(0i128, 0i128), (1, 0), (4, 0), (3, 0), (2, 0)];
    let plays_two = [(2i128, 1), (3, 2), (6, 5), (4, 3), (5, 4)];

    let plays = plays_one.iter().interleave(&plays_two);
    for (i, (x, y)) in plays.enumerate() {
        let player = if i % 2 == 0 {
            !FIRST_TO_PLAY
        } else {
            FIRST_TO_PLAY
        };
        area.mark(player, *x, *y)?;
        print_area(&area);
        if area.winner().is_some() {
            println!(
                "Longest line: {:?}",
                area.longest_consecutive_line(*x, *y).unwrap()
            );
            break;
        }
    }

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

    println!("\n\nGame has ended!");
    Ok(())
}

fn print_area(area: &GameArea) {
    println!("{}", area);
    if let Some(player) = area.winner() {
        println!("{:?} has won!", player);
    }
}
