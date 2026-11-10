// WIN Bez konsoli w trybie --release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// WIN Bez konsoli
//#![windows_subsystem = "windows"]

use eframe::egui;

// fn main() -> Result<(), eframe::Error> {
//     let options = eframe::NativeOptions::default();
//     eframe::run_native(
//         "Hello Rust with eGUI",
//         options,
//         Box::new(|_cc| Ok(Box::<MyApp>::default())),
//     )
// }

fn load_icon() -> egui::IconData {
    // Ładujemy plik graficzny (np. PNG) bezpośrednio do pliku wykonywalnego
    let icon_bytes = include_bytes!("assets/icon.png"); 
    
    let image = image::load_from_memory(icon_bytes)
        .expect("Nie udało się załadować ikony z pamięci")
        .into_rgba8();
        
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    
    egui::IconData { rgba, width, height }
}

fn main() -> Result<(), eframe::Error> {
    //let options = eframe::NativeOptions::default();
    // Definiujemy wymiary i opcje okna (viewport)
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([240.0, 250.0]) // [szerokość, wysokość] w punktach logicznych
        .with_icon(std::sync::Arc::new(load_icon())), // ładowanie ikony programu
        ..Default::default()
    };
    eframe::run_native(
        "Hello Rust with eGUI",
        options,
        Box::new(|cc| {
            // 1. Rejestrujemy ładowarki obrazków w kontekście egui
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // 2. Zwracamy naszą aplikację opakowaną w Ok (wymagane w nowszych wersjach eframe)
            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Default)]
// #[default]
struct MyApp {
    name: String,
    age: u32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello Rust with eGUI");
            ui.horizontal(|ui| {
                ui.label("Your Name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=150).text("age"));
            if ui.button("Increment age").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', your age is {}.", self.name, self.age));
            ui.label("Some image:");
            ui.image(egui::include_image!("img.png"));
        });
    }
}

// https://github.com/emilk/egui
// https://docs.rs/egui/latest/egui/
