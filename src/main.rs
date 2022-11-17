
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
use std::io::prelude::*;

fn main() {
    app_view::start();
    new_db();
}

#[allow(dead_code)]
fn test1() -> Result<(), Error> {
    let file_path = "./res/test_input/kdbx4-argon2.kdbx";
    let kdbx = kdbx_rs::open(file_path)?;
    let key = CompositeKey::from_password("kdbxrs");
    let unlocked = kdbx.unlock(&key)?;

    let password = unlocked.find_entry(|e| e.url() == Some("https://example.com"))
        .unwrap()
        .password();

    println!("{}", password.unwrap());
    Ok(())
}

fn new_db() {
    let mut database = Database::default();
    database.set_name("My First Database");
    database.set_description("Created with kdbx-rs");

    let mut entry = Entry::default();
    entry.set_title("Moje heslo - name");
    entry.set_password("password1");
    entry.set_url("https://example.com");
    entry.set_username("User123");
    entry.set_uuid(Uuid::from_u128(0x654320));

    database.add_entry(entry);

    let mut kdbx = Kdbx::from_database(database);
    kdbx.set_key(CompositeKey::from_password("foo123"));

    let mut file = File::create("kdbx-rs-example.kdbx");
    let mut x = kdbx.write(file.unwrap());
}

fn set_sample_times(times: &mut Times) {
    times.last_access_time = NaiveDate::from_ymd_opt(2020, 5, 1).expect("Date error.").and_hms_opt(1, 2, 3).expect("Wrong format.");
    times.last_modification_time = NaiveDate::from_ymd_opt(2020, 4, 1).expect("Date error.").and_hms_opt(1, 2, 3).expect("Wrong format.");
    times.creation_time = NaiveDate::from_ymd_opt(2020, 4, 1).expect("Date error.").and_hms_opt(1, 1, 3).expect("Wrong format.");
    times.location_changed = NaiveDate::from_ymd_opt(2020, 4, 1).expect("Date error.").and_hms_opt(1, 1, 3).expect("Wrong format.");
    times.expiry_time = NaiveDate::from_ymd_opt(2020, 4, 1).expect("Date error.").and_hms_opt(1, 1, 3).expect("Wrong format.");
    times.expires = false;
    times.usage_count = 1;
}

fn generate_xml() -> Result<(), kdbx_rs::Error> {
    let mut expected_path = PathBuf::new();
    expected_path.push(env!("CARGO_MANIFEST_DIR"));
    expected_path.push("res");
    expected_path.push("test_output");
    expected_path.push("generate_xml.xml");

    let expected_xml_string = read_to_string(expected_path).unwrap().replace("\r\n", "\n");

    let mut db = kdbx_rs::Database::default();
    db.set_name("BarName");
    db.set_description("BazDesc");
    let group = db.root_mut();
    group.set_name("FooGroup");
    group.set_uuid(Uuid::from_u128(0x12345678));
    set_sample_times(group.times_mut());

    let mut entry = Entry::default();
    entry.set_title("Bar");
    entry.set_password("kdbxrs");
    entry.set_uuid(Uuid::from_u128(0x654321));
    set_sample_times(entry.times_mut());
    group.add_entry(entry);

    let mut output_buffer = Vec::new();

    let key = vec![0xA0; 16];
    let mut stream_cipher = InnerStreamCipherAlgorithm::ChaCha20
        .stream_cipher(&key)
        .unwrap();
    write_xml(&mut output_buffer, &db, stream_cipher.as_mut())?;
    let xml_string = String::from_utf8(output_buffer).unwrap();
    assert_eq!(expected_xml_string, xml_string);
    Ok(())
}
