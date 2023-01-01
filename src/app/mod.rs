use eframe::egui::{self, PointerButton, Slider};
use log::info;

use egui::mutex::Mutex;
use std::sync::Arc;

mod state;
pub use state::State;

mod pos;
pub use pos::Pos;

mod drag_panel;
use drag_panel::DragPanel;

mod fractal_gl;
use fractal_gl::FractalGl;

pub struct FractalApp {
    /// Behind an `Arc<Mutex<…>>` so we can pass it to [`egui::PaintCallback`] and paint later.
    fractal: Arc<Mutex<FractalGl>>,
    state: State,
}

impl FractalApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");
        Self {
            fractal: Arc::new(Mutex::new(FractalGl::new(gl))),
            state: State::new(),
        }
    }
}

impl eframe::App for FractalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("Settings").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add(
                    Slider::new(&mut self.state.zoom, 1.0..=5000.0)
                        .logarithmic(true)
                        .text("Zoom"),
                );

                ui.separator();

                ui.toggle_value(&mut self.state.julia_expanded, "Julia parameters");

                if self.state.julia_expanded {
                    ui.add(DragPanel::new(
                        &mut self.state.c_julia.x,
                        &mut self.state.c_julia.y,
                        -0.2..=0.2,
                        -0.2..=0.2,
                    ));

                    ui.add(
                        Slider::new(&mut self.state.c_julia.x, -1.0..=1.0)
                            .text("Julia 1")
                            .clamp_to_range(false),
                    );
                    ui.add(
                        Slider::new(&mut self.state.c_julia.y, -1.0..=1.0)
                            .text("Julia 2")
                            .clamp_to_range(false),
                    );
                }

                ui.separator();

                ui.toggle_value(&mut self.state.color_expanded, "Color parameters");

                if self.state.color_expanded {
                    ui.add(DragPanel::new(
                        &mut self.state.contrast,
                        &mut self.state.brightness,
                        -1.0..=1.0,
                        -2.0..=2.0,
                    ));

                    ui.add(
                        Slider::new(&mut self.state.contrast, -1.0..=1.0)
                            .text("Contrast")
                            .clamp_to_range(false),
                    );
                    ui.add(
                        Slider::new(&mut self.state.brightness, -2.0..=2.0)
                            .text("Brightness")
                            .clamp_to_range(false),
                    );
                }
                ui.separator();

                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.custom_painting(ui);
            });
        });
    }

    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.fractal.lock().destroy(gl);
        }
    }
}

impl FractalApp {
    fn custom_painting(&mut self, ui: &mut egui::Ui) {
        let (rect, response) =
            ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());

        if response.double_clicked_by(PointerButton::Primary) {
            let old_zoom_level = self.state.zoom;
            self.state.zoom *= 1.2;
            info!(
                "Zoom level change: {} -> {}",
                old_zoom_level, self.state.zoom
            );
        } else if response.clicked_by(PointerButton::Primary) {
            let pixels_per_point = ui.ctx().pixels_per_point();

            let new_center_screen = Pos::from_screen_space(
                pixels_per_point,
                response.interact_pointer_pos().expect("Non mais quoi...."),
            );
            let current_center = Pos::from_screen_space(pixels_per_point, rect.center());
            let diff_gl_space = (current_center - new_center_screen) / self.state.zoom;

            info!(
                "new_center_screen: {:?}, current_center: {:?}, diff gl space: {:?}",
                new_center_screen, current_center, diff_gl_space
            );
            self.state.pos.x += diff_gl_space.x;
            self.state.pos.y -= diff_gl_space.y;
        } else if response.double_clicked_by(PointerButton::Secondary) {
            let old_zoom_level = self.state.zoom;
            self.state.zoom /= 1.2;
            info!(
                "Zoom level change: {} -> {}",
                old_zoom_level, self.state.zoom
            );
        }

        if response.dragged() && response.drag_delta().length_sq() > 0.0 {
            let drag_in_gl_space = response.drag_delta() * response.ctx.pixels_per_point();
            info!("Dragged: {:?} pixels ", drag_in_gl_space);

            self.state.pos.x += drag_in_gl_space.x / self.state.zoom;
            self.state.pos.y -= drag_in_gl_space.y / self.state.zoom;
        }

        // Clone locals so we can move them into the paint callback:
        let data = self.state;
        let fractal = self.fractal.clone();

        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                fractal.lock().paint(painter.gl(), data);
            })),
        };
        ui.painter().add(callback);
    }
}

