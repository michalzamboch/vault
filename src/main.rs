extern crate keepass;
extern crate kdbx_rs;

use keepass::{Database, NodeRef, Result, Error};
use std::fs::File;

fn main() {

    test1();
}

fn test1() -> Result<()> {
    
    println!("Hello world");

    // Open KeePass database
    let path = std::path::Path::new("tests/resources/test_db_with_password.kdbx");
    let db = Database::open(&mut File::open(path)?, Some("demopass"), None)?;

    // Iterate over all Groups and Nodes
    for node in &db.root {
        match node {
            NodeRef::Group(g) => {
                println!("Saw group '{0}'", g.name);
            },
            NodeRef::Entry(e) => {
                let title = e.get_title().unwrap();
                let user = e.get_username().unwrap();
                let pass = e.get_password().unwrap();
                println!("Entry '{0}': '{1}' : '{2}'", title, user, pass);
            }
        }
    }

    Ok(())
}

fn test2() {
    
}