use eframe::egui;

// fn main() -> Result<(), eframe::Error> {
//     let options = eframe::NativeOptions::default();
//     eframe::run_native(
//         "Hello Rust with eGUI",
//         options,
//         Box::new(|_cc| Ok(Box::<MyApp>::default())),
//     )
// }

fn main() -> Result<(), eframe::Error> {
    //let options = eframe::NativeOptions::default();
    // Definiujemy wymiary i opcje okna (viewport)
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([240.0, 250.0]), // [szerokość, wysokość] w punktach logicznych
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
            ui.heading("Helo Rust with eGUI");
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
