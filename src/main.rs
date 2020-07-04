extern crate reqwest;
extern crate tokio;
extern crate tokio_io;

mod parser;
mod request;

const TITLE: &str = "C++";

async fn parse_hh_index() {
    if let Some(first_page) = request::get_page(0, TITLE).await {
        let _pages = parser::get_num_of_pages(&first_page);
    }
    // TODO add return value (list of vacancies)
}

#[tokio::main]
async fn main() {
    parse_hh_index().await;
    //TODO filter already cached vacancies
    //TODO perform requests for new vacancies
}
