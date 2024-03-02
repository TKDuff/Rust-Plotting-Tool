use gtk::prelude::*;
use gtk::{DrawingArea, Window, WindowType};
use std::sync::{Arc, RwLock};
use plotters_cairo::CairoBackend;



struct Data {
    points: Vec<[f64; 2]>,
}

impl Data {
    pub fn new () -> Self {
        Self {
            points: Vec::new(),
        }
    }
}

struct MyApp {
    data_points: Arc<RwLock<Data>>,

}

impl MyApp {

    pub fn new (
        data_points: Arc<RwLock<Data>>,
    ) -> Self {
        Self { data_points }
    }
}

fn main() {
    gtk::init().expect("Failed to initialize GTK.");
    let window = Window::new(WindowType::Toplevel);
    let drawing_area = DrawingArea::new();

    let data = Arc::new(RwLock::new(Data::new()));
    let my_app = MyApp::new(data.clone());


    let drawing_area = DrawingArea::new();
    window.add(&drawing_area);

    drawing_area.connect_draw(move |_, cr| {
        let data = data.clone();
        let mut data = data.read().unwrap();

        let backend = CairoBackend::new(cr, (500, 300)).expect("Cannot create backend");
        let root = backend.into_drawing_area();
        root.fill(&WHITE).expect("Failed to fill");

        let mut chart = ChartBuilder::on(&root)
            .caption("Plot", ("sans-serif", 50))
            .build_cartesian_2d(0..10, 0..10)
            .expect("Failed to build chart");

            chart.configure_mesh().draw().expect("Failed to draw mesh");

            for point in &data.points {
                chart.draw_series(PointSeries::of_element(
                    [point[0], point[1]].iter().cloned(),
                    5,
                    &RED,
                    &|coord, size, style| {
                        EmptyElement::at(coord)
                            + Circle::new((0, 0), size, style.filled())
                    },
                )).expect("Failed to draw point");
            }

            root.present().expect("Failed to present");
        Inhibit(false)


    });

}