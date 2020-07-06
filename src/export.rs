//FIXME it just prints debug
use std::collections::HashMap;
use std::fs::File;
use std::io::{self};

///Export vector of dictionaries (vacancies)
/// # Arguments
/// * `fmt`  - format  parameter.  values:
///         print - just prints to stdout
///         txt  - prints to data,txt file
///         json - prints to data.json in json format
/// * `filename`  - export filename used in txt/json formats:
/// * `data`  - slice of vacancies, represented in key/value:
pub fn export(fmt: &str, filename: &str, data: &[HashMap<String, String>]) {
    match fmt {
        "print" => {
            print(io::stdout(), &data)
                .unwrap_or_else(|err| println!("direct print export error: {}", err));
        }

        "txt" => {
            txt_file_export(filename, &data)
                .unwrap_or_else(|err| println!("txt file export error : {}", err));
        }

        "json" => json_file_export(filename, &data)
            .unwrap_or_else(|err| println!("json file export error : {}", err)),

        _ => println!("export format not supported : {}", fmt),
    }
}

fn txt_file_export(filename: &str, data: &[HashMap<String, String>]) -> std::io::Result<()> {
    let file = File::create(filename)?;
    print(file, data)?;
    Ok(())
}

fn json_file_export(filename: &str, data: &[HashMap<String, String>]) -> std::io::Result<()> {
    let file = File::create(filename)?;
    serde_json::to_writer(file, data)?;
    Ok(())
}

// Export generic function, can be used to print to any sink -  file/stdio
fn print<T: std::io::Write>(mut sink: T, data: &[HashMap<String, String>]) -> std::io::Result<()> {
    for map in data {
        for (key, value) in map {
            writeln!(sink, "{} = {}  ", key, value)?;
        }
        writeln!(sink, "\r\n")?;
    }
    Ok(())
}
