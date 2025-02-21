use eframe::egui;
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;
use std::collections::HashMap;

#[derive(Clone, Default)]
struct Recipe {
    id: i32,
    nume: String,
    description: String,
    ingredients: HashMap<String, String>,
    instructions: Vec<String>,
}

#[derive(Clone)]
enum AppState {
    MainMenu,
    AddRecipe(i32),
    ViewRecipes(Vec<Recipe>),
    ViewDetails(Recipe),
    EditRecipe(Recipe),
    DeleteRecipe(Recipe),
}

impl Default for AppState {
    fn default() -> Self { AppState::MainMenu }
}

fn main() -> Result<(), eframe::Error> {
    // Create a new native app window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([960.0, 630.0])
        .with_min_inner_size([640.0, 425.0]),// Set window size
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
    recipes: Vec<Recipe>
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            menu: AppState::MainMenu,
            recipes: Vec::new(),
        }
    }
}

// Implement the app logic
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let a:i32 = 5;
        // Create a new window
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut style = (*ctx.style()).clone();
            style.text_styles = [
                (Heading, FontId::new(30.0, Proportional)),
                (Name("Heading2".into()), FontId::new(25.0, Proportional)),
                (Name("Context".into()), FontId::new(23.0, Proportional)),
                (Body, FontId::new(18.0, Proportional)),
                (Monospace, FontId::new(14.0, Proportional)),
                (Button, FontId::new(24.0, Proportional)),
                (Small, FontId::new(10.0, Proportional)),
              ].into();
            ui.style_mut().text_styles = style.text_styles;

            // Add a heading
            egui::SidePanel::left("option_menu")
                .resizable(true)
                .default_width(300.0)
                .show_inside(ui, |ui| {

                ui.vertical_centered(|ui| {
                    ui.heading("Recipe Maker");
                    ui.separator();
                    if ui.button("Add recipe").clicked() {
                        self.menu = AppState::AddRecipe(0);
                    }
                    if ui.button("Edit recipe").clicked() {
                        self.menu = AppState::EditRecipe(Recipe::default());
                    }
                    if ui.button("Delete recipe").clicked() {
                        self.menu = AppState::DeleteRecipe(Recipe::default());
                    }
                    ui.separator();
                    if ui.button("View recipe").clicked() {
                        self.menu = AppState::ViewDetails(Recipe::default());
                    }
                    if ui.button("View all recipes").clicked() {
                        self.menu = AppState::ViewRecipes(Vec::new());
                    }
                });
            });
            ui.vertical_centered(|ui| {
                ui.heading("TEXT AREA");
            });
            // Add a text input field for the name
            // ui.horizontal(|ui| {
            //     ui.label("Your name: ");
            //     // ui.text_edit_singleline(&mut self.name);
            // });

            // // Add a number input field for the age
            // ui.horizontal(|ui| {
            //     ui.label("Your age: ");
            //     // ui.add(egui::DragValue::new(&mut self.age));
            // });

            // Add a checkbox for happiness
            // ui.checkbox(&mut self.is_happy, "Are you {a} happy?");

            // Add a button to display the entered data
            // if ui.button("Submit").clicked() {
            //     println!(
            //         "Name: {}, Age: {}, Happy: {}",
            //         self.name, self.age, self.is_happy
            //     );
            // }
        });
    }
}