use egui::{Frame, Layout, Sense};

use crate::{Scene, SceneEvent};

pub struct StartScene {}

impl StartScene {
    pub fn new() -> Self {
        StartScene {}
    }
}

impl Scene for StartScene {
    fn show(self: &mut Self, ui: &mut egui::Ui) -> egui::InnerResponse<crate::SceneEvent> {
        ui.with_layout(Layout::left_to_right(egui::Align::BOTTOM), |ui| {
            ui.columns(7, |columns| {
                columns[1].add(
                    egui::Image::new(egui::include_image!("../../assets/girl_character.png"))
                        .fit_to_original_size(0.5)
                        .sense(Sense::click()),
                );

                columns[4].add(
                    egui::Image::new(egui::include_image!("../../assets/girl_character.png"))
                        .fit_to_original_size(0.5)
                        .sense(Sense::click()),
                );

                SceneEvent::None()
            })
        })
    }
}
