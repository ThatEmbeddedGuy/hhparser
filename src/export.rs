//FIXME it just prints debug
use std::collections::HashMap;
use std::fs::File;

pub fn export(fmt: &str, data: Vec<HashMap<String, String>>) {
    match fmt {
        "print" => print(&data),
        "txt" => {
            txt_file_export(data).unwrap_or_else(|err| println!("txt file export error : {}", err))
        }
        "json" => json_file_export(data)
            .unwrap_or_else(|err| println!("json file export error : {}", err)),
        _ => println!("export format not supported : {}", fmt),
    }
}

fn txt_file_export(data: Vec<HashMap<String, String>>) -> std::io::Result<()> {
    let mut file = File::create("data.txt")?;
    //TODO implement  txt file writer
    Ok(())
}

fn json_file_export(data: Vec<HashMap<String, String>>) -> std::io::Result<()> {
    let mut file = File::create("data.json")?;
    serde_json::to_writer(file, &data)?;
    Ok(())
}


fn print(data:  &Vec<HashMap<String, String>>)
{ 
    for map in data {
        print!("\r\n\r\n");
        for (key, value) in map {
            println!("{} = {}  ", key, value);
        }
        print!("\r\n\r\n");
    }

}
