pub struct App {

}

impl Default for App {
    fn default() -> Self {
        Self {
        }
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
        } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("coucou")
        });
    }
}
