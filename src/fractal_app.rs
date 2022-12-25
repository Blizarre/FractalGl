use eframe::egui::{self, PointerButton, Slider, Ui};
use log::info;

use egui::mutex::Mutex;
use std::{ops::RangeInclusive, sync::Arc};

use crate::Fractal;

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

pub struct MyApp {
    /// Behind an `Arc<Mutex<…>>` so we can pass it to [`egui::PaintCallback`] and paint later.
    fractal: Arc<Mutex<Fractal>>,
    data: State,
}

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub pos: Pos,
    pub c_julia: Pos,
    pub zoom: f32,
    pub contrast: f32,
    pub brightness: f32,
}

impl State {
    fn new() -> State {
        State {
            pos: Pos { x: 0.0, y: 0.0 },
            c_julia: Pos { x: -0.76, y: -0.08 },
            zoom: 3000.0,
            contrast: 0.3,
            brightness: -0.73,
        }
    }
}
impl MyApp {
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

fn add_slider<'a>(
    ui: &mut Ui,
    label: &str,
    value: &'a mut f32,
    range: RangeInclusive<f32>,
    log: bool,
) {
    let slider = Slider::new(value, range).logarithmic(log);
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(slider)
    });
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("Settings").show(ctx, |ui| {
            ui.vertical(|ui| {
                add_slider(ui, "Zoom level", &mut self.data.zoom, 0.0..=10000.0, true);
                add_slider(ui, "Julia 1", &mut self.data.c_julia.x, -1.0..=1.0, false);
                add_slider(ui, "Julia 2", &mut self.data.c_julia.y, -1.0..=1.0, false);
                add_slider(ui, "Contrast", &mut self.data.contrast, -1.0..=1.0, false);
                add_slider(
                    ui,
                    "Brightness",
                    &mut self.data.brightness,
                    -2.0..=2.0,
                    false,
                );

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

impl MyApp {
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
            let pointer_pos = response.interact_pointer_pos().expect("Non mais quoi....");
            let new_center_in_screen_space =
                (pointer_pos - rect.left_top()) * response.ctx.pixels_per_point();
            info!(
                "[click] Move x={:05.2}, y={:05.2} to screen center",
                new_center_in_screen_space.x, new_center_in_screen_space.y
            );
            self.data.pos.x += (response.ctx.pixels_per_point() * rect.width() * 0.5
                - new_center_in_screen_space.x)
                / self.data.zoom;
            self.data.pos.y -= (response.ctx.pixels_per_point() * rect.height() * 0.5
                - new_center_in_screen_space.y)
                / self.data.zoom;
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
