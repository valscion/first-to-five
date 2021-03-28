mod rules;
use rules::{GameArea, Player};

const FIRST_TO_PLAY: rules::Player = Player::Cross;

extern crate piston_window;

use piston_window::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut area = GameArea::default();
    let naught_plays = [(0i128, 0i128), (1, 0), (4, 0), (3, 0), (2, 0)]
        .iter()
        .map(|(x, y)| (x, y, Player::Naught))
        .collect::<Vec<_>>();
    let cross_plays = [(2i128, 1), (3, 2), (6, 5), (4, 3), (5, 4)]
        .iter()
        .map(|(x, y)| (x, y, Player::Cross))
        .collect::<Vec<_>>();

    let plays = if FIRST_TO_PLAY == Player::Naught {
        naught_plays.iter().zip(cross_plays.iter())
    } else {
        cross_plays.iter().zip(naught_plays.iter())
    };
    for (round, ((x1, y1, player1), (x2, y2, player2))) in plays.enumerate() {
        println!("# Round {} starts!\n", round + 1);

        area.mark(*player1, **x1, **y1)?;
        print_area(&area);
        if area.winner().is_some() {
            break;
        }

        area.mark(*player2, **x2, **y2)?;
        print_area(&area);
        if area.winner().is_some() {
            break;
        }
        println!();
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
