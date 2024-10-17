mod utils;

use utils::round;

const GRAVITY: f32 = -9.8;

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

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        let actual_position = init_position.clone();
        let actual_velocity = init_velocity.clone();

        Self {
            init_position,
            init_velocity,
            actual_position,
            actual_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    /// The current state of the object is
    /// updated for the next call of the method.
    fn next(&mut self) -> Option<Self::Item> {
        self.time += 1.0; // Update Time

        // Update Velocity.
        self.actual_velocity.x = round(self.init_velocity.x * self.time);
        self.actual_velocity.y = round(self.init_velocity.y + GRAVITY * self.time);

        // Calculate Position.
        self.actual_position.x += self.actual_velocity.x;
        self.actual_position.y = round(
            self.init_position.y
                + self.init_velocity.y * self.time
                + 0.5 * GRAVITY * self.time.powf(2.0),
        );

        // Check if the object has reached the ground.
        if self.actual_position.y <= 0.0 {
            None
        } else {
            Some(self.to_owned())
        }
    }
}
