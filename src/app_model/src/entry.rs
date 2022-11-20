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
    title: String,
    password: String,
    url: Option<String>,
    note: Option<String>,
}

impl EntryWrapper {
    pub fn new(title: &str, password: &str) -> Self {
        Self { 
            id: -1,
            entry: Entry::default(),
            time: Times::default(),
            title: title.to_owned(),
            password: password.to_owned(),
            url: None,
            note: None,
        }
    }

    pub fn get_id(self) -> i32 {
        self.id
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn get_title(self) -> String {
        self.title
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
    
    pub fn get_password(self) -> String {
        self.title
    }

    pub fn set_url(&mut self, url: String) {
        self.url = Some(url);
    }
    
    pub fn set_note(&mut self, note: String) {
        self.note = Some(note);
    }
  
}

// ENTRY BUILDER ----------------------------------------------------------------

pub struct EntryBuilder {
    id: u32,
    title: String,
    password: String,
    url: Option<String>,
    note: Option<String>,
}

impl EntryBuilder {

    pub fn set_title(&mut self, title: String) -> &mut Self {
        self.title = title;
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
    
    pub fn set_note(&mut self, note: String) -> &mut Self {
        self.note = Some(note);
        self
    }
  
}