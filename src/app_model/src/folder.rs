#![allow(dead_code, unused_variables, non_camel_case_types)]

use kdbx_rs::{self, CompositeKey, Error, Database, Kdbx};
use kdbx_rs::binary::InnerStreamCipherAlgorithm;
use kdbx_rs::database::{Entry, Times};
use kdbx_rs::xml::write_xml;

use chrono::NaiveDate;
use std::fs::{read_to_string, File};
use std::path::PathBuf;
use uuid::Uuid;

pub struct GroupWrapper {
    title: String
}