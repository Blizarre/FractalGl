use eframe::egui::{self, PointerButton, Slider};
use log::info;

use egui::mutex::Mutex;
use std::sync::Arc;

use crate::Fractal;

mod state;
pub use state::{Pos, State};

pub struct FractalGl {
    /// Behind an `Arc<Mutex<…>>` so we can pass it to [`egui::PaintCallback`] and paint later.
    fractal: Arc<Mutex<Fractal>>,
    data: State,
}

impl FractalGl {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");
        Self {
            fractal: Arc::new(Mutex::new(Fractal::new(gl))),
            data: State::new(),
        }
    }
}

impl eframe::App for FractalGl {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("Settings").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add(
                    Slider::new(&mut self.data.zoom, 1.0..=5000.0)
                        .logarithmic(true)
                        .text("Zoom"),
                );

                ui.add(Slider::new(&mut self.data.c_julia.x, -1.0..=1.0).text("Julia 1"));
                ui.add(Slider::new(&mut self.data.c_julia.y, -1.0..=1.0).text("Julia 2"));

                ui.add(Slider::new(&mut self.data.contrast, -1.0..=1.0).text("Contrast"));
                ui.add(Slider::new(&mut self.data.brightness, -2.0..=2.0).text("Brightness"));

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

impl FractalGl {
    fn custom_painting(&mut self, ui: &mut egui::Ui) {
        let (rect, response) =
            ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());

        if response.double_clicked_by(PointerButton::Primary) {
            let old_zoom_level = self.data.zoom;
            self.data.zoom *= 1.2;
            info!(
                "Zoom level change: {} -> {}",
                old_zoom_level, self.data.zoom
            );
        } else if response.clicked_by(PointerButton::Primary) {
            let pixels_per_point = ui.ctx().pixels_per_point();

            let new_center_screen = Pos::from_screen_space(
                pixels_per_point,
                response.interact_pointer_pos().expect("Non mais quoi...."),
            );
            let current_center = Pos::from_screen_space(pixels_per_point, rect.center());
            let diff_gl_space = (current_center - new_center_screen) / self.data.zoom;

            info!(
                "new_center_screen: {:?}, current_center: {:?}, diff gl space: {:?}",
                new_center_screen, current_center, diff_gl_space
            );
            self.data.pos.x += diff_gl_space.x;
            self.data.pos.y -= diff_gl_space.y;
        } else if response.double_clicked_by(PointerButton::Secondary) {
            let old_zoom_level = self.data.zoom;
            self.data.zoom /= 1.2;
            info!(
                "Zoom level change: {} -> {}",
                old_zoom_level, self.data.zoom
            );
        }

        if response.dragged() {
            let drag_in_gl_space = response.drag_delta() * response.ctx.pixels_per_point();
            info!("Dragged: {:?} pixels ", drag_in_gl_space);

            self.data.pos.x += drag_in_gl_space.x / self.data.zoom;
            self.data.pos.y -= drag_in_gl_space.y / self.data.zoom;
        }

        // Clone locals so we can move them into the paint callback:
        let data = self.data;
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
