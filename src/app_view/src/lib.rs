#![allow(dead_code, unused_variables, non_camel_case_types, unused_imports)]

use app_controller::*;

use iced::alignment::{self, Alignment};
use iced::event::{self, Event};
use iced::keyboard;
use iced::subscription;
use iced::theme::{self, Theme};
use iced::widget::{
    self, button, checkbox, column, container, row, scrollable, text,
    text_input, Text,
};
use iced::window;
use iced::{Application, Element};
use iced::{Color, Command, Font, Length, Settings, Subscription};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

// -----------------------------------------------------------------------

pub fn start() {
    println!("App start...");
    test_controller_start();
}

// -----------------------------------------------------------------------
