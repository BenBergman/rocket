use drawing::Point;
use super::Vector;
use traits::{Advance, Collide, Position};

use graphics::{Context, Ellipse};
use opengl_graphics::GlGraphics;

/// Enemies follow the player in order to cause a collision and let him explode 
pub struct Enemy {
    vector: Vector,
    speed: f64,
}

derive_position_direction!(Enemy);

impl Enemy {
    /// Create a enemy with the given vector
    pub fn new(vector: Vector, speed: f64) -> Enemy {
        Enemy {
            vector: vector,
            speed: speed,
        }
    }

    /// Draw the enemy
    pub fn draw(&self, c: &Context, gl: &mut GlGraphics) {
        Ellipse::new([1.0, 1.0, 0.0, 1.0]).draw(
            [self.x() - 10.0, self.y() - 10.0, 20.0, 20.0],
            &c.draw_state, c.transform, gl);
    }

    /// Update the enemy
    pub fn update(&mut self, dt: f64, player_position: Point) {
        // Point to the player
        self.point_to(player_position);
        let speed = self.speed;
        self.advance(dt * speed);
    }
}

impl Collide for Enemy {
    fn radius(&self) -> f64 { 10.0 }
}
