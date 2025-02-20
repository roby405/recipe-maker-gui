use eframe::egui;
use std::collections::HashMap;

#[derive(Clone, Default)]
struct Recipe {
    id: i32,
    nume: String,
    description: String,
    ingredients: HashMap<String, String>,
    instructions: Vec<String>,
}

#[derive(Default, Clone)]
enum AppState {
    MainMenu,
    AddRecipe(i32),
    ViewRecipes(Vec<Recipe>),
    ViewDetails(Recipe),
    EditRecipe(Recipe),
    DeleteRecipe(Recipe),
}

fn main() -> Result<(), eframe::Error> {
    // Create a new native app window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([400.0, 300.0])
        .with_min_inner_size([300.0, 220.0]),// Set window size
        ..Default::default()
    };

    // Run the app
    eframe::run_native(
        "My First egui App", // Window title
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))), // App state
    )
}

// Define the app state
#[derive(Default)]
struct MyApp {
    menu: AppState,
    name: String,       // A text input field
    age: u32,           // A number input field
    is_happy: bool,     // A checkbox
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

// Implement the app logic
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let a:i32 = 5;
        // Create a new window
        egui::CentralPanel::default().show(ctx, |ui| {
            // Add a heading
            ui.heading("My First egui App");

            // Add a text input field for the name
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            // Add a number input field for the age
            ui.horizontal(|ui| {
                ui.label("Your age: ");
                ui.add(egui::DragValue::new(&mut self.age));
            });

            // Add a checkbox for happiness
            ui.checkbox(&mut self.is_happy, "Are you {a} happy?");

            // Add a button to display the entered data
            if ui.button("Submit").clicked() {
                println!(
                    "Name: {}, Age: {}, Happy: {}",
                    self.name, self.age, self.is_happy
                );
            }
        });
    }
}