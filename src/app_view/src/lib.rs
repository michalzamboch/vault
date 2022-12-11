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
    eframe::run_native(
        "Confirm exit",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

#[derive(Default)]
struct MyApp {
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
    search_string: String,
}

impl eframe::App for MyApp {
    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Vault");

            let text_edit_search = egui::TextEdit::singleline(&mut self.search_string)
                .desired_width(f32::INFINITY)
                .cursor_at_end(true);
            let response = ui.add( text_edit_search);
            if response.changed() {
                println!("Changed: \"{}\"", self.search_string);
            }
            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                println!("Entered: \"{}\"", self.search_string);
            }
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
}
