//FIXME it just prints debug
use std::collections::HashMap;

pub fn export(fmt: &str, data: Vec<HashMap<String, String>>) {
    match fmt {
        "print" => println!("FIXME  {:?}",data ),
        _ => println!("export format not supported : {}", fmt),
    }
}