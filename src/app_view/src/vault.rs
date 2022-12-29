use eframe::{
    egui::{
        self, Button, CentralPanel, Color32, Context, FontData, FontDefinitions, FontFamily,
        Hyperlink, Label, Layout, RichText, Separator, TextStyle, TopBottomPanel, Ui, Window,
    },
    CreationContext,
};

use kdbx_rs::{self, CompositeKey, Error, Database, Kdbx};
use kdbx_rs::binary::InnerStreamCipherAlgorithm;
use kdbx_rs::binary::Cipher;
use kdbx_rs::database::{Entry, Times};
use kdbx_rs::xml::write_xml;

#[derive(Default)]
pub struct Vault {
    pub database: Database,
    pub dummy: String,
}
