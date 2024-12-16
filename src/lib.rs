mod platform;
pub mod smoothing;

use serde::{Deserialize, Serialize};
use std::time::Instant;

pub use smoothing::MouseSmoother;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub smoothing_amount: f64,
    pub ema_alpha: f64,
    pub min_movement: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            smoothing_amount: 0.15,
            ema_alpha: 0.7,
            min_movement: 0.0,
        }
    }
}

#[derive(Clone)]
pub struct MouseState {
    pub position: (f64, f64),
    pub last_update: Instant,
    pub ema_x: f64,
    pub ema_y: f64,
}

impl Default for MouseState {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0),
            last_update: Instant::now(),
            ema_x: 0.0,
            ema_y: 0.0,
        }
    }
}

impl MouseState {
    pub fn update(&mut self, config: &Config, delta_x: f64, delta_y: f64) {
        let now = Instant::now();
        self.last_update = now;

        self.ema_x = config.ema_alpha * delta_x + (1.0 - config.ema_alpha) * self.ema_x;
        self.ema_y = config.ema_alpha * delta_y + (1.0 - config.ema_alpha) * self.ema_y;

        let smoothed_dx = self.ema_x * config.smoothing_amount;
        let smoothed_dy = self.ema_y * config.smoothing_amount;

        let movement = (smoothed_dx * smoothed_dx + smoothed_dy * smoothed_dy).sqrt();
        if movement > config.min_movement {
            self.position.0 += smoothed_dx;
            self.position.1 += smoothed_dy;
        }
    }

    pub fn get_smoothed_position(&self) -> (i32, i32) {
        (self.position.0 as i32, self.position.1 as i32)
    }
}