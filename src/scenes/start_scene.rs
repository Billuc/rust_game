use egui::{Frame, Sense};

use crate::{Scene, SceneEvent};

pub struct StartScene {
    renders: u8,
}

impl StartScene {
    pub fn new() -> Self {
        StartScene { renders: 0 }
    }
}

impl Scene for StartScene {
    fn show(self: &mut Self, ui: &mut egui::Ui) -> egui::InnerResponse<crate::SceneEvent> {
        self.renders = self.renders + 1;
        println!("{}", self.renders);

        Frame::default().show(ui, |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(80.0);

                let _girl = ui.add(
                    egui::Image::new(egui::include_image!("../../assets/girl_character.png"))
                        .fit_to_original_size(0.5)
                        .sense(Sense::click()),
                );

                ui.add_space(80.0);

                SceneEvent::None()
            })
            .inner
        })
    }
}
