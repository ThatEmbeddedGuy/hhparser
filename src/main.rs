extern crate reqwest;
extern crate tokio;
extern crate tokio_io;

mod parser;
mod request;

const TITLE: &str = "C++";

async fn parse_hh_index() {
    if let Some(json) = request::get_page(0, TITLE).await.and_then(parser::parse_json_own) {
        let _pages = parser::parse_num_of_pages(&json);
        println!("First page parsed, num of pages: {}", _pages);
    }
    // TODO add return value (list of vacancies)
}

#[tokio::main]
async fn main() {
    println!("Started");
    parse_hh_index().await;
    //TODO filter already cached vacancies
    //TODO perform requests for new vacancies
    println!("Finished");
}
