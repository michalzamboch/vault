#![allow(dead_code, unused_variables, non_camel_case_types, unused_imports)]

use kdbx_rs::{self, CompositeKey, Error, Database, Kdbx};
use kdbx_rs::binary::InnerStreamCipherAlgorithm;
use kdbx_rs::database::{Entry, Times};
use kdbx_rs::xml::write_xml;

use chrono::NaiveDate;
use std::fs::{read_to_string, File};
use std::path::PathBuf;
use uuid::Uuid;

pub struct EntryWrapper {
    id: i32,
    entry: Entry,
    time: Times,
}

impl EntryWrapper {
    pub fn new(title: &str, password: &str) -> Self {
        Self { 
            id: -1,
            entry: Entry::default(),
            time: Times::default(),
        }
    }

    pub fn get_id(self) -> i32 {
        self.id
    }

    pub fn set_title(&mut self, title: &str) {
        self.entry.set_title(title);
    }

    pub fn get_title(self) -> String {
        self.entry.title().unwrap_or("").to_string()
    }

    pub fn set_password(&mut self, password: &str) {
        self.entry.set_password(password);
    }
    
    pub fn get_password(self) -> String {
        self.entry.password().unwrap_or("").to_string()
    }

    pub fn set_url(&mut self, url: &str) {
        self.entry.set_url(url);
    }

    pub fn get_url(self) -> String {
        self.entry.url().unwrap_or("").to_string()
    }

}

// ENTRY BUILDER ----------------------------------------------------------------

pub struct EntryBuilder {
    id: u32,
    title: String,
    password: String,
    url: Option<String>,
}

impl EntryBuilder {

    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_owned();
        self
    }

    pub fn set_password(&mut self, password: String) -> &mut Self {
        self.password = password;
        self
    }
    
    pub fn set_url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }

}