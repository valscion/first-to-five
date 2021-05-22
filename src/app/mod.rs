use crate::rules::{GameArea, Player};
use opengl_graphics::GlGraphics;
use piston::input::{GenericEvent, RenderArgs, UpdateArgs};

pub struct AppSettings {
  pub scale_factor: f64,
}

pub struct App<'a> {
  gl: GlGraphics,          // OpenGL drawing backend.
  game_area: &'a GameArea, // The game area we're running
  settings: AppSettings,
}

impl<'a> App<'a> {
  pub fn new(gl: GlGraphics, game_area: &'a mut GameArea, settings: AppSettings) -> App<'a> {
    let app = Self {
      gl,
      game_area,
      settings,
    };
    println!("Initialized App with game area:\n{}", app.game_area);
    app
  }

  pub fn event(&mut self, e: &impl GenericEvent) {
    if let Some(args) = e.render_args() {
      self.render(&args);
    }

    if let Some(args) = e.update_args() {
      self.update(&args);
    }
  }

  fn render(&mut self, args: &RenderArgs) {
    use graphics::*;

    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const GRAY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

    let AppSettings { scale_factor } = self.settings;
    let w_w = args.window_size[0];
    let w_h = args.window_size[1];

    // How large will we render a single play
    let play_size = 80.0 / scale_factor;
    // There should be a some margin between plays
    let margin: f64 = 10.0 / scale_factor;
    // The width of the lines for the plays
    let stroke: f64 = 2.0 / scale_factor;
    // The grid's line stroke width
    let grid_stroke: f64 = 1.0 / scale_factor;

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

      // Then draw the grid.
      let horizontal_lines_count = (w_w / play_size).ceil() as u64;
      let vertical_lines_count = (w_h / play_size).ceil() as u64;
      for i in 1..(vertical_lines_count) {
        let y = (i as f64) * play_size;
        line_from_to(GRAY, grid_stroke, [0.0, y], [w_w, y], transform, gl);
      }
      for i in 1..(horizontal_lines_count) {
        let x = (i as f64) * play_size;
        line_from_to(GRAY, grid_stroke, [x, 0.0], [x, w_h], transform, gl);
      }

      for (i, maybe_player) in all_plays.iter().enumerate() {
        let x = (i % area_width) as f64;
        let y = (i / area_width) as f64;

        let start_x = (play_size * x) + margin;
        let start_y = (play_size * y) + margin;
        let size = play_size - margin * 2.0;

        match maybe_player {
          Some(Player::Cross) => {
            // Draw the cross
            line_from_to(
              WHITE,
              stroke,
              [start_x, start_y],
              [start_x + size, start_y + size],
              transform,
              gl,
            );
            line_from_to(
              WHITE,
              stroke,
              [start_x + size, start_y],
              [start_x, start_y + size],
              transform,
              gl,
            );
          }
          Some(Player::Naught) => {
            ellipse(WHITE, [start_x, start_y, size, size], transform, gl);
            ellipse(
              BLACK,
              [
                start_x + (stroke * 2.0),
                start_y + (stroke * 2.0),
                size - (stroke * 4.0),
                size - (stroke * 4.0),
              ],
              transform,
              gl,
            );
          }
          None => {
            // Empty on purpose
          }
        }
      }
    });
  }

  fn update(&mut self, _args: &UpdateArgs) {
    // TODO: Do something
  }
}
