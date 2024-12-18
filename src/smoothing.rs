use crate::platform::{MouseController, PlatformController};
use crate::Config;
use crate::MouseState;

pub struct MouseSmoother {
    config: Config,
    state: MouseState,
    controller: PlatformController,
}

impl MouseSmoother {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            state: MouseState::default(),
            controller: PlatformController::new(),
        }
    }

    pub fn get_state(&self) -> MouseState {
        self.state.clone()
    }

    pub fn update_config(&mut self, new_config: &Config) {
        self.config = new_config.clone();
    }

    pub fn update(&mut self) {
        let current_pos = self.controller.get_position();
        let (prev_x, prev_y) = self.state.position;
        let (curr_x, curr_y) = (current_pos.0 as f64, current_pos.1 as f64);

        let delta_x = curr_x - prev_x;
        let delta_y = curr_y - prev_y;

        let threshold = 0.5;
        let current_speed = (delta_x * delta_x + delta_y * delta_y).sqrt();

        if current_speed > threshold {
            self.state.update(&self.config, delta_x, delta_y);
            let (smooth_x, smooth_y) = self.state.get_smoothed_position();

            if let Err(e) = self.controller.set_position(smooth_x, smooth_y) {
                eprintln!("Failed to set cursor position: {:?}", e);
            }
        }
    }
}
