#![allow(dead_code, unused_variables, non_camel_case_types)]

pub struct Entry {
    id: u32,
    title: String,
    password: String,
    url: Option<String>,
    note: Option<String>,
}

impl Entry {
    pub fn new(title: String) -> Self {
        Self { 
            id: 0,
            title: title,
            password: "********".to_string(),
            url: None,
            note: None,
        }
    }

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