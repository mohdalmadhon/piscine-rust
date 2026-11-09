#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

const GRAVITY: f32 = 9.8;

fn round1(v: f32) -> f32 {
    (v * 10.0).round() / 10.0
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            actual_position: init_position.clone(),
            actual_velocity: init_velocity.clone(),
            init_position,
            init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.time + 1.0;

        // p = p0 + v0*t + a*t²/2   with a = (0, -g)
        let y = round1(self.init_position.y + self.init_velocity.y * t - 0.5 * GRAVITY * t * t);
        if y <= 0.0 {
            return None;
        }
        let x = round1(self.init_position.x + self.init_velocity.x * t);

        // v = v0 + a*t
        let vx = round1(self.init_velocity.x);
        let vy = round1(self.init_velocity.y - GRAVITY * t);

        self.time = t;
        self.actual_position = Object { x, y };
        self.actual_velocity = Object { x: vx, y: vy };
        Some(self.clone())
    }
}