use crate::common::{self, App};

#[derive(Default)]
pub struct MyApp {}

impl App for MyApp {
    fn title() -> &'static str {
        "My App"
    }

    fn new(app: &mut common::System) -> Self {
        Default::default()
    }

    fn build_ui(&mut self, ctx: &egui::Context) {
    }
}