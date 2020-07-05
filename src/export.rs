//FIXME it just prints debug
pub fn export<T: std::fmt::Display + std::fmt::Debug>(fmt: &str, data: Vec<T>) {
    match fmt {
        "print" => println!("FIXME  {:?}",data ),
        _ => println!("export format not supported : {}", fmt),
    }
}
