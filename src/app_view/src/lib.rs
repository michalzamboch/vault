#![allow(dead_code, unused_variables, non_camel_case_types, unused_imports)]

use app_controller::*;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use eframe::egui;

// -----------------------------------------------------------------------

pub fn start() {
    println!("App start...");
    test_controller_start();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native("Vault", options, Box::new(|_cc| Box::new(MyApp::default())));
}

#[derive(Default)]
struct MyApp {
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
    search_string: String,
    table_id: String,
}

impl eframe::App for MyApp {
    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        let duration = std::time::Duration::default();
        duration
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Vault");

            let text_edit_search = egui::TextEdit::singleline(&mut self.search_string)
                .desired_width(f32::INFINITY)
                .cursor_at_end(true);
            let response = ui.add(text_edit_search);
            if response.changed() {
                println!("Changed: \"{}\"", self.search_string);
            }
            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                println!("Entered: \"{}\"", self.search_string);
            }

            let main_grid = egui::Grid::new("some_unique_id");

            main_grid.show(ui, |ui| {
                ui.label("Seznam.cz");
                ui.label("john.doe@seznam.cz");
                ui.label("heslo");
                ui.end_row();
            });

            /*let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            let total_rows = 1000;
            egui::ScrollArea::vertical().show_rows(ui, row_height, total_rows, |ui, row_range| {
                for row in row_range {
                    let text = format!("Row {}/{}", row + 1, total_rows);
                    ui.label(text);
                }
            });*/
        });

        if self.show_confirmation_dialog {
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_confirmation_dialog = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.allowed_to_close = true;
                            frame.close();
                        }
                    });
                });
        }
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {}
}
