use egui::{Color32, FontId, Frame, RichText, Sense};

use crate::{scenes::StartScene, Scene, SceneEvent};

pub struct MainScene {}

impl MainScene {
    pub fn new() -> Self {
        MainScene {}
    }
}

impl Scene for MainScene {
    fn show(self: &mut Self, ui: &mut egui::Ui) -> egui::InnerResponse<crate::SceneEvent> {
        Frame::default().show(ui, |ui| {
            ui.horizontal_centered(|ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(80.0);

                    ui.label(
                        RichText::new("The Game")
                            .font(FontId::proportional(80.0))
                            .color(Color32::BLACK),
                    );

                    ui.add_space(120.0);

                    let play_btn = ui.add(
                        egui::Image::new(egui::include_image!("../../assets/play_button.png"))
                            .fit_to_original_size(0.5)
                            .sense(Sense::click()),
                    );

                    ui.add_space(80.0);

                    if play_btn.clicked() {
                        SceneEvent::ChangeScene(Box::new(StartScene::new()))
                    } else {
                        SceneEvent::None()
                    }
                })
                .inner
            })
            .inner
        })
    }
}
