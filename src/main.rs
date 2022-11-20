#![allow(dead_code, unused_variables, non_camel_case_types, unused_imports)]

use kdbx_rs::{self, CompositeKey, Error, Database};
use kdbx_rs::Kdbx;

use kdbx_rs::binary::InnerStreamCipherAlgorithm;
use kdbx_rs::database::{Entry, Times};
use kdbx_rs::xml::write_xml;

use chrono::NaiveDate;
use std::fs::read_to_string;
use std::path::PathBuf;
use uuid::Uuid;

use std::fs::File;

fn main() {
    app_view::start();
}
