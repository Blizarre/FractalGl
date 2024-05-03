use log::info;
use std::ops::RangeInclusive;

use eframe::{
    egui::{Frame, Label, Response, Sense, Ui, Widget},
    epaint::Vec2,
};

pub struct DragPanel<'a> {
    x: &'a mut f32,
    y: &'a mut f32,
    range: Vec2,
}

impl<'a> DragPanel<'a> {
    pub fn new(
        x: &'a mut f32,
        y: &'a mut f32,
        x_range: RangeInclusive<f32>,
        y_range: RangeInclusive<f32>,
    ) -> DragPanel<'a> {
        DragPanel {
            x,
            y,
            range: Vec2::new(
                x_range.end() - x_range.start(),
                y_range.end() - y_range.start(),
            ),
        }
    }
}

impl<'a> Widget for DragPanel<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let square_size = Vec2::new(ui.available_width(), ui.available_width()) * 0.5;
        Frame::canvas(ui.style())
            .show(ui, |ui| {
                let resp = ui.add_sized(
                    square_size,
                    Label::new("Drag for fine tuning")
                        .selectable(false)
                        .sense(Sense::drag()),
                );
                if resp.dragged() {
                    let points_delta = resp.drag_delta();
                    let values_delta = (points_delta / resp.rect.width()) * self.range;
                    info!(
                        "ClickPanel dragged {:?} points, change to x,y {:?}",
                        points_delta, values_delta
                    );
                    *self.x += values_delta.x;
                    *self.y += values_delta.y;
                }
            })
            .response
    }
}
