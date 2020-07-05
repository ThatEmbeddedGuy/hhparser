extern crate reqwest;
extern crate tokio;
extern crate tokio_io;

mod parser;
mod request;

const TITLE: &str = "C++";

async fn parse_hh_index() -> std::vec::Vec<impl std::future::Future> {
    let json = match request::get_page(0, TITLE)
        .await
        .and_then(parser::parse_json_own)
    {
        Some(value) => value,
        _ => return Vec::new(),
    };

    let _pages = parser::parse_num_of_pages(&json);
    println!("First page parsed, num of pages: {}", _pages);

    let index_pages = {
        let mut index_pages_futures = Vec::new();
        let mut page = 0;
        index_pages_futures.resize_with(_pages as usize, || {
            page += 1;
            request::get_page(page, TITLE)
        });
        index_pages_futures
    };

    index_pages
}

#[tokio::main]
async fn main() {
    println!("Started");
    let indexes =  parse_hh_index().await;
    //TODO filter already cached vacancies
    //TODO perform requests for new vacancies
    println!("Finished");
}
