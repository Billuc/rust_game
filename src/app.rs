
use egui::{Color32, Frame};

use crate::{scenes::MainScene, Scene};

pub struct App {
    current_scene: Box<dyn Scene>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_scene: Box::new(MainScene::new()),
        }
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for App {
    fn update(self: &mut Self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        let response = egui::CentralPanel::default()
            .frame(Frame::default().fill(Color32::WHITE))
            .show(ctx, |ui| self.current_scene.show(ui));

        match response.inner.inner {
            crate::SceneEvent::ChangeScene(scene) => {
                self.current_scene = scene;
            }
            crate::SceneEvent::None() => (),
        }
    }
}
