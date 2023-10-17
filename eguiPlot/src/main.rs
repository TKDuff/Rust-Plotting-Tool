use egui;
fn main() {
    let ctx = egui::Context::new();
    let mut app = egui::Window::new("My Window");
    egui::run(ctx, &mut app, |ui| {
        ui.heading("My egui Application");
        ui.horizontal(|ui| {
            ui.label("Your name: ");
            ui.text_edit_singleline(&mut name);
        });
    });
}