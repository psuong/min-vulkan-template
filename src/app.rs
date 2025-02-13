use crate::common::{self, App};

pub struct MyApp {}

impl App for MyApp {
    fn title() -> &'static str {
        todo!()
    }

    fn new(app: &mut common::System) -> Self {
        todo!()
    }

    fn build_ui(&mut self, ctx: &egui::Context) {
        todo!()
    }
}