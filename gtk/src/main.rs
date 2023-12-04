use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

fn main() {
    // Initialize GTK Application
    let app = Application::builder()
        .application_id("com.example.myapp")
        .build();

    app.connect_activate(|app| {
        // Create a new window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("My GTK4 Window")
            .default_width(300)
            .default_height(200)
            .build();

        // Show the window
        window.show();
    });

    // Run the application
    app.run();
}
