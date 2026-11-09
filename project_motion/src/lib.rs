const GRAVITY: f64 = 9.8;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ThrownObject {
    initial_position: Vec2,
    initial_velocity: Vec2,
    elapsed: f64,
}

impl ThrownObject {
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        Self {
            initial_position: position,
            initial_velocity: velocity,
            elapsed: 0.0,
        }
    }
}

impl Iterator for ThrownObject {
    type Item = (Vec2, Vec2);

    fn next(&mut self) -> Option<Self::Item> {
        self.elapsed += 1.0;
        let t = self.elapsed;

        let position = Vec2 {
            x: self.initial_position.x + self.initial_velocity.x * t,
            y: self.initial_position.y + self.initial_velocity.y * t - 0.5 * GRAVITY * t * t,
        };

        let velocity = Vec2 {
            x: self.initial_velocity.x,
            y: self.initial_velocity.y - GRAVITY * t,
        };

        if position.y <= 0.0 {
            return None;
        }

        Some((position, velocity))
    }
}