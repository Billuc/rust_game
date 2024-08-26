use egui::{InnerResponse, Ui};

pub trait Scene<DataType> {
    fn show(self: &Self, ui: &mut Ui) -> InnerResponse<SceneEvent<DataType>>;
}

pub struct SceneEvent<DataType> {
    pub event_type: String,
    pub data: DataType,
}

impl<T> SceneEvent<T> {
    pub fn new(ev_type: String, ev_data: T) -> Self {
        SceneEvent {
            event_type: ev_type,
            data: ev_data,
        }
    }
}