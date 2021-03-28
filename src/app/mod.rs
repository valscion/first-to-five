use crate::GameArea;
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

    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    let square = rectangle::square(0.0, 0.0, 50.0);
    let rotation = self.rotation;
    let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

    self.gl.draw(args.viewport(), |c, gl| {
      // Clear the screen.
      clear(GREEN, gl);

      let transform = c
        .transform
        .trans(x, y)
        .rot_rad(rotation)
        .trans(-25.0, -25.0);

      // Draw a box rotating around the middle of the screen.
      rectangle(RED, square, transform, gl);
    });
  }

  pub fn update(&mut self, args: &UpdateArgs) {
    // Rotate 2 radians per second.
    self.rotation += 2.0 * args.dt;
  }
}
