use egui::{InnerResponse, Ui};

pub trait Scene {
    fn show(self: &mut Self, ui: &mut Ui) -> InnerResponse<SceneEvent>;
}

pub enum SceneEvent {
    None(),
    ChangeScene(Box<dyn Scene>),
}


