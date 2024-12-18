use eframe::egui;
use kinetic::{Config, MouseSmoother, MouseState};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;

struct KineticApp {
    config: Arc<Mutex<Config>>,
    mouse_state: Arc<Mutex<MouseState>>,
    is_active: Arc<AtomicBool>,
}

impl KineticApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let config = Arc::new(Mutex::new(Config::default()));
        let mouse_state = Arc::new(Mutex::new(MouseState::default()));
        let is_active = Arc::new(AtomicBool::new(false));

        let config_clone = config.clone();
        let mouse_state_clone = mouse_state.clone();
        let is_active_clone = is_active.clone();

        thread::spawn(move || {
            let mut smoother = MouseSmoother::new(Config::default());

            loop {
                if !is_active_clone.load(Ordering::Relaxed) {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }

                if let Ok(cfg) = config_clone.lock() {
                    smoother.update_config(&cfg);
                }

                smoother.update();

                if let Ok(mut state) = mouse_state_clone.lock() {
                    *state = smoother.get_state();
                }

                thread::sleep(Duration::from_millis(1));
            }
        });

        Self {
            config,
            mouse_state,
            is_active,
        }
    }
}

impl eframe::App for KineticApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut config = self.config.lock().unwrap();

            let active = self.is_active.load(Ordering::Relaxed);
            if ui
                .button(if active {
                    "Disable Smoothing"
                } else {
                    "Enable Smoothing"
                })
                .clicked()
            {
                self.is_active.store(!active, Ordering::Relaxed);
            }

            ui.add_space(10.0);
            ui.heading("Smoothing Settings");
            ui.add_space(10.0);

            ui.label("Smoothing Amount");
            ui.add(egui::Slider::new(&mut config.smoothing_amount, 0.0..=1.0));

            ui.label("EMA Alpha");
            ui.add(egui::Slider::new(&mut config.ema_alpha, 0.0..=1.0));

            ui.label("Min Movement");
            ui.add(egui::Slider::new(&mut config.min_movement, 0.0..=1.0));

            if let Ok(mouse_state) = self.mouse_state.lock() {
                ui.add_space(20.0);
                ui.label("Status");
                ui.label(format!(
                    "Position: ({:.1}, {:.1})",
                    mouse_state.position.0, mouse_state.position.1
                ));
            }
        });

        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 500.0)),
        always_on_top: true,
        ..Default::default()
    };

    eframe::run_native(
        "Kinetic",
        options,
        Box::new(|cc| Box::new(KineticApp::new(cc))),
    )
}

