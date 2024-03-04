
#![allow(warnings)] 
use nix::libc::ARPD_FLUSH;
//Remove warning, be sure to remove this
use project_library::{CountRawData, DataStrategy, TierData, Bin, main_threads, process_tier, setup_my_app};
use std::any::Any;
use std::fmt::format;
use std::process::id;
use std::{num, thread, usize};
use eframe::{egui, NativeOptions, App}; 
use egui::{Style, Visuals, ViewportBuilder, Slider, Color32, Vec2, CentralPanel, Id};
use egui_plot :: {BoxElem, BoxPlot, BoxSpread, CoordinatesFormatter, Corner, Legend, Line, LineStyle, Plot, PlotPoint, PlotResponse, log_grid_spacer};
use std::sync::{Arc, RwLock};
use crossbeam::channel;
use tokio::runtime::Runtime;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::time::{self, Duration, Interval};
use tokio::fs::File;
use tokio::sync::mpsc;
use std::env;
use std::time::{ Instant};
use rayon::{prelude::*, ThreadPool};
use std::sync::atomic::{AtomicBool, Ordering};
use std::error::Error;
use egui::color_picker::color_picker_color32;


struct MyApp {
    raw_data: Arc<RwLock<dyn DataStrategy + Send + Sync>>,  //'dyn' mean 'dynamic dispatch', specified for that instance. Allow polymorphism for that instance, don't need to know concrete type at compile time
    tiers: Vec<Arc<RwLock<TierData>>>,
    should_halt: Arc<AtomicBool>,
    line_plot: bool,
    clicked_bin:  Option<((Bin, Bin), usize)>,
    colours: [Color32; 6],    //maintain line colours between repaints
    selected_line_index: usize,
}

impl MyApp {

    pub fn new(
        raw_data: Arc<RwLock<dyn DataStrategy + Send + Sync>>, 
        tiers: Vec<Arc<RwLock<TierData>>>,
        should_halt: Arc<AtomicBool>,
        line_plot: bool,
        selected_line_index: usize,
        colours: [Color32; 6],
    ) -> Self {
        let default_bin = Bin::new(0.0, 0.0, 0.0, 0.0, 0, 0.0, 0.0);
        Self { raw_data, tiers ,should_halt, clicked_bin: Some(((default_bin, default_bin), 0)), line_plot ,selected_line_index, colours }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (rd_sender, hd_receiver) = channel::unbounded();

    let (aggregation_strategy, strategy, tiers, catch_all_policy, should_halt, num_tiers)  =  setup_my_app()?;
    let mut colours = [Color32::RED, Color32::BLUE, Color32::GREEN, Color32::BLACK, Color32::BROWN, Color32::YELLOW];
    let my_app = MyApp::new(aggregation_strategy, tiers, should_halt, false,0, colours);

    /*If no strategy selected, so just the raw data, then don't need to run all this code, can just run the tokio thread to read in raw data */

    
    let should_halt_clone = my_app.should_halt.clone();
    let raw_data_thread_for_setup = my_app.raw_data.clone();
    
    let raw_data_accessor = my_app.raw_data.clone();
    let initial_tier_accessor = my_app.tiers[0].clone();
    let tier_vector = my_app.tiers.clone();

    let raw_data_accessor_for_thread = my_app.raw_data.clone();
    let initial_tier_accessor_for_thread = my_app.tiers[0].clone();

    /*
    create the thread to handle tier merging
    both branches create thread to move live data to initial, tier 1
    Both branches do it since if no aggregation, no need to create this thread
    */
    if strategy == "count" {
        setup_count(raw_data_accessor, initial_tier_accessor, num_tiers, catch_all_policy, tier_vector);
        //create thread to handle the live data being pushed to the initial tier
        main_threads::create_raw_data_to_initial_tier(hd_receiver, raw_data_accessor_for_thread, initial_tier_accessor_for_thread);
    } else if strategy == "interval"{
        setup_interval(raw_data_accessor, initial_tier_accessor, num_tiers, catch_all_policy, tier_vector);
        //create thread to handle the live data being pushed to the initial tier
        main_threads::create_raw_data_to_initial_tier(hd_receiver, raw_data_accessor_for_thread, initial_tier_accessor_for_thread);
    }
   


    let rt = Runtime::new().unwrap();
    let raw_data_thread = my_app.raw_data.clone();
    //create tokio thread to read in standard input, then append to live data vector
    match strategy.as_str() {
        "count" => main_threads::create_count_stdin_read(&rt, should_halt_clone, raw_data_thread, rd_sender),
        "interval" => main_threads::create_interval_stdin_read(&rt, should_halt_clone, raw_data_thread, rd_sender),
        _ => main_threads::crate_none_stdin_read(&rt, should_halt_clone, raw_data_thread),
    }
    //create threads in reverse order of who access live data. Tokio last, as want other merging threads to be ready to handle aggregate live data

    /*
    if strategy == "count" {
        main_threads::create_count_stdin_read(&rt, should_halt_clone, raw_data_thread, rd_sender);
    } else {
        main_threads::create_interval_stdin_read(&rt, should_halt_clone, raw_data_thread, rd_sender);
    }*/
    


    fn setup_count(raw_data_accessor: Arc<RwLock<dyn DataStrategy + Send + Sync>>, initial_tier_accessor: Arc<RwLock<TierData>>, num_tiers: usize, catch_all_policy: bool, tier_vector: Vec<Arc<RwLock<TierData>>>) {
        if num_tiers == 4 {
            main_threads::count_rd_to_ca_edge(raw_data_accessor, initial_tier_accessor); 
        } else {
            let num_tiers = tier_vector.len();
            let catch_all_tier = tier_vector[num_tiers-1].clone(); //correctly gets the catch all tier, have to minus one since len not 0 indexed 
    
            catch_all_tier.write().unwrap().x_stats.drain(0..1);
            catch_all_tier.write().unwrap().y_stats.drain(0..1);
    
            if catch_all_policy {
                main_threads::count_check_cut_ca(tier_vector, catch_all_tier, num_tiers);
            } else {
                main_threads::count_check_cut_no_ca(tier_vector, catch_all_tier, num_tiers);
            }
        }
    }

    fn setup_interval(raw_data_accessor: Arc<RwLock<dyn DataStrategy + Send + Sync>>, initial_tier_accessor: Arc<RwLock<TierData>>, num_tiers: usize, catch_all_policy: bool, tier_vector: Vec<Arc<RwLock<TierData>>>) {
        if num_tiers == 4 {
            main_threads::rd_to_ca_edge(raw_data_accessor, initial_tier_accessor);
        } else {
            let num_tiers = tier_vector.len();
            let catch_all_tier = tier_vector[num_tiers-1].clone(); //correctly gets the catch all tier, have to minus one since len not 0 indexed 

            catch_all_tier.write().unwrap().x_stats.drain(0..1);
            catch_all_tier.write().unwrap().y_stats.drain(0..1);

            if catch_all_policy {
                main_threads::interval_check_cut_ca(tier_vector, catch_all_tier, num_tiers)
            } else {        
                main_threads::interval_check_cut_no_ca(tier_vector, catch_all_tier, num_tiers);
            }
        }
    }
    

    let native_options = NativeOptions{
        viewport: egui::ViewportBuilder::default().with_inner_size([1900.0, 1000.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",native_options,Box::new(move |_|{Box::new(my_app)}),
    );

    Ok(())
}


impl App for MyApp<>  {    //implementing the App trait for the MyApp type, MyApp provides concrete implementations for the methods defined in the App
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { //'update()' is the method being implemented 
        egui::CentralPanel::default().show(ctx, |ui| { 
            ctx.set_visuals(Visuals::light());

            ui.radio_value(&mut self.line_plot, true, "Line Plot");
            ui.radio_value(&mut self.line_plot, false, "Box Plot");
           

            let mut position: Option<PlotPoint> =None;
            let mut hovered_item: Option<String> = None;


            let plot_width = 1800.0;
            let plot_height = 600.0;

            let fill_line_id = ui.id().with("line_filled");
            let mut fill_plot_line = ui.data_mut(|d| d.get_temp::<bool>(fill_line_id).unwrap_or(true));

            let lines_width_id = ui.id().with("lines_width_id");
            let mut lines_width = ui.data_mut(|d| d.get_temp::<f32>(lines_width_id).unwrap_or(2.0));

            //Plot axis are updated on each frame, given the default name 'X/Y axis', have to be sure to check for single line respose to ensure defautl values to don't overide text input on each frame update
            let x_axis_label_id = ui.id().with("x_axis_label_id");
            let y_axis_label_id = ui.id().with("y_axis_label_id");
            let mut x_axis_label = ui.data_mut(|d| d.get_temp::<String>(x_axis_label_id).unwrap_or("X Axis".to_string()));
            let mut y_axis_label = ui.data_mut(|d| d.get_temp::<String>(y_axis_label_id).unwrap_or("Y Axis".to_string()));

            let axis_log_base_id = ui.id().with("axis_log_base_id");

            let mut axis_log_base = ui.data_mut(|d| d.get_temp::<i64>(axis_log_base_id).unwrap_or(10)); //fdefault base 10

            if ui.button("Halt Processing").clicked() {
                self.should_halt.store(true, Ordering::SeqCst);
            }


            let mut tier_plot_lines = Vec::new();
            let mut tier_box_plots = Vec::new();
            let mut tier_plot_lines_length: Vec<usize> = Vec::new();
            let number_of_tiers = self.tiers.len();

            let raw_data = self.raw_data.read().unwrap().get_raw_data();
            tier_plot_lines_length.push(raw_data.len());     
                        
                    

            let mut plot = Plot::new("plot").width(plot_width).height(plot_height).legend(Legend::default()).x_axis_label(&x_axis_label).y_axis_label(&y_axis_label)
            .x_grid_spacer(log_grid_spacer(axis_log_base))
            .y_grid_spacer(log_grid_spacer(axis_log_base));


            let plot_responese: PlotResponse<()> = plot.show(ui, |plot_ui| {
                match self.line_plot {
                    true => {
                        //store length of Stdin data line
                        let mut raw_plot_line = Line::new(raw_data).width(lines_width).color(self.colours[0]).name("Stdin Data");
                        if fill_plot_line { raw_plot_line = raw_plot_line.fill(0.0)}

                        for (i, tier) in self.tiers.iter().enumerate() {
                            //increment as colurs vector first element is Stdin data line colour, red. Since colour not used, pass in argument directly 
                            create_tier_lines(i, tier, lines_width, self.colours[i+1], fill_plot_line, &mut tier_plot_lines_length, &mut tier_plot_lines)
                        } 
                        plot_ui.line(raw_plot_line);
                        for line in tier_plot_lines {
                            plot_ui.line(line);
                        }
                    },
                    false => {
                        for (i, tier) in self.tiers.iter().enumerate() {
                            create_x_box_plots(i, tier, &mut tier_plot_lines_length, &mut tier_box_plots); 
                        }
                        for box_plot in tier_box_plots {
                            plot_ui.box_plot(box_plot)
                        }
                        /*
                        let tier1_box = create_x_box_plots(0, &self.tiers[0], &mut tier_plot_lines_length);
                        let tier2_box = create_x_box_plots(0, &self.tiers[1], &mut tier_plot_lines_length);
                        plot_ui.box_plot(tier2_box);
                        plot_ui.box_plot(tier1_box);*/
                    }
                }

                /*
                 plot_ui.line(raw_plot_line);
                 for line in tier_plot_lines {
                    plot_ui.line(line);
                 }
                 position = plot_ui.pointer_coordinate();*/
                position = plot_ui.pointer_coordinate();
            });

            let line_length_area_pos = egui::pos2(40.0, 630.0);
            
            /*
            egui::Area::new("Line Length")
            .fixed_pos(line_length_area_pos) // Position the area as desired
            .show(ui.ctx(), |ui| {
                
                egui::CollapsingHeader::new("Tier Lengths").default_open(true)
                .default_open(true) // You can set this to false if you want it to start collapsed
                .show(ui, |ui| {
                    ui.add(egui::Label::new(formatted_label(&format!("Stdin data: {}", tier_plot_lines_length[0]), Color32::BLACK, 16.0 , false)));
                    for i in 1..=number_of_tiers {   //have to increment by 1 for case when there is only single catch all
                        ui.add(egui::Label::new(formatted_label(&format!("Tier {}: {}", i, tier_plot_lines_length[i]), Color32::BLACK, 16.0 , false)));
                    }});
            });*/

            let click = ctx.input(|i| i.pointer.any_click());

            //refactor this
            if click {
                let tier_index = plot_responese.hovered_plot_item
                .and_then(|id| (0..self.tiers.len()).find(|&i| id == egui::Id::new(i)))
                .unwrap_or_else(|| usize::MAX);
            
                if tier_index != usize::MAX {
                if let Some(new_clicked_bin) = find_closest(position, &self.tiers[tier_index], tier_index) {
                    self.clicked_bin = Some(new_clicked_bin);
                }
                }
            }

            let bin_info_area_pos = egui::pos2(800.0, 630.0);
            egui::Area::new("Bin Information Area")
            .fixed_pos(bin_info_area_pos)
            .show(ui.ctx(), |ui| {
                ui.heading(formatted_label("  Selected Bin Information", Color32::LIGHT_YELLOW, 20.0, true));
                if let Some(((x_bin, y_bin), tier_index)) = self.clicked_bin {
                    let colour = self.colours[tier_index + 1];
                    bin_grid_helper(ui, &x_bin, &y_bin, colour, tier_index+1); //lazy to increment one twice, but the function does not have a reference to self, so need pass colour
                } 
            });


            let plot_styling_area_pos = egui::pos2(1375.0, 630.0);
            egui::Area::new("plot_styling area")
            .fixed_pos(plot_styling_area_pos)
            .show(ui.ctx(), |ui| {
                ui.heading(formatted_label("Plot Styling Options", Color32::BLACK, 20.0, true));
                

                //Box to edit axis name, store the edited name 
                ui.label("Change axis name"); 
                let edit_x_axis_name = ui.add(egui::TextEdit::singleline(&mut x_axis_label).desired_width(70.0));
                let edit_y_axis_name = ui.add(egui::TextEdit::singleline(&mut y_axis_label).desired_width(70.0));
                

                //If the x axis name has been edited (changed() function checks this), then write it to egui memory. Cannot use changed() on lines, since not interactive elements, can only use it on buttons
                if edit_x_axis_name.changed() {
                    ui.data_mut(|d| d.insert_temp(x_axis_label_id, x_axis_label.clone()));
                }
                if edit_y_axis_name.changed() {
                    ui.data_mut(|d| d.insert_temp(y_axis_label_id, y_axis_label.clone()));
                }
                ui.add_space(10.0);
                ui.label("Change plot log scale");
                if ui.add(Slider::new(&mut axis_log_base, 2..=20)).changed() {
                    ui.data_mut(|d| d.insert_temp(axis_log_base_id, axis_log_base));

                }
   
            });

            let line_styling_area_pos = egui::pos2(1550.0, 630.0);
            egui::Area::new("line_styling_area")
            .fixed_pos(line_styling_area_pos)
            .show(ui.ctx(), |ui| {
                ui.heading(formatted_label("Line Styling Options", Color32::BLACK, 20.0, true));

                if ui.button("Fill Lines").clicked() {
                    fill_plot_line = !fill_plot_line;
                    ui.data_mut(|d: &mut egui::util::IdTypeMap| d.insert_temp(fill_line_id, fill_plot_line));
                    
                }

                /*Slider to adjust width, using  'lines_width' variable created above*/
                ui.add(Slider::new(&mut lines_width, 0.0..=10.0).text("Lines width"));
                ui.data_mut(|d| d.insert_temp(lines_width_id, lines_width));

                /*To change line colur, remember stdin data not included in vector of tier lines, thus it has to manually be checked and colour changed */
                egui::ComboBox::from_label("Select a line to change colour")
                .selected_text( 
                    if self.selected_line_index == 0 {
                        format!("Stdin Data")
                    } else {
                        format!("Tier {}", self.selected_line_index)
                    }
                 )
                .show_ui(ui, |ui| {
                    if ui.selectable_label( self.selected_line_index == 0, "Stdin Data").clicked() {
                        self.selected_line_index = 0;
                    }
                    for i in 1..number_of_tiers+1 {
                        if ui.selectable_label(self.selected_line_index == i, format!("Tier {}", i)).clicked() {
                            self.selected_line_index = i;
                        } 
                    }
                }); 
                egui::color_picker::color_picker_color32(ui, &mut self.colours[self.selected_line_index], egui::widgets::color_picker::Alpha::OnlyBlend); 

            })
        });
        ctx.request_repaint();
    }
}


fn find_closest(position: Option<PlotPoint>, tier: &Arc<RwLock<TierData>>, tier_index: usize) -> Option<((Bin, Bin), usize)> {
    let plot_point = position?;
    let x = plot_point.x;
    let y = plot_point.y;
    let tier_data = tier.read().unwrap();

    let tolerance = 5.0;
    tier_data.x_stats.iter().zip(tier_data.y_stats.iter())
        .find(|&(x_bin, y_bin)| {
            (x - x_bin.get_mean()).abs() <= tolerance && (y - y_bin.get_mean()).abs() <= tolerance
        })
        .map(|(x_closest, y_closest)| ((*x_closest, *y_closest), tier_index))

        /*
        Bug occurs with select bin, if click but not in threshold, will return defualt bin, thus set the grid to initial values. In order to overcome this, if no threshold is found.
        Want to make it that the bin grid updates only when succesfly click on other bin. Can solve this by not returning anything if nothing found. 
        */
}  



/*Helper function, to format line length text
Is a 'builder' function, as it creates rich text label but can format the style upon each call based on the passed in parameters
* color
* size
* bold*/
fn formatted_label(text: &str, color: Color32, size: f32, bold: bool) -> egui::RichText {
    let mut text = egui::RichText::new(text)
        .color(egui::Color32::BLACK)
        .size(16.0);

    if bold {
        text = text.strong()
    }
    
    text
}

fn bin_grid_helper(ui: &mut egui::Ui, x_bin: &Bin, y_bin: &Bin, colour: Color32, tier_index: usize) {
    let even_row_transparent = Color32::from_rgba_premultiplied(colour.r(), colour.g(), colour.b(), 64); // 50% opacity
    let odd_row_transparent = Color32::from_rgba_premultiplied(colour.r(), colour.g(), colour.b(), 128); // 50% opacity

    egui::Grid::new("bin_info_grid")
    .striped(true)
    .num_columns(2)
    .spacing([40.0, 4.0])
    //this makes the row have a different colour depending on whether odd or even 
    .with_row_color(move |row_index, _style| {
        if row_index % 2 == 0 {
            Some(even_row_transparent)
        } else {
            Some(odd_row_transparent)
        }
    })
    .show(ui, |ui| {
        ui.label(formatted_label(&format!("\t\t\t\t\t\t\tTier {}", tier_index), Color32::BLACK, 16.0, true));
        ui.end_row();

        ui.label(formatted_label("X Values", Color32::BLACK, 16.0, true));
        ui.label(formatted_label("Y Values", Color32::BLACK, 16.0, true));
        ui.end_row();

        ui.label(formatted_label(&format!("Mean: {:.2}", x_bin.mean), Color32::BLACK, 16.0, false));
        ui.label(formatted_label(&format!("Mean: {:.2}", y_bin.mean), Color32::BLACK, 16.0, false));
        ui.end_row();

        ui.label(formatted_label(&format!("Sum: {:.2}", x_bin.sum), Color32::BLACK, 16.0, false));
        ui.label(formatted_label(&format!("Sum: {:.2}", y_bin.sum), Color32::BLACK, 16.0, false));
        ui.end_row();

        ui.label(formatted_label(&format!("Min: {:.2}", x_bin.min), Color32::BLACK, 16.0, false));
        ui.label(formatted_label(&format!("Min: {:.2}", y_bin.min), Color32::BLACK, 16.0, false));
        ui.end_row();

        ui.label(formatted_label(&format!("Max: {:.2}", x_bin.max), Color32::BLACK, 16.0, false));
        ui.label(formatted_label(&format!("Max: {:.2}", y_bin.max), Color32::BLACK, 16.0, false));
        ui.end_row();

        ui.label(formatted_label(&format!("Count: {}", x_bin.count), Color32::BLACK, 16.0, false));
        ui.label(formatted_label(&format!("Count: {}", y_bin.count), Color32::BLACK, 16.0, false));
        ui.end_row();

        
        ui.label(formatted_label(&format!("Variance: {}", x_bin.variance), Color32::BLACK, 16.0, false));
        ui.label(formatted_label(&format!("Variance: {}", y_bin.variance), Color32::BLACK, 16.0, false));
        ui.end_row();

        ui.label(formatted_label(&format!("SoS: {}", x_bin.sum_of_squares), Color32::BLACK, 16.0, false));
        ui.label(formatted_label(&format!("SoS: {}", y_bin.sum_of_squares), Color32::BLACK, 16.0, false));
        ui.end_row();

        ui.label(formatted_label(&format!("Std Dev: {}", x_bin.standard_deviation), Color32::BLACK, 16.0, false));
        ui.label(formatted_label(&format!("Std Dev: {}", y_bin.standard_deviation), Color32::BLACK, 16.0, false));
        ui.end_row();
});
}

//vectors passed by reference, not by value, so can modify them in the functions 
fn create_tier_lines(i: usize, tier: &Arc<RwLock<TierData>>, lines_width: f32 , color: Color32, fill_plot_line: bool, tier_plot_lines_length: &mut Vec<usize>,tier_plot_lines: &mut Vec<Line>) {
    let line_id = format!("Tier {}", i+1);
    let values = tier.read().unwrap().get_means(); 
    tier_plot_lines_length.push(values.len());  //want to store length of each line
    


    let mut line = Line::new(values)
    .width(lines_width)
    .color(color)
    .name(&line_id)
    .id(egui::Id::new(i));

    //if user wants to fill line, do so for both tiers and raw data
    if fill_plot_line {
        line = line.fill(0.0);
    }
    
    tier_plot_lines.push(line);
}

fn create_x_box_plots(i: usize, tier: &Arc<RwLock<TierData>>, tier_plot_lines_length: &mut Vec<usize>, tier_box_plots: &mut Vec<BoxPlot>) {
    let mut box_elems = Vec::new();


    let box_id = format!("Tier {}", i+1);
    let box_stats = tier.read().unwrap().get_box_plot_stats();
    tier_plot_lines_length.push(box_stats    .len());

    for (index, stats) in box_stats.iter().enumerate() {
        let spread = BoxSpread {
            lower_whisker: stats.1, // min
            quartile1: stats.3,     // estimated_q1
            median: stats.0,        // mean
            quartile3: stats.4,     // estimated_q2
            upper_whisker: stats.2, // max
        };

        let elem = BoxElem::new(stats.0, spread).name(&format!("{} {}", box_id, index));
        box_elems.push(elem)
    }

    tier_box_plots.push(BoxPlot::new(box_elems).name(&box_id).id(egui::Id::new(i)));
}


/*
- MyApp<T> is a generic struct, T is a type that implements the DataStrategy trait. T can be either StdinData or ADWIN_window
- genercs in myapp<T> to allow it to be agnostic about the specific type of data it's working with, as long as that type conforms to the DataStrategy 

- integrate with eframe and egui, your MyApp struct needs to implement the App trait from the eframe crate.
- This trait requires you to define an update method, where you will handle drawing the UI and processing events.

- 'update' function creates the Egui window and where access MyApp data and methods, allowing to interact with the underlying data (handled by T) and reflect changes in the UI.
*/