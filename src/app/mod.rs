use crate::rules::{GameArea, Player};
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

pub struct App<'a> {
  gl: GlGraphics,          // OpenGL drawing backend.
  rotation: f64,           // Rotation for the square.
  game_area: &'a GameArea, // The game area we're running
}

impl<'a> App<'a> {
  pub fn new(gl: GlGraphics, game_area: &'a mut GameArea) -> App<'a> {
    let app = Self {
      gl,
      rotation: 0.0,
      game_area,
    };
    println!("Initialized App with game area:\n{}", app.game_area);
    app
  }

  pub fn render(&mut self, args: &RenderArgs) {
    use graphics::*;

    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

    let w_w = args.window_size[0];
    let w_h = args.window_size[1];

    // At least 10 items should be renderable in the minimum direction
    let play_size = w_w.min(w_h) / 10.0;
    // There should be a 2px margin between plays
    const MARGIN: f64 = 2.0;

    let area_width = self.game_area.width() as usize;
    let all_plays = self.game_area.all_plays();

    self.gl.draw(args.viewport(), |c, gl| {
      // Clear the screen.
      clear(BLACK, gl);

      let transform = c.transform;

      // Draw an empty rectangle around the play area
      for (from, to) in &[
        ([0.0, 0.0], [w_w, 0.0]),
        ([w_w, 0.0], [w_w, w_h]),
        ([w_w, w_h], [0.0, w_h]),
        ([0.0, w_h], [0.0, 0.0]),
      ] {
        line_from_to(RED, 2.0, *from, *to, transform, gl);
      }

      for (i, maybe_player) in all_plays.iter().enumerate() {
        let x = (i % area_width) as f64;
        let y = (i / area_width) as f64;

        let start_x = (play_size * x) + MARGIN;
        let start_y = (play_size * y) + MARGIN;
        let size = play_size - MARGIN * 2.0;

        match maybe_player {
          Some(Player::Cross) => {
            rectangle(GREEN, [start_x, start_y, size, size], transform, gl);
          }
          Some(Player::Naught) => {
            ellipse(WHITE, [start_x, start_y, size, size], transform, gl);
          }
          None => {
            // Empty on purpose
          }
        }
      }
    });
  }

  pub fn update(&mut self, args: &UpdateArgs) {
    // Rotate 2 radians per second.
    self.rotation += 2.0 * args.dt;
  }
}
