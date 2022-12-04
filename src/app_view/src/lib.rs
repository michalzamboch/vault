#![allow(dead_code, unused_variables, non_camel_case_types, unused_imports)]

use app_controller::*;

use iced::alignment::{self};
use iced::event::{self, Event};
use iced::keyboard;
use iced::subscription;
use iced::theme::{self, Theme};
use iced::widget::{
    self, button, checkbox, column, container, row, scrollable, text,
    text_input, Text,
};
use iced::{Alignment, Element, Sandbox, Settings};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------

fn run_view() -> iced::Result {
    Counter::run(Settings::default())
}

pub fn start() {
    println!("App start...");
    test_controller_start();
    
    let result = run_view();
}

// -----------------------------------------------------------------------

struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Vault")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 10;
            }
            Message::DecrementPressed => {
                self.value -= 10;
            }
        }
    }
    
    fn view(&self) -> Element<Message> {
        column![
            button("Increment").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("Decrement").on_press(Message::DecrementPressed),
            search_bar()
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}


fn search_bar() -> Element<'static, Message> {
    row![
        text("Text").size(50),
        button("Search").on_press(Message::DecrementPressed)
    ]
    .padding(20)
    .align_items(Alignment::Center)
    .into()
}
