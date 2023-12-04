use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Label};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;
use glib::ControlFlow::Continue;

fn main() {
    // Initialize GTK application
    let app = Application::new(Some("com.example.async_data"), Default::default());
    
    // Create an Arc and Mutex wrapped data point
    let data = Arc::new(Mutex::new(0.0));

    // Clone the Arc to move into the data generation thread
    let data_for_thread = Arc::clone(&data);
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            // Generate a random number and store it in the shared data
            let mut data = data_for_thread.lock().unwrap();
            *data = rng.gen::<f64>();
            drop(data); // Drop the lock manually
            
            // Sleep for a short duration to simulate data generation interval
            thread::sleep(Duration::from_millis(500));
        }
    });

    app.connect_activate(move |app| {
        // Create a new window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Random Number Generator")
            .default_width(300)
            .default_height(100)
            .build();
        
        // Create a new label
        let label = Label::new(None);
        window.set_child(Some(&label));
        window.present();

        // Clone the Arc to move into the GTK thread
        let data_for_gtk = Arc::clone(&data);
        gtk4::glib::timeout_add_local(Duration::from_millis(1000), move || {
            // Lock the mutex and update the label with the new data
            let data = data_for_gtk.lock().unwrap();
            label.set_label(&format!("Latest data: {:.2}", data));
            Continue
        });
    });

    app.run();
}
